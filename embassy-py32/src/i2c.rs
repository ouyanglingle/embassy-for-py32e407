//! I2C 接口（同步 Blocking 主机驱动）
//!
//! 适配 PY32E407。寄存器流程依据：
//! - HAL C: `py32e407_hal_i2c.c`（`HAL_I2C_Init`、
//!   `I2C_MasterRequestWrite/Read`、`HAL_I2C_Master_Transmit/Receive、
//!   `I2C_RequestMemoryWrite/Read`）与 `py32e407_hal_i2c.h`（`I2C_InitTypeDef`、
//!   `I2C_SPEED`/`I2C_CCR_CALCULATION`/`I2C_RISE_TIME` 时序宏）
//! - LL:    `py32e407_ll_i2c.h`（`LL_I2C_GenerateStartCondition`、
//!   `LL_I2C_ClearFlag_ADDR`、`LL_I2C_IsActiveFlag_*`、`LL_I2C_TransmitData8`）
//! - RM:    PY32E407 参考手册 V0.6 第 36 章（I2C 接口，36.4 寄存器）
//! - DS:    PY32E407 数据手册 V0.13 端口复用功能映射表（I2C 引脚 AF 号）
//!
//! 目标外设是 **I2C1 / I2C2 / I2C3 / I2C4**（PAC: [`crate::pac::i2c1`]，四实例共用同一
//! `RegisterBlock`；PAC 中 `i2c2`/`i2c3`/`i2c4` 是 `i2c1` 的别名）。
//!
//! # 关键实现事实（已用 RM + SVD + PAC + HAL 交叉验证）
//!
//! - **经典 F1 式寄存器模型**（RM 36.4）：`CR1/CR2/OAR1/OAR2/DR/SR1/SR2/CCR/TRISE`
//!   加 PY32 专有的 `TIMEOUTR`；**没有** `TIMINGR/ISR/ICR/PECR/RXDR/TXDR`（那是
//!   STM32F0/F3 的新模型）。因此时序需自行按 `FREQ/CCR/TRISE` 计算。
//! - **`SR1` 的错误位是 RC_W0（写 0 清除）**（RM 36.4.6）：`AF`(NACK)、`ARLO`、
//!   `BERR`、`OVR`、`PECERR`、`TIMEOUT`、`SMBALERT`。本驱动的 [`I2c::check_error_flags`]
//!   通过写全 0（`sr1().write(|w| w)`，只读位不受影响）一次性清除全部错误标志。
//!   `ADDR` 由"读 `SR1` 再读 `SR2`"清除；`SB`/`BTF` 由"读 `SR1` 后读写 `DR`"清除。
//! - **`CCR`/`TRISE` 只能在 `PE = 0` 时写**（RM 36.4.8/36.4.9 注），故初始化先清 `PE`，
//!   写完时序/地址后**最后**置 `PE = 1`（对照 `HAL_I2C_Init`）。
//! - **PAC 偏差（SVD 缺陷，以 RM + CMSIS 为准）**：RM 36.4.2 与 CMSIS
//!   （`I2C_CR2_FREQ_Msk = 0x7F`）规定 `CR2.FREQ` 为 **7 位** `[6:0]`，RM 36.4.9 与
//!   CMSIS（`I2C_TRISE_TRISE_Msk = 0x7F`）规定 `TRISE` 为 **7 位** `[6:0]`；但 SVD/PAC
//!   把两者都只生成了 6 位字段。故本驱动对这两个字段改用 PAC 的整寄存器
//!   `modify(|r, w| w.bits(...))` 按 7 位写入（仍经 PAC，不裸写地址），见
//!   [`write_freq`]/[`write_trise`]。`CCR`（12 位）PAC 与 RM/CMSIS 一致，无此问题。
//! - **实例时钟**：I2C1/2/3 在 `RCC_APB1ENR1` bit21/22/30，**I2C4 在 `RCC_APB1ENR2` bit2**；
//!   复位位同号于 `APB1RSTR1/RSTR2`。四者内核时钟均为 PCLK1。
//!
//! # 使用示例
//!
//! ```ignore
//! use embassy_py32::i2c::{Config, I2c};
//! use embassy_py32::{init, Hertz};
//!
//! let p = init(Default::default());
//! // I2C3：PC8 = SCL、PC9 = SDA（AF8），与官方例程
//! // Example_LL/I2C/I2C_TwoBoards_Com_Pollig_Init 一致
//! let mut i2c = I2c::new_blocking(p.I2C3, p.PC8, p.PC9, Config::default()).unwrap();
//! // 读某个 7 位地址寄存器设备（如 EEPROM/传感器）
//! let mut buf = [0u8; 2];
//! i2c.blocking_read_reg(0x50, 0x00, &mut buf).unwrap();
//! ```
//!
//! 仅实现同步主机：7 位地址、阻塞轮询 + 超时；不含从机、10 位地址、SMBus/PEC 与
//! DMA/中断；后续将按需扩展 Async。

use crate::gpio::{Flex, OutputType, Pin, Pull, Speed};
use crate::pac;
use crate::{Hertz, Peri, PeripheralType};

/// 轮询等待状态位的循环上限（次数）。
///
/// 量级依据：HSI16 下每次循环约 4~6 个周期，`1_000_000` 次 ≈ 0.25~0.4 s，
/// 远大于 100 kHz 下单字节（含 ACK 约 90 µs）的传输时间。
const TIMEOUT_LOOPS: u32 = 1_000_000;

/// I2C 错误类型。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// 等待状态位超过 [`TIMEOUT_LOOPS`] 仍未就绪。
    Timeout,
    /// 应答失败（`SR1.AF`）：从设备未应答（常见于地址错误或总线上无从机）。
    Nack,
    /// 总线错误（`SR1.BERR`）：检测到非法的起始/停止条件。
    Bus,
    /// 仲裁丢失（`SR1.ARLO`）：多主机竞争时失去总线控制。
    Arbitration,
    /// 过载/欠载（`SR1.OVR`）。
    Overrun,
    /// 配置非法：频率为 0、`PCLK` 低于该模式最小要求，或算出的时序值越界。
    InvalidConfig,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            Self::Timeout => "Operation Timed Out",
            Self::Nack => "No Acknowledge Received",
            Self::Bus => "Bus Error",
            Self::Arbitration => "Arbitration Lost",
            Self::Overrun => "Overrun / Underrun",
            Self::InvalidConfig => "Invalid Configuration",
        };
        f.write_str(msg)
    }
}

impl core::error::Error for Error {}

/// I2C 配置。
///
/// `#[non_exhaustive]`，外部构造请用 [`Config::default()`] 后逐字段赋值。
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// SCL 频率（标准 100 kHz、快速 400 kHz、快速增强 1 MHz）。
    pub frequency: Hertz,
    /// SCL/SDA 复用引脚的输出速度（`OSPEEDR`）。
    pub gpio_speed: Speed,
    /// SCL 引脚的内部上拉。
    ///
    /// I2C 总线的上拉通常由**外部电阻**提供；仅当外部没有上拉时才启用内部上拉
    /// （内部上拉较弱，高速率下可能不满足上升时间要求）。
    pub scl_pull: Pull,
    /// SDA 引脚的内部上拉（同上，仅作外部上拉缺失时的兜底）。
    pub sda_pull: Pull,
}

impl Default for Config {
    /// 默认：100 kHz 标准模式、GPIO 超高速档、不启用内部上拉（依赖外部上拉）。
    fn default() -> Self {
        Self {
            frequency: Hertz(100_000),
            gpio_speed: Speed::VeryHigh,
            scl_pull: Pull::None,
            sda_pull: Pull::None,
        }
    }
}

// ---------------------------------------------------------------------------
// 外设实例
// ---------------------------------------------------------------------------

/// 密封的实例信息（防止外部为任意类型实现 [`Instance`]）。
mod sealed {
    use crate::pac;

    /// I2C 实例的寄存器块与 RCC 位信息。
    pub(super) trait SealedInstance {
        /// 取该实例的寄存器块（基址来自 PAC `Periph` 的常量指针）。
        ///
        /// I2C1/2/3/4 共用同一 `RegisterBlock`（PAC 中 `i2c2/3/4` 是 `i2c1` 的别名）。
        fn regs() -> &'static pac::i2c1::RegisterBlock;

        /// `RCC_APB1ENR1` 与 `RCC_APB1ENR2` 中的时钟使能（= 复位）位掩码。
        fn enable_masks() -> (u32, u32);
    }

    /// 密封的 SCL 引脚查询。
    pub trait SealedSclPin<T: super::Instance> {
        /// 该引脚在此实例下用作 SCL 时的复用功能编号（查 DS 端口复用功能映射表）。
        fn af_num(&self) -> u8;
    }

    /// 密封的 SDA 引脚查询。
    pub trait SealedSdaPin<T: super::Instance> {
        /// 该引脚在此实例下用作 SDA 时的复用功能编号。
        fn af_num(&self) -> u8;
    }
}

/// I2C 外设实例（[`crate::peripherals::I2C1`] … [`crate::peripherals::I2C4`]）。
#[allow(private_bounds)]
pub trait Instance: sealed::SealedInstance + PeripheralType + Copy + 'static {}

/// 可作为某实例 SCL 的引脚。
#[allow(private_bounds)]
pub trait SclPin<T: Instance>: sealed::SealedSclPin<T> + Pin {}

/// 可作为某实例 SDA 的引脚。
#[allow(private_bounds)]
pub trait SdaPin<T: Instance>: sealed::SealedSdaPin<T> + Pin {}

/// 为实例生成 [`sealed::SealedInstance`] 与 [`Instance`] 实现。
///
/// `$enr1`：`RCC_APB1ENR1` 位掩码；`$enr2`：`RCC_APB1ENR2` 位掩码
/// （SVD：I2C1 = APB1ENR1 bit21、I2C2 = bit22、I2C3 = bit30、I2C4 = APB1ENR2 bit2；
/// 复位位同号于 `APB1RSTR1/RSTR2`）。
macro_rules! impl_instance {
    ($inst:ident, $pac:ident, $enr1:expr, $enr2:expr) => {
        impl sealed::SealedInstance for crate::peripherals::$inst {
            #[inline]
            fn regs() -> &'static pac::i2c1::RegisterBlock {
                // 基址与类型均来自 PAC（I2C1 = 0x4000_5400、I2C2 = 0x4000_5800、
                // I2C3 = 0x4000_7800、I2C4 = 0x4000_8400），四实例共用 i2c1::RegisterBlock。
                unsafe { &*pac::$pac::PTR }
            }

            #[inline]
            fn enable_masks() -> (u32, u32) {
                ($enr1, $enr2)
            }
        }

        impl Instance for crate::peripherals::$inst {}
    };
}

impl_instance!(I2C1, I2c1, 1 << 21, 0);
impl_instance!(I2C2, I2c2, 1 << 22, 0);
impl_instance!(I2C3, I2c3, 1 << 30, 0);
impl_instance!(I2C4, I2c4, 0, 1 << 2);

/// 为引脚生成 SCL 引脚实现（含 AF 号）。
macro_rules! impl_scl_pin {
    ($inst:ident, $pin:ident, $af:literal) => {
        impl sealed::SealedSclPin<crate::peripherals::$inst> for crate::peripherals::$pin {
            #[inline]
            fn af_num(&self) -> u8 {
                $af
            }
        }
        impl SclPin<crate::peripherals::$inst> for crate::peripherals::$pin {}
    };
}

/// 为引脚生成 SDA 引脚实现（含 AF 号）。
macro_rules! impl_sda_pin {
    ($inst:ident, $pin:ident, $af:literal) => {
        impl sealed::SealedSdaPin<crate::peripherals::$inst> for crate::peripherals::$pin {
            #[inline]
            fn af_num(&self) -> u8 {
                $af
            }
        }
        impl SdaPin<crate::peripherals::$inst> for crate::peripherals::$pin {}
    };
}

// 引脚与 AF 号来自 DS V0.13 端口复用功能映射表（表 3-3 端口 A … 表 3-8 端口 F）。
// 证据强度：I2C3 的 PC8/PC9 = AF8 有官方例程
// （Example_LL/I2C/I2C_TwoBoards_Com_Pollig_Init）实打实佐证；
// I2C1/I2C2 = AF4（HAL 常量 GPIO_AF4_I2C1/I2C2 亦为 0x04）；I2C4 = AF3（PC6/PC7）
// 仅有 DS 表。I2C1 的 PA13/PA14 属 SWD 调试脚（SWDIO/SWCLK），复用需板级取舍。

// I2C1（AF4）
impl_scl_pin!(I2C1, PA13, 4);
impl_scl_pin!(I2C1, PA15, 4);
impl_scl_pin!(I2C1, PB8, 4);
impl_sda_pin!(I2C1, PA14, 4);
impl_sda_pin!(I2C1, PB7, 4);
impl_sda_pin!(I2C1, PB9, 4);

// I2C2（AF4）
impl_scl_pin!(I2C2, PA9, 4);
impl_scl_pin!(I2C2, PC3, 4);
impl_sda_pin!(I2C2, PA8, 4);
impl_sda_pin!(I2C2, PF0, 4);

// I2C3：PC8/PC9 = AF8（官方例程已证）；PA8/PA9 = AF2（DS 表，备用）
impl_scl_pin!(I2C3, PC8, 8);
impl_sda_pin!(I2C3, PC9, 8);
impl_scl_pin!(I2C3, PA8, 2);
impl_sda_pin!(I2C3, PA9, 2);

// I2C4（AF3，仅 DS 表佐证）
impl_scl_pin!(I2C4, PC6, 3);
impl_sda_pin!(I2C4, PC7, 3);
impl_scl_pin!(I2C4, PA13, 3);
impl_sda_pin!(I2C4, PA14, 3);

// ---------------------------------------------------------------------------
// 驱动
// ---------------------------------------------------------------------------

/// I2C 同步主机驱动。
///
/// 引脚所有权由 [`Peri`] 保证：同一个物理引脚同一时刻至多被一个驱动持有。
/// 驱动被 Drop 时 SCL/SDA 的 [`Flex`] 会恢复为模拟（高阻）态；`CR1.PE` 保持置位。
pub struct I2c<'d, T: Instance> {
    _peri: Peri<'d, T>,
    _scl: Flex<'d>,
    _sda: Flex<'d>,
    kernel_clock: u32,
}

impl<'d, T: Instance> I2c<'d, T> {
    /// 创建主机驱动，并完成外设与引脚初始化（阻塞）。
    ///
    /// 流程对照 `HAL_I2C_Init`：
    /// 1. 使能该实例时钟（APB1，含 LL 要求的读回等待）并复位该实例；
    /// 2. 把 `scl`/`sda` 配为开漏复用的 AF（I2C 要求开漏 + 外部上拉）；
    /// 3. `PE = 0` → `SWRST` 脉冲 → 写 `CR2.FREQ`、`CCR`、`TRISE` → `PE = 1`
    ///    （RM 要求 `CCR`/`TRISE` 只能在 `PE = 0` 时写）。
    ///
    /// # Errors
    ///
    /// 频率为 0、`PCLK1` 低于该模式最小要求（标准 ≥4 MHz、快速 ≥8 MHz、
    /// 快速增强 ≥16 MHz），或时序值越界时返回 [`Error::InvalidConfig`]。
    pub fn new_blocking(
        peri: Peri<'d, T>,
        scl: Peri<'d, impl SclPin<T>>,
        sda: Peri<'d, impl SdaPin<T>>,
        config: Config,
    ) -> Result<Self, Error> {
        enable_and_reset::<T>();

        // AF 号要在引脚被 Flex 接管之前取出（AF 查表见上方 impl_*_pin! 宏）
        let scl_af = scl.af_num();
        let sda_af = sda.af_num();

        let mut scl = Flex::new(scl);
        let mut sda = Flex::new(sda);
        // I2C 总线必须开漏：输出高电平时释放总线，由外部上拉拉高
        scl.set_as_alternate(
            scl_af,
            config.gpio_speed,
            config.scl_pull,
            OutputType::OpenDrain,
        );
        sda.set_as_alternate(
            sda_af,
            config.gpio_speed,
            config.sda_pull,
            OutputType::OpenDrain,
        );

        let kernel_clock = crate::rcc::clocks().apb1.0;
        init_regs::<T>(kernel_clock, &config)?;

        Ok(Self {
            _peri: peri,
            _scl: scl,
            _sda: sda,
            kernel_clock,
        })
    }

    /// 阻塞写入 `buf` 到 7 位地址 `address`（对照 `HAL_I2C_Master_Transmit`）。
    ///
    /// # Errors
    ///
    /// 见 [`Error`]。
    pub fn blocking_write(&mut self, address: u8, buf: &[u8]) -> Result<(), Error> {
        let r = self.write_seq(address, buf);
        self.finish(r)
    }

    /// 阻塞从 7 位地址 `address` 读出填满 `buf`（对照 `HAL_I2C_Master_Receive`）。
    ///
    /// # Errors
    ///
    /// `buf` 为空时返回 [`Error::Overrun`]；其余见 [`Error`]。
    pub fn blocking_read(&mut self, address: u8, buf: &mut [u8]) -> Result<(), Error> {
        if buf.is_empty() {
            return Err(Error::Overrun);
        }
        let r = self.read_seq(address, buf);
        self.finish(r)
    }

    /// 阻塞"先写后读"：写 `write_buf` 后以重复起始条件读 `read_buf`
    /// （对照 `HAL_I2C_Master_Transmit` 尾接 `HAL_I2C_Master_Receive` 的复合事务）。
    ///
    /// # Errors
    ///
    /// `read_buf` 为空时返回 [`Error::Overrun`]；其余见 [`Error`]。
    pub fn blocking_write_read(
        &mut self,
        address: u8,
        write_buf: &[u8],
        read_buf: &mut [u8],
    ) -> Result<(), Error> {
        if read_buf.is_empty() {
            return Err(Error::Overrun);
        }
        let r = self.write_read_seq(address, write_buf, read_buf);
        self.finish(r)
    }

    /// 阻塞写入寄存器：先发寄存器地址 `reg`，再发 `buf`（对照 `HAL_I2C_Mem_Write` 8 位地址）。
    ///
    /// # Errors
    ///
    /// 见 [`Error`]。
    pub fn blocking_write_reg(&mut self, address: u8, reg: u8, buf: &[u8]) -> Result<(), Error> {
        let r = self.write_reg_seq(address, reg, buf);
        self.finish(r)
    }

    /// 阻塞读寄存器：先发寄存器地址 `reg`，重复起始后读 `buf`
    /// （对照 `HAL_I2C_Mem_Read` 8 位地址）。
    ///
    /// # Errors
    ///
    /// `buf` 为空时返回 [`Error::Overrun`]；其余见 [`Error`]。
    pub fn blocking_read_reg(
        &mut self,
        address: u8,
        reg: u8,
        buf: &mut [u8],
    ) -> Result<(), Error> {
        if buf.is_empty() {
            return Err(Error::Overrun);
        }
        let r = self.read_reg_seq(address, reg, buf);
        self.finish(r)
    }

    /// 该实例的内核时钟（PCLK1，Hz）。
    #[inline]
    pub fn kernel_clock(&self) -> Hertz {
        Hertz(self.kernel_clock)
    }

    // ---- 事务序列（出错时由 [`I2c::finish`] 补一个 STOP 释放总线）----

    /// 写事务：START → 地址(W) → 逐字节 → STOP。
    fn write_seq(&self, address: u8, buf: &[u8]) -> Result<(), Error> {
        self.wait_bus_free()?;
        self.start_and_addr(address, false)?;
        for &b in buf {
            self.write_byte(b)?;
        }
        self.stop();
        Ok(())
    }

    /// 读事务：START → 地址(R) → 逐字节 → STOP。
    fn read_seq(&self, address: u8, buf: &mut [u8]) -> Result<(), Error> {
        self.wait_bus_free()?;
        self.start_and_addr(address, true)?;
        self.read_payload(buf)
    }

    /// 先写后读事务：START → 地址(W) → 数据字节 → 重复 START → 地址(R) → 逐字节 → STOP。
    fn write_read_seq(
        &self,
        address: u8,
        write_buf: &[u8],
        read_buf: &mut [u8],
    ) -> Result<(), Error> {
        self.wait_bus_free()?;
        self.start_and_addr(address, false)?;
        for &b in write_buf {
            self.write_byte(b)?;
        }
        // 重复起始条件：不再发 STOP，直接重新 START + 地址（读方向）
        self.start_condition()?;
        self.send_addr(address, true)?;
        self.read_payload(read_buf)
    }

    /// 写寄存器事务：START → 地址(W) → 寄存器地址 → 数据字节 → STOP。
    fn write_reg_seq(&self, address: u8, reg: u8, buf: &[u8]) -> Result<(), Error> {
        self.wait_bus_free()?;
        self.start_and_addr(address, false)?;
        self.write_byte(reg)?;
        for &b in buf {
            self.write_byte(b)?;
        }
        self.stop();
        Ok(())
    }

    /// 读寄存器事务：START → 地址(W) → 寄存器地址 → 重复 START → 地址(R) → 逐字节 → STOP。
    fn read_reg_seq(&self, address: u8, reg: u8, buf: &mut [u8]) -> Result<(), Error> {
        self.wait_bus_free()?;
        self.start_and_addr(address, false)?;
        self.write_byte(reg)?;
        self.start_condition()?;
        self.send_addr(address, true)?;
        self.read_payload(buf)
    }

    /// 事务收尾：出错时补发 STOP 释放总线（对照 HAL 在 `AF` 上置 `CR1.STOP`）。
    #[inline]
    fn finish(&self, r: Result<(), Error>) -> Result<(), Error> {
        if r.is_err() {
            self.stop();
        }
        r
    }

    // ---- 内部原语 ----

    /// 等待总线空闲（`SR2.BUSY = 0`，对照 `HAL_I2C_Master_*` 开头的 BUSY 等待）。
    fn wait_bus_free(&self) -> Result<(), Error> {
        let regs = T::regs();
        for _ in 0..TIMEOUT_LOOPS {
            if !regs.sr2().read().busy().bit_is_set() {
                return Ok(());
            }
        }
        Err(Error::Timeout)
    }

    /// 仅在总线空闲后产生起始条件并等待 `SB`（用于重复起始）。
    fn start_condition(&self) -> Result<(), Error> {
        let regs = T::regs();
        // 置 START（RM 36.4.1 bit8）；写 DR 或置 STOP 前 START 由硬件保持
        regs.cr1().modify(|_, w| w.start().set_bit());
        self.wait_flag(|sr1| sr1.sb().bit_is_set())?;
        // 校验确由本机产生 START 并进入主模式（对照 embassy v1 / HAL）
        if regs.cr1().read().start().bit_is_set() || !regs.sr2().read().msl().bit_is_set() {
            return Err(Error::Arbitration);
        }
        Ok(())
    }

    /// 产生起始条件并发送 7 位地址（`read` 决定 R/W 位），等待 `ADDR` 后清除之。
    fn start_and_addr(&self, address: u8, read: bool) -> Result<(), Error> {
        self.start_condition()?;
        self.send_addr(address, read)
    }

    /// 发送 7 位地址并等待 `ADDR`，随后"读 SR1 再读 SR2"清除 `ADDR`。
    fn send_addr(&self, address: u8, read: bool) -> Result<(), Error> {
        let regs = T::regs();
        // DR[7:1] = 地址、bit0 = R/W（RM 36.4.5 / HAL I2C_7BIT_ADD_WRITE/READ）
        let addr_byte = (address << 1) | (read as u8);
        regs.dr().write(|w| unsafe { w.dr().bits(addr_byte) });
        self.wait_flag(|sr1| sr1.addr().bit_is_set())?;
        // 清 ADDR：读 SR1 再读 SR2（RM 36.4.6 / LL_I2C_ClearFlag_ADDR）
        let _ = regs.sr1().read();
        let _ = regs.sr2().read();
        Ok(())
    }

    /// 写一个字节：等 `TXE` → 写 `DR` → 等 `BTF`（对照 embassy v1 `send_byte`）。
    fn write_byte(&self, byte: u8) -> Result<(), Error> {
        let regs = T::regs();
        self.wait_flag(|sr1| sr1.txe().bit_is_set())?;
        regs.dr().write(|w| unsafe { w.dr().bits(byte) });
        self.wait_flag(|sr1| sr1.btf().bit_is_set())?;
        Ok(())
    }

    /// 接收 `buf.len()` 个字节：前 N-1 个应答，最后一个前清 `ACK` 并置 `STOP`。
    fn read_payload(&self, buf: &mut [u8]) -> Result<(), Error> {
        let regs = T::regs();
        let n = buf.len();
        // 除最后一字节外全部 ACK（RM 36.4.1 bit10）
        regs.cr1().modify(|_, w| w.ack().set_bit());
        for slot in buf.iter_mut().take(n - 1) {
            *slot = self.read_byte()?;
        }
        // 最后一字节：先清 ACK（发 NACK）再置 STOP，然后读取（对照 embassy v1 "program ACK/STOP before last byte"）
        regs.cr1().modify(|_, w| {
            w.ack().clear_bit();
            w.stop().set_bit();
            w
        });
        buf[n - 1] = self.read_byte()?;
        Ok(())
    }

    /// 读一个字节：等 `RXNE` → 读 `DR`。
    fn read_byte(&self) -> Result<u8, Error> {
        let regs = T::regs();
        self.wait_flag(|sr1| sr1.rxne().bit_is_set())?;
        Ok(regs.dr().read().dr().bits())
    }

    /// 产生停止条件（`CR1.STOP`，RM 36.4.1 bit9）。
    #[inline]
    fn stop(&self) {
        T::regs().cr1().modify(|_, w| w.stop().set_bit());
    }

    /// 轮询 `SR1` 直到 `cond` 为真；期间若出现错误标志立即返回对应错误。
    ///
    /// 超过 [`TIMEOUT_LOOPS`] 次返回 [`Error::Timeout`]。
    fn wait_flag(&self, cond: impl Fn(&pac::i2c1::sr1::R) -> bool) -> Result<(), Error> {
        let regs = T::regs();
        for _ in 0..TIMEOUT_LOOPS {
            let sr1 = regs.sr1().read();
            check_error_flags(regs, &sr1)?;
            if cond(&sr1) {
                return Ok(());
            }
        }
        Err(Error::Timeout)
    }
}

// ---------------------------------------------------------------------------
// 内部实现
// ---------------------------------------------------------------------------

/// 使能实例时钟并复位实例（四者皆 APB1，分派到 `APB1ENR1`/`APB1ENR2`）。
#[inline]
fn enable_and_reset<T: Instance>() {
    let (enr1, enr2) = T::enable_masks();
    crate::rcc::enable_apb1(enr1, enr2);
    // Safety: 复位会立即停止该外设并恢复其寄存器复位值；此刻本驱动尚未配置它，
    // 且外设单例由 Peri 保证只被取出一次。
    unsafe { crate::rcc::reset_apb1(enr1, enr2) };
}

/// 判定 `SR1` 快照中的错误标志位（RM 36.4.6），命中即写 0 清除全部错误标志。
///
/// 优先级：`Nack`(AF) > `Arbitration`(ARLO) > `Bus`(BERR) > `Overrun`(OVR)。
/// `SR1` 的错误位均为 RC_W0（写 0 清除），故 `write(|w| w)`（写全 0）即清除全部
/// 错误标志；只读位（SB/ADDR/…）不受写操作影响。
#[inline]
fn check_error_flags(
    regs: &'static pac::i2c1::RegisterBlock,
    sr1: &pac::i2c1::sr1::R,
) -> Result<(), Error> {
    let err = if sr1.af().bit_is_set() {
        Some(Error::Nack)
    } else if sr1.arlo().bit_is_set() {
        Some(Error::Arbitration)
    } else if sr1.berr().bit_is_set() {
        Some(Error::Bus)
    } else if sr1.ovr().bit_is_set() {
        Some(Error::Overrun)
    } else {
        None
    };

    match err {
        Some(e) => {
            // 写全 0 清除全部 RC_W0 错误标志（读-改-写会重新写 1，对 RC_W0 无意义）
            regs.sr1().write(|w| w);
            Err(e)
        }
        None => Ok(()),
    }
}

/// I2C 时序计算（对照 `py32e407_hal_i2c.h` 的 `I2C_FREQRANGE`/`I2C_SPEED`/`I2C_RISE_TIME`）。
struct Timings {
    /// `CR2.FREQ = PCLK / 1 MHz`（**7 位**，见模块 doc 的 SVD 缺陷说明）。
    freq: u8,
    /// `CCR.CCR[11:0]` 分频系数。
    ccr: u16,
    /// `CCR.FS`：快速模式。
    fs: bool,
    /// `CCR.FP`：快速增强模式。
    fp: bool,
    /// `CCR.DUTY`：快速/快速增强模式占空比（0 = 2:1）。
    duty: bool,
    /// `TRISE.TRISE`：最大 SCL 上升时间（以 PCLK 周期计）。
    trise: u8,
}

/// 由 `PCLK` 与目标 SCL 频率计算 I2C 时序（RM 36.4.8/36.4.9 + HAL 时序宏）。
fn compute_timings(pclk: u32, frequency: u32) -> Result<Timings, Error> {
    if pclk == 0 || frequency == 0 {
        return Err(Error::InvalidConfig);
    }

    let freqrange = pclk / 1_000_000;
    // FREQ 为 7 位字段（≤0x7F，见模块 doc 的 SVD 缺陷说明），且各模式对 PCLK 有最小要求
    if freqrange == 0 || freqrange > 0x7F {
        return Err(Error::InvalidConfig);
    }
    let min_pclk = if frequency <= 100_000 {
        4_000_000
    } else if frequency <= 400_000 {
        8_000_000
    } else {
        16_000_000
    };
    if pclk < min_pclk {
        return Err(Error::InvalidConfig);
    }

    // HAL: I2C_CCR_CALCULATION(pclk, speed, coeff) = ((pclk-1)/(speed*coeff)) + 1
    let calc = |coeff: u32| -> u32 { (pclk - 1) / (frequency * coeff) + 1 };

    let (fs, fp, duty, ccr);
    let trise;
    if frequency <= 100_000 {
        // 标准模式：CCR = max(4, PCLK/(f·2))，TRISE = FREQ + 1
        fs = false;
        fp = false;
        duty = false;
        ccr = calc(2).max(4);
        trise = freqrange + 1;
    } else if frequency <= 400_000 {
        // 快速模式（2:1 占空比）：CCR = PCLK/(f·3)，TRISE = FREQ·300/1000 + 1
        fs = true;
        fp = false;
        duty = false;
        ccr = calc(3);
        trise = freqrange * 300 / 1000 + 1;
    } else {
        // 快速增强模式（2:1 占空比）：CCR = PCLK/(f·8)，TRISE = FREQ·300/2500 + 1
        fs = true;
        fp = true;
        duty = false;
        ccr = calc(8);
        trise = freqrange * 300 / 2500 + 1;
    }

    // CCR 为 12 位字段；TRISE 为 7 位（见模块 doc 的 SVD 缺陷说明）
    if ccr == 0 || ccr > 0xFFF || trise > 0x7F {
        return Err(Error::InvalidConfig);
    }

    Ok(Timings {
        freq: freqrange as u8,
        ccr: ccr as u16,
        fs,
        fp,
        duty,
        trise: trise as u8,
    })
}

/// 写 `CR2.FREQ[6:0]`（**7 位**）。
///
/// SVD/PAC 把 `FREQ` 只生成成 6 位字段（`cr2.freq()`），但 RM 36.4.2 与 CMSIS
/// （`I2C_CR2_FREQ_Msk = 0x7F`）均规定为 7 位。故此处经 PAC 整寄存器写：保留高 25 位、
/// 写入低 7 位（SVD 缺陷，以 RM + CMSIS 为准；仍走 PAC 不裸写地址）。
#[inline]
fn write_freq(regs: &'static pac::i2c1::RegisterBlock, freq: u8) {
    regs.cr2()
        .modify(|r, w| unsafe { w.bits((r.bits() & !0x7F) | freq as u32) });
}

/// 写 `TRISE.TRISE[6:0]`（**7 位**）。
///
/// 同 [`write_freq`]：SVD/PAC 只生成 6 位，RM 36.4.9 与 CMSIS
/// （`I2C_TRISE_TRISE_Msk = 0x7F`）为 7 位；此处保留 `THOLDDATA`/`THOLDDATA_SEL` 等高位，
/// 只写低 7 位。
#[inline]
fn write_trise(regs: &'static pac::i2c1::RegisterBlock, trise: u8) {
    regs.trise()
        .modify(|r, w| unsafe { w.bits((r.bits() & !0x7F) | trise as u32) });
}

/// 写 I2C 时序寄存器（不含 GPIO；引脚配置在构造器内完成）。
///
/// 顺序对照 `HAL_I2C_Init`：`PE = 0` → `SWRST` 脉冲 → `CR2.FREQ` → `CCR` → `TRISE`
/// → `PE = 1`（RM 要求 `CCR`/`TRISE` 只能在 `PE = 0` 时写）。
fn init_regs<T: Instance>(pclk: u32, config: &Config) -> Result<(), Error> {
    let timings = compute_timings(pclk, config.frequency.0)?;
    let regs = T::regs();

    // ---- 关闭 I2C（HAL: __HAL_I2C_DISABLE）----
    regs.cr1().modify(|_, w| w.pe().clear_bit());

    // ---- 软件复位脉冲（清空内部状态；RM 36.4.1 SWRST，PE=0 时才可写）----
    regs.cr1().modify(|_, w| w.swrst().set_bit());
    regs.cr1().modify(|_, w| w.swrst().clear_bit());

    // ---- CR2.FREQ：PCLK（MHz），RM 36.4.2（7 位，见 write_freq 说明）----
    write_freq(regs, timings.freq);

    // ---- CCR：F/S、FP、DUTY、CCR[11:0]，RM 36.4.8 ----
    regs.ccr().modify(|_, w| unsafe {
        w.fs().bit(timings.fs);
        w.fp().bit(timings.fp);
        w.duty().bit(timings.duty);
        w.ccr().bits(timings.ccr)
    });

    // ---- TRISE：最大 SCL 上升时间（保持 THOLDDATA/THOLDDATA_SEL 复位默认），RM 36.4.9 ----
    write_trise(regs, timings.trise);

    // ---- 使能 I2C（HAL: __HAL_I2C_ENABLE）----
    regs.cr1().modify(|_, w| w.pe().set_bit());
    Ok(())
}

// ---------------------------------------------------------------------------
// embedded-hal 1.0 兼容实现（由 crate feature `embedded-hal-1` 开启）
// ---------------------------------------------------------------------------
//
// impl 写在类型所属 crate 内部（孤儿规则，理由见 `crate::gpio` 的对应小节）。
//
// `embedded-hal` 的 `I2c` trait 里只有 `transaction()` 是必实现的，`read` /
// `write` / `write_read` 都由它默认转发。本驱动只有整段的
// `blocking_write` / `blocking_read` / `blocking_write_read`，因此
// `transaction()` 只支持这三种形态（覆盖寄存器读写等绝大多数场景）；
// 更复杂的组合（如 `Read` 后接 `Write`、连续两段 `Write`）无法用现有 API 表达，
// 返回 [`Error::InvalidConfig`]。
#[cfg(feature = "embedded-hal-1")]
mod embedded_hal_impls {
    use super::{Error, I2c, Instance};
    use embedded_hal_1::i2c::{
        ErrorKind, ErrorType, I2c as I2cTrait, NoAcknowledgeSource, Operation,
    };

    /// `embedded-hal` 的 I2C 错误分类映射。
    impl embedded_hal_1::i2c::Error for Error {
        fn kind(&self) -> ErrorKind {
            match self {
                // 本驱动不区分"地址未应答"与"数据未应答"，按规范填 Unknown
                Self::Nack => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown),
                Self::Bus => ErrorKind::Bus,
                Self::Arbitration => ErrorKind::ArbitrationLoss,
                Self::Overrun => ErrorKind::Overrun,
                // `ErrorKind` 没有 Timeout 变体；配置错误亦无对应变体
                Self::Timeout | Self::InvalidConfig => ErrorKind::Other,
            }
        }
    }

    impl<'d, T: Instance> ErrorType for I2c<'d, T> {
        type Error = Error;
    }

    impl<'d, T: Instance> I2cTrait for I2c<'d, T> {
        fn transaction(
            &mut self,
            address: u8,
            operations: &mut [Operation<'_>],
        ) -> Result<(), Self::Error> {
            match operations {
                [Operation::Write(data)] => self.blocking_write(address, data),
                [Operation::Read(data)] => self.blocking_read(address, data),
                [Operation::Write(prefix), Operation::Read(data)] => {
                    self.blocking_write_read(address, prefix, data)
                }
                _ => Err(Error::InvalidConfig),
            }
        }
    }
}
