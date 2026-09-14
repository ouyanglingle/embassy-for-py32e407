//! TIM 定时器（TIM1–TIM19）时基（同步 Blocking 驱动）
//!
//! 适配 PY32E407V1ET。寄存器流程依据：
//! - HAL C: `py32e407_hal_tim.c`（`HAL_TIM_Base_Init` → `TIM_Base_SetConfig`，第 7112 行起）
//! - LL:    `py32e407_ll_tim.h` / 官方例程 `Example_LL/TIM/TIM1_TimeBase_Init`
//! - RM:    PY32E407 参考手册 V0.6 第 19–24 章（高级 TIM1/8、通用 TIM2–5/18、
//!          TIM9/12、TIM10/11/13/14/19、TIM15/16/17、基本 TIM6/7）与第 6 章（RCC 时钟树）
//!
//! 本模块实现**时基**功能（PSC/ARR/CNT/更新事件、更新中断 `DIER.UIE`）；
//! PWM 输出见子模块 [`pwm`]，正交编码器见 [`qei`]。不含输入捕获与 DMA/Async。
//!
//! 周期/频率可用 [`Timer::set_period_ms`] / [`Timer::set_period_us`] /
//! [`Timer::set_period_secs`] / [`Timer::set_frequency`] 配合 [`RoundTo`] 直接设置；
//! 中断由 [`Timer::enable_update_interrupt`] 使能，服务函数内用
//! [`Timer::clear_update_interrupt`] 清标志。
//!
//! # 实例与寄存器块（PAC 权威，py32e407-pac/src/lib.rs）
//!
//! | 寄存器块 | 实例 | 总线/使能位 | 计数器位宽 | RCR |
//! |---|---|---|---|---|
//! | `tim1`  | TIM1（APB2 bit11）、TIM8（bit13） | APB2ENR | 16 | 16 位 |
//! | `tim2`  | TIM2（APB1 bit0）、TIM3/4/5（bit1–3）、TIM18（bit9） | APB1ENR1 | TIM2=32，其余 16 | 无 |
//! | `tim6`  | TIM6（bit4）、TIM7（bit5） | APB1ENR1 | 16 | 无 |
//! | `tim9`  | TIM9（APB2 bit19）、TIM12（APB1 bit6） | 见前 | 16 | 8 位 |
//! | `tim10` | TIM10（APB2 bit20）、TIM11（21）、TIM13（APB1 bit7）、TIM14（bit8）、TIM19（APB2 bit25） | 见前 | 16 | 无 |
//! | `tim15` | TIM15（APB2 bit22） | APB2ENR | 16 | 8 位 |
//! | `tim16` | TIM16（APB2 bit23）、TIM17（bit24） | APB2ENR | 16 | 8 位 |
//!
//! 注意 PY32E407 的 `RCC_APB2ENR` 定时器位号与 STM32 **完全不同**
//! （TIM1 = bit11 而非 bit0，TIM15/16/17 = 22/23/24 而非 16/17/18）。
//!
//! # 与 STM32 同名外设的关键差异（已用 RM + SVD/PAC 交叉验证）
//!
//! - **仅 TIM2 是 32 位计数器**（RM 20.1.1："32 位（TIM2）或 16 位（TIM3/4/5/18）"）。
//!   TIM3/4/5/18 虽共用 tim2 寄存器块，硬件上仍是 16 位——`ARR > 0xFFFF` 只对 TIM2 合法。
//! - **RCR 支持面比 STM32F4 宽**：除 TIM1/TIM8 外，TIM9/12/15/16/17 也有 RCR
//!   （但为 8 位；TIM1/TIM8 为 16 位）。TIM2–7/10/11/13/14/18/19 没有 RCR。
//! - **CR1 位域随块裁剪**：TIM6/7 的 CR1 没有 DIR/CMS/CKD；TIM9–19 大多没有
//!   DIR/CMS。PY32 还在部分块上增加了 UIFREMAP/DITHEN 位（复位值 0，本驱动不使用）。
//! - **定时器内核时钟 x1/x2 规则**（RM 第 6 章时钟树 "x1/x2" 分支）：APB 预分频
//!   为 1 时 TIMCLK = PCLK，否则 TIMCLK = 2 × PCLK。官方例程
//!   `Example_LL/TIM/TIM1_TimeBase_Init`（PCLK2 = HCLK/2 = 8 MHz 时 PSC 取
//!   `SystemCoreClock/1000 - 1` 得 1 Hz 更新）证实了 ×2 行为。
//!
//! # 使用示例
//!
//! ```ignore
//! use embassy_py32::{init, rcc, timer::{Config, RoundTo, Timer}};
//!
//! let p = init(Default::default());
//! // TIM6：1 Hz 更新事件，阻塞轮询
//! let mut tim = Timer::new_blocking(p.TIM6, Config::default()).unwrap();
//! tim.set_period_ms(1000, RoundTo::Slower);
//! tim.start();
//! loop {
//!     tim.wait_for_update().unwrap(); // 每次更新事件返回
//! }
//! ```
//!
//! 更新中断用法（对齐 embassy-stm32 `timer::low_level`）：
//!
//! ```ignore
//! let tim7 = Timer::new_blocking(p.TIM7, Config::default()).unwrap();
//! tim7.set_period_ms(10, RoundTo::Slower);
//! tim7.enable_update_interrupt(true);
//! tim7.set_autoreload_preload(true);
//! tim7.start();
//! unsafe { cortex_m::peripheral::NVIC::unmask(embassy_py32::pac::Interrupt::TIM7) };
//!
//! #[interrupt]
//! fn TIM7() {
//!     // 清 SR.UIF，否则不会收到下一次中断
//!     let tim7 = unsafe { &*embassy_py32::pac::Tim7::PTR };
//!     tim7.sr().write(|w| w.uif().clear_bit());
//! }
//! ```
//!
//! # 命名约定
//!
//! 构造函数为 [`Timer::new_blocking`]，`new` 留给后续 Async 版本；
//! 其余方法直接用动词名（`start`/`stop`/`wait_for_update`）。
//!
//! # 子模块
//!
//! - [`pwm`]：同步 PWM 输出（边沿对齐、单通道使能/占空比/频率）。
//! - [`qei`]：同步正交编码器接口（模式/极性/滤波/预分频，位置与方向读取）。

pub mod pwm;
pub mod qei;

pub use pwm::{Ch1, Ch2, Ch3, Ch4, Channel, Polarity, Pwm, PwmPin};
pub use qei::{Direction, EncoderInstance, Prescaler, Qei, QeiMode};

use crate::pac;
use crate::{Hertz, Peri, PeripheralType};

/// 轮询等待更新标志的循环上限（次数）。
///
/// 量级依据：HSI16 下每次循环约 4~6 个周期，`1_000_000` 次 ≈ 0.25~0.4 s。
/// 更新周期超过该上限时（如 1 Hz），请在调用侧循环调用 [`Timer::wait_for_update`]
/// 并在每轮超时后重试，或改用更低的轮询频率。
const TIMEOUT_LOOPS: u32 = 1_000_000;

/// TIM 错误类型。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// 等待更新标志（`SR.UIF`）超过 [`TIMEOUT_LOOPS`] 仍未置位。
    Timeout,
    /// 配置非法：`ARR` 超出该实例计数器位宽、`RCR` 超出位宽，或
    /// [`Config::from_frequency`] 的目标频率超出可达范围。
    InvalidConfig,
    /// PWM 通道号超出该实例的通道数（见 [`pwm`] 的通道数表）。
    InvalidChannel,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            Self::Timeout => "Operation Timed Out",
            Self::InvalidConfig => "Invalid Configuration",
            Self::InvalidChannel => "Invalid PWM Channel",
        };
        f.write_str(msg)
    }
}

impl core::error::Error for Error {}

/// 周期/频率换算时的取整方向（对齐 embassy-stm32 `timer::low_level::RoundTo`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundTo {
    /// 向更慢的方向取整（周期更长、频率更低）：实际周期 **≥** 请求值。
    Slower,
    /// 向更快的方向取整（周期更短、频率更高）：实际周期 **≤** 请求值。
    Faster,
}

/// 按取整方向做除法（对齐 embassy-stm32 的同名辅助函数）。
#[inline]
fn div_round(numerator: u64, denominator: u64, round: RoundTo) -> u64 {
    match round {
        RoundTo::Faster => numerator / denominator,
        RoundTo::Slower => numerator.div_ceil(denominator),
    }
}

/// 由目标周期（CK_INT 周期数）计算 `(PSC, ARR)`。
///
/// 算法逐位对照 embassy-stm32 `timer/low_level.rs` 的 `calculate_psc_arr`：
/// 取满足 `(psc + 1) × (arr + 1)` 尽量逼近 `period_clocks` 的最小预分频；
/// `ARR` 合法范围为 `1..=max_arr`（`ARR = 0` 非法）。
///
/// `max_arr_bits` 为该实例计数器位宽（16 或 32）。无法满足请求（如
/// `RoundTo::Faster` 下周期不足 2 个 CK_INT，或 `RoundTo::Slower` 下 PSC 溢出）
/// 时返回 [`None`]。
fn calculate_psc_arr(period_clocks: u64, round: RoundTo, max_arr_bits: usize) -> Option<(u16, u32)> {
    let max_arr: u64 = if max_arr_bits >= 32 {
        0xFFFF_FFFF
    } else {
        (1u64 << max_arr_bits) - 1
    };

    // ARR = 0 非法，故最小周期为 (psc=0, arr=1) = 2 个 CK_INT
    const MIN_PERIOD: u64 = 2;
    if round == RoundTo::Faster && period_clocks < MIN_PERIOD {
        return None;
    }

    // 需要 (psc + 1) × (arr + 1) ≈ period_clocks：取最小 psc
    let psc_min = period_clocks.saturating_sub(1) / (max_arr + 1);
    let psc: u16 = match u16::try_from(psc_min) {
        Ok(v) => v,
        Err(_) => match round {
            // Slower：需要实际周期 ≥ 请求值，PSC 溢出即无法满足
            RoundTo::Slower => return None,
            // Faster：只要求实际周期 ≤ 请求值，PSC 取满即可
            RoundTo::Faster => u16::MAX,
        },
    };

    let psc_plus_1 = u64::from(psc) + 1;
    let arr = match round {
        RoundTo::Faster => (period_clocks / psc_plus_1).saturating_sub(1),
        RoundTo::Slower => period_clocks.div_ceil(psc_plus_1).saturating_sub(1),
    };
    let arr = arr.clamp(1, max_arr);

    Some((psc, arr as u32))
}

/// TIM 时基配置（对应 `TIM_Base_InitTypeDef`，`py32e407_hal_tim.h`）。
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Config {
    /// 预分频器（`PSC`）。实际分频 = `prescaler + 1`，范围 0..=0xFFFF。
    pub prescaler: u16,
    /// 自动重装载值（`ARR`）。更新事件周期 = `(prescaler + 1) × (period + 1)` 个
    /// CK_INT 周期。16 位实例须 ≤ 0xFFFF，TIM2 可到 0xFFFF_FFFF。
    pub period: u32,
    /// 自动重装载预装载（`CR1.ARPE`）：true 时写 `period` 在下个更新事件才生效。
    pub auto_reload_preload: bool,
    /// 单脉冲模式（`CR1.OPM`）：true 时计数器启动并在产生一次更新事件后自动停止。
    pub one_pulse: bool,
    /// 更新请求源（`CR1.URS`）：true 时仅计数器溢出/下溢产生更新事件，
    /// 软件 `UG` 与从模式事件不再置 `UIF`（仍会重装影子寄存器）。
    pub update_request_source: bool,
    /// 重复计数器（`RCR`）。仅 TIM1/TIM8（16 位）与 TIM9/12/15/16/17（8 位）
    /// 支持；每 `repetition + 1` 次溢出才产生一次更新事件。
    /// 其余实例该字段被忽略（写入为空操作）。
    pub repetition: u16,
}

impl Config {
    /// 按目标更新频率计算 `prescaler` / `period`（其余字段取默认）。
    ///
    /// 满足 `f = clk / ((prescaler + 1) × (period + 1))`，采用先分频到 16 位
    /// ARR 可表达范围的算法：
    /// `prescaler = ceil(div / 65536) - 1`，`period = div / (prescaler + 1) - 1`，
    /// 其中 `div = clk / f`（四舍五入）。结果 `period ≤ 0xFFFF`，对全部实例合法；
    /// TIM2 若想用更低的频率可在此基础上手工加大 `period`（32 位）。
    ///
    /// # Errors
    ///
    /// `clk`/`f` 为 0、`f > clk`，或所需 `prescaler` 超过 16 位时返回
    /// [`Error::InvalidConfig`]。
    pub fn from_frequency(clk: Hertz, f: Hertz) -> Result<Self, Error> {
        if clk.0 == 0 || f.0 == 0 {
            return Err(Error::InvalidConfig);
        }
        // 四舍五入取分频比，相对误差 < 1/2 个输出周期
        let div = (clk.0 + f.0 / 2) / f.0;
        if div == 0 {
            return Err(Error::InvalidConfig); // f > clk，不可达
        }
        let psc = ((div - 1) / 0x1_0000) as u32;
        if psc > 0xFFFF {
            return Err(Error::InvalidConfig); // f 太低，超出 (psc+1)×(arr+1) 表达范围
        }
        let arr = div / (psc + 1) - 1;
        Ok(Self {
            prescaler: psc as u16,
            period: arr,
            ..Self::default()
        })
    }
}

impl Default for Config {
    /// 默认值：PSC = 0、ARR = 0xFFFF（16 位最大周期），其余关闭——
    /// 与 HAL `TIM_Base_InitTypeDef` 的复位默认（全 0）相比，仅把 `period`
    /// 取为 16 位实例的上限以避免"周期为 1 个 CK_INT"的意外配置。
    fn default() -> Self {
        Self {
            prescaler: 0,
            period: 0xFFFF,
            auto_reload_preload: false,
            one_pulse: false,
            update_request_source: false,
            repetition: 0,
        }
    }
}

/// 定时器所在 APB 总线（决定 RCC 使能/复位寄存器与时钟源）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Bus {
    /// APB1（TIM2–7、TIM12–14、TIM18）。
    Apb1,
    /// APB2（TIM1/8/9/10/11/15–17/19）。
    Apb2,
}

mod sealed {
    use super::Bus;
    use crate::pac;

    /// 各 TIM 寄存器块（`tim1::RegisterBlock` … `tim16::RegisterBlock`）统一的
    /// 时基寄存器操作集。
    ///
    /// 七类寄存器块在 svd2rust 下是不同的命名类型，无法用单一类型表达；
    /// 本 trait 把时基所需的寄存器操作归一化，具体实现里全部通过
    /// `py32e407-pac` 的类型化 API 访问（不裸写地址）。
    pub(super) trait SealedBlock {
        /// 写 `CR1`：清 `UDIS`（更新事件必须保持使能，轮询 `UIF` 依赖它）与
        /// 保留/方向位，配置 `OPM`/`URS`/`ARPE`，`CEN` 按参数。
        /// 各块可用的位域不同（见模块级文档），由实现各自用类型化字段完成。
        fn cr1_write(&self, cen: bool, arpe: bool, opm: bool, urs: bool);

        /// 置/清 `CR1.CEN`（`__HAL_TIM_ENABLE` / `__HAL_TIM_DISABLE`）。
        fn cr1_set_cen(&self, on: bool);

        /// 读 `CR1.CEN`。
        fn cr1_cen(&self) -> bool;

        /// 写 `ARR`（自动重装载值）。
        fn arr_write(&self, val: u32);

        /// 写 `PSC`（预分频值）。
        fn psc_write(&self, val: u16);

        /// 写 `RCR`（重复计数器）。无 RCR 的寄存器块实现为空操作。
        fn rcr_write(&self, val: u16);

        /// 置 `EGR.UG`，软件产生更新事件以立即装载 PSC/ARR/RCR 影子寄存器
        /// （`TIM_Base_SetConfig` 的收尾序列，py32e407_hal_tim.c:7149）。
        fn egr_ug(&self);

        /// 读 `SR.UIF`（更新事件标志）。
        fn sr_uif(&self) -> bool;

        /// 清 `SR.UIF`（写 0 清除）。
        fn sr_uif_clear(&self);

        /// 读 `CNT` 当前计数值。TIM2 的 bit31 是 UIFREMAP 位，实现须屏蔽。
        fn cnt_read(&self) -> u32;

        /// 写 `CNT`（复位计数用）。
        fn cnt_write(&self, val: u32);

        /// 读 `ARR`（自动重装载值）。
        fn arr_read(&self) -> u32;

        /// 读 `PSC`（预分频值）。
        fn psc_read(&self) -> u32;

        /// 置/清 `DIER.UIE`（更新中断使能）。
        fn uie_set(&self, on: bool);

        /// 置/清 `CR1.ARPE`（自动重装载预装载）。
        fn arpe_set(&self, on: bool);
    }

    /// 单个 TIM 实例（`TIM1`…`TIM19`）的静态信息。
        pub(super) trait SealedInstance {
        /// 返回该实例的寄存器块（归一化操作集）。
        fn regs(&self) -> &'static dyn SealedBlock;

        /// 所在 APB 总线。
        fn bus(&self) -> Bus;

        /// RCC 使能位掩码（`APB1ENR1` / `APB2ENR` 中该定时器位）。
        fn enable_mask(&self) -> u32;

        /// 计数器位宽是否为 32 位（仅 TIM2，RM 20.1.1）。
        fn is_32bit(&self) -> bool;

        /// `RCR` 最大值；无 RCR 的实例返回 `None`。
        fn max_repetition(&self) -> Option<u32>;

        /// 使能定时器时钟（复用 `rcc` 模块的 APB 使能函数，内部已复刻
        /// LL "置位后读回一次" 的延迟语义）。
        fn enable_clock(&self) {
            match self.bus() {
                Bus::Apb1 => crate::rcc::enable_apb1(self.enable_mask(), 0),
                Bus::Apb2 => crate::rcc::enable_apb2(self.enable_mask()),
            }
        }

        /// 复位定时器（Force + Release，恢复寄存器复位默认值）。
        ///
        /// # Safety
        ///
        /// 复位会立即停止该定时器；调用前必须确认其未被其他代码使用。
        unsafe fn reset(&self) {
            match self.bus() {
                // Safety: 本函数即 unsafe，调用方已保证外设未被使用
                Bus::Apb1 => unsafe { crate::rcc::reset_apb1(self.enable_mask(), 0) },
                Bus::Apb2 => unsafe { crate::rcc::reset_apb2(self.enable_mask()) },
            }
        }

        /// 定时器内核时钟 CK_INT（Hz）。
        ///
        /// RM 第 6 章时钟树：APB 预分频为 1 时 TIMCLK = PCLK，否则
        /// TIMCLK = 2 × PCLK（"x1/x2" 分支）。官方例程
        /// `Example_LL/TIM/TIM1_TimeBase_Init`（PCLK2 = HCLK/2，PSC 按
        /// `SystemCoreClock/1000 - 1` 计算）证实 ×2 行为。
        ///
        /// 预分频编码见 RM 6.4.1 `CFGR.PPRE1/PPRE2`：`0xx` = 不分频。
        fn timer_clk(&self) -> u32 {
            let rcc = unsafe { &*pac::Rcc::PTR };
            let cfgr = rcc.cfgr().read();
            let clocks = crate::rcc::clocks();
            match self.bus() {
                Bus::Apb1 => {
                    clocks.apb1.0 * if cfgr.ppre1().bits() & 0b100 == 0 { 1 } else { 2 }
                }
                Bus::Apb2 => {
                    clocks.apb2.0 * if cfgr.ppre2().bits() & 0b100 == 0 { 1 } else { 2 }
                }
            }
        }
    }
}

/// CR1 写入：带 `DIR`/`CMS`/`CKD` 的块（tim1 = TIM1/TIM8、tim2 = TIM2–5/18）。
macro_rules! cr1_write_full {
    ($regs:expr, $cen:expr, $arpe:expr, $opm:expr, $urs:expr) =>  {{
        $regs.cr1().write(|w| unsafe {
            w.cen()
                .bit($cen)
                .udis()
                .clear_bit()
                .urs()
                .bit($urs)
                .opm()
                .bit($opm)
                .dir()
                .clear_bit()
                .cms()
                .bits(0)
                .arpe()
                .bit($arpe)
                .ckd()
                .bits(0)
        });
    }};
}

/// CR1 写入：无 `DIR`/`CMS` 的块（tim9/tim10/tim15/tim16 族）。
macro_rules! cr1_write_plain {
    ($regs:expr, $cen:expr, $arpe:expr, $opm:expr, $urs:expr) =>  {{
        $regs.cr1().write(|w| unsafe {
            w.cen()
                .bit($cen)
                .udis()
                .clear_bit()
                .urs()
                .bit($urs)
                .opm()
                .bit($opm)
                .arpe()
                .bit($arpe)
                .ckd()
                .bits(0)
        });
    }};
}

/// CR1 写入：基本定时器（tim6 = TIM6/TIM7，CR1 无 DIR/CMS/CKD）。
macro_rules! cr1_write_basic {
    ($regs:expr, $cen:expr, $arpe:expr, $opm:expr, $urs:expr) =>  {{
        $regs.cr1().write(|w| {
            w.cen()
                .bit($cen)
                .udis()
                .clear_bit()
                .urs()
                .bit($urs)
                .opm()
                .bit($opm)
                .arpe()
                .bit($arpe)
        });
    }};
}

/// 各块公共的时基寄存器操作（在 `impl SealedBlock` 体内展开）。
macro_rules! common_ops {
    () => {
        #[inline]
        fn cr1_set_cen(&self, on: bool) {
            self.cr1().modify(|_, w| w.cen().bit(on));
        }
        #[inline]
        fn cr1_cen(&self) -> bool {
            self.cr1().read().cen().bit_is_set()
        }
        #[inline]
        fn arr_write(&self, val: u32) {
            self.arr().write(|w| unsafe { w.bits(val as _) });
        }
        #[inline]
        fn psc_write(&self, val: u16) {
            // tim2（TIM2–5/18）的 PSC 在 SVD 中为 32 位寄存器，W.bits 为 u32；
            // 其余块为 16 位，用 `as _` 统一适配
            self.psc().write(|w| unsafe { w.bits(val as _) });
        }
        /// 置 `EGR.UG`（bit0，W，RM 19.3.6/20.3.6 位表），软件产生更新事件以
        /// 立即装载 PSC/ARR/RCR 影子寄存器（`TIM_Base_SetConfig` 收尾序列，
        /// py32e407_hal_tim.c:7149）。
        ///
        /// PAC 的 EGR 未生成 W 侧字段方法（SVD 中各位为 write-only），故按
        /// RM 位表写位模式：仅置 bit0（UG）；其余 CCxG/BG 等位同为"置 1 触发"
        /// 语义，写 0 无动作。
        #[inline]
        fn egr_ug(&self) {
            self.egr().write(|w| unsafe { w.bits(1) });
        }
        #[inline]
        fn sr_uif(&self) -> bool {
            self.sr().read().uif().bit_is_set()
        }
        #[inline]
        fn sr_uif_clear(&self) {
            self.sr().write(|w| w.uif().clear_bit());
        }
        #[inline]
        fn cnt_write(&self, val: u32) {
            // TIM2 的 CNT bit31 为 UIFREMAP（RM 20.4.10），写 0 无副作用
            self.cnt().write(|w| unsafe { w.bits(val as _) });
        }
        #[inline]
        fn arr_read(&self) -> u32 {
            // tim1/tim2/tim15/tim16 的 ARR 为 32 位寄存器，tim6/tim9/tim10 为 16 位；
            // 用 `as _` 统一收敛到 u32
            self.arr().read().bits() as _
        }
        #[inline]
        fn psc_read(&self) -> u32 {
            // tim2（TIM2–5/18）的 PSC 在 SVD 中为 32 位，其余为 16 位
            self.psc().read().bits() as _
        }
        #[inline]
        fn uie_set(&self, on: bool) {
            self.dier().modify(|_, w| w.uie().bit(on));
        }
        #[inline]
        fn arpe_set(&self, on: bool) {
            self.cr1().modify(|_, w| w.arpe().bit(on));
        }
    };
}

/// 为一个 PAC 寄存器块实现 [`sealed::SealedBlock`]。
///
/// 参数：块模块名、CR1 变体（full = 带 DIR/CMS，plain = 无 DIR/CMS，
/// basic = 基本定时器）、RCR 形态（yes16 = tim9、yes32 = tim15/16、no = 无）。
/// TIM2 的 CNT 读法特殊（bit31 是 UIFREMAP 位），由 `no_tim2` 变体单独处理。
macro_rules! impl_sealed_block {
    ($pacmod:ident, full, yes) => {
        impl sealed::SealedBlock for pac::$pacmod::RegisterBlock {
            #[inline]
            fn cr1_write(&self, cen: bool, arpe: bool, opm: bool, urs: bool) {
                cr1_write_full!(self, cen, arpe, opm, urs)
            }
            #[inline]
            fn rcr_write(&self, val: u16) {
                self.rcr().write(|w| unsafe { w.bits(val) });
            }
            #[inline]
            fn cnt_read(&self) -> u32 {
                self.cnt().read().bits() as u32
            }
            common_ops!();
        }
    };
    ($pacmod:ident, full, no_tim2) => {
        impl sealed::SealedBlock for pac::$pacmod::RegisterBlock {
            #[inline]
            fn cr1_write(&self, cen: bool, arpe: bool, opm: bool, urs: bool) {
                cr1_write_full!(self, cen, arpe, opm, urs)
            }
            #[inline]
            fn rcr_write(&self, _val: u16) {}
            // TIM2 CNT 是 32 位：bit31 为 UIFREMAP 位（RM 20.4.10），读时屏蔽
            #[inline]
            fn cnt_read(&self) -> u32 {
                self.cnt().read().bits() & 0x7FFF_FFFF
            }
            common_ops!();
        }
    };
    ($pacmod:ident, plain, yes16) => {
        impl sealed::SealedBlock for pac::$pacmod::RegisterBlock {
            #[inline]
            fn cr1_write(&self, cen: bool, arpe: bool, opm: bool, urs: bool) {
                cr1_write_plain!(self, cen, arpe, opm, urs)
            }
            #[inline]
            fn rcr_write(&self, val: u16) {
                // 8 位 RCR：[15:8] 为保留位，写 0；RCR 寄存器在 SVD 中为 16 位（W.bits: u16）
                self.rcr().write(|w| unsafe { w.bits(val as u8 as u16) });
            }
            #[inline]
            fn cnt_read(&self) -> u32 {
                self.cnt().read().bits() as u32
            }
            common_ops!();
        }
    };
    ($pacmod:ident, plain, yes32) => {
        impl sealed::SealedBlock for pac::$pacmod::RegisterBlock {
            #[inline]
            fn cr1_write(&self, cen: bool, arpe: bool, opm: bool, urs: bool) {
                cr1_write_plain!(self, cen, arpe, opm, urs)
            }
            #[inline]
            fn rcr_write(&self, val: u16) {
                // 8 位 RCR：[15:8] 为保留位，写 0；RCR 寄存器在 SVD 中为 32 位（W.bits: u32）
                self.rcr().write(|w| unsafe { w.bits(val as u8 as u32) });
            }
            #[inline]
            fn cnt_read(&self) -> u32 {
                self.cnt().read().bits() as u32
            }
            common_ops!();
        }
    };
    ($pacmod:ident, plain, no) => {
        impl sealed::SealedBlock for pac::$pacmod::RegisterBlock {
            #[inline]
            fn cr1_write(&self, cen: bool, arpe: bool, opm: bool, urs: bool) {
                cr1_write_plain!(self, cen, arpe, opm, urs)
            }
            #[inline]
            fn rcr_write(&self, _val: u16) {}
            #[inline]
            fn cnt_read(&self) -> u32 {
                self.cnt().read().bits() as u32
            }
            common_ops!();
        }
    };
    ($pacmod:ident, basic, no) => {
        impl sealed::SealedBlock for pac::$pacmod::RegisterBlock {
            #[inline]
            fn cr1_write(&self, cen: bool, arpe: bool, opm: bool, urs: bool) {
                cr1_write_basic!(self, cen, arpe, opm, urs)
            }
            #[inline]
            fn rcr_write(&self, _val: u16) {}
            #[inline]
            fn cnt_read(&self) -> u32 {
                self.cnt().read().bits() as u32
            }
            common_ops!();
        }
    };
}

// 七类寄存器块 → SealedBlock 实现
impl_sealed_block!(tim1, full, yes);
impl_sealed_block!(tim2, full, no_tim2);
impl_sealed_block!(tim6, basic, no);
impl_sealed_block!(tim9, plain, yes16);
impl_sealed_block!(tim10, plain, no);
impl_sealed_block!(tim15, plain, yes32);
impl_sealed_block!(tim16, plain, yes32);

/// 可用时基功能的 TIM 实例标记 trait（TIM1–TIM19，见模块级文档的实例表）。
// 密封超 trait 故意保持 crate 内私有（用户不可自行实现 Instance），
// 但它出现在 pub trait 的 supertrait 位置会触发 private_bounds lint。
#[allow(private_bounds)]
pub trait Instance: sealed::SealedInstance + PeripheralType + Copy + 'static {}

/// 为单个 TIM 实例实现 `SealedInstance`。
///
/// 参数：外设单例名、PAC 外设类型名、总线、RCC 使能/复位位号（来自
/// `py32e407-pac/src/rcc/{apb1enr1,apb2enr,apb1rstr1,apb2rstr}.rs`，使能与
/// 复位寄存器位号一致）、计数器是否 32 位、RCR 最大值。
macro_rules! impl_instance {
    ($inst:ident, $pac:ident, $bus:expr, $bit:literal, $is32:literal, $maxrep:expr) => {
        impl sealed::SealedInstance for crate::peripherals::$inst {
            #[inline]
            fn regs(&self) -> &'static dyn sealed::SealedBlock {
                // 基址与类型均来自 PAC（如 Tim6 = crate::Periph<tim6::RegisterBlock, 0x4000_1000>）
                unsafe { &*pac::$pac::PTR }
            }
            #[inline]
            fn bus(&self) -> Bus {
                $bus
            }
            #[inline]
            fn enable_mask(&self) -> u32 {
                1 << $bit
            }
            #[inline]
            fn is_32bit(&self) -> bool {
                $is32
            }
            #[inline]
            fn max_repetition(&self) -> Option<u32> {
                $maxrep
            }
        }

        impl Instance for crate::peripherals::$inst {}
    };
}

// APB1 定时器（RCC_APB1ENR1 / RCC_APB1RSTR1）
impl_instance!(TIM2, Tim2, Bus::Apb1, 0, true, None);
impl_instance!(TIM3, Tim3, Bus::Apb1, 1, false, None);
impl_instance!(TIM4, Tim4, Bus::Apb1, 2, false, None);
impl_instance!(TIM5, Tim5, Bus::Apb1, 3, false, None);
impl_instance!(TIM6, Tim6, Bus::Apb1, 4, false, None);
impl_instance!(TIM7, Tim7, Bus::Apb1, 5, false, None);
impl_instance!(TIM12, Tim12, Bus::Apb1, 6, false, Some(0xFF));
impl_instance!(TIM13, Tim13, Bus::Apb1, 7, false, None);
impl_instance!(TIM14, Tim14, Bus::Apb1, 8, false, None);
impl_instance!(TIM18, Tim18, Bus::Apb1, 9, false, None);

// APB2 定时器（RCC_APB2ENR / RCC_APB2RSTR；位号与 STM32 不同，见模块级文档）
impl_instance!(TIM1, Tim1, Bus::Apb2, 11, false, Some(0xFFFF));
impl_instance!(TIM8, Tim8, Bus::Apb2, 13, false, Some(0xFFFF));
impl_instance!(TIM9, Tim9, Bus::Apb2, 19, false, Some(0xFF));
impl_instance!(TIM10, Tim10, Bus::Apb2, 20, false, None);
impl_instance!(TIM11, Tim11, Bus::Apb2, 21, false, None);
impl_instance!(TIM15, Tim15, Bus::Apb2, 22, false, Some(0xFF));
impl_instance!(TIM16, Tim16, Bus::Apb2, 23, false, Some(0xFF));
impl_instance!(TIM17, Tim17, Bus::Apb2, 24, false, Some(0xFF));
impl_instance!(TIM19, Tim19, Bus::Apb2, 25, false, None);

/// TIM 时基驱动句柄。
///
/// 持有外设单例的所有权（同一实例不会被第二个驱动再次初始化）。
/// `Drop` 不关闭定时器时钟——与 embassy-stm32 行为一致，时钟常开。
pub struct Timer<'d, T: Instance> {
    _peri: Peri<'d, T>,
}

impl<'d, T: Instance> Timer<'d, T> {
    /// 创建时基定时器并完成初始化（阻塞）。
    ///
    /// 流程：使能时钟 → 复位外设（Force + Release，保证寄存器处于复位默认）
    /// → 按官方序列写配置寄存器 → 产生一次更新事件装载影子寄存器 → 清 UIF。
    /// 配置写入顺序严格对照 `py32e407_hal_tim.c` 的 `TIM_Base_SetConfig`
    /// （CR1 → ARR → PSC → RCR → EGR.UG → 清 UIF）。
    ///
    /// # Errors
    ///
    /// - `period` 超出该实例计数器位宽（16 位实例 > 0xFFFF，或 TIM2 > 0xFFFF_FFFF）
    /// - `repetition` 超出该实例 RCR 位宽（无 RCR 的实例会忽略非零值——
    ///   如需严格校验请在调用侧判断，见模块级文档实例表）
    pub fn new_blocking(peri: Peri<'d, T>, config: Config) -> Result<Self, Error> {
        // 位宽校验（RM 20.1.1：仅 TIM2 为 32 位计数器）
        let max_arr: u32 = if peri.is_32bit() { 0xFFFF_FFFF } else { 0xFFFF };
        if config.period > max_arr {
            return Err(Error::InvalidConfig);
        }
        if let Some(max_rep) = peri.max_repetition() {
            if config.repetition as u32 > max_rep {
                return Err(Error::InvalidConfig);
            }
        }

        // 1. RCC 使能时钟 + 2. 复位外设（对齐 embassy-stm32 的 new 流程）
        peri.enable_clock();
        // Safety: 持有该外设唯一句柄，复位不会影响其他使用者
        unsafe { peri.reset() };

        let regs = peri.regs();
        // 3. 配置序列（py32e407_hal_tim.c: TIM_Base_SetConfig）
        //    UDIS 必须保持 0：更新事件被禁止时 UIF 永远不会置位，wait_for_update 将超时
        regs.cr1_write(
            false,
            config.auto_reload_preload,
            config.one_pulse,
            config.update_request_source,
        );
        regs.arr_write(config.period); // TIMx->ARR = Period
        regs.psc_write(config.prescaler); // TIMx->PSC = Prescaler
        regs.rcr_write(config.repetition); // 无 RCR 的实例为空操作
        // 4. 软件产生更新事件，立即装载 PSC/ARR/RCR 影子寄存器
        regs.egr_ug();
        // 5. HAL 收尾：UG 会置位 UIF，按 HAL_TIM_Base_Init 语义将其清零，
        //    避免首次 wait_for_update 立即返回
        regs.sr_uif_clear();

        Ok(Self { _peri: peri })
    }

    /// 启动计数器（`CR1.CEN = 1`，`__HAL_TIM_ENABLE`）。
    ///
    /// 注意：置位 CEN 后计数器在一个 CK_INT 周期后才开始计数（RM 20.4.1）。
    #[inline]
    pub fn start(&self) {
        self._peri.regs().cr1_set_cen(true);
    }

    /// 停止计数器（`CR1.CEN = 0`）。计数器值保持，`start` 后继续计数。
    #[inline]
    pub fn stop(&self) {
        self._peri.regs().cr1_set_cen(false);
    }

    /// 计数器是否正在运行（`CR1.CEN`）。
    #[inline]
    pub fn is_running(&self) -> bool {
        self._peri.regs().cr1_cen()
    }

    /// 修改预分频器（`PSC`）。
    ///
    /// `PSC` 带影子寄存器：新值在**下个更新事件**才生效；如需立即生效，
    /// 随后调用 [`Timer::generate_update`]。
    #[inline]
    pub fn set_prescaler(&self, psc: u16) {
        self._peri.regs().psc_write(psc);
    }

    /// 修改自动重装载值（`ARR`）。
    ///
    /// `ARPE = 0`（默认）时立即生效；`ARPE = 1` 时在下个更新事件生效。
    ///
    /// # Errors
    ///
    /// `period` 超出该实例计数器位宽时返回 [`Error::InvalidConfig`]。
    pub fn set_reload(&self, period: u32) -> Result<(), Error> {
        let max_arr: u32 = if self._peri.is_32bit() { 0xFFFF_FFFF } else { 0xFFFF };
        if period > max_arr {
            return Err(Error::InvalidConfig);
        }
        self._peri.regs().arr_write(period);
        Ok(())
    }

    /// 读当前计数值（`CNT`）。
    #[inline]
    pub fn current_count(&self) -> u32 {
        self._peri.regs().cnt_read()
    }

    /// 复位计数器到 0（`CNT = 0`，对齐 embassy-stm32 `Timer::reset`）。
    #[inline]
    pub fn reset(&self) {
        self._peri.regs().cnt_write(0);
    }

    /// 使能/关闭自动重装载预装载（`CR1.ARPE`，对齐 embassy-stm32
    /// `Timer::set_autoreload_preload`）。开启后写 `ARR` 在下个更新事件才生效。
    #[inline]
    pub fn set_autoreload_preload(&self, enable: bool) {
        self._peri.regs().arpe_set(enable);
    }

    /// 软件产生一次更新事件（`EGR.UG`）并清除随之置位的 `UIF`。
    ///
    /// 与 `TIM_Base_SetConfig` 尾部序列相同：UG 会重装 PSC/ARR/RCR 影子
    /// 寄存器并置 `UIF`；此处清 `UIF` 使后续 [`Timer::wait_for_update`]
    /// 只等待真正到来的更新事件。
    #[inline]
    pub fn generate_update(&self) {
        let regs = self._peri.regs();
        regs.egr_ug();
        regs.sr_uif_clear();
    }

    /// 清除更新事件标志（`SR.UIF`，写 0 清除）。
    #[inline]
    pub fn clear_update_flag(&self) {
        self._peri.regs().sr_uif_clear();
    }

    /// 更新事件标志是否已置位（`SR.UIF`）。
    #[inline]
    pub fn update_flag(&self) -> bool {
        self._peri.regs().sr_uif()
    }

    /// 阻塞等待一次更新事件（轮询 `SR.UIF` + 超时）。
    ///
    /// 返回 [`Ok(())`] 时标志刚被置位并已由本方法清除，可直接进入下一轮等待。
    ///
    /// # Errors
    ///
    /// 超过 [`TIMEOUT_LOOPS`] 次轮询仍未置位时返回 [`Error::Timeout`]——
    /// 常见于更新周期长于超时窗口（见常量文档）。
    pub fn wait_for_update(&self) -> Result<(), Error> {
        for _ in 0..TIMEOUT_LOOPS {
            if self._peri.regs().sr_uif() {
                self._peri.regs().sr_uif_clear();
                return Ok(());
            }
        }
        Err(Error::Timeout)
    }

    /// 使能/关闭更新中断（`DIER.UIE`，对齐 embassy-stm32
    /// `Timer::enable_update_interrupt`）。
    ///
    /// 置位后每次计数器溢出/下溢产生更新事件即置 `SR.UIF` 并向 NVIC 发中断请求；
    /// 中断服务函数里须调用 [`Timer::clear_update_interrupt`]（或直接清 `SR.UIF`）
    /// 才能收到下一次。使能中断前请先用 `cortex_m::peripheral::NVIC::unmask`
    /// 打开对应 `TIMx` 中断，否则请求不会被 CPU 响应。
    #[inline]
    pub fn enable_update_interrupt(&self, enable: bool) {
        self._peri.regs().uie_set(enable);
    }

    /// 清除更新中断标志（`SR.UIF`），返回清除前该位是否已置位
    /// （对齐 embassy-stm32 `Timer::clear_update_interrupt`）。
    ///
    /// 典型用法：在 `TIMx` 中断服务函数中调用，判断本次是否真为更新事件。
    #[inline]
    pub fn clear_update_interrupt(&self) -> bool {
        let regs = self._peri.regs();
        if regs.sr_uif() {
            regs.sr_uif_clear();
            true
        } else {
            false
        }
    }

    /// 按目标周期（CK_INT 周期数）设置 `PSC`/`ARR`（对齐 embassy-stm32
    /// `Timer::set_period_clocks`）。
    ///
    /// # Panics
    ///
    /// 目标周期超出该实例 `(psc+1) × (arr+1)` 表达能力时 panic（与 embassy-stm32
    /// 一致）。常见合法输入不会触发；如输入不可信，请先用
    /// [`Config::from_frequency`] 校验，或避开极端值。
    pub fn set_period_clocks(&self, clocks: u64, round: RoundTo) {
        let max_bits = if self._peri.is_32bit() { 32 } else { 16 };
        let (psc, arr) = calculate_psc_arr(clocks, round, max_bits)
            .expect("timer: requested period is out of range for this instance");
        let regs = self._peri.regs();
        regs.psc_write(psc);
        regs.arr_write(arr);
    }

    /// 设置更新周期（毫秒），对齐 embassy-stm32 `Timer::set_period_ms`。
    ///
    /// # Panics
    ///
    /// 周期超出表达能力时 panic（见 [`Timer::set_period_clocks`]）。
    pub fn set_period_ms(&self, ms: u32, round: RoundTo) {
        let timer_f = self._peri.timer_clk() as u64;
        let clocks = div_round(timer_f * ms as u64, 1_000, round);
        self.set_period_clocks(clocks, round);
    }

    /// 设置更新周期（微秒），对齐 embassy-stm32 `Timer::set_period_us`。
    ///
    /// # Panics
    ///
    /// 周期超出表达能力时 panic（见 [`Timer::set_period_clocks`]）。
    pub fn set_period_us(&self, us: u32, round: RoundTo) {
        let timer_f = self._peri.timer_clk() as u64;
        let clocks = div_round(timer_f * us as u64, 1_000_000, round);
        self.set_period_clocks(clocks, round);
    }

    /// 设置更新周期（秒），对齐 embassy-stm32 `Timer::set_period_secs`。
    ///
    /// # Panics
    ///
    /// 周期超出表达能力时 panic（见 [`Timer::set_period_clocks`]）。
    pub fn set_period_secs(&self, secs: u32, round: RoundTo) {
        let timer_f = self._peri.timer_clk() as u64;
        let clocks = timer_f * secs as u64;
        self.set_period_clocks(clocks, round);
    }

    /// 按目标更新频率设置 `PSC`/`ARR`（对齐 embassy-stm32 `Timer::set_frequency`）。
    ///
    /// # Panics
    ///
    /// `frequency` 为 0，或结果超出表达能力时 panic。
    pub fn set_frequency(&self, frequency: Hertz, round: RoundTo) {
        assert!(frequency.0 > 0, "timer: frequency must be non-zero");
        let timer_f = self._peri.timer_clk() as u64;
        let clocks = div_round(timer_f, frequency.0 as u64, round);
        self.set_period_clocks(clocks, round);
    }

    /// 读当前实际更新频率（由 `PSC`/`ARR` 反推：
    /// `CK_INT / ((psc + 1) × (arr + 1))`）。
    #[inline]
    pub fn get_frequency(&self) -> Hertz {
        let timer_f = self._peri.timer_clk() as u64;
        let regs = self._peri.regs();
        let arr = regs.arr_read() as u64;
        let psc = regs.psc_read() as u64;
        Hertz((timer_f / ((psc + 1) * (arr + 1))) as u32)
    }

    /// 定时器内核时钟 CK_INT（APB 预分频 ≠ 1 时为 PCLK × 2，见
    /// [`sealed::SealedInstance::timer_clk`] 的依据说明）。
    #[inline]
    pub fn timer_clk(&self) -> Hertz {
        Hertz(self._peri.timer_clk())
    }
}
