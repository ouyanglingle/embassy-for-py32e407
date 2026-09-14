//! Low-power Universal Asynchronous Receiver/Transmitter (LPUART)（同步 Blocking 驱动）
//!
//! 适配 PY32E407。寄存器流程依据：
//! - HAL C: `py32e407_hal_lpuart.c`（`HAL_LPUART_Init` → `LPUART_SetConfig` →
//!   `LPUART_CheckIdleState`，以及 `HAL_LPUART_Transmit` / `HAL_LPUART_Receive`）
//! - LL:    `py32e407_ll_lpuart.h` / `py32e407_ll_lpuart.c`（`LL_LPUART_Init`、
//!   `LL_LPUART_SetBaudRate`、`__LL_LPUART_DIV`、`LPUART_PRESCALER_TAB`）
//! - RM:    PY32E407 参考手册 V0.6 第 39 章（LPUART，39.4 寄存器）与第 6 章（RCC/CCIPR）
//! - DS:    PY32E407 数据手册 V0.13（端口复用功能映射，LPUART1 引脚 AF 号）
//!
//! 目标外设是 **LPUART1**（RM 第 39 章，PAC: [`crate::pac::lpuart::RegisterBlock`]，
//! 单实例）。本芯片另有两套串口外设与本模块无关：UART1/2/3（RM 第 38 章，见
//! [`crate::uart`]，16550 风格、无 `UE` 位）与 USART1/2/3（RM 第 37 章）。
//!
//! # 与其它串口外设的关系（移植时最容易搞混的地方）
//!
//! LPUART 的寄存器布局与语义**很接近 STM32 的 LPUART（v3）**，但与本芯片第 38 章的
//! UART 完全不同；已用 RM + SVD + PAC 交叉验证的关键事实：
//!
//! - **有 `CR1.UE` 使能位**（bit0），且 `CR1`/`CR2`/`PRESC` 大量位域"只能在
//!   `UE = 0` 时写"（RM 39.4.1/39.4.2/39.4.10 反复注明）。HAL 的做法是
//!   `__HAL_LPUART_DISABLE` → 配置 → `__HAL_LPUART_ENABLE` → 等 `TEACK`/`REACK`；
//!   本驱动复刻该序列。
//! - **`CR1.PS` 极性与 STM32 一致**：RM 39.4.1 明确 `PS = 0` 为**偶**校验、
//!   `PS = 1` 为**奇**校验——**与第 38 章 UART 的 `CR1.PS` 恰好相反**，切勿互相抄。
//! - **波特率是 256 倍过采样**：`LPUARTDIV = round(256 × fck / baud)`，`fck` 为
//!   "外设时钟 ÷ PRESC 预分频"（RM 39.2.5 + `__LL_LPUART_DIV`）。除数写进 20 位的
//!   `BRR`（PAC 字段名为 `bbr`），合法范围 `0x300..=0xFFFFF`。
//! - **`PRESC` 预分频表**不是 2 的幂：`1,2,4,6,8,10,12,16,32,64,128,256`
//!   （`LPUART_PRESCALER_TAB`，RM 39.4.10）。
//! - **`CR2.STOP[1:0]` 只有两档**：`00` = 1 个停止位、`10` = 2 个停止位，
//!   `01`/`11` 保留（RM 39.4.2）。与 STM32 的 0.5/1.5 位编码不同。
//! - **LPUART1 的时钟源可控**：`RCC_CCIPR.LPUART1SEL[1:0]`（RM 6.10）
//!   00 = PCLK1、01 = SYSCLK、10 = HSI、11 = LSE；复位默认 PCLK1。
//!   本驱动读取该字段换算实际时钟（LSE 档暂不支持，见 [`Error::InvalidConfig`]）。
//! - 状态位在 `ISR`：`PE`(0) `FE`(1) `NE`(2) `ORE`(3) `IDLE`(4) `RXFNE`(5) `TC`(6)
//!   `TXFNF`(7) `CTSIF`(9) `CTS`(10) `BUSY`(16) `TEACK`(21) `REACK`(22) …
//!   错误位与 `TC`/`IDLE` 都由 `ICR` 对应位**写 1 清零**（RM 39.4.6/39.4.7）。
//! - 收發数据寄存器独立：发送写 `TDR`、接收读 `RDR`（不像第 38 章 UART 共用一个 `DR`）。
//!
//! # 使用示例
//!
//! ```ignore
//! use embassy_py32::lpuart::{Config, Lpuart};
//! use embassy_py32::init;
//!
//! let p = init(Default::default());
//! // PC0 = RX、PC1 = TX（AF8），与官方 STK 例程 LPUART_HyperTerminal_Polling 一致
//! let mut uart = Lpuart::new_blocking(p.LPUART1, p.PC1, p.PC0, Config::default()).unwrap();
//! uart.write(b"hello\r\n").unwrap();
//! ```
//!
//! # 命名约定
//!
//! 构造函数一律带 `_blocking` 后缀（[`Lpuart::new_blocking`]、[`LpuartTx::new_blocking`]、
//! [`LpuartRx::new_blocking`]，以及 [`Lpuart::new_blocking_with_tx`] /
//! [`Lpuart::new_blocking_with_rx`]），**`new` 留给后续的 Async 版本**——与
//! embassy-stm32 的 `Uart::new_blocking` / `UartTx::new_blocking` 命名完全一致。
//!
//! 仅实现同步（轮询 + 超时）驱动：不使用 DMA 与中断（`CR1`/`CR3` 的中断使能位
//! 全部保持 0），不启用 FIFO 与硬件流控（CTS/RTS/DE），9 位字长（`CR1.M0` 单独置位）
//! 亦未实现。后续将按需扩展 Async。

use crate::gpio::{Flex, OutputType, Pin, Pull, Speed};
use crate::pac;
use crate::{Peri, PeripheralType};

/// 轮询等待状态位的循环上限（次数）。
///
/// 量级依据：HSI16 下每次循环约 4~6 个周期，`1_000_000` 次 ≈ 0.25~0.4 s，
/// 足以覆盖 1200 bps 下单字节 8.3 ms 的等待并留出约 30 倍裕量。
const TIMEOUT_LOOPS: u32 = 1_000_000;

/// `PRESC.PRESCALER` 的分频表（`py32e407_ll_lpuart.h` 的 `LPUART_PRESCALER_TAB`）。
///
/// 注意不是 2 的幂次序列：索引 3/5/6 分别对应 /6、/10、/12。
const PRESCALER_TAB: [u16; 12] = [1, 2, 4, 6, 8, 10, 12, 16, 32, 64, 128, 256];

/// `BRR` 允许的最小值（`LPUART_BRR_MIN`/`LPUART_BRR_MIN_VALUE`）。
const BRR_MIN: u32 = 0x0000_0300;
/// `BRR` 允许的最大值（20 位字段，`LPUART_BRR_MAX`）。
const BRR_MAX: u32 = 0x000F_FFFF;

/// LPUART 错误类型。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// 奇偶校验错误（`ISR.PE`，RM 39.4.6）。
    Parity,
    /// 帧错误：未检测到有效停止位（`ISR.FE`）。
    Framing,
    /// 起始位噪声（`ISR.NE`）。
    Noise,
    /// 接收溢出：前一字符未被读走又收到新字符，`RDR` 被覆盖（`ISR.ORE`）。
    Overrun,
    /// 等待状态位超过 [`TIMEOUT_LOOPS`] 仍未就绪。
    Timeout,
    /// 配置非法：波特率为 0、算出的分频值/预分频后时钟越界，
    /// 或 LPUART1 时钟源为 LSE（本 crate 的 rcc 模块未实现 LSE，频率不可知）。
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

/// 数据位长度（`CR1.M1` / `CR1.M0`，RM 39.4.1）。
///
/// 编码来自 `py32e407_hal_lpuart_ex.h`：`7B = M1`、`8B = 0`、`9B = M0`。
/// 9 位字长需要 16 位数据通路，本同步驱动暂未实现。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataBits {
    /// 每字符 7 个数据位（`M1 = 1`）。
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

/// 奇偶校验方式（`CR1.PCE` / `CR1.PS`，RM 39.4.1）。
///
/// 注意：`PS = 0` 为**偶**校验、`PS = 1` 为**奇**校验——与 STM32 一致，
/// 但与本芯片第 38 章 UART（`PS = 1` 为偶校验）**相反**。
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

/// 停止位长度（`CR2.STOP[1:0]`，RM 39.4.2）。
///
/// RM 只定义两档：`00` = 1 个停止位、`10` = 2 个停止位；`01`/`11` 保留。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StopBits {
    /// 1 个停止位（`STOP = 00`）。
    STOP1,
    /// 2 个停止位（`STOP = 10`）。
    STOP2,
}

impl StopBits {
    /// 映射为 `STOP[1:0]` 的 2-bit 编码。
    #[inline]
    fn to_cr2(self) -> u8 {
        match self {
            Self::STOP1 => 0b00,
            Self::STOP2 => 0b10,
        }
    }
}

/// 位序（`CR2.MSBFIRST`，RM 39.4.2）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BitOrder {
    /// LSB 优先（复位默认）。
    LsbFirst,
    /// MSB 优先。
    MsbFirst,
}

/// 输入时钟预分频（`PRESC.PRESCALER[3:0]`，RM 39.4.10）。
///
/// 分频值见 [`PRESCALER_TAB`]，**不是** 2 的幂次序列。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClockPrescaler {
    /// 1 分频（复位默认）。
    Div1,
    /// 2 分频。
    Div2,
    /// 4 分频。
    Div4,
    /// 6 分频。
    Div6,
    /// 8 分频。
    Div8,
    /// 10 分频。
    Div10,
    /// 12 分频。
    Div12,
    /// 16 分频。
    Div16,
    /// 32 分频。
    Div32,
    /// 64 分频。
    Div64,
    /// 128 分频。
    Div128,
    /// 256 分频。
    Div256,
}

impl ClockPrescaler {
    /// 映射为 `PRESC.PRESCALER` 的 4-bit 编码（= [`PRESCALER_TAB`] 的下标）。
    #[inline]
    fn to_bits(self) -> u8 {
        match self {
            Self::Div1 => 0,
            Self::Div2 => 1,
            Self::Div4 => 2,
            Self::Div6 => 3,
            Self::Div8 => 4,
            Self::Div10 => 5,
            Self::Div12 => 6,
            Self::Div16 => 7,
            Self::Div32 => 8,
            Self::Div64 => 9,
            Self::Div128 => 10,
            Self::Div256 => 11,
        }
    }

    /// 分频值（`LPUART_PRESCALER_TAB` 的对应项）。
    #[inline]
    fn div(self) -> u32 {
        PRESCALER_TAB[self.to_bits() as usize] as u32
    }
}

/// LPUART 配置。
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
    /// 交换 TX/RX 引脚功能（`CR2.SWAP`），用于交叉线序的板级适配。
    pub swap_tx_rx: bool,
    /// 输入时钟预分频。低波特率下若"预分频后时钟 ÷ 波特率"过大，需要调大分频
    /// （`BRR` 上限 0xFFFFF ≈ 64 倍过采样下限）。
    pub clock_prescaler: ClockPrescaler,
    /// TX 引脚的内部上/下拉。
    pub tx_pull: Pull,
    /// RX 引脚的内部上/下拉。
    ///
    /// 默认取 [`Pull::Up`]：与官方 STK 例程 `LPUART_HyperTerminal_Polling`
    /// （`LL_GPIO_PULL_UP`）一致，避免 RX 悬空时被噪声触发起始位。
    pub rx_pull: Pull,
    /// 复用引脚的输出速度（`OSPEEDR`）。
    pub speed: Speed,
}

impl Default for Config {
    /// 默认值对照 `LL_LPUART_StructInit` 与官方 STK 例程的引脚配置：
    /// 115200-8N1、LSB 优先、不交换引脚、不分频、全双工。
    fn default() -> Self {
        Self {
            baudrate: 115_200,
            data_bits: DataBits::DataBits8,
            stop_bits: StopBits::STOP1,
            parity: Parity::ParityNone,
            bit_order: BitOrder::LsbFirst,
            swap_tx_rx: false,
            clock_prescaler: ClockPrescaler::Div1,
            tx_pull: Pull::None,
            rx_pull: Pull::Up,
            speed: Speed::VeryHigh,
        }
    }
}

// ---------------------------------------------------------------------------
// 外设实例
// ---------------------------------------------------------------------------

/// 密封的实例信息（防止外部为任意类型实现 [`Instance`]）。
mod sealed {
    use crate::pac;

    /// LPUART 实例的寄存器块与 RCC 位信息。
    pub trait SealedInstance {
        /// 取该实例的寄存器块（基址来自 PAC `Periph` 的常量指针）。
        fn regs() -> &'static pac::lpuart::RegisterBlock;

        /// `RCC_APB1ENR2` 中的时钟使能位掩码。
        fn enable_mask() -> u32;

        /// `RCC_APB1RSTR2` 中的复位位掩码。
        fn reset_mask() -> u32;
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

/// LPUART 外设实例（本芯片只有 [`crate::peripherals::LPUART1`]）。
#[allow(private_bounds)]
pub trait Instance: sealed::SealedInstance + PeripheralType + Copy + 'static {}

/// 可作为某实例 TX 的引脚。
#[allow(private_bounds)]
pub trait TxPin<T: Instance>: sealed::SealedTxPin<T> + Pin {}

/// 可作为某实例 RX 的引脚。
#[allow(private_bounds)]
pub trait RxPin<T: Instance>: sealed::SealedRxPin<T> + Pin {}

// LPUART1：时钟使能在 `RCC_APB1ENR2` bit1、复位在 `RCC_APB1RSTR2` bit1
// （RM 6.10 寄存器描述；对应 `LL_APB1_GRP2_PERIPH_LPUART1`）。
impl sealed::SealedInstance for crate::peripherals::LPUART1 {
    #[inline]
    fn regs() -> &'static pac::lpuart::RegisterBlock {
        // 基址与类型均来自 PAC（LPUART = 0x4000_8000）。
        unsafe { &*pac::Lpuart::PTR }
    }

    #[inline]
    fn enable_mask() -> u32 {
        1 << 1
    }

    #[inline]
    fn reset_mask() -> u32 {
        1 << 1
    }
}

impl Instance for crate::peripherals::LPUART1 {}

/// 为引脚生成 TX 引脚实现。
macro_rules! impl_tx_pin {
    ($pin:ident, $af:literal) => {
        impl sealed::SealedTxPin<crate::peripherals::LPUART1> for crate::peripherals::$pin {
            #[inline]
            fn af_num(&self) -> u8 {
                $af
            }
        }

        impl TxPin<crate::peripherals::LPUART1> for crate::peripherals::$pin {}
    };
}

/// 为引脚生成 RX 引脚实现。
macro_rules! impl_rx_pin {
    ($pin:ident, $af:literal) => {
        impl sealed::SealedRxPin<crate::peripherals::LPUART1> for crate::peripherals::$pin {
            #[inline]
            fn af_num(&self) -> u8 {
                $af
            }
        }

        impl RxPin<crate::peripherals::LPUART1> for crate::peripherals::$pin {}
    };
}

// 引脚与 AF 号来自 DS V0.13 端口复用功能映射表（表 3-3 端口 A / 表 3-4 端口 B /
// 表 3-5 端口 C）。PC0/PC1 + AF8 与官方 STK 例程 LPUART_HyperTerminal_Polling
// 完全一致（例程中 `GPIO_InitStruct.Alternate = LL_GPIO_AF_8`）；PB10/PB11 与
// PB12/PB13（RTS/CTS）在复用表中同处 AF8 列，已逐列核对。
impl_tx_pin!(PA2, 12);
impl_rx_pin!(PA3, 12);
impl_rx_pin!(PB10, 8);
impl_tx_pin!(PB11, 8);
impl_rx_pin!(PC0, 8);
impl_tx_pin!(PC1, 8);

// ---------------------------------------------------------------------------
// 驱动
// ---------------------------------------------------------------------------

/// 双向 LPUART 驱动，内部由 [`LpuartTx`] 与 [`LpuartRx`] 两半组合而成。
///
/// 引脚的所有权由 [`Peri`] 保证：同一个物理引脚同一时刻至多被一个驱动持有。
/// 驱动被 Drop 时，两个引脚的 [`Flex`] 会各自把引脚恢复为高阻（模拟）态；
/// 但 `CR1.UE` 保持置位（本驱动未实现 Drop 时关闭外设）。
pub struct Lpuart<'d, T: Instance> {
    tx: LpuartTx<'d, T>,
    rx: LpuartRx<'d, T>,
}

/// 仅发送的 LPUART 驱动。
///
/// 由 [`Lpuart::new_blocking_with_tx`] 或 [`Lpuart::split`] 获得。
pub struct LpuartTx<'d, T: Instance> {
    _peri: Peri<'d, T>,
    tx: Flex<'d>,
}

/// 仅接收的 LPUART 驱动。
///
/// 由 [`Lpuart::new_blocking_with_rx`] 或 [`Lpuart::split`] 获得。
pub struct LpuartRx<'d, T: Instance> {
    _peri: Peri<'d, T>,
    rx: Flex<'d>,
}

impl<'d, T: Instance> Lpuart<'d, T> {
    /// 创建双向驱动，并完成外设与引脚初始化（阻塞）。
    ///
    /// 流程对照 `HAL_LPUART_Init`：
    /// 1. 使能 `RCC_APB1ENR2` 中该实例的时钟（含 LL 要求的读回等待）；
    /// 2. 复位该实例（`RCC_APB1RSTR2` Force + Release）；
    /// 3. 把 `tx`/`rx` 配为对应 AF（顺序同 `HAL_GPIO_Init`：
    ///    AFR → MODER → OSPEEDR → OTYPER → PUPDR）；
    /// 4. `UE = 0` → 写 `CR1`/`CR2`/`CR3`/`PRESC`/`BRR` → `UE = 1`
    ///    → 等 `TEACK`/`REACK`（`LPUART_CheckIdleState`）。
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
            tx: LpuartTx {
                _peri: peri,
                tx: Flex::new(tx),
            },
            rx: LpuartRx {
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
    /// 等价于 [`LpuartTx::new_blocking`]，保留本方法是为了对齐 embassy-stm32 的
    /// `Uart::new_blocking_with_*` 构造器族。
    pub fn new_blocking_with_tx(
        peri: Peri<'d, T>,
        tx: Peri<'d, impl TxPin<T>>,
        config: Config,
    ) -> Result<LpuartTx<'d, T>, Error> {
        LpuartTx::new_blocking(peri, tx, config)
    }

    /// 仅创建接收半部（不占用 TX 引脚）。
    pub fn new_blocking_with_rx(
        peri: Peri<'d, T>,
        rx: Peri<'d, impl RxPin<T>>,
        config: Config,
    ) -> Result<LpuartRx<'d, T>, Error> {
        LpuartRx::new_blocking(peri, rx, config)
    }

    /// 拆分成独立的发送/接收半部，便于分别借用。
    pub fn split(self) -> (LpuartTx<'d, T>, LpuartRx<'d, T>) {
        (self.tx, self.rx)
    }

    /// 阻塞发送 `buf` 的全部字节，返回时最后一帧已移出（对照 `HAL_LPUART_Transmit`）。
    pub fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        self.tx.write(buf)
    }

    /// 阻塞接收填满 `buf`（对照 `HAL_LPUART_Receive`）。
    pub fn read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        self.rx.read(buf)
    }

    /// 等待发送彻底结束（`ISR.TC` 置位）。
    pub fn flush(&mut self) -> Result<(), Error> {
        self.tx.flush()
    }
}

impl<'d, T: Instance> LpuartTx<'d, T> {
    /// 创建仅发送驱动，并完成外设与 TX 引脚初始化（阻塞）。
    ///
    /// 流程同 [`Lpuart::new_blocking`]，但不涉及 RX 引脚（`CR1.RE` 仍会被置位，
    /// 因为本芯片的 LPUART 只用一个 `Mode` 位组控制收发；未接 RX 引脚时
    /// 接收侧会被上拉保持空闲，不影响发送）。
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

    /// 阻塞发送一个字节：先等 `ISR.TXFNF`（= HAL 的 `LPUART_FLAG_TXE`，发送寄存器未满），
    /// 再写 `TDR`。
    pub fn write_byte(&mut self, byte: u8) -> Result<(), Error> {
        let regs = T::regs();
        // RM 39.4.6 bit7：TXFNF = 1 表示发送数据寄存器未满、可写入新数据
        wait_until(|| regs.isr().read().txfnf().bit_is_set())?;
        // RM 39.4.9：TDR[8:0]，8 位字长时只用低 8 位
        regs.tdr().write(|w| unsafe { w.tdr().bits(byte as u16) });
        Ok(())
    }

    /// 阻塞发送 `buf` 的全部字节，返回时最后一帧已移出。
    pub fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        // 先清掉上一帧遗留的 TC（RM 39.4.6：TC 只读，只能由 ICR.TCCF 写 1 清零），
        // 否则末尾 flush 可能读到旧值而提前返回。
        T::regs().icr().write(|w| w.tccf().set_bit());
        for &byte in buf {
            self.write_byte(byte)?;
        }
        self.flush()
    }

    /// 等待发送彻底结束（`ISR.TC` 置位，最后一帧已移出）。
    ///
    /// 对照 `HAL_LPUART_Transmit` 收尾处对 `LPUART_FLAG_TC` 的等待。
    /// 单独调用本方法时，若发送早已结束、`TC` 仍为 1，会立即返回 `Ok`；
    /// 需要"等待刚写入的字节真正发完"时请直接用 [`LpuartTx::write`]。
    pub fn flush(&mut self) -> Result<(), Error> {
        let regs = T::regs();
        // RM 39.4.6 bit6：TC 在"数据帧传送完成且 TXE=1"时由硬件置位
        wait_until(|| regs.isr().read().tc().bit_is_set())
    }
}

impl<'d, T: Instance> LpuartRx<'d, T> {
    /// 创建仅接收驱动，并完成外设与 RX 引脚初始化（阻塞）。
    ///
    /// 流程同 [`Lpuart::new_blocking`]，但不涉及 TX 引脚。
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
    /// 先等 `ISR.RXFNE`（= HAL 的 `LPUART_FLAG_RXNE`），读 `RDR`（读操作即清零该位），
    /// 再检查并清零错误位（`PE/FE/NE/ORE`）。若存在错误，字节已被取出但不会返回给
    /// 调用方——错误优先级依次为 `Overrun` > `Framing` > `Noise` > `Parity`。
    pub fn read_byte(&mut self) -> Result<u8, Error> {
        let regs = T::regs();
        // RM 39.4.6 bit5：RXFNE = 1 表示 RDR 中至少有一个字符
        wait_until(|| regs.isr().read().rxfne().bit_is_set())?;
        let byte = regs.rdr().read().rdr().bits() as u8;
        take_line_error(regs)?;
        Ok(byte)
    }

    /// 阻塞接收填满 `buf`（对照 `HAL_LPUART_Receive`）。
    ///
    /// 逐字节调用 [`LpuartRx::read_byte`]；命中线路状态错误时立刻返回，
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
/// 两份句柄都只存活在同一个 [`Lpuart`] 内、指向同一寄存器块，并且只做寄存器读写，
/// 不构造 Rust 别名可变引用；"同一实例至多一个驱动"的单例语义仍由 [`Peri`] 保证。
#[inline]
unsafe fn dup_peri<'d, T: PeripheralType + Copy>(peri: &Peri<'d, T>) -> Peri<'d, T> {
    unsafe { Peri::new_unchecked(**peri) }
}

/// 使能实例时钟并复位实例。
///
/// LPUART1 的使能/复位位都在 APB1 的"第二组"寄存器上（`RCC_APB1ENR2` /
/// `RCC_APB1RSTR2`，bit1），因此用 [`crate::rcc::enable_apb1`] /
/// [`crate::rcc::reset_apb1`] 的第二个参数。
#[inline]
fn enable_and_reset<T: Instance>() {
    crate::rcc::enable_apb1(0, T::enable_mask());
    // Safety: 复位会立即停止该外设并恢复其寄存器复位值；此刻本驱动尚未配置它，
    // 且外设单例由 Peri 保证只被取出一次。
    unsafe { crate::rcc::reset_apb1(0, T::reset_mask()) };
}

/// 取 LPUART1 的外设时钟频率（Hz）。
///
/// 时钟源由 `RCC_CCIPR.LPUART1SEL[1:0]` 决定（RM 6.10，"LPUART1 时钟源选择"）：
/// - `00` = PCLK1（复位默认，也是官方例程的配置）
/// - `01` = SYSCLK
/// - `10` = HSI
/// - `11` = LSE —— 本 crate 的 `rcc` 模块尚未实现 LSE，频率不可知，
///   返回 [`Error::InvalidConfig`]（此时请在 `init()` 前选择其它时钟源，
///   或自行确认 LSE 频率后扩展本函数）。
#[inline]
fn peripheral_clock() -> Result<u32, Error> {
    let rcc = unsafe { &*pac::Rcc::PTR };
    match rcc.ccipr().read().lpuart1sel().bits() {
        0b00 => Ok(crate::rcc::clocks().apb1.0),
        0b01 => Ok(crate::rcc::clocks().sys.0),
        // HSI16 是固定值（`py32e407_ll_rcc.h` 的 HSI_VALUE）
        0b10 => Ok(crate::rcc::HSI_HZ),
        _ => Err(Error::InvalidConfig),
    }
}

/// 写 LPUART 寄存器（不含 GPIO；引脚 AF 配置在构造器内完成）。
///
/// 顺序对照 `HAL_LPUART_Init` + `LPUART_SetConfig`：
/// `UE = 0` → `CR1`（字长/校验/收发使能）→ `CR2`（停止位/位序/引脚交换）
/// → `CR3`（流控/HDSEL 清零）→ `PRESC` → `BRR` → `UE = 1` → 等 `TEACK`/`REACK`。
///
/// 其中 `UE = 0` 是硬性前提：RM 39.4.1/39.4.2/39.4.10 反复注明相关位
/// "只能在 LPUART 未使能时（UE=0）才能写入"。
fn init_regs<T: Instance>(config: &Config) -> Result<(), Error> {
    let regs = T::regs();

    // ---- 1. 关闭 LPUART（HAL: __HAL_LPUART_DISABLE）----
    regs.cr1().modify(|_, w| w.ue().clear_bit());

    // ---- 2. CR1：字长 / 校验 / 收发使能（RM 39.4.1）----
    let (m0, m1) = config.data_bits.to_cr1();
    let (pce, ps) = config.parity.to_cr1();
    regs.cr1().modify(|_, w| {
        if m0 {
            w.m0().set_bit()
        } else {
            w.m0().clear_bit()
        };
        if m1 {
            w.m1().set_bit()
        } else {
            w.m1().clear_bit()
        };
        if pce {
            w.pce().set_bit()
        } else {
            w.pce().clear_bit()
        };
        // Note: PS = 0 为偶校验、1 为奇校验（RM 39.4.1），与第 38 章 UART 相反
        if ps {
            w.ps().set_bit()
        } else {
            w.ps().clear_bit()
        };
        // 全双工：TE + RE 一起使能（HAL 的 Mode = LPUART_MODE_TX_RX）
        w.te().set_bit();
        w.re().set_bit();
        // FIFO 模式关闭：TDR/RDR 表现为单字节保持寄存器，
        // 轮询语义与 HAL 的逐字节收发完全一致
        w.fifoen().clear_bit();
        w
    });

    // ---- 3. CR2：停止位 / 位序 / 引脚交换（RM 39.4.2）----
    // 只动本驱动负责的字段：TXOE_ALWAYS_ON 保持复位默认 1（软件控制输出使能常开），
    // 其"半双工模式需关闭"的说明与本驱动的全双工用途无关。
    let stop = config.stop_bits.to_cr2();
    let msbfirst = config.bit_order == BitOrder::MsbFirst;
    let swap = config.swap_tx_rx;
    regs.cr2().modify(|_, w| unsafe {
        w.stop().bits(stop);
        w.msbfirst().bit(msbfirst);
        w.swap().bit(swap);
        w
    });

    // ---- 4. CR3：本驱动不用流控/DMA（RM 39.4.3）----
    // HAL_LPUART_Init 明确要求异步模式下 HDSEL 必须保持 0。
    regs.cr3().modify(|_, w| {
        w.hdsel().clear_bit();
        w.rtse().clear_bit();
        w.ctse().clear_bit();
        w
    });

    // ---- 5. PRESC：输入时钟预分频（RM 39.4.10）----
    let prescaler = config.clock_prescaler;
    regs.presc()
        .modify(|_, w| unsafe { w.prescaler().bits(prescaler.to_bits()) });

    // ---- 6. BRR：波特率（RM 39.4.4 + `__LL_LPUART_DIV`）----
    let fck_hz = peripheral_clock()?;
    let div = baudrate_div(fck_hz, prescaler, config.baudrate)?;
    regs.brr().modify(|_, w| unsafe { w.bbr().bits(div) });

    // ---- 7. 使能 LPUART（HAL: __HAL_LPUART_ENABLE）----
    regs.cr1().modify(|_, w| w.ue().set_bit());

    // ---- 8. 等 TEACK / REACK（HAL: LPUART_CheckIdleState）----
    wait_until(|| regs.isr().read().teack().bit_is_set())?;
    wait_until(|| regs.isr().read().reack().bit_is_set())?;
    Ok(())
}

/// 按 `__LL_LPUART_DIV`（`py32e407_ll_lpuart.h`）计算 `BRR` 的 20 位除数。
///
/// 与 C 宏逐位一致（注意 C 里是**先整除分频、再乘 256**）：
/// ```text
/// fck_pres = PeripheralClk / LPUART_PRESCALER_TAB[presc]
/// LPUARTDIV = (fck_pres * 256 + baud / 2) / baud   // 四舍五入
/// ```
/// 同时复刻 `LPUART_SetConfig` 的两项校验：
/// - 预分频后的时钟必须落在 `[3 × baud, 4096 × baud]`；
/// - 结果必须落在 `BRR_MIN..=BRR_MAX`（`0x300..=0xFFFFF`）。
///
/// 任一不满足即返回 [`Error::InvalidConfig`]（可调大 [`Config::clock_prescaler`] 重试）。
fn baudrate_div(fck_hz: u32, prescaler: ClockPrescaler, baudrate: u32) -> Result<u32, Error> {
    if baudrate == 0 {
        return Err(Error::InvalidConfig);
    }

    let fck_pres = (fck_hz as u64) / (prescaler.div() as u64);
    let baud = baudrate as u64;
    if fck_pres < 3 * baud || fck_pres > 4096 * baud {
        return Err(Error::InvalidConfig);
    }

    let div = (fck_pres * 256 + baud / 2) / baud;
    if !(BRR_MIN as u64..=BRR_MAX as u64).contains(&div) {
        return Err(Error::InvalidConfig);
    }

    Ok(div as u32)
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

/// 读取并清零 `ISR` 的线路状态错误位（经 `ICR` 写 1 清零，RM 39.4.7）。
///
/// 优先级：`Overrun` > `Framing` > `Noise` > `Parity`。无错误时返回 `Ok(())`。
#[inline]
fn take_line_error(regs: &pac::lpuart::RegisterBlock) -> Result<(), Error> {
    let isr = regs.isr().read();
    let err = if isr.ore().bit_is_set() {
        Some(Error::Overrun)
    } else if isr.fe().bit_is_set() {
        Some(Error::Framing)
    } else if isr.ne().bit_is_set() {
        Some(Error::Noise)
    } else if isr.pe().bit_is_set() {
        Some(Error::Parity)
    } else {
        None
    };

    if err.is_some() {
        // ICR 的 PECF/FECF/NECF/ORECF 均为"写 1 清零"
        regs.icr().write(|w| {
            w.pecf().set_bit();
            w.fecf().set_bit();
            w.necf().set_bit();
            w.orecf().set_bit();
            w
        });
    }

    match err {
        Some(e) => Err(e),
        None => Ok(()),
    }
}
