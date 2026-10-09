//! CORDIC 协处理器（同步 Blocking 驱动）
//!
//! 适配 PY32E407V1ET（单实例，`0x4002_C000`）。寄存器流程依据：
//! - HAL C: `py32e407_hal_cordic.c`（`HAL_CORDIC_Configure` / `HAL_CORDIC_Calculate` /
//!   `HAL_CORDIC_CalculateZO`）
//! - LL:    `py32e407_ll_cordic.h` / `py32e407_ll_bus.h`（CORDIC 在 `RCC_AHB1ENR`，bit5）
//! - RM:    PY32E407 参考手册 V0.6 第 43 章（43.2 编程流程、43.3.1 CSR）
//! - PAC:   `py32e407-pac/src/cordic/`（CSR / WDATA / RDATA 三个寄存器）
//!
//! 与 STM32G4 同名外设的差异（已对照 RM + PAC 交叉验证）：
//! - **精度上限 6 周期**：`CSR.PRECISION[3:0]` 字段虽为 4 bit（可写 0..15），但
//!   RM 表 43-14 只定义了 1..=6 周期的残差（1 周期 = 4 次迭代），HAL 的
//!   `IS_CORDIC_PRECISION` 也只放行 1..=6，故 [`Precision`] 枚举止于 [`Precision::Cycles6`]；
//! - CSR 复位值 `0x0000_0050`（FUNC=0 Cosine、PRECISION=5、SCALE=0）；
//! - **ARG2 未写入时默认 +1**（Q1.31 的 `0x7FFF_FFFF`，RM 43.2.6）——Cosine/Sine
//!   固定模长 m = 1 时可以不写 ARG2（`NbWrite::One`）。
//!
//! 仅实现同步（轮询 + 超时）驱动；不含中断（`CSR.IEN`）与 DMA（`CSR.DMAREN/DMAWEN`），
//! 这三位在初始化时显式清零。预留后续扩展 Async。
//!
//! # 数据格式（RM 43.2.3）
//!
//! 输入/输出统一用 `i32` 缓冲，数值语义由配置的 [`Width`] 决定：
//! - [`Width::Bits32`]：Q1.31 定点（`-1.0` = `0x8000_0000`，`1.0 - 2^-31` = `0x7FFF_FFFF`）；
//! - [`Width::Bits16`]：Q1.15 定点，**两个 16 位参数/结果打包在一个 32 位字内**
//!   （低半字在前，RM 43.2.6），此时 `NbWrite::One` / `NbRead::One` 即可覆盖双参数计算。
//!
//! 打包顺序与缩放处理（SCALE 因子）由调用侧负责，与 HAL C 语义一致。

use crate::pac;
use crate::peripherals::CORDIC;
use crate::Peri;

/// CORDIC 错误类型。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// 等待结果就绪标志（`CSR.RRDY`）超过上限仍未置位。
    Timeout,
    /// 传入配置非法（功能/精度越界，或缓冲长度与配置的读写字数不匹配）。
    InvalidConfig,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            Self::Timeout => "Operation Timed Out",
            Self::InvalidConfig => "Invalid Configuration",
        };
        f.write_str(msg)
    }
}

impl core::error::Error for Error {}

/// 计算功能选择（`CSR.FUNC[3:0]`，RM 表 43-1）。
///
/// 每个变体注明主/次参数（ARG1/ARG2）与主/次结果（RES1/RES2），
/// 角度统一用"圈"为单位（1.0 = 360°，Q1.31 表示 `[-1, 1)`）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Function {
    /// 余弦：ARG1=角度、ARG2=模长 m → RES1=m·cos(θ)、RES2=m·sin(θ)。
    Cosine,
    /// 正弦：ARG1=角度、ARG2=模长 m → RES1=m·sin(θ)、RES2=m·cos(θ)。
    Sine,
    /// 相位：ARG1=x、ARG2=y → RES1=atan2(y,x)、RES2=√(x²+y²)。
    Phase,
    /// 模长：ARG1=x、ARG2=y → RES1=√(x²+y²)、RES2=atan2(y,x)。
    Modulus,
    /// 反正切：ARG1=x·2⁻ⁿ → RES1=2⁻ⁿ·tan⁻¹x（SCALE=n 预缩放，结果需 ×2ⁿ）。
    Arctangent,
    /// 双曲余弦：ARG1=x·2⁻ⁿ → RES1=2⁻ⁿ·cosh x、RES2=2⁻ⁿ·sinh x（SCALE=n，常用 n=1）。
    HCosine,
    /// 双曲正弦：ARG1=x·2⁻ⁿ → RES1=2⁻ⁿ·sinh x、RES2=2⁻ⁿ·cosh x（SCALE=n，常用 n=1）。
    HSine,
    /// 反双曲正切：ARG1=x·2⁻ⁿ → RES1=2⁻ⁿ·atanh x（SCALE=n，常用 n=1）。
    HArctangent,
    /// 自然对数：ARG1=x·2⁻ⁿ → RES1=2⁻⁽ⁿ⁺¹⁾·ln x（SCALE=n，范围 1..=4，结果需 ×2ⁿ⁺¹）。
    NaturalLog,
    /// 平方根：ARG1=x·2⁻ⁿ → RES1=√x·2⁻ⁿ（SCALE=n，范围 0..=2）。
    SquareRoot,
}

impl Function {
    /// `CSR.FUNC[3:0]` 编码（RM 43.3.1 / HAL `CORDIC_FUNCTION_*`）。
    #[inline]
    fn to_bits(self) -> u8 {
        self as u8
    }
}

/// 计算精度（`CSR.PRECISION[3:0]`）。
///
/// 数值 = 迭代周期数，**1 周期 = 4 次CORDIC 迭代**；周期越多精度越高、耗时越长
/// （残差为 RM 表 43-14 的最坏值，q1.31 / q1.15 两列）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Precision {
    /// 1 周期（4 次迭代）：cos/sin/phase/mod/atan 残差 2⁻³；双曲族 2⁻²；sqrt 2⁻⁷。
    Cycles1,
    /// 2 周期（8 次迭代）：2⁻⁷ / 2⁻⁶ / 2⁻¹⁴。
    Cycles2,
    /// 3 周期（12 次迭代）：2⁻¹¹ / 2⁻¹⁰ / 2⁻¹⁹。
    Cycles3,
    /// 4 周期（16 次迭代）：2⁻¹⁵ / 2⁻¹³；sqrt 未定义（RM 表仅列到 3 周期）。
    Cycles4,
    /// 5 周期（20 次迭代）：2⁻¹⁸(2⁻¹⁶) / 2⁻¹⁷(2⁻¹⁵)；sqrt 未定义。
    Cycles5,
    /// 6 周期（24 次迭代）：2⁻¹⁹(2⁻¹⁶) / 2⁻¹⁸(2⁻¹⁵)；sqrt 未定义。
    Cycles6,
}

impl Precision {
    /// `CSR.PRECISION[3:0]` 编码：直接写周期数（RM 43.3.1；HAL `CORDIC_PRECISION_*`）。
    #[inline]
    fn to_bits(self) -> u8 {
        match self {
            Self::Cycles1 => 1,
            Self::Cycles2 => 2,
            Self::Cycles3 => 3,
            Self::Cycles4 => 4,
            Self::Cycles5 => 5,
            Self::Cycles6 => 6,
        }
    }
}

/// 缩放因子（`CSR.SCALE[2:0]`，RM 43.2.4）。
///
/// 硬件按 `2^-n` 缩放**输入**、按 `2^n` 缩放**输出**：输入先乘 2⁻ⁿ 送入（或等效地
/// 把结果乘 2ⁿ 取回），用于把超范围的实际值折进 Q1.31 可表示区间。
/// 各功能推荐的 n 见 [`Function`] 各变体与 RM 表 43-2..43-13。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Scale {
    /// n = 0（不缩放）。
    N0,
    /// n = 1（×2）。
    N1,
    /// n = 2（×4）。
    N2,
    /// n = 3（×8）。
    N3,
    /// n = 4（×16）。
    N4,
    /// n = 5（×32）。
    N5,
    /// n = 6（×64）。
    N6,
    /// n = 7（×128）。
    N7,
}

impl Scale {
    /// `CSR.SCALE[2:0]` 编码（HAL `CORDIC_SCALE_*`）。
    #[inline]
    fn to_bits(self) -> u8 {
        match self {
            Self::N0 => 0,
            Self::N1 => 1,
            Self::N2 => 2,
            Self::N3 => 3,
            Self::N4 => 4,
            Self::N5 => 5,
            Self::N6 => 6,
            Self::N7 => 7,
        }
    }
}

/// 数据位宽（`CSR.ARGSIZE` / `CSR.RESSIZE`，RM 43.2.3）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Width {
    /// 32 位（Q1.31）：每个参数/结果占一个完整的 32 位读写字。
    Bits32,
    /// 16 位（Q1.15）：**两个参数/结果打包在一个 32 位读写字内**（低半字在前），
    /// 此时一次计算只需 `NbWrite::One` / `NbRead::One`。
    Bits16,
}

/// 一次计算的 32 位写入次数（`CSR.NARGS`，HAL `CORDIC_NBWRITE_*`）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NbWrite {
    /// 一次 32 位写：仅 ARG1（Q1.31），或打包在一起的 ARG1+ARG2（Q1.15）。
    ///
    /// 注意 Q1.31 下 Cosine/Sine 不写 ARG2 时，硬件按默认值 +1（模长 1）计算。
    One,
    /// 两次 32 位写：依次写 ARG1、ARG2（仅 Q1.31）。
    Two,
}

/// 一次计算的 32 位读取次数（`CSR.NRES`，HAL `CORDIC_NBREAD_*`）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NbRead {
    /// 一次 32 位读：仅 RES1（Q1.31），或打包在一起的 RES1+RES2（Q1.15）。
    One,
    /// 两次 32 位读：依次读 RES1、RES2（仅 Q1.31）。
    Two,
}

/// CORDIC 计算配置（字段对齐 HAL 的 `CORDIC_ConfigTypeDef`）。
///
/// `#[non_exhaustive]`，外部构造请用 [`Config::default()`] 后逐字段赋值。
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Config {
    /// 计算功能。
    pub function: Function,
    /// 迭代精度（周期数）。
    pub precision: Precision,
    /// 缩放因子。
    pub scale: Scale,
    /// 输入数据位宽（`CSR.ARGSIZE`）。
    pub in_width: Width,
    /// 输出数据位宽（`CSR.RESSIZE`）。
    pub out_width: Width,
    /// 一次计算的写入次数（`CSR.NARGS`）。
    pub nb_write: NbWrite,
    /// 一次计算的读取次数（`CSR.NRES`）。
    pub nb_read: NbRead,
}

impl Default for Config {
    /// 默认配置 = CSR 复位值语义（Cosine、5 周期、不缩放、Q1.31、单写单读——
    /// Cosine 不写 ARG2 时模长按 1 计算）。
    fn default() -> Self {
        Self {
            function: Function::Cosine,
            precision: Precision::Cycles5,
            scale: Scale::N0,
            in_width: Width::Bits32,
            out_width: Width::Bits32,
            nb_write: NbWrite::One,
            nb_read: NbRead::One,
        }
    }
}

/// 等待 `CSR.RRDY` 的轮询上限（循环次数）。
///
/// 量级依据：单次计算最多 6 周期 × 4 迭代 = 24 次迭代，CORDIC 挂在 AHB 上、
/// 每迭代约 1 个时钟，合计 < 100 周期；`100_000` 次循环 ≈ 1 ms @ 80 MHz，
/// 裕量约 1000 倍。
const TIMEOUT_LOOPS: u32 = 100_000;

/// CORDIC 协处理器驱动句柄。
///
/// 持有外设单例的所有权（单实例，不存在第二个驱动并发使用的可能）。
/// `Drop` 不关闭时钟——与 crate 内其他驱动一致，时钟常开。
pub struct Cordic<'d> {
    _peri: Peri<'d, CORDIC>,
    /// 当前生效的配置快照（[`Self::configure`] 会更新），用于 `calculate` 校验缓冲长度。
    config: Config,
}

impl<'d> Cordic<'d> {
    /// 创建驱动并完成初始化（阻塞）。
    ///
    /// 流程对照 `py32e407_ll_bus.h`（时钟）与 `HAL_CORDIC_Configure`（CSR）：
    /// 1. 使能时钟（`RCC_AHB1ENR.CORDICEN`），置位后读回一次（LL 的
    ///    "Delay after an RCC peripheral clock enabling" 规则）；
    /// 2. 复位外设（`RCC_AHB1RSTR.CORDICRST` 置位后清零），保证寄存器处于复位默认
    ///    （CSR = 0x0000_0050）；
    /// 3. 写 CSR 控制字段（FUNC/PRECISION/SCALE/NARGS/NRES/ARGSIZE/RESSIZE），
    ///    并显式清零 IEN / DMAREN / DMAWEN（同步驱动不使用中断与 DMA）。
    ///
    /// # Errors
    ///
    /// 配置非法时返回 [`Error::InvalidConfig`]（当前实现下各枚举已保证合法，
    /// 校验保留以对齐 HAL 的 `assert_param` 语义）。
    pub fn new_blocking(peri: Peri<'d, CORDIC>, config: Config) -> Result<Self, Error> {
        let rcc = unsafe { &*pac::Rcc::PTR };
        let p = unsafe { &*pac::Cordic::PTR };

        // 1. 使能 AHB1 时钟 + 读回（py32e407_ll_bus.h：LL_AHB1_GRP1_EnableClock）
        rcc.ahb1enr().modify(|_, w| w.cordicen().set_bit());
        let _ = rcc.ahb1enr().read();

        // 2. 复位外设（Force + Release，对齐 crate 内 timer 驱动的做法）
        rcc.ahb1rstr().modify(|_, w| w.cordicrst().set_bit());
        rcc.ahb1rstr().modify(|_, w| w.cordicrst().clear_bit());
        let _ = rcc.ahb1rstr().read();

        // 3. 写 CSR（HAL_CORDIC_Configure 的 MODIFY_REG 字段集 + 显式关中断/DMA）
        write_csr(p, &config);

        Ok(Self { _peri: peri, config })
    }

    /// 运行中更换配置（对齐 `HAL_CORDIC_Configure`）。
    ///
    /// 直接改写 CSR 控制字段。若上一笔计算的结果尚未读出（RRDY 仍置位），
    /// 改配置后该结果作废——与 HAL 一致，不做额外同步。
    pub fn configure(&mut self, config: Config) {
        let p = unsafe { &*pac::Cordic::PTR };
        write_csr(p, &config);
        self.config = config;
    }

    /// 当前生效的配置快照。
    #[inline]
    pub fn config(&self) -> Config {
        self.config
    }

    /// 轮询模式计算（对齐 `HAL_CORDIC_Calculate`，RM 43.2.7）。
    ///
    /// 序列：写 WDATA（触发计算）→ 轮询 `CSR.RRDY` → 读 RDATA（读回自动清 RRDY）。
    ///
    /// * `input` — 输入缓冲，长度必须是 `nb_write` 的整数倍（Q1.15 打包时每字含两个参数）；
    /// * `output` — 输出缓冲，长度必须是 `nb_read` 的整数倍，且计算次数与 `input` 一致。
    ///
    /// # Errors
    ///
    /// - 缓冲长度与配置不匹配：[`Error::InvalidConfig`]；
    /// - 任一结果等待超时：[`Error::Timeout`]。
    pub fn calculate(&mut self, input: &[i32], output: &mut [i32]) -> Result<(), Error> {
        let p = unsafe { &*pac::Cordic::PTR };
        let nw = nb_write_words(&self.config);
        let nr = nb_read_words(&self.config);

        if input.is_empty() || input.len() % nw != 0 || output.len() % nr != 0
            || input.len() / nw != output.len() / nr
        {
            return Err(Error::InvalidConfig);
        }

        for (args, res) in input.chunks_exact(nw).zip(output.chunks_exact_mut(nr)) {
            // 写入本次计算的全部参数（最后一笔 WDATA 写入触发计算开始）
            for &v in args {
                p.wdata().write(|w| unsafe { w.bits(v as u32) });
            }

            // 轮询 RRDY（RM 43.2.7：结果就绪标志）
            let mut ready = false;
            for _ in 0..TIMEOUT_LOOPS {
                if p.csr().read().rrdy().bit_is_set() {
                    ready = true;
                    break;
                }
            }
            if !ready {
                return Err(Error::Timeout);
            }

            // 读回全部结果（读 RDATA 自动清 RRDY）
            for r in res {
                *r = p.rdata().read().bits() as i32;
            }
        }

        Ok(())
    }

    /// 零开销模式计算（对齐 `HAL_CORDIC_CalculateZO`，RM 43.2.6）。
    ///
    /// 写完参数后**不等待 RRDY**、立即读 RDATA——硬件在总线上自动等待结果就绪
    /// （AHB 读停顿），省去一次标志轮询，适合连续批量计算。缓冲长度要求同
    /// [`Self::calculate`]。
    ///
    /// # Errors
    ///
    /// 缓冲长度与配置不匹配：[`Error::InvalidConfig`]。
    ///
    /// # 注意
    ///
    /// 若配置与写入的参数个数不符，读侧总线停顿可能持续到配置匹配为止
    /// （RM 未定义该情形下的超时行为）——配置错误请在开发期用
    /// [`Self::calculate`] 排查，确认无误后再切换本模式。
    pub fn calculate_zo(&mut self, input: &[i32], output: &mut [i32]) -> Result<(), Error> {
        let p = unsafe { &*pac::Cordic::PTR };
        let nw = nb_write_words(&self.config);
        let nr = nb_read_words(&self.config);

        if input.is_empty() || input.len() % nw != 0 || output.len() % nr != 0
            || input.len() / nw != output.len() / nr
        {
            return Err(Error::InvalidConfig);
        }

        for (args, res) in input.chunks_exact(nw).zip(output.chunks_exact_mut(nr)) {
            for &v in args {
                p.wdata().write(|w| unsafe { w.bits(v as u32) });
            }
            // 不轮询 RRDY，直接读（RM 43.2.6：读侧总线等待结果就绪）
            for r in res {
                *r = p.rdata().read().bits() as i32;
            }
        }

        Ok(())
    }
}

/// 按 [`Config`] 写 CSR 控制字段（HAL_CORDIC_Configure 的 MODIFY_REG 字段集）。
fn write_csr(p: &pac::cordic::RegisterBlock, config: &Config) {
    let two_args = config.nb_write == NbWrite::Two;
    let two_res = config.nb_read == NbRead::Two;
    p.csr().modify(|_, w| unsafe {
        w.func().bits(config.function.to_bits());
        w.precision().bits(config.precision.to_bits());
        w.scale().bits(config.scale.to_bits());
        w
    });
    // 字段间没有位重叠，合并在一次 modify 里会因闭包借用冲突拆成两步
    p.csr().modify(|_, w| {
        let w = if two_args {
            w.nargs().set_bit()
        } else {
            w.nargs().clear_bit()
        };
        let w = if two_res {
            w.nres().set_bit()
        } else {
            w.nres().clear_bit()
        };
        let w = if config.in_width == Width::Bits16 {
            w.argsize().set_bit()
        } else {
            w.argsize().clear_bit()
        };
        let w = if config.out_width == Width::Bits16 {
            w.ressize().set_bit()
        } else {
            w.ressize().clear_bit()
        };
        // 同步驱动不使用中断与 DMA：显式清零（复位默认即 0，这里保证幂等）
        let w = w.ien().clear_bit();
        let w = w.dmaren().clear_bit();
        w.dmawen().clear_bit()
    });
}

/// 一次计算的 32 位写入字数（1 或 2）。
#[inline]
fn nb_write_words(config: &Config) -> usize {
    match config.nb_write {
        NbWrite::One => 1,
        NbWrite::Two => 2,
    }
}

/// 一次计算的 32 位读取字数（1 或 2）。
#[inline]
fn nb_read_words(config: &Config) -> usize {
    match config.nb_read {
        NbRead::One => 1,
        NbRead::Two => 2,
    }
}
