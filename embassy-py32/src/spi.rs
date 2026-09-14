//! 串行外设接口 SPI（同步 Blocking 驱动）
//!
//! 适配 PY32E407。寄存器流程依据：
//! - HAL C: `py32e407_hal_spi.c`（`HAL_SPI_Init`、
//!   `HAL_SPI_TransmitReceive` / `HAL_SPI_Transmit` / `HAL_SPI_Receive`、
//!   `SPI_EndRxTxTransaction`）与 `py32e407_hal_spi.h`（`SPI_InitTypeDef`）
//! - LL:    `py32e407_ll_spi.h`（`LL_SPI_Init`、`LL_SPI_Enable`、
//!   `LL_SPI_SetBaudRatePrescaler`、`LL_SPI_IsActiveFlag_TXE/RXNE/BSY`、
//!   `LL_SPI_SetTransferBitOrder` 等）与 `py32e407_ll_bus.h`（时钟使能/复位宏）
//! - RM:    PY32E407 参考手册 V0.6 第 35 章（SPI/I2S，35.4 寄存器）与第 6 章（RCC）
//! - DS:    PY32E407 数据手册 V0.13 表 3-3…3-8（端口复用功能映射，SPI 引脚 AF 号）
//!
//! 目标外设是 **SPI1 / SPI2 / SPI3**（PAC: [`crate::pac::spi1`]，三实例共用同一
//! `RegisterBlock`；PAC 中 `spi2`/`spi3` 是 `spi1` 的别名）。SPI1 位于 APB2，
//! SPI2/SPI3 位于 APB1。
//!
//! # 关键实现事实（已用 RM + SVD + PAC + HAL 交叉验证）
//!
//! - **寄存器映射是 CR1/CR2/SR/DR 的经典布局**（与 STM32F4 相似），但**带 4 深度
//!   TX/RX FIFO**（STM32F0/F3 风格）：`CR2` 有 `FRXTH`/`CLRTXFIFO`，`SR` 有
//!   `FTLVL[12:11]`/`FRLVL[10:9]`。HAL 的阻塞收发只轮询 `SR.TXE`/`SR.RXNE`，
//!   收尾用 `SPI_EndRxTxTransaction`（FTLVL 空 → `BSY=0` → FRLVL 空），本驱动照做。
//! - **`CR2.FRXTH` 保持复位默认 0**：RM 35.4.2 规定 `FRXTH=0` 时 FIFO 中只要有数据即
//!   置 `RXNE`（`FRXTH=1` 需 ≥1/2）。HAL 全程不写该位，本驱动同样保持 0，
//!   这是单字节接收能正确触发 `RXNE` 的前提。
//! - **数据访问宽度须与帧长匹配**：8 位帧经字节宽度访问 `DR`、16 位帧经半字宽度访问
//!   （HAL 8 位路径用 `(__IO uint8_t *)&DR`，RM 35.4.4 亦要求读访问宽度与阈值的对应
//!   关系）。本驱动用 [`Word`] 的按宽度读写实现复刻该行为，而非 32 位整寄存器访问。
//! - **主/从 NSS 管理不同**：主机使用软件 NSS（`SSM=1`、`SSI=1`），
//!   `SSOE = !nss_output_disable`；从机使用硬件 NSS（`SSM=0`，由 CS 引脚输入选中），
//!   且从机模式波特率分频无意义（HAL 强制 `BR = fPCLK/2`）。
//!
//! # 使用示例（主机）
//!
//! ```ignore
//! use embassy_py32::spi::{Config, Spi};
//! use embassy_py32::init;
//!
//! let p = init(Default::default());
//! // PA5 = SCK、PA6 = MISO、PA7 = MOSI（AF5），与官方 STK 例程
//! // Example_LL/SPI/SPI_TwoBoards_FullDuplex_Polling 一致
//! let mut spi = Spi::new_blocking(p.SPI1, p.PA5, p.PA7, p.PA6, Config::default()).unwrap();
//! let mut buf = [0xAAu8, 0x55];
//! spi.blocking_transfer_in_place(&mut buf).unwrap();
//! ```
//!
//! 仅实现同步（轮询 + 超时）驱动：不使用 DMA 与中断（`CR2` 的 DMA/中断使能位全部
//! 清零），不实现 CRC、TI 模式、双向（半双工）模式与 I2S。后续将按需扩展 Async。

use core::marker::PhantomData;

use crate::gpio::{Flex, OutputType, Pin, Pull, Speed};
use crate::pac;
use crate::{Hertz, Peri, PeripheralType};

/// 轮询等待状态位的循环上限（次数）。
///
/// 量级依据：HSI16 下每次循环约 4~6 个周期，`1_000_000` 次 ≈ 0.25~0.4 s，
/// 足以覆盖最慢分频（fPCLK/256，40 MHz 下约 156 kHz、单字节约 51 µs）的等待并留
/// 出数千倍裕量，同时保证线路异常时能及时返回 [`Error::Timeout`]。
const TIMEOUT_LOOPS: u32 = 1_000_000;

/// SPI 错误类型。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// 接收溢出：接收 FIFO 中尚有数据未被读走又收到新数据（`SR.OVR`，RM 35.4.3）。
    Overrun,
    /// 模式错误：主模式下 NSS 被拉低（`SR.MODF`）。本驱动主机用软件 NSS，通常不会出现。
    ModeFault,
    /// CRC 错误：收到的 CRC 与 `RXCRCR` 不匹配（`SR.CRCERR`，仅在启用硬件 CRC 时有效）。
    Crc,
    /// 帧格式错误：TI 从模式下检测到帧错误（`SR.FRE`）。
    Framing,
    /// 等待状态位超过 [`TIMEOUT_LOOPS`] 仍未就绪。
    Timeout,
    /// 配置非法（频率为 0 等）。
    InvalidConfig,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            Self::Overrun => "RX Buffer Overrun",
            Self::ModeFault => "Mode Fault",
            Self::Crc => "Hardware CRC Check Failed",
            Self::Framing => "Invalid Framing",
            Self::Timeout => "Operation Timed Out",
            Self::InvalidConfig => "Invalid Configuration",
        };
        f.write_str(msg)
    }
}

impl core::error::Error for Error {}

/// 时钟极性（`CR1.CPOL`，RM 35.4.1）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Polarity {
    /// 空闲时 SCK 为低电平，第一个边沿为上升沿（`CPOL = 0`，复位默认）。
    IdleLow,
    /// 空闲时 SCK 为高电平，第一个边沿为下降沿（`CPOL = 1`）。
    IdleHigh,
}

/// 时钟相位（`CR1.CPHA`，RM 35.4.1）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Phase {
    /// 数据在第一个时钟边沿采样（`CPHA = 0`，复位默认）。
    FirstEdge,
    /// 数据在第二个时钟边沿采样（`CPHA = 1`）。
    SecondEdge,
}

/// SPI 工作模式（CPOL + CPHA 组合）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Mode {
    /// 时钟极性。
    pub polarity: Polarity,
    /// 时钟相位。
    pub phase: Phase,
}

/// 模式 0：CPOL = 0、CPHA = 0。
pub const MODE_0: Mode = Mode {
    polarity: Polarity::IdleLow,
    phase: Phase::FirstEdge,
};

/// 模式 1：CPOL = 0、CPHA = 1。
pub const MODE_1: Mode = Mode {
    polarity: Polarity::IdleLow,
    phase: Phase::SecondEdge,
};

/// 模式 2：CPOL = 1、CPHA = 0。
pub const MODE_2: Mode = Mode {
    polarity: Polarity::IdleHigh,
    phase: Phase::FirstEdge,
};

/// 模式 3：CPOL = 1、CPHA = 1。
pub const MODE_3: Mode = Mode {
    polarity: Polarity::IdleHigh,
    phase: Phase::SecondEdge,
};

/// 位序（`CR1.LSBFIRST`，RM 35.4.1）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BitOrder {
    /// LSB 优先（`LSBFIRST = 1`）。
    LsbFirst,
    /// MSB 优先（`LSBFIRST = 0`，复位默认）。
    MsbFirst,
}

/// SPI 配置。
///
/// `#[non_exhaustive]`，外部构造请用 [`Config::default()`] 后逐字段赋值。
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// 工作模式（CPOL/CPHA）。
    pub mode: Mode,
    /// 位序。
    pub bit_order: BitOrder,
    /// 目标 SCK 频率（主机有效；从机忽略）。实际频率取不超过该值的最快分频档。
    pub frequency: Hertz,
    /// MISO 引脚的内部上/下拉（主机输入脚常用于抑制悬空噪声）。
    pub miso_pull: Pull,
    /// SCK/MOSI/MISO 复用引脚的输出速度（`OSPEEDR`）。
    pub gpio_speed: Speed,
    /// 主机模式下达标：
    /// 为 `true` 时强制 `SSOE = 0`（不输出 NSS），用于多主机/外部片选场景；
    /// 默认 `false` 时按主机自动使能 `SSOE`。从机模式该字段无效。
    pub nss_output_disable: bool,
}

impl Default for Config {
    /// 默认值对照 `LL_SPI_StructInit` 的常规建议：模式 0、MSB 优先、1 MHz、
    /// MISO 无上下拉、GPIO 超高速档、使能 NSS 输出。
    fn default() -> Self {
        Self {
            mode: MODE_0,
            bit_order: BitOrder::MsbFirst,
            frequency: Hertz(1_000_000),
            miso_pull: Pull::None,
            gpio_speed: Speed::VeryHigh,
            nss_output_disable: false,
        }
    }
}

// ---------------------------------------------------------------------------
// 数据帧宽度
// ---------------------------------------------------------------------------

mod sealed_word {
    use crate::pac;

    /// 密封的帧宽度信息（防止外部为任意类型实现 [`super::Word`]）。
    pub trait SealedWord: Copy {
        /// 该帧长是否设置 `CR1.DFF`（`true` = 16 位帧）。
        const DFF_16BIT: bool;

        /// 按该帧宽对应的访问宽度读取 `DR`（RM 35.4.4 / HAL 收发路径）。
        fn read_dr(regs: &'static pac::spi1::RegisterBlock) -> Self;

        /// 按该帧宽对应的访问宽度写入 `DR`。
        fn write_dr(regs: &'static pac::spi1::RegisterBlock, val: Self);
    }
}

/// 可用于 SPI 收发的一个数据帧（字长）。
///
/// 实现于 `u8`（8 位帧）与 `u16`（16 位帧）。两个帧长的 `DR` 访问宽度不同：
/// 8 位帧用字节宽度、16 位帧用半字宽度，复刻 HAL 的访问方式。
#[allow(private_bounds)]
pub trait Word: sealed_word::SealedWord + Default + 'static {}

impl sealed_word::SealedWord for u8 {
    const DFF_16BIT: bool = false;

    #[inline]
    fn read_dr(regs: &'static pac::spi1::RegisterBlock) -> Self {
        // 8 位帧：HAL 经 (__IO uint8_t *)&DR 访问，读取宽度须与 FRXTH=0 匹配
        unsafe { core::ptr::read_volatile(regs.dr().as_ptr() as *const u8) }
    }

    #[inline]
    fn write_dr(regs: &'static pac::spi1::RegisterBlock, val: Self) {
        unsafe { core::ptr::write_volatile(regs.dr().as_ptr() as *mut u8, val) }
    }
}

impl Word for u8 {}

impl sealed_word::SealedWord for u16 {
    const DFF_16BIT: bool = true;

    #[inline]
    fn read_dr(regs: &'static pac::spi1::RegisterBlock) -> Self {
        // 16 位帧：HAL 经 (__IO uint16_t *)&DR 访问
        unsafe { core::ptr::read_volatile(regs.dr().as_ptr() as *const u16) }
    }

    #[inline]
    fn write_dr(regs: &'static pac::spi1::RegisterBlock, val: Self) {
        unsafe { core::ptr::write_volatile(regs.dr().as_ptr() as *mut u16, val) }
    }
}

impl Word for u16 {}

// ---------------------------------------------------------------------------
// 通信角色（主/从）
// ---------------------------------------------------------------------------

mod sealed_mode {
    /// 密封的通信角色（防止外部新增角色）。
    pub trait SealedCommunicationMode {}
}

/// SPI 通信角色：主机或从机。
#[allow(private_bounds)]
pub trait CommunicationMode: sealed_mode::SealedCommunicationMode {
    /// 是否为主机（写入 `CR1.MSTR`）。
    const IS_MASTER: bool;
}

/// 主机角色。
pub struct Master;

/// 从机角色。
pub struct Slave;

impl sealed_mode::SealedCommunicationMode for Master {}
impl CommunicationMode for Master {
    const IS_MASTER: bool = true;
}

impl sealed_mode::SealedCommunicationMode for Slave {}
impl CommunicationMode for Slave {
    const IS_MASTER: bool = false;
}

// ---------------------------------------------------------------------------
// 外设实例
// ---------------------------------------------------------------------------

/// SPI 实例所在 APB 总线（决定 RCC 使能/复位寄存器与外设时钟频率）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Bus {
    /// APB1（SPI2、SPI3）。
    Apb1,
    /// APB2（SPI1）。
    Apb2,
}

/// 密封的实例信息（防止外部为任意类型实现 [`Instance`]）。
mod sealed {
    use super::Bus;
    use crate::pac;

    /// SPI 实例的寄存器块与 RCC 位信息。
    pub(super) trait SealedInstance {
        /// 取该实例的寄存器块（基址来自 PAC `Periph` 的常量指针）。
        ///
        /// SPI1/2/3 共用同一 `RegisterBlock`（PAC 中 `spi2`/`spi3` 是 `spi1` 的别名）。
        fn regs() -> &'static pac::spi1::RegisterBlock;

        /// 所在 APB 总线。
        fn bus() -> Bus;

        /// `RCC_APB1ENR1`（SPI2/3）或 `RCC_APB2ENR`（SPI1）中的时钟使能（= 复位）位掩码。
        fn enable_mask() -> u32;
    }

    /// 密封的 SCK 引脚查询。
    pub trait SealedSckPin<T: super::Instance> {
        /// 该引脚在此实例下用作 SCK 时的复用功能编号（查 DS 端口复用功能映射表）。
        fn af_num(&self) -> u8;
    }

    /// 密封的 MOSI 引脚查询。
    pub trait SealedMosiPin<T: super::Instance> {
        /// 该引脚在此实例下用作 MOSI 时的复用功能编号。
        fn af_num(&self) -> u8;
    }

    /// 密封的 MISO 引脚查询。
    pub trait SealedMisoPin<T: super::Instance> {
        /// 该引脚在此实例下用作 MISO 时的复用功能编号。
        fn af_num(&self) -> u8;
    }

    /// 密封的 NSS（片选）引脚查询。
    pub trait SealedCsPin<T: super::Instance> {
        /// 该引脚在此实例下用作 NSS（从机片选）时的复用功能编号。
        fn af_num(&self) -> u8;
    }
}

/// SPI 外设实例（[`crate::peripherals::SPI1`] / [`crate::peripherals::SPI2`] /
/// [`crate::peripherals::SPI3`]）。
#[allow(private_bounds)]
pub trait Instance: sealed::SealedInstance + PeripheralType + Copy + 'static {}

/// 可作为某实例 SCK 的引脚。
#[allow(private_bounds)]
pub trait SckPin<T: Instance>: sealed::SealedSckPin<T> + Pin {}

/// 可作为某实例 MOSI 的引脚。
#[allow(private_bounds)]
pub trait MosiPin<T: Instance>: sealed::SealedMosiPin<T> + Pin {}

/// 可作为某实例 MISO 的引脚。
#[allow(private_bounds)]
pub trait MisoPin<T: Instance>: sealed::SealedMisoPin<T> + Pin {}

/// 可作为某实例 NSS（从机片选）的引脚。
#[allow(private_bounds)]
pub trait CsPin<T: Instance>: sealed::SealedCsPin<T> + Pin {}

/// 为实例生成 [`sealed::SealedInstance`] 与 [`Instance`] 实现。
///
/// `$bus`：所在总线；`$bit`：`RCC_APB1ENR1`（SPI2/3）或 `RCC_APB2ENR`（SPI1）
/// 中的位号（RM 6.10 / SVD：SPI1 = APB2 bit12、SPI2 = APB1ENR1 bit14、
/// SPI3 = APB1ENR1 bit15）；复位位与使能位同号，分别在 `APB2RSTR` / `APB1RSTR1`。
macro_rules! impl_instance {
    ($inst:ident, $pac:ident, $bus:ident, $bit:literal) => {
        impl sealed::SealedInstance for crate::peripherals::$inst {
            #[inline]
            fn regs() -> &'static pac::spi1::RegisterBlock {
                // 基址与类型均来自 PAC（SPI1 = 0x4001_3000、SPI2 = 0x4000_3800、
                // SPI3 = 0x4000_3c00），三个实例共用 spi1::RegisterBlock。
                unsafe { &*pac::$pac::PTR }
            }

            #[inline]
            fn bus() -> Bus {
                Bus::$bus
            }

            #[inline]
            fn enable_mask() -> u32 {
                1 << $bit
            }
        }

        impl Instance for crate::peripherals::$inst {}
    };
}

impl_instance!(SPI1, Spi1, Apb2, 12);
impl_instance!(SPI2, Spi2, Apb1, 14);
impl_instance!(SPI3, Spi3, Apb1, 15);

/// 为引脚生成 SCK 引脚实现（含 AF 号）。
macro_rules! impl_sck_pin {
    ($inst:ident, $pin:ident, $af:literal) => {
        impl sealed::SealedSckPin<crate::peripherals::$inst> for crate::peripherals::$pin {
            #[inline]
            fn af_num(&self) -> u8 {
                $af
            }
        }
        impl SckPin<crate::peripherals::$inst> for crate::peripherals::$pin {}
    };
}

/// 为引脚生成 MOSI 引脚实现（含 AF 号）。
macro_rules! impl_mosi_pin {
    ($inst:ident, $pin:ident, $af:literal) => {
        impl sealed::SealedMosiPin<crate::peripherals::$inst> for crate::peripherals::$pin {
            #[inline]
            fn af_num(&self) -> u8 {
                $af
            }
        }
        impl MosiPin<crate::peripherals::$inst> for crate::peripherals::$pin {}
    };
}

/// 为引脚生成 MISO 引脚实现（含 AF 号）。
macro_rules! impl_miso_pin {
    ($inst:ident, $pin:ident, $af:literal) => {
        impl sealed::SealedMisoPin<crate::peripherals::$inst> for crate::peripherals::$pin {
            #[inline]
            fn af_num(&self) -> u8 {
                $af
            }
        }
        impl MisoPin<crate::peripherals::$inst> for crate::peripherals::$pin {}
    };
}

/// 为引脚生成 NSS/CS 引脚实现（含 AF 号）。
macro_rules! impl_cs_pin {
    ($inst:ident, $pin:ident, $af:literal) => {
        impl sealed::SealedCsPin<crate::peripherals::$inst> for crate::peripherals::$pin {
            #[inline]
            fn af_num(&self) -> u8 {
                $af
            }
        }
        impl CsPin<crate::peripherals::$inst> for crate::peripherals::$pin {}
    };
}

// 引脚与 AF 号来自 DS V0.13 端口复用功能映射表（表 3-3 端口 A … 表 3-8 端口 F），
// 并用官方例程 Example_LL/SPI/SPI_TwoBoards_FullDuplex_Polling（PA5/PA6/PA7 = AF5）
// 交叉校准。SPI1/SPI2 的信号落在 AF5 列，SPI3 落在 AF6 列。

// SPI1（AF5）：SCK = PA5/PB3，MISO = PA6/PB4，MOSI = PA7/PB5，NSS = PA4/PA15。
// Note: PB3/PB4/PA15 属 SWJ 调试脚（JTDO/NJTRST/JTDI），复用需在板级确认调试口取舍。
impl_sck_pin!(SPI1, PA5, 5);
impl_sck_pin!(SPI1, PB3, 5);
impl_miso_pin!(SPI1, PA6, 5);
impl_miso_pin!(SPI1, PB4, 5);
impl_mosi_pin!(SPI1, PA7, 5);
impl_mosi_pin!(SPI1, PB5, 5);
impl_cs_pin!(SPI1, PA4, 5);
impl_cs_pin!(SPI1, PA15, 5);

// SPI2（AF5，NSS 的 PD15 为 AF6）：SCK = PB13/PF1/PF3/PF4，
// MISO = PB14/PA10，MOSI = PB15/PA11，NSS = PB12/PF0(AF5)/PD15(AF6)。
impl_sck_pin!(SPI2, PB13, 5);
impl_sck_pin!(SPI2, PF1, 5);
impl_sck_pin!(SPI2, PF3, 5);
impl_sck_pin!(SPI2, PF4, 5);
impl_miso_pin!(SPI2, PB14, 5);
impl_miso_pin!(SPI2, PA10, 5);
impl_mosi_pin!(SPI2, PB15, 5);
impl_mosi_pin!(SPI2, PA11, 5);
impl_cs_pin!(SPI2, PB12, 5);
impl_cs_pin!(SPI2, PF0, 5);
impl_cs_pin!(SPI2, PD15, 6);

// SPI3（AF6）：SCK = PB3/PC10，MISO = PB4/PC11，MOSI = PB5/PC12，
// NSS = PA4/PA15。
impl_sck_pin!(SPI3, PB3, 6);
impl_sck_pin!(SPI3, PC10, 6);
impl_miso_pin!(SPI3, PB4, 6);
impl_miso_pin!(SPI3, PC11, 6);
impl_mosi_pin!(SPI3, PB5, 6);
impl_mosi_pin!(SPI3, PC12, 6);
impl_cs_pin!(SPI3, PA4, 6);
impl_cs_pin!(SPI3, PA15, 6);

// ---------------------------------------------------------------------------
// 驱动
// ---------------------------------------------------------------------------

/// SPI 同步驱动。
///
/// 类型参数 `CM` 为主/从角色（[`Master`] / [`Slave`]）。引脚所有权由 [`Peri`] 保证：
/// 同一个物理引脚同一时刻至多被一个驱动持有。驱动被 Drop 时各引脚的 [`Flex`]
/// 会恢复为模拟（高阻）态。
pub struct Spi<'d, T: Instance, CM: CommunicationMode> {
    _peri: Peri<'d, T>,
    _sck: Flex<'d>,
    _mosi: Flex<'d>,
    _miso: Flex<'d>,
    _cs: Option<Flex<'d>>,
    word_size_16: bool,
    _mode: PhantomData<CM>,
}

impl<'d, T: Instance> Spi<'d, T, Master> {
    /// 创建主机驱动，并完成外设与引脚初始化（阻塞）。
    ///
    /// 流程对照 `HAL_SPI_Init`：
    /// 1. 使能该实例时钟（APB2 或 APB1，含 LL 要求的读回等待）并复位该实例；
    /// 2. 把 `sck`/`mosi`/`miso` 配为 AF（引脚配置见各 `impl_*_pin!`）；
    /// 3. `SPE = 0` → 写 `CR1`（CPOL/CPHA/MSTR/BR/BIDIMODE=0/RXONLY=0）→ `CR2`
    ///    （SSOE、FRF=0、FRXTH=0、关闭 DMA/中断）→ 清 `I2SCFGR.I2SMOD` → `SPE = 1`。
    ///
    /// 主机使用软件 NSS：`SSM = 1`、`SSI = 1`，`SSOE = !config.nss_output_disable`。
    ///
    /// # Errors
    ///
    /// 频率为 0 时返回 [`Error::InvalidConfig`]。
    pub fn new_blocking(
        peri: Peri<'d, T>,
        sck: Peri<'d, impl SckPin<T>>,
        mosi: Peri<'d, impl MosiPin<T>>,
        miso: Peri<'d, impl MisoPin<T>>,
        config: Config,
    ) -> Result<Self, Error> {
        enable_and_reset::<T>();

        // AF 号要在引脚被 Flex 接管之前取出（AF 查表见上方 impl_*_pin! 宏）
        let sck_af = sck.af_num();
        let mosi_af = mosi.af_num();
        let miso_af = miso.af_num();

        let mut this = Self {
            _peri: peri,
            _sck: Flex::new(sck),
            _mosi: Flex::new(mosi),
            _miso: Flex::new(miso),
            _cs: None,
            word_size_16: false,
            _mode: PhantomData,
        };
        this.configure_pins(sck_af, mosi_af, miso_af, None, &config);
        init_regs::<T, Master>(&config)?;
        Ok(this)
    }
}

impl<'d, T: Instance> Spi<'d, T, Slave> {
    /// 创建从机驱动，并完成外设与引脚初始化（阻塞）。
    ///
    /// 与 [`Spi::<T, Master>::new_blocking`] 流程相同，区别在于：
    /// - 使用硬件 NSS（`SSM = 0`，`MSTR = 0`），需额外占用 `cs` 作为 NSS 输入；
    /// - 波特率分频无意义（HAL 在从机模式强制 `BR = fPCLK/2`），`config.frequency` 被忽略；
    /// - `SSOE` 置 0（从机不输出 NSS）。
    ///
    /// # Errors
    ///
    /// 频率为 0 时返回 [`Error::InvalidConfig`]。
    pub fn new_blocking_slave(
        peri: Peri<'d, T>,
        sck: Peri<'d, impl SckPin<T>>,
        mosi: Peri<'d, impl MosiPin<T>>,
        miso: Peri<'d, impl MisoPin<T>>,
        cs: Peri<'d, impl CsPin<T>>,
        config: Config,
    ) -> Result<Self, Error> {
        enable_and_reset::<T>();

        let sck_af = sck.af_num();
        let mosi_af = mosi.af_num();
        let miso_af = miso.af_num();
        let cs_af = cs.af_num();

        let mut this = Self {
            _peri: peri,
            _sck: Flex::new(sck),
            _mosi: Flex::new(mosi),
            _miso: Flex::new(miso),
            _cs: Some(Flex::new(cs)),
            word_size_16: false,
            _mode: PhantomData,
        };
        this.configure_pins(sck_af, mosi_af, miso_af, Some(cs_af), &config);
        init_regs::<T, Slave>(&config)?;
        Ok(this)
    }
}

impl<'d, T: Instance, CM: CommunicationMode> Spi<'d, T, CM> {
    /// 配置 SCK/MOSI/MISO（及从机 NSS）为复用功能。
    ///
    /// 复用模式下引脚方向由外设控制，故统一用推挽输出类型；上拉仅对输入脚有意义
    /// （SCK/MOSI 主机输出、MISO 输入；从机 CS 为输入，取上拉避免悬空误选中）。
    fn configure_pins(
        &mut self,
        sck_af: u8,
        mosi_af: u8,
        miso_af: u8,
        cs_af: Option<u8>,
        config: &Config,
    ) {
        self._sck
            .set_as_alternate(sck_af, config.gpio_speed, Pull::None, OutputType::PushPull);
        self._mosi
            .set_as_alternate(mosi_af, config.gpio_speed, Pull::None, OutputType::PushPull);
        self._miso
            .set_as_alternate(miso_af, config.gpio_speed, config.miso_pull, OutputType::PushPull);
        if let (Some(cs), Some(af)) = (self._cs.as_mut(), cs_af) {
            cs.set_as_alternate(af, config.gpio_speed, Pull::Up, OutputType::PushPull);
        }
    }

    /// 切换数据帧宽度（`CR1.DFF`）。
    ///
    /// `DFF` 只能在 `SPE = 0` 时修改（RM 35.4.1 对通信中改配置的限制），故先关后开；
    /// 与目标宽度一致时不做任何寄存器操作。
    fn set_word_size<W: Word>(&mut self) {
        let want16 = W::DFF_16BIT;
        if self.word_size_16 == want16 {
            return;
        }
        let regs = T::regs();
        regs.cr1().modify(|_, w| w.spe().clear_bit());
        regs.cr1().modify(|_, w| w.dff().bit(want16));
        regs.cr1().modify(|_, w| w.spe().set_bit());
        self.word_size_16 = want16;
    }

    /// 阻塞发送 `words` 的全部数据，同时读走全双工回读的字节（丢弃）。
    ///
    /// 8 位/16 位帧由 `W` 推断（[`u8`] / [`u16`]）。
    pub fn blocking_write<W: Word>(&mut self, words: &[W]) -> Result<(), Error> {
        self.set_word_size::<W>();
        let regs = T::regs();
        clear_pending_errors::<W>(regs);
        for &word in words {
            // 全双工收发：发送的同时读走 RXFIFO，避免溢出并保证末帧正确收尾
            transfer_word::<W>(regs, word)?;
        }
        Ok(())
    }

    /// 阻塞接收填满 `words`（主机模式通过发送哑元产生时钟）。
    pub fn blocking_read<W: Word>(&mut self, words: &mut [W]) -> Result<(), Error> {
        self.set_word_size::<W>();
        let regs = T::regs();
        clear_pending_errors::<W>(regs);
        for word in words.iter_mut() {
            *word = transfer_word::<W>(regs, W::default())?;
        }
        Ok(())
    }

    /// 阻塞全双工收发：读 `read.len().max(write.len())` 个帧。
    ///
    /// 发送侧不足时以 [`Default::default`] 补零；接收侧短于发送侧时多出的部分丢弃。
    pub fn blocking_transfer<W: Word>(&mut self, read: &mut [W], write: &[W]) -> Result<(), Error> {
        self.set_word_size::<W>();
        let regs = T::regs();
        clear_pending_errors::<W>(regs);
        let len = read.len().max(write.len());
        for i in 0..len {
            let tx = write.get(i).copied().unwrap_or_default();
            let rx = transfer_word::<W>(regs, tx)?;
            if let Some(slot) = read.get_mut(i) {
                *slot = rx;
            }
        }
        Ok(())
    }

    /// 阻塞原地全双工收发：逐帧发旧值、收新值写回同一缓冲区。
    pub fn blocking_transfer_in_place<W: Word>(&mut self, words: &mut [W]) -> Result<(), Error> {
        self.set_word_size::<W>();
        let regs = T::regs();
        clear_pending_errors::<W>(regs);
        for word in words.iter_mut() {
            *word = transfer_word::<W>(regs, *word)?;
        }
        Ok(())
    }

    /// 等待全部收发彻底结束（对照 `SPI_EndRxTxTransaction`）。
    ///
    /// 依次等待：`SR.BSY = 0`（最后一帧发完且发送缓冲已空）→ `SR.FRLVL = 00`（RxFIFO 空）。
    pub fn flush(&mut self) -> Result<(), Error> {
        let regs = T::regs();
        wait_flag(regs, |sr| !sr.bsy().bit_is_set())?;
        wait_flag(regs, |sr| sr.frlvl().bits() == 0)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 内部实现
// ---------------------------------------------------------------------------

/// 使能实例时钟并复位实例（按所在总线分派）。
///
/// 时钟使能经 [`crate::rcc::enable_apb2`] / [`crate::rcc::enable_apb1`]（内部已复刻
/// LL 的"置位后读回一次"），复位经对应的 `reset_*`（Force + Release）。
#[inline]
fn enable_and_reset<T: Instance>() {
    match T::bus() {
        Bus::Apb1 => {
            crate::rcc::enable_apb1(T::enable_mask(), 0);
            // Safety: 复位会立即停止该外设并恢复其寄存器复位值；此刻本驱动尚未配置
            // 它，且外设单例由 Peri 保证只被取出一次。
            unsafe { crate::rcc::reset_apb1(T::enable_mask(), 0) };
        }
        Bus::Apb2 => {
            crate::rcc::enable_apb2(T::enable_mask());
            // Safety: 同上。
            unsafe { crate::rcc::reset_apb2(T::enable_mask()) };
        }
    }
}

/// 取该实例的外设时钟频率（Hz）：SPI1 为 PCLK2、SPI2/SPI3 为 PCLK1。
#[inline]
fn kernel_clock<T: Instance>() -> u32 {
    let clocks = crate::rcc::clocks();
    match T::bus() {
        Bus::Apb1 => clocks.apb1.0,
        Bus::Apb2 => clocks.apb2.0,
    }
}

/// 由 `fPCLK` 与目标频率换算 `CR1.BR[2:0]`（RM 35.4.1：000 = fPCLK/2 … 111 = fPCLK/256）。
///
/// 取使实际 SCK 频率**不超过**目标值的最快分频档（即最小 `BR`）。
fn compute_baud_rate(pclk: u32, frequency: u32) -> u8 {
    for br in 0u8..8 {
        if pclk / (2u32 << br) <= frequency {
            return br;
        }
    }
    7
}

/// 写 SPI 配置寄存器（不含 GPIO；引脚 AF 配置在构造器内完成）。
///
/// 顺序严格对照 `HAL_SPI_Init`：关 `SPE` → 写 `CR1`（一次整写，未列出的位保持复位
/// 值 0）→ 写 `CR2` → 清 `I2SCFGR.I2SMOD` →（HAL 不置 `SPE`，由 `LL_SPI_Enable`
/// 单独使能）置 `SPE`。
fn init_regs<T: Instance, CM: CommunicationMode>(config: &Config) -> Result<(), Error> {
    if config.frequency.0 == 0 {
        return Err(Error::InvalidConfig);
    }

    let regs = T::regs();
    let is_master = CM::IS_MASTER;

    // HAL: __HAL_SPI_DISABLE —— 配置前必须先关 SPI（RM 35.4.1 要求 SPE=0 时改配置）
    regs.cr1().modify(|_, w| w.spe().clear_bit());

    let cpol = matches!(config.mode.polarity, Polarity::IdleHigh);
    let cpha = matches!(config.mode.phase, Phase::SecondEdge);
    let lsbfirst = matches!(config.bit_order, BitOrder::LsbFirst);
    // 从机模式分频无意义（HAL 强制 fPCLK/2，即 BR = 0）
    let br = if is_master {
        compute_baud_rate(kernel_clock::<T>(), config.frequency.0)
    } else {
        0
    };
    // 主机用软件 NSS（SSM=1、SSI=1）；从机用硬件 NSS（SSM=0）
    let ssm = is_master;
    let ssi = is_master;
    let ssoe = is_master && !config.nss_output_disable;

    // ---- CR1（RM 35.4.1）----
    regs.cr1().write(|w| {
        w.cpha().bit(cpha);
        w.cpol().bit(cpol);
        w.mstr().bit(is_master);
        // BR 为 3 位多 bit 字段，svd2rust 的 bits() 为 unsafe（本驱动不引入 CRC/双向模式）
        unsafe { w.br().bits(br) };
        w.spe().clear_bit();
        w.lsbfirst().bit(lsbfirst);
        w.ssi().bit(ssi);
        w.ssm().bit(ssm);
        w.rxonly().clear_bit();
        w.dff().clear_bit();
        w.crcnext().clear_bit();
        w.crcen().clear_bit();
        w.bidioe().clear_bit();
        w.bidimode().clear_bit();
        w
    });

    // ---- CR2（RM 35.4.2）----
    regs.cr2().write(|w| {
        w.ssoe().bit(ssoe);
        // FRXTH=0：FIFO 有数据即触发 RXNE（单字节接收前提，保持 HAL 默认）
        w.frxth().clear_bit();
        w.clrtxfifo().clear_bit();
        w.frf().clear_bit();
        w.errie().clear_bit();
        w.rxneie().clear_bit();
        w.txeie().clear_bit();
        w.rxdmaen().clear_bit();
        w.txdmaen().clear_bit();
        w
    });

    // ---- 确保处于 SPI 模式（清 I2SCFGR.I2SMOD，HAL_SPI_Init 收尾）----
    regs.i2s_cfgr().modify(|_, w| w.i2smod().clear_bit());

    // ---- 使能 SPI（HAL: __HAL_SPI_ENABLE / LL_SPI_Enable）----
    regs.cr1().modify(|_, w| w.spe().set_bit());
    Ok(())
}

/// 判定 `SR` 快照中的错误标志位（RM 35.4.3）。
///
/// 只读快照，不清标志（`OVR`/`MODF` 的清零序列由 [`clear_pending_errors`] 负责）。
/// 优先级：`Overrun` > `ModeFault` > `Crc` > `Framing`。
#[inline]
fn check_error_flags(sr: &pac::spi1::sr::R) -> Result<(), Error> {
    if sr.ovr().bit_is_set() {
        Err(Error::Overrun)
    } else if sr.modf().bit_is_set() {
        Err(Error::ModeFault)
    } else if sr.crcerr().bit_is_set() {
        Err(Error::Crc)
    } else if sr.fre().bit_is_set() {
        Err(Error::Framing)
    } else {
        Ok(())
    }
}

/// 轮询 `SR` 直到 `cond` 为真；期间若出现错误标志则立即返回对应错误。
///
/// 超过 [`TIMEOUT_LOOPS`] 次返回 [`Error::Timeout`]。
#[inline]
fn wait_flag(
    regs: &'static pac::spi1::RegisterBlock,
    cond: impl Fn(&pac::spi1::sr::R) -> bool,
) -> Result<(), Error> {
    for _ in 0..TIMEOUT_LOOPS {
        let sr = regs.sr().read();
        check_error_flags(&sr)?;
        if cond(&sr) {
            return Ok(());
        }
    }
    Err(Error::Timeout)
}

/// 全双工收发一个帧：等 `TXE` → 写 `DR` → 等 `RXNE` → 读 `DR`（对照
/// `HAL_SPI_TransmitReceive` 的逐帧循环）。
fn transfer_word<W: Word>(
    regs: &'static pac::spi1::RegisterBlock,
    tx: W,
) -> Result<W, Error> {
    wait_flag(regs, |sr| sr.txe().bit_is_set())?;
    W::write_dr(regs, tx);
    wait_flag(regs, |sr| sr.rxne().bit_is_set())?;
    Ok(W::read_dr(regs))
}

/// 清除上一事务可能残留的接收数据与错误标志。
///
/// - 读 `DR`（按当前帧宽）直到 `RXNE = 0`，丢弃残留接收数据；
/// - 读 `DR` 再读 `SR` 完成 `OVR` 清除序列（RM 35.4.3 / `LL_SPI_ClearFlag_OVR`）；
/// - `CRCERR` 为写 0 清除（`RC_W0`），若置位则写 0 清掉。
///
/// `MODF` 不在本函数自动清除：其清零需 `SPE=0`，且主机软件 NSS 下不会出现。
/// 本函数只做清理、不判定错误，故无返回值。
#[inline]
fn clear_pending_errors<W: Word>(regs: &'static pac::spi1::RegisterBlock) {
    for _ in 0..TIMEOUT_LOOPS {
        if !regs.sr().read().rxne().bit_is_set() {
            break;
        }
        let _ = W::read_dr(regs);
    }
    // OVR 清除序列：读 DR 后读 SR
    let _ = W::read_dr(regs);
    let sr = regs.sr().read();
    if sr.crcerr().bit_is_set() {
        regs.sr().modify(|_, w| w.crcerr().clear_bit());
    }
}
