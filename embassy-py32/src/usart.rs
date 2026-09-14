//! Universal Synchronous/Asynchronous Receiver/Transmitter (USART)（同步 Blocking 驱动）
//!
//! 适配 PY32E407。寄存器流程依据：
//! - HAL C: `py32e407_hal_usart.c`（`HAL_USART_Init` / `USART_SetConfig` /
//!   `HAL_USART_Transmit` / `HAL_USART_Receive`）与 `py32e407_hal_usart.h`
//!   （`USART_BRR_SAMPLING16` 波特率宏）
//! - LL:    `py32e407_ll_usart.h`（`LL_USART_Init`、`LL_USART_Enable`、
//!   `LL_USART_IsActiveFlag_TXE/TC/RXNE`、`LL_USART_SetBaudRate`）
//! - RM:    PY32E407 参考手册 V0.6 第 37 章（USART，37.4 寄存器）与第 6 章（RCC/CCIPR）
//! - DS:    PY32E407 数据手册 V0.13 表 3-3…3-7（端口复用功能映射，USART 引脚 AF 号）
//!
//! 目标外设是 **USART1 / USART2 / USART3**（RM 第 37 章，PAC: [`crate::pac::usart1`]，
//! 三实例共用同一 `RegisterBlock`；PAC 中 `usart2`/`usart3` 是 `usart1` 的别名）。
//! 本芯片另有两套串口外设与本模块无关：UART1/2/3（RM 第 38 章，16550 风格、无
//! `UE` 位，见 [`crate::uart`]）与 LPUART1（RM 第 39 章，见 [`crate::lpuart`]）。
//!
//! # 关键实现事实（已用 RM + SVD + PAC + HAL 交叉验证）
//!
//! - **数据寄存器是偏移 0x04 的单个 `DR`**（读=收、写=发）：HAL/LL、RM 37.4.2 与
//!   CMSIS 头文件一致。PAC 里另有 `rdr`（offset 0x20），但 RM 第 37 章寄存器表止于
//!   0x1C，该 `rdr` 属 SVD 生成缺陷（疑为 LPUART 独立 RDR/TDR 的抄写残留）。
//!   本驱动收发**一律经 `regs.tdr()`（0x04）**，不使用 `rdr`。
//! - **有 `UE` 使能位**（`CR1` bit13），初始化须 `UE=0` 时配置、最后置 `UE=1`；
//!   HAL 不等待 `TEACK`/`REACK`（该等待是 LPUART 的要求），本驱动照做。
//! - **`CR1.PS` 极性与第 38 章 UART 相反**：RM 37.4.4 规定 `PS = 0` 为**偶**校验、
//!   `PS = 1` 为**奇**校验（同 STM32 与本芯片 LPUART）。切勿与 [`crate::uart`] 互抄。
//! - **状态位在 `SR`**：`PE`(0) `FE`(1) `NE`(2) `ORE`(3) `IDLE`(4) `RXNE`(5)
//!   `TC`(6) `TXE`(7)。发送等 `TXE` 后写 `DR`；接收等 `RXNE` 后**先读 `SR` 再读 `DR`**
//!   （该序列同时清 `RXNE` 与 `PE/FE/NE/ORE/IDLE`，RM 37.4.1；`TC` 由写 `DR` 清零）。
//! - **外设时钟源可选**：`RCC_CCIPR.USART1SEL/USART2SEL/USART3SEL`（00 = PCLK
//!   ——USART1 为 PCLK2、USART2/3 为 PCLK1；01 = SYSCLK；10 = HSI；11 = LSE）。
//!   本驱动读取该字段换算波特率（LSE 档未实现，见 [`Error::InvalidConfig`]）。
//!
//! # 使用示例
//!
//! ```ignore
//! use embassy_py32::usart::{Config, Usart};
//! use embassy_py32::init;
//!
//! let p = init(Default::default());
//! // PA9 = TX、PA10 = RX（AF7），与官方 STK 例程 SCI_HyperTerminal_Polling 一致
//! let mut usart = Usart::new_blocking(p.USART1, p.PA9, p.PA10, Config::default()).unwrap();
//! usart.write(b"hello\r\n").unwrap();
//! ```
//!
//! # 命名约定
//!
//! 构造函数一律带 `_blocking` 后缀（[`Usart::new_blocking`]、[`UsartTx::new_blocking`]、
//! [`UsartRx::new_blocking`]，以及 [`Usart::new_blocking_with_tx`] /
//! [`Usart::new_blocking_with_rx`]），**`new` 留给后续的 Async 版本**——与
//! embassy-stm32 的 `Uart::new_blocking` / `UartTx::new_blocking` 命名完全一致。
//!
//! 仅实现同步（轮询 + 超时）驱动：不使用 DMA 与中断（`CR1`/`CR3` 的中断使能位全部
//! 清零），不启用 FIFO、硬件流控（CTS/RTS）与同步模式（`CR2.CLKEN=0`，不占用 CK 引脚），
//! 9 位字长（`CR1.M0` 单独置位）亦未实现。后续将按需扩展 Async。

use crate::gpio::{Flex, OutputType, Pin, Pull, Speed};
use crate::pac;
use crate::{Peri, PeripheralType};

/// 轮询等待状态位的循环上限（次数）。
///
/// 量级依据：HSI16 下每次循环约 4~6 个周期，`1_000_000` 次 ≈ 0.25~0.4 s，
/// 足以覆盖 1200 bps 下单字节 8.3 ms 的等待并留出约 30 倍裕量。
const TIMEOUT_LOOPS: u32 = 1_000_000;

/// USART 错误类型。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// 奇偶校验错误（`SR.PE`，RM 37.4.1）。
    Parity,
    /// 帧错误：接收数据中未检测到有效停止位（`SR.FE`）。
    Framing,
    /// 噪声错误：起始位采样检测到噪声（`SR.NE`）。
    Noise,
    /// 接收溢出：上一个字符尚未被读走又收到新字符，数据寄存器已被覆盖（`SR.ORE`）。
    Overrun,
    /// 等待状态位超过 [`TIMEOUT_LOOPS`] 仍未就绪。
    Timeout,
    /// 配置非法：波特率为 0、算出的分频值越界（须落在 16..=0xFFFF），
    /// 或该实例的时钟源为 LSE（本 crate 的 rcc 模块未实现 LSE，频率不可知）。
    InvalidConfig,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            Self::Parity => "Parity Check Error",
            Self::Framing => "Framing Error",
            Self::Noise => "Noise Error",
            Self::Overrun => "RX Buffer Overrun",
            Self::Timeout => "Operation Timed Out",
            Self::InvalidConfig => "Invalid Configuration",
        };
        f.write_str(msg)
    }
}

impl core::error::Error for Error {}

/// 数据位长度（`CR1.M1` / `CR1.M0`，RM 37.4.4）。
///
/// 编码来自 `py32e407_hal_usart.h`：`M[1:0] = 00` 为 8 位、`10` 为 7 位、
/// `01` 为 9 位。9 位字长需要 16 位数据通路，本同步驱动暂未实现。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataBits {
    /// 每字符 7 个数据位（`M0 = 0, M1 = 1`）。
    DataBits7,
    /// 每字符 8 个数据位（`M0 = M1 = 0`，复位默认）。
    DataBits8,
}

impl DataBits {
    /// 映射为 `(M0, M1)`。
    #[inline]
    fn to_cr1(self) -> (bool, bool) {
        match self {
            Self::DataBits7 => (false, true),
            Self::DataBits8 => (false, false),
        }
    }
}

/// 奇偶校验方式（`CR1.PCE` / `CR1.PS`，RM 37.4.4）。
///
/// 注意：`PS = 0` 为**偶**校验、`PS = 1` 为**奇**校验——与 STM32 及本芯片
/// 第 39 章 LPUART 一致，但与本芯片第 38 章 UART（`PS = 1` 为偶校验）**相反**。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Parity {
    /// 不生成也不检测校验位。
    ParityNone,
    /// 偶校验（`PCE = 1, PS = 0`）。
    ParityEven,
    /// 奇校验（`PCE = 1, PS = 1`）。
    ParityOdd,
}

impl Parity {
    /// 映射为 `(PCE, PS)`。
    #[inline]
    fn to_cr1(self) -> (bool, bool) {
        match self {
            Self::ParityNone => (false, false),
            Self::ParityEven => (true, false),
            Self::ParityOdd => (true, true),
        }
    }
}

/// 停止位长度（`CR2.STOP[1:0]`，RM 37.4.5）。
///
/// 编码：`00` = 1 个、`01` = 0.5 个、`10` = 2 个、`11` = 1.5 个停止位。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StopBits {
    /// 1 个停止位（`STOP = 00`，复位默认）。
    STOP1,
    /// 0.5 个停止位（`STOP = 01`，仅接收端有效）。
    STOP0_5,
    /// 2 个停止位（`STOP = 10`）。
    STOP2,
    /// 1.5 个停止位（`STOP = 11`）。
    STOP1_5,
}

impl StopBits {
    /// 映射为 `CR2.STOP[1:0]` 的 2-bit 编码。
    #[inline]
    fn to_cr2(self) -> u8 {
        match self {
            Self::STOP1 => 0b00,
            Self::STOP0_5 => 0b01,
            Self::STOP2 => 0b10,
            Self::STOP1_5 => 0b11,
        }
    }
}

/// 位序（`CR1.MSB`，RM 37.4.4）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BitOrder {
    /// LSB 优先（复位默认）。
    LsbFirst,
    /// MSB 优先。
    MsbFirst,
}

/// USART 配置。
///
/// `#[non_exhaustive]`，外部构造请用 [`Config::default()`] 后逐字段赋值。
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// 波特率，单位 bps（必须非 0）。
    pub baudrate: u32,
    /// 数据位长度。
    pub data_bits: DataBits,
    /// 停止位长度。
    pub stop_bits: StopBits,
    /// 奇偶校验方式。
    pub parity: Parity,
    /// 位序。
    pub bit_order: BitOrder,
    /// 交换 TX/RX 引脚功能（`CR1.SWAP`），用于交叉线序的板级适配。
    pub swap_tx_rx: bool,
    /// TX 引脚的内部上/下拉。
    pub tx_pull: Pull,
    /// RX 引脚的内部上/下拉。
    ///
    /// 默认取 [`Pull::Up`]：与官方 STK 例程 `SCI_HyperTerminal_Polling`
    /// （`LL_GPIO_PULL_UP`）一致，避免 RX 悬空时被噪声触发起始位。
    pub rx_pull: Pull,
    /// 复用引脚的输出速度（`OSPEEDR`）。
    pub speed: Speed,
}

impl Default for Config {
    /// 默认值对照 `LL_USART_StructInit` 与官方 STK 例程的引脚配置：
    /// 115200-8N1、LSB 优先、不交换引脚、全双工、GPIO 超高速档。
    fn default() -> Self {
        Self {
            baudrate: 115_200,
            data_bits: DataBits::DataBits8,
            stop_bits: StopBits::STOP1,
            parity: Parity::ParityNone,
            bit_order: BitOrder::LsbFirst,
            swap_tx_rx: false,
            tx_pull: Pull::None,
            rx_pull: Pull::Up,
            speed: Speed::VeryHigh,
        }
    }
}

/// USART 实例所在 APB 总线（决定 RCC 使能/复位寄存器与默认时钟源）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Bus {
    /// APB1（USART2、USART3）。
    Apb1,
    /// APB2（USART1）。
    Apb2,
}

// ---------------------------------------------------------------------------
// 外设实例
// ---------------------------------------------------------------------------

/// 密封的实例信息（防止外部为任意类型实现 [`Instance`]）。
mod sealed {
    use super::Bus;
    use crate::pac;

    /// USART 实例的寄存器块与 RCC 位信息。
    pub(super) trait SealedInstance {
        /// 取该实例的寄存器块（基址来自 PAC `Periph` 的常量指针）。
        fn regs() -> &'static pac::usart1::RegisterBlock;

        /// 所在 APB 总线。
        fn bus() -> Bus;

        /// `RCC_APB1ENR1` / `RCC_APB2ENR` 中的时钟使能（= 复位）位掩码。
        fn enable_mask() -> u32;

        /// `RCC_CCIPR` 中该实例时钟源选择字段的 2-bit 值
        /// （`00` = PCLK、`01` = SYSCLK、`10` = HSI、`11` = LSE）。
        fn sel_bits() -> u8;
    }

    /// 密封的 TX 引脚查询。
    pub trait SealedTxPin<T: super::Instance> {
        /// 该引脚在此实例下用作 TX 时的复用功能编号（查 DS 端口复用功能映射表）。
        fn af_num(&self) -> u8;
    }

    /// 密封的 RX 引脚查询。
    pub trait SealedRxPin<T: super::Instance> {
        /// 该引脚在此实例下用作 RX 时的复用功能编号（查 DS 端口复用功能映射表）。
        fn af_num(&self) -> u8;
    }
}

/// USART 外设实例（[`crate::peripherals::USART1`] / [`crate::peripherals::USART2`] /
/// [`crate::peripherals::USART3`]）。
#[allow(private_bounds)]
pub trait Instance: sealed::SealedInstance + PeripheralType + Copy + 'static {}

/// 可作为某实例 TX 的引脚。
#[allow(private_bounds)]
pub trait TxPin<T: Instance>: sealed::SealedTxPin<T> + Pin {}

/// 可作为某实例 RX 的引脚。
#[allow(private_bounds)]
pub trait RxPin<T: Instance>: sealed::SealedRxPin<T> + Pin {}

/// 为实例生成 [`sealed::SealedInstance`] 与 [`Instance`] 实现。
///
/// `$bus`：所在总线；`$bit`：`RCC_APB1ENR1`（USART2/3）或 `RCC_APB2ENR`（USART1）
/// 中的位号（RM 6.10 寄存器描述：USART1 = APB2 bit14、USART2 = APB1ENR1 bit17、
/// USART3 = APB1ENR1 bit18）；`$sel`：`RCC_CCIPR` 中对应的时钟源选择字段访问器。
macro_rules! impl_instance {
    ($inst:ident, $pac:ident, $bus:ident, $bit:literal, $sel:ident) => {
        impl sealed::SealedInstance for crate::peripherals::$inst {
            #[inline]
            fn regs() -> &'static pac::usart1::RegisterBlock {
                // 基址与类型均来自 PAC（USART1 = 0x4001_3800、USART2 = 0x4000_4400、
                // USART3 = 0x4000_4800），三个实例共用 usart1::RegisterBlock。
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

            #[inline]
            fn sel_bits() -> u8 {
                let rcc = unsafe { &*pac::Rcc::PTR };
                rcc.ccipr().read().$sel().bits()
            }
        }

        impl Instance for crate::peripherals::$inst {}
    };
}

impl_instance!(USART1, Usart1, Apb2, 14, usart1sel);
impl_instance!(USART2, Usart2, Apb1, 17, usart2sel);
impl_instance!(USART3, Usart3, Apb1, 18, usart3sel);

/// 为引脚生成 TX 引脚实现。
macro_rules! impl_tx_pin {
    ($inst:ident, $pin:ident) => {
        impl sealed::SealedTxPin<crate::peripherals::$inst> for crate::peripherals::$pin {
            #[inline]
            fn af_num(&self) -> u8 {
                // DS 复用表：USART1/2/3 的全部 TX/RX 信号均位于 AF7 列。
                7
            }
        }

        impl TxPin<crate::peripherals::$inst> for crate::peripherals::$pin {}
    };
}

/// 为引脚生成 RX 引脚实现。
macro_rules! impl_rx_pin {
    ($inst:ident, $pin:ident) => {
        impl sealed::SealedRxPin<crate::peripherals::$inst> for crate::peripherals::$pin {
            #[inline]
            fn af_num(&self) -> u8 {
                // DS 复用表：USART1/2/3 的全部 TX/RX 信号均位于 AF7 列。
                7
            }
        }

        impl RxPin<crate::peripherals::$inst> for crate::peripherals::$pin {}
    };
}

// 引脚与 AF 号来自 DS V0.13 端口复用功能映射表（表 3-3 端口 A … 表 3-7 端口 E）。
// DS 的 PDF 抽取表会丢失空单元格，故已用"引脚定义表 + 固件宏 `GPIO_AF7_USART1/2/3`
// = 0x07 + 已实现的 LPUART 引脚（如 PB10 = LPUART1_RX 在 AF8，故同行的 USART3_TX
// 只能落在 AF7）"三路交叉校准；USART1/2/3 的全部信号在同端口内唯一落在 AF7 列。
//
// USART1（PA9/PA10 与官方例程 SCI_HyperTerminal_Polling 一致）：
impl_tx_pin!(USART1, PA9);
impl_rx_pin!(USART1, PA10);
impl_tx_pin!(USART1, PB6);
impl_rx_pin!(USART1, PB7);
impl_tx_pin!(USART1, PC4);
impl_rx_pin!(USART1, PC5);
impl_tx_pin!(USART1, PE0);
impl_rx_pin!(USART1, PE1);

// USART2：
impl_tx_pin!(USART2, PA2);
impl_rx_pin!(USART2, PA3);
// Note: PA14/PA15/PB3/PB4 属 SWJ 调试脚（SWCLK/SWDIO/JTDO/JTRST），
// 用作 USART2 需在板级确认调试口取舍，驱动只做 GPIO 层面的 AF 配置。
impl_tx_pin!(USART2, PA14);
impl_rx_pin!(USART2, PA15);
impl_tx_pin!(USART2, PB3);
impl_rx_pin!(USART2, PB4);
impl_tx_pin!(USART2, PD5);
impl_rx_pin!(USART2, PD6);

// USART3：
impl_rx_pin!(USART3, PB8);
impl_tx_pin!(USART3, PB9);
impl_tx_pin!(USART3, PB10);
impl_rx_pin!(USART3, PB11);
impl_tx_pin!(USART3, PC10);
impl_rx_pin!(USART3, PC11);
impl_tx_pin!(USART3, PD8);
impl_rx_pin!(USART3, PD9);
impl_rx_pin!(USART3, PE15);

// ---------------------------------------------------------------------------
// 驱动
// ---------------------------------------------------------------------------

/// 双向 USART 驱动，内部由 [`UsartTx`] 与 [`UsartRx`] 两半组合而成。
///
/// 引脚的所有权由 [`Peri`] 保证：同一个物理引脚同一时刻至多被一个驱动持有。
/// 驱动被 Drop 时，两个引脚的 [`Flex`] 会各自把引脚恢复为高阻（模拟）态；
/// 但 `CR1.UE` 保持置位（本驱动未实现 Drop 时关闭外设）。
pub struct Usart<'d, T: Instance> {
    tx: UsartTx<'d, T>,
    rx: UsartRx<'d, T>,
}

/// 仅发送的 USART 驱动。
///
/// 由 [`Usart::new_blocking_with_tx`] 或 [`Usart::split`] 获得。硬件层面
/// `CR1.TE`/`RE` 可独立使能，但本驱动构造时恒为全双工（发送半部也会置 `RE`，
/// 未接 RX 引脚时接收侧会被上拉保持空闲，不影响发送）。
pub struct UsartTx<'d, T: Instance> {
    _peri: Peri<'d, T>,
    tx: Flex<'d>,
}

/// 仅接收的 USART 驱动。
///
/// 由 [`Usart::new_blocking_with_rx`] 或 [`Usart::split`] 获得。
pub struct UsartRx<'d, T: Instance> {
    _peri: Peri<'d, T>,
    rx: Flex<'d>,
}

impl<'d, T: Instance> Usart<'d, T> {
    /// 创建双向驱动，并完成外设与引脚初始化（阻塞）。
    ///
    /// 流程对照 `HAL_USART_Init` → `USART_SetConfig`：
    /// 1. 使能该实例时钟（APB2 或 APB1，含 LL 要求的读回等待）；复位该实例；
    /// 2. 把 `tx`/`rx` 配为 AF7（顺序同 `HAL_GPIO_Init`：
    ///    AFR → MODER → OSPEEDR → OTYPER → PUPDR）；
    /// 3. `UE = 0` → 写 `CR1`（字长/校验/收发使能/位序/引脚交换）→ `CR2`（停止位、
    ///    关闭 CK 与 LIN）→ `CR3`（关闭过采样 8x、流控、IrDA、智能卡、DMA）→ `BRR`
    ///    → `UE = 1`（HAL 对 USART 不等待 `TEACK`/`REACK`）。
    ///
    /// # Errors
    ///
    /// 见 [`Error::InvalidConfig`]。
    pub fn new_blocking(
        peri: Peri<'d, T>,
        tx: Peri<'d, impl TxPin<T>>,
        rx: Peri<'d, impl RxPin<T>>,
        config: Config,
    ) -> Result<Self, Error> {
        enable_and_reset::<T>();

        // AF 号要在引脚被 Flex 接管之前取出（AF 查表见上方 impl_tx_pin!/impl_rx_pin!）
        let tx_af = tx.af_num();
        let rx_af = rx.af_num();
        // 同一实例的寄存器块由 tx/rx 两半共享，故复制一份句柄
        let rx_peri = unsafe { dup_peri(&peri) };

        let mut this = Self {
            tx: UsartTx {
                _peri: peri,
                tx: Flex::new(tx),
            },
            rx: UsartRx {
                _peri: rx_peri,
                rx: Flex::new(rx),
            },
        };
        // 寄存器顺序对照 HAL_GPIO_Init：AFR → MODER → OSPEEDR → OTYPER → PUPDR
        this.tx
            .tx
            .set_as_alternate(tx_af, config.speed, config.tx_pull, OutputType::PushPull);
        this.rx
            .rx
            .set_as_alternate(rx_af, config.speed, config.rx_pull, OutputType::PushPull);

        init_regs::<T>(&config)?;
        Ok(this)
    }

    /// 仅创建发送半部（不占用 RX 引脚）。
    ///
    /// 等价于 [`UsartTx::new_blocking`]，保留本方法是为了对齐 embassy-stm32 的
    /// `Uart::new_blocking_with_*` 构造器族。
    pub fn new_blocking_with_tx(
        peri: Peri<'d, T>,
        tx: Peri<'d, impl TxPin<T>>,
        config: Config,
    ) -> Result<UsartTx<'d, T>, Error> {
        UsartTx::new_blocking(peri, tx, config)
    }

    /// 仅创建接收半部（不占用 TX 引脚）。
    ///
    /// 等价于 [`UsartRx::new_blocking`]，保留本方法是为了对齐 embassy-stm32 的
    /// `Uart::new_blocking_with_*` 构造器族。
    pub fn new_blocking_with_rx(
        peri: Peri<'d, T>,
        rx: Peri<'d, impl RxPin<T>>,
        config: Config,
    ) -> Result<UsartRx<'d, T>, Error> {
        UsartRx::new_blocking(peri, rx, config)
    }

    /// 拆分成独立的发送/接收半部，便于分别借用。
    pub fn split(self) -> (UsartTx<'d, T>, UsartRx<'d, T>) {
        (self.tx, self.rx)
    }

    /// 阻塞发送 `buf` 的全部字节，返回时最后一帧已移出（对照 `HAL_USART_Transmit`）。
    pub fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        self.tx.write(buf)
    }

    /// 阻塞接收填满 `buf`（对照 `HAL_USART_Receive`）。
    pub fn read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        self.rx.read(buf)
    }

    /// 等待发送彻底结束（`SR.TC` 置位，最后一帧已移出）。
    pub fn flush(&mut self) -> Result<(), Error> {
        self.tx.flush()
    }
}

impl<'d, T: Instance> UsartTx<'d, T> {
    /// 创建仅发送驱动，并完成外设与 TX 引脚初始化（阻塞）。
    ///
    /// 流程同 [`Usart::new_blocking`]，但不涉及 RX 引脚。
    pub fn new_blocking(
        peri: Peri<'d, T>,
        tx: Peri<'d, impl TxPin<T>>,
        config: Config,
    ) -> Result<Self, Error> {
        enable_and_reset::<T>();
        let af = tx.af_num();
        let mut this = Self {
            _peri: peri,
            tx: Flex::new(tx),
        };
        this.tx
            .set_as_alternate(af, config.speed, config.tx_pull, OutputType::PushPull);
        init_regs::<T>(&config)?;
        Ok(this)
    }

    /// 阻塞发送一个字节：先等 `SR.TXE`（发送数据寄存器空），再写 `DR`。
    ///
    /// 发送路径不判定线路状态错误位（对照 `HAL_USART_Transmit`）：`ORE/PE/FE/NE`
    /// 属接收方向标志，由 [`UsartRx`] 读取并清零。
    pub fn write_byte(&mut self, byte: u8) -> Result<(), Error> {
        let regs = T::regs();
        // RM 37.4.1 bit7：TXE = 1 表示发送数据寄存器为空、可以写入新数据
        wait_until(|| regs.sr().read().txe_txfnf().bit_is_set())?;
        // RM 37.4.2：DR[8:0]，8 位字长时只用低 8 位（写 DR 会清 TXE 与 TC）
        regs.tdr().write(|w| unsafe { w.tdr().bits(byte as u16) });
        Ok(())
    }

    /// 阻塞发送 `buf` 的全部字节，返回时最后一帧已移出。
    ///
    /// 写 `DR` 会自动清 `TC`（RM 37.4.1），故无需像 LPUART 那样显式清 `TC`；
    /// 逐字节发送后等待 `TC` 置位即表示最后一帧已完整移出。
    pub fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        for &byte in buf {
            self.write_byte(byte)?;
        }
        self.flush()
    }

    /// 等待发送彻底结束（`SR.TC` 置位，最后一帧已移出）。
    ///
    /// 对照 `HAL_USART_Transmit` 收尾处对 `USART_FLAG_TC` 的等待。单独调用本方法时，
    /// 若发送早已结束、`TC` 仍为 1，会立即返回 `Ok`；需要"等待刚写入的字节真正发完"
    /// 时请直接用 [`UsartTx::write`]。
    pub fn flush(&mut self) -> Result<(), Error> {
        let regs = T::regs();
        // RM 37.4.1 bit6：TC 在"数据帧传送完成且 TXE=1"时由硬件置位
        wait_until(|| regs.sr().read().tc().bit_is_set())
    }
}

impl<'d, T: Instance> UsartRx<'d, T> {
    /// 创建仅接收驱动，并完成外设与 RX 引脚初始化（阻塞）。
    ///
    /// 流程同 [`Usart::new_blocking`]，但不涉及 TX 引脚。
    pub fn new_blocking(
        peri: Peri<'d, T>,
        rx: Peri<'d, impl RxPin<T>>,
        config: Config,
    ) -> Result<Self, Error> {
        enable_and_reset::<T>();
        let af = rx.af_num();
        let mut this = Self {
            _peri: peri,
            rx: Flex::new(rx),
        };
        this.rx
            .set_as_alternate(af, config.speed, config.rx_pull, OutputType::PushPull);
        init_regs::<T>(&config)?;
        Ok(this)
    }

    /// 阻塞接收一个字节。
    ///
    /// 先等 `SR.RXNE`，再按 RM 37.4.1 要求的"读 `SR` → 读 `DR`"序列取数据，
    /// 该序列同时清零 `RXNE` 与线路状态错误位（`PE/FE/NE/ORE/IDLE`）。
    /// 若存在错误，字节已被取出、但不会返回给调用方——错误优先级依次为
    /// `Overrun` > `Framing` > `Noise` > `Parity`。
    pub fn read_byte(&mut self) -> Result<u8, Error> {
        let regs = T::regs();
        // RM 37.4.1 bit5：RXNE = 1 表示接收数据寄存器非空
        wait_until(|| regs.sr().read().rxne_rxfnf().bit_is_set())?;
        // 先读 SR 再读 DR：清 RXNE 与 PE/FE/NE/ORE/IDLE（RM 37.4.1）
        let sr = regs.sr().read();
        // Note: RX 数据在偏移 0x04 的 DR（PAC 中即 tdr），PAC 的 rdr(0x20) 是
        // SVD 缺陷（RM 第 37 章寄存器表止于 0x1C），不可用于接收。
        let byte = regs.tdr().read().tdr().bits() as u8;
        take_line_error(&sr)?;
        Ok(byte)
    }

    /// 阻塞接收填满 `buf`（对照 `HAL_USART_Receive`）。
    ///
    /// 逐字节调用 [`UsartRx::read_byte`]；命中线路状态错误时立刻返回，
    /// 此时 `buf` 中已写入的字节数不再报告（如需逐字节容错请直接使用 `read_byte`）。
    pub fn read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        for b in buf.iter_mut() {
            *b = self.read_byte()?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 内部实现
// ---------------------------------------------------------------------------

/// 复制一份外设句柄（注册块的访问权由 tx/rx 两半共享）。
///
/// # Safety
///
/// 两份句柄都只存活在同一个 [`Usart`] 内、指向同一寄存器块，并且只做寄存器读写，
/// 不构造 Rust 别名可变引用；"同一实例至多一个驱动"的单例语义仍由 [`Peri`] 保证。
#[inline]
unsafe fn dup_peri<'d, T: PeripheralType + Copy>(peri: &Peri<'d, T>) -> Peri<'d, T> {
    unsafe { Peri::new_unchecked(**peri) }
}

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

/// 取该实例的外设时钟频率（Hz）。
///
/// 时钟源由 `RCC_CCIPR` 的 `USARTxSEL[1:0]` 决定（RM 6.10）：
/// - `00` = PCLK（USART1 为 PCLK2、USART2/3 为 PCLK1，官方例程即此配置）
/// - `01` = SYSCLK
/// - `10` = HSI
/// - `11` = LSE —— 本 crate 的 `rcc` 模块尚未实现 LSE，频率不可知，
///   返回 [`Error::InvalidConfig`]。
#[inline]
fn peripheral_clock<T: Instance>() -> Result<u32, Error> {
    let clocks = crate::rcc::clocks();
    match T::sel_bits() {
        0b00 => Ok(match T::bus() {
            Bus::Apb1 => clocks.apb1.0,
            Bus::Apb2 => clocks.apb2.0,
        }),
        0b01 => Ok(clocks.sys.0),
        // HSI16 是固定值（`py32e407_ll_rcc.h` 的 HSI_VALUE）
        0b10 => Ok(crate::rcc::HSI_HZ),
        _ => Err(Error::InvalidConfig),
    }
}

/// 写 USART 寄存器（不含 GPIO；引脚 AF 配置在构造器内完成）。
///
/// 顺序对照 `HAL_USART_Init` + `USART_SetConfig`：
/// `UE = 0` → `CR1`（字长/校验/收发使能/位序/引脚交换）→ `CR2`（停止位、关闭 CK/LIN）
/// → `CR3`（关闭 OVER8/流控/IrDA/智能卡/DMA）→ `BRR` → `UE = 1`。
/// HAL 对 USART 不等待 `TEACK`/`REACK`（该等待仅 LPUART 需要）。
fn init_regs<T: Instance>(config: &Config) -> Result<(), Error> {
    let regs = T::regs();

    // ---- 1. 关闭 USART（HAL: __HAL_USART_DISABLE）----
    regs.cr1().modify(|_, w| w.ue().clear_bit());

    // ---- 2. CR1：字长 / 校验 / 收发使能 / 位序 / 引脚交换（RM 37.4.4）----
    let (m0, m1) = config.data_bits.to_cr1();
    let (pce, ps) = config.parity.to_cr1();
    let msb_first = config.bit_order == BitOrder::MsbFirst;
    let swap = config.swap_tx_rx;
    regs.cr1().modify(|_, w| {
        w.m0().bit(m0);
        w.m1().bit(m1);
        w.pce().bit(pce);
        // Note: PS = 0 为偶校验、1 为奇校验（RM 37.4.4），与第 38 章 UART 相反
        w.ps().bit(ps);
        // 全双工：TE + RE 一起使能（HAL 的 Mode = USART_MODE_TX_RX）
        w.te().set_bit();
        w.re().set_bit();
        w.msb().bit(msb_first);
        w.swap().bit(swap);
        // FIFO 关闭：DR 表现为单字节寄存器，轮询语义与 HAL 的逐字节收发一致
        w.fifoen().clear_bit();
        // 同步轮询驱动：所有中断使能位置 0
        w.peie().clear_bit();
        w.txeie().clear_bit();
        w.tcie().clear_bit();
        w.rxneie().clear_bit();
        w.idleie().clear_bit();
        w
    });

    // ---- 3. CR2：停止位；关闭 CK / LIN 与同步时钟位（RM 37.4.5）----
    let stop = config.stop_bits.to_cr2();
    regs.cr2().modify(|_, w| unsafe {
        w.stop().bits(stop);
        // 本驱动仅异步 TX/RX：CLKEN=0 不输出 CK，复位 CPOL/CPHA/LBCL
        w.clken().clear_bit();
        w.cpol().clear_bit();
        w.cpha().clear_bit();
        w.lbcl().clear_bit();
        w.linen().clear_bit();
        w
    });

    // ---- 4. CR3：关闭 8x 过采样、硬件流控、IrDA、智能卡与 DMA（RM 37.4.6）----
    // HAL 的 USART_SetConfig 清 RTSE/CTSE，HAL_USART_Init 收尾清 SCEN/HDSEL/IREN；
    // 本驱动恒为 16x 过采样（HAL 与 USART_InitTypeDef 均无 OverSampling 字段），
    // 故显式清 OVER8。DMAR/DMAT/EIE 一并清零以保持纯轮询。
    regs.cr3().modify(|_, w| {
        w.over8().clear_bit();
        w.rtse().clear_bit();
        w.ctse().clear_bit();
        w.hdsel().clear_bit();
        w.iren().clear_bit();
        w.scen().clear_bit();
        w.dmar().clear_bit();
        w.dmat().clear_bit();
        w.eie().clear_bit();
        w
    });

    // ---- 5. BRR：波特率（RM 37.4.3 + `USART_BRR_SAMPLING16`）----
    let pclk = peripheral_clock::<T>()?;
    let (mant, frac) = baudrate_div(pclk, config.baudrate)?;
    regs.brr()
        .write(|w| unsafe { w.div_m().bits(mant).div_f().bits(frac) });

    // ---- 6. 使能 USART（HAL: __HAL_USART_ENABLE）----
    regs.cr1().modify(|_, w| w.ue().set_bit());
    Ok(())
}

/// 按 `USART_BRR_SAMPLING16`（`py32e407_hal_usart.h`）计算 `(DIV_M, DIV_F)`。
///
/// 与 C 宏逐位一致（C 里以"百分之一"为定标，故含 `*25`）：
/// ```text
/// div100 = PCLK * 25 / (4 * baud)
/// MANT   = div100 / 100
/// FRAQ   = ((div100 - MANT * 100) * 16 + 50) / 100
/// BRR    = (MANT << 4) + (FRAQ & 0xF0) + (FRAQ & 0x0F) = (MANT << 4) + FRAQ
/// ```
/// `BRR` 直接拆为 `DIV_M = BRR[15:4]`、`DIV_F = BRR[3:0]`。`FRAQ` 可能取到 16，
/// 此时进位自然落到尾数部分。`BRR` 须落在 `16..=0xFFFF`
/// （LL 的 `IS_LL_USART_BRR_MIN/MAX`），否则返回 [`Error::InvalidConfig`]。
fn baudrate_div(pclk: u32, baudrate: u32) -> Result<(u16, u8), Error> {
    if baudrate == 0 {
        return Err(Error::InvalidConfig);
    }

    // 用 u64 避免 PCLK * 25 在 85 MHz 下接近 u32 上限的问题
    let div100 = (pclk as u64) * 25 / (4 * baudrate as u64);
    let mant = div100 / 100;
    let fraq = ((div100 - mant * 100) * 16 + 50) / 100;

    let brr = (mant << 4) + fraq;
    if !(16..=0xFFFF).contains(&brr) {
        return Err(Error::InvalidConfig);
    }

    Ok(((brr >> 4) as u16, (brr & 0xF) as u8))
}

/// 轮询等待 `cond` 为真，超过 [`TIMEOUT_LOOPS`] 次返回 [`Error::Timeout`]。
#[inline]
fn wait_until(cond: impl Fn() -> bool) -> Result<(), Error> {
    for _ in 0..TIMEOUT_LOOPS {
        if cond() {
            return Ok(());
        }
    }
    Err(Error::Timeout)
}

/// 判定 `SR` 快照中的线路状态错误位（RM 37.4.1）。
///
/// 这些位由"读 `SR` 后读 `DR`"的序列清零，[`UsartRx::read_byte`] 已执行该序列，
/// 故此处只读快照、不写寄存器（PAC 的 `SR` 也未暴露 `PE/FE/NE/ORE` 的写访问器）。
/// 优先级：`Overrun` > `Framing` > `Noise` > `Parity`。
#[inline]
fn take_line_error(sr: &pac::usart1::sr::R) -> Result<(), Error> {
    if sr.ore().bit_is_set() {
        Err(Error::Overrun)
    } else if sr.fe().bit_is_set() {
        Err(Error::Framing)
    } else if sr.ne().bit_is_set() {
        Err(Error::Noise)
    } else if sr.pe().bit_is_set() {
        Err(Error::Parity)
    } else {
        Ok(())
    }
}
