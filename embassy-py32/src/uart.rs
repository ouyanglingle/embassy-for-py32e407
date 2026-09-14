//! Universal Asynchronous Receiver/Transmitter (UART)（同步 Blocking 驱动）
//!
//! 适配 PY32E407。寄存器流程依据：
//! - HAL C: `py32e407_hal_uart.c`（`HAL_UART_Init` / `HAL_UART_Transmit` / `HAL_UART_Receive`）
//! - LL:    `py32e407_ll_uart.h` / `py32e407_ll_uart.c`（`LL_UART_Init`、`LL_UART_SetBaudRate`）
//! - RM:    PY32E407 参考手册 V0.6 第 38 章（UART，38.4 寄存器）与第 6 章（RCC）
//! - DS:    PY32E407 数据手册 V0.13 表 3-3…（端口复用功能映射，UART 引脚 AF 号）
//!
//! 目标外设是 **UART1 / UART2 / UART3**（RM 第 38 章），三者共用同一寄存器块
//! （PAC: [`crate::pac::uart1::RegisterBlock`]），时钟同在 `RCC_APB1ENR1`。
//! 本芯片另有两套串口外设与本模块无关：USART1/2/3（RM 第 37 章，带 FIFO/同步模式）
//! 与 LPUART（RM 第 39 章），寄存器映射完全不同。
//!
//! # 与 STM32 同名外设的关键差异
//!
//! 以下差异已用 RM + SVD + PAC 交叉验证，**不可照搬 STM32 的 USART 写法**：
//!
//! - **没有 `UE` / `TE` / `RE` 使能位**。该 UART 是 16550 风格：`CR1` 只含字符格式位，
//!   发送由"写 `DR`"、接收由"读 `DR`"触发；`BRR` 非零即产生波特率时钟
//!   （RM 38.4.2："当除数锁存寄存器 BRR 设置为零时，波特时钟被禁用，不会发生串行通信"）。
//! - **波特率分频的 4 位小数放在独立寄存器 `BRRF`**（不在 `BRR` 里）。`BRR` 只保存
//!   整数部分，等价于把 STM32 `BRR[15:4]` 与 `BRR[3:0]` 拆成两个寄存器。计算式见
//!   RM 38.4.2 与 `py32e407_hal_uart.h` 的 `UART_BRR_SAMPLING16`。
//! - **`CR1.PS` 极性与 STM32 相反**：RM 38.4.4 规定 `PS = 1` 为**偶**校验、`PS = 0` 为
//!   **奇**校验（`LL_UART_PARITY_EVEN = PCE | PS` 与之吻合）；STM32 的 `USART_CR1.PS`
//!   恰好相反。
//! - 状态位集中在 `SR`（地址 0x10）：`RXNE`(0) `ORE`(1) `PE`(2) `FE`(3) `BRI`(4)
//!   `TDRE`(5) `TXE`(6) `ADDR_RCVD`(8) `BUSY`(9) `BUSY_ERR`(10)，其中
//!   `ORE/PE/FE/BRI/ADDR_RCVD/BUSY_ERR` 均为 RC_W1（写 1 清零）。
//! - `SP/PS/PCE/STOP/M/MSBFIRST` 只能在 `SR.BUSY == 0` 时写，否则置 `BUSY_ERR`。
//!   故初始化顺序采用 `LL_UART_Init` 的"先 `CR1`/`CR3`、后 `BRR`"——`BRR` 一旦非零
//!   就把 UART 拉进 busy 语义。
//!
//! # 使用示例
//!
//! ```ignore
//! use embassy_py32::uart::{Config, Uart};
//! use embassy_py32::init;
//!
//! let p = init(Default::default());
//! let mut uart = Uart::new_blocking(p.UART2, p.PC12, p.PD2, Config::default()).unwrap();
//! uart.write(b"hello\r\n").unwrap();
//! ```
//!
//! # 命名约定
//!
//! 构造函数一律带 `_blocking` 后缀（[`Uart::new_blocking`]、[`UartTx::new_blocking`]、
//! [`UartRx::new_blocking`]，以及 [`Uart::new_blocking_with_tx`] /
//! [`Uart::new_blocking_with_rx`]），**`new` 留给后续的 Async 版本**——与
//! embassy-stm32 的 `Uart::new_blocking` / `UartTx::new_blocking` 命名完全一致
//! （见 `embassy-stm32-0.6.0/src/usart/mod.rs` 中 `impl UartTx<'d, Blocking>`）。
//! 阻塞方法与其异步对应版本（`write`/`read`/`flush`）目前同名的原因，
//! 同样是"异步版尚未实现、名称待其加入时再区分"。
//!
//! 仅实现同步（轮询 + 超时）驱动：不使用 DMA 与中断（`CR2` 中的 4 个中断使能位
//! 全部清零），也不支持 9 位字长（`CR3.M_E`）。后续将按需扩展 Async。

use crate::gpio::{Flex, OutputType, Pin, Pull, Speed};
use crate::pac;
use crate::{Peri, PeripheralType};

/// 轮询等待状态位的循环上限（次数）。
///
/// 量级依据：HSI16 下每次循环约 4~6 个周期，`1_000_000` 次 ≈ 0.25~0.4 s，
/// 足以覆盖 1200 bps 下单字节 8.3 ms 的等待并留出约 30 倍裕量。
/// 若使用更低波特率或需要更长超时，请在使用侧自行加外层重试。
const TIMEOUT_LOOPS: u32 = 1_000_000;

/// UART 错误类型。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// 帧错误：接收数据中未检测到有效停止位（`SR.FE`，RM 38.4.3）。
    Framing,
    /// 奇偶校验错误（`SR.PE`，RM 38.4.3）。
    Parity,
    /// 接收溢出：上一个字符尚未被读走又收到新字符，`RDR` 已被覆盖（`SR.ORE`）。
    Overrun,
    /// 检测到 break 序列（`SR.BRI`，RM 38.4.3）。
    Break,
    /// 等待状态位超过 [`TIMEOUT_LOOPS`] 仍未就绪。
    Timeout,
    /// 配置非法：波特率为 0，或算出的分频值超出 `BRR` 允许范围（1..=0xFFFF）。
    InvalidConfig,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            Self::Framing => "Framing Error",
            Self::Parity => "Parity Check Error",
            Self::Overrun => "RX Buffer Overrun",
            Self::Break => "Break Detected",
            Self::Timeout => "Operation Timed Out",
            Self::InvalidConfig => "Invalid Configuration",
        };
        f.write_str(msg)
    }
}

impl core::error::Error for Error {}

/// 数据位长度（`CR1.M`，RM 38.4.4：00=5 / 01=6 / 10=7 / 11=8）。
///
/// 9 位字长需要 `CR3.M_E` 与 16 位数据通路（RM 38.4.6），本同步驱动暂未实现。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataBits {
    /// 每字符 5 个数据位。
    DataBits5,
    /// 每字符 6 个数据位。
    DataBits6,
    /// 每字符 7 个数据位。
    DataBits7,
    /// 每字符 8 个数据位。
    DataBits8,
}

impl DataBits {
    /// 映射为 `CR1.M` 的 2-bit 编码。
    #[inline]
    fn to_cr1_m(self) -> u8 {
        match self {
            Self::DataBits5 => 0b00,
            Self::DataBits6 => 0b01,
            Self::DataBits7 => 0b10,
            Self::DataBits8 => 0b11,
        }
    }
}

/// 奇偶校验方式（`CR1.PCE` / `CR1.PS`，RM 38.4.4）。
///
/// 注意：`PS = 1` 表示**偶**校验，与本芯片 USART（RM 第 37 章）及 STM32 的约定相反。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Parity {
    /// 不生成也不检测校验位。
    ParityNone,
    /// 偶校验（`PCE = 1, PS = 1`）。
    ParityEven,
    /// 奇校验（`PCE = 1, PS = 0`）。
    ParityOdd,
}

impl Parity {
    /// 映射为 `(PCE, PS)`。
    ///
    /// 依据 RM 38.4.4 PS 位定义："如果设置为 1，则传输或检查偶数个逻辑 1"，
    /// 与 `py32e407_ll_uart.h` 的 `LL_UART_PARITY_EVEN = (UART_CR1_PCE | UART_CR1_PS)` 一致。
    #[inline]
    fn to_cr1(self) -> (bool, bool) {
        match self {
            Self::ParityNone => (false, false),
            Self::ParityEven => (true, true),
            Self::ParityOdd => (true, false),
        }
    }
}

/// 停止位长度（`CR1.STOP`，RM 38.4.4）。
///
/// RM 说明：`STOP = 0` 为 1 个停止位；`STOP = 1` 时若数据位为 5 位则发送 1.5 个停止位，
/// 否则发送 2 个停止位。接收端始终只检查第一个停止位。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StopBits {
    /// 1 个停止位（`STOP = 0`）。
    STOP1,
    /// 1.5 或 2 个停止位（`STOP = 1`）。
    STOP2,
}

impl StopBits {
    /// 映射为 `CR1.STOP`。
    #[inline]
    fn to_cr1(self) -> bool {
        match self {
            Self::STOP1 => false,
            Self::STOP2 => true,
        }
    }
}

/// 位序（`CR1.MSBFIRST`，RM 38.4.4）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BitOrder {
    /// LSB 优先（复位默认）。
    LsbFirst,
    /// MSB 优先。
    MsbFirst,
}

/// UART 配置。
///
/// `#[non_exhaustive]`，外部构造请用 [`Config::default()`] 后逐字段赋值。
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// 波特率，单位 bps（必须非 0；分频值需落在 `BRR` 的 1..=0xFFFF 内）。
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
    /// 默认取 [`Pull::Up`]：与官方 STK 例程 `UART_HyperTerminal_Polling`
    /// （`LL_GPIO_PULL_UP`）一致，避免 RX 悬空时被噪声触发起始位。
    pub rx_pull: Pull,
    /// 复用引脚的输出速度（`OSPEEDR`）。
    pub speed: Speed,
}

impl Default for Config {
    /// 默认值对照 `LL_UART_StructInit` 的建议初值与官方 STK 例程的引脚配置：
    /// 115200-8N1、LSB 优先、全双工、TX/RX 不交换、GPIO 高速档。
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

// ---------------------------------------------------------------------------
// 外设实例
// ---------------------------------------------------------------------------

/// 密封的实例信息（防止外部为任意类型实现 [`Instance`]）。
mod sealed {
    use crate::pac;

    /// UART 实例的寄存器块与 RCC 位信息。
    pub trait SealedInstance {
        /// 取该实例的寄存器块（基址来自 PAC `Periph` 的常量指针）。
        fn regs() -> &'static pac::uart1::RegisterBlock;

        /// `RCC_APB1ENR1` 中的时钟使能位掩码。
        fn enable_mask() -> u32;

        /// `RCC_APB1RSTR1` 中的复位位掩码。
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

/// UART 外设实例（[`crate::peripherals::UART1`] / [`crate::peripherals::UART2`] /
/// [`crate::peripherals::UART3`]）。
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
/// `$bit` 是 `RCC_APB1ENR1` / `RCC_APB1RSTR1` 中的位号（RM 6.10 寄存器描述：
/// UART1 = bit19、UART2 = bit20、UART3 = bit23）。
macro_rules! impl_instance {
    ($inst:ident, $pac:ident, $bit:literal) => {
        impl sealed::SealedInstance for crate::peripherals::$inst {
            #[inline]
            fn regs() -> &'static pac::uart1::RegisterBlock {
                // 基址与类型均来自 PAC（UART1 = 0x4000_4C00、UART2 = 0x4000_5000、
                // UART3 = 0x4000_5C00），三个实例共用 uart1::RegisterBlock。
                unsafe { &*pac::$pac::PTR }
            }

            #[inline]
            fn enable_mask() -> u32 {
                1 << $bit
            }

            #[inline]
            fn reset_mask() -> u32 {
                1 << $bit
            }
        }

        impl Instance for crate::peripherals::$inst {}
    };
}

impl_instance!(UART1, Uart1, 19);
impl_instance!(UART2, Uart2, 20);
impl_instance!(UART3, Uart3, 23);

/// 为引脚生成 TX 引脚实现。
macro_rules! impl_tx_pin {
    ($inst:ident, $pin:ident, $af:literal) => {
        impl sealed::SealedTxPin<crate::peripherals::$inst> for crate::peripherals::$pin {
            #[inline]
            fn af_num(&self) -> u8 {
                $af
            }
        }

        impl TxPin<crate::peripherals::$inst> for crate::peripherals::$pin {}
    };
}

/// 为引脚生成 RX 引脚实现。
macro_rules! impl_rx_pin {
    ($inst:ident, $pin:ident, $af:literal) => {
        impl sealed::SealedRxPin<crate::peripherals::$inst> for crate::peripherals::$pin {
            #[inline]
            fn af_num(&self) -> u8 {
                $af
            }
        }

        impl RxPin<crate::peripherals::$inst> for crate::peripherals::$pin {}
    };
}

// 引脚与 AF 号来自 DS V0.13 的端口复用功能映射表（表 3-3 端口 A / 表 3-5 端口 C /
// 表 3-6 端口 D）。UART1/2/3 只有 TX/RX 两个信号，每个信号只有一个可选引脚。
// UART2 的 PC12/PD2 + AF5 与官方 STK 例程 UART_HyperTerminal_Polling 完全一致。
impl_tx_pin!(UART1, PC10, 5);
impl_rx_pin!(UART1, PC11, 5);
impl_tx_pin!(UART2, PC12, 5);
impl_rx_pin!(UART2, PD2, 5);
// Note: DS 复用表中 UART3 排在 AF14 列（与 UART1/2 的 AF5 不同），已按表逐列核对。
impl_tx_pin!(UART3, PA2, 14);
impl_rx_pin!(UART3, PA1, 14);

// ---------------------------------------------------------------------------
// 驱动
// ---------------------------------------------------------------------------

/// 双向 UART 驱动，内部由 [`UartTx`] 与 [`UartRx`] 两半组合而成。
///
/// 引脚的所有权由 [`Peri`] 保证：同一个物理引脚同一时刻至多被一个驱动持有。
/// 驱动被 Drop 时，两个引脚的 [`Flex`] 会各自把引脚恢复为高阻（模拟）态。
pub struct Uart<'d, T: Instance> {
    tx: UartTx<'d, T>,
    rx: UartRx<'d, T>,
}

/// 仅发送的 UART 驱动。
///
/// 由 [`Uart::new_blocking_with_tx`] 或 [`Uart::split`] 获得。硬件层面 UART1/2/3 没有独立的
/// 发送/接收使能位，因此"仅发送"只意味着不占用也不配置 RX 引脚。
pub struct UartTx<'d, T: Instance> {
    _peri: Peri<'d, T>,
    tx: Flex<'d>,
}

/// 仅接收的 UART 驱动。
///
/// 由 [`Uart::new_blocking_with_rx`] 或 [`Uart::split`] 获得。
pub struct UartRx<'d, T: Instance> {
    _peri: Peri<'d, T>,
    rx: Flex<'d>,
}

impl<'d, T: Instance> Uart<'d, T> {
    /// 创建双向驱动，并完成外设与引脚初始化（阻塞）。
    ///
    /// 流程对照 `LL_UART_Init` 与官方例程的 `MspInit`：
    /// 1. 使能 `RCC_APB1ENR1` 中该实例的时钟（含 LL 要求的读回等待）；
    /// 2. 复位该实例（`RCC_APB1RSTR1` Force + Release）；
    /// 3. 把 `tx`/`rx` 配为对应 AF（顺序同 `HAL_GPIO_Init`：
    ///    AFR → MODER → OSPEEDR → OTYPER → PUPDR）；
    /// 4. 写 `CR1`/`CR3`（字符格式）→ `CR2`（关闭中断）→ `BRR`/`BRRF`（波特率）。
    ///
    /// # Errors
    ///
    /// 波特率为 0 或算出的分频值越界时返回 [`Error::InvalidConfig`]。
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
            tx: UartTx {
                _peri: peri,
                tx: Flex::new(tx),
            },
            rx: UartRx {
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
    /// 等价于 [`UartTx::new_blocking`]，保留本方法是为了对齐 embassy-stm32 的
    /// `Uart::new_blocking_with_*` 构造器族。
    pub fn new_blocking_with_tx(
        peri: Peri<'d, T>,
        tx: Peri<'d, impl TxPin<T>>,
        config: Config,
    ) -> Result<UartTx<'d, T>, Error> {
        UartTx::new_blocking(peri, tx, config)
    }

    /// 仅创建接收半部（不占用 TX 引脚）。
    pub fn new_blocking_with_rx(
        peri: Peri<'d, T>,
        rx: Peri<'d, impl RxPin<T>>,
        config: Config,
    ) -> Result<UartRx<'d, T>, Error> {
        UartRx::new_blocking(peri, rx, config)
    }

    /// 拆分成独立的发送/接收半部，便于分别借用。
    pub fn split(self) -> (UartTx<'d, T>, UartRx<'d, T>) {
        (self.tx, self.rx)
    }

    /// 阻塞发送 `buf` 的全部字节，返回时最后一位已移出（对照 `HAL_UART_Transmit`）。
    pub fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        self.tx.write(buf)
    }

    /// 阻塞接收填满 `buf`（对照 `HAL_UART_Receive`）。
    pub fn read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        self.rx.read(buf)
    }

    /// 等待发送彻底结束（`SR.TXE` 置位，移位寄存器已空）。
    pub fn flush(&mut self) -> Result<(), Error> {
        self.tx.flush()
    }
}

impl<'d, T: Instance> UartTx<'d, T> {
    /// 创建仅发送驱动，并完成外设与 TX 引脚初始化（阻塞）。
    ///
    /// 流程同 [`Uart::new_blocking`]，但不涉及 RX 引脚。
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

    /// 阻塞发送一个字节：先等 `TDRE`（发送保持寄存器空），再写 `DR`。
    ///
    /// 发送路径不判定线路状态错误位（对照 `HAL_UART_Transmit`）：`ORE/PE/FE/BRI`
    /// 属接收方向标志，由 [`UartRx`] 读取并清零。
    pub fn write_byte(&mut self, byte: u8) -> Result<(), Error> {
        let regs = T::regs();
        // RM 38.4.3：TDRE = 1 表示 TDR 为空、可以写入新数据
        wait_until(|| regs.sr().read().tdre().bit_is_set())?;
        // RM 38.4.1：DR[8:0]、低 8 位为数据（8 位字长时第 9 位忽略）
        regs.dr().write(|w| unsafe { w.dr().bits(byte as u16) });
        Ok(())
    }

    /// 阻塞发送 `buf` 的全部字节，返回时最后一位已移出。
    pub fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        for &byte in buf {
            self.write_byte(byte)?;
        }
        self.flush()
    }

    /// 等待发送彻底结束（`SR.TXE` 置位，移位寄存器已空）。
    ///
    /// 对照 `HAL_UART_Transmit` 收尾处对 `UART_FLAG_TXE` 的等待。
    pub fn flush(&mut self) -> Result<(), Error> {
        let regs = T::regs();
        wait_until(|| regs.sr().read().txe().bit_is_set())
    }
}

impl<'d, T: Instance> UartRx<'d, T> {
    /// 创建仅接收驱动，并完成外设与 RX 引脚初始化（阻塞）。
    ///
    /// 流程同 [`Uart::new_blocking`]，但不涉及 TX 引脚。
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
    /// 先等 `RXNE`，读出 `DR`，再检查并清零线路状态错误位（`ORE/PE/FE/BRI`）。
    /// 若存在错误，字节已经被取出、但不会返回给调用方——错误优先级依次为
    /// `Overrun` > `Framing` > `Parity` > `Break`。
    pub fn read_byte(&mut self) -> Result<u8, Error> {
        let regs = T::regs();
        // RM 38.4.3：RXNE = 1 表示 RDR 中至少有一个字符，读 DR 会清零该位
        wait_until(|| regs.sr().read().rxne().bit_is_set())?;
        let byte = regs.dr().read().dr().bits() as u8;
        take_line_error(regs)?;
        Ok(byte)
    }

    /// 阻塞接收填满 `buf`（对照 `HAL_UART_Receive`）。
    ///
    /// 逐字节调用 [`UartRx::read_byte`]；命中线路状态错误时立刻返回，
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
/// 两份句柄都只存活在同一个 [`Uart`] 内、指向同一寄存器块，并且只做寄存器读写，
/// 不构造 Rust 别名可变引用；"同一实例至多一个驱动"的单例语义仍由 [`Peri`] 保证。
#[inline]
unsafe fn dup_peri<'d, T: PeripheralType + Copy>(peri: &Peri<'d, T>) -> Peri<'d, T> {
    unsafe { Peri::new_unchecked(**peri) }
}

/// 使能实例时钟并复位实例。
///
/// 时钟使能经 [`crate::rcc::enable_apb1`]（内部已复刻 LL 的"置位后读回一次"），
/// 复位经 [`crate::rcc::reset_apb1`]（Force + Release）。
#[inline]
fn enable_and_reset<T: Instance>() {
    crate::rcc::enable_apb1(T::enable_mask(), 0);
    // Safety: 复位会立即停止该外设并恢复其寄存器复位值；此刻本驱动尚未配置它，
    // 且外设单例由 Peri 保证只被取出一次。
    unsafe { crate::rcc::reset_apb1(T::reset_mask(), 0) };
}

/// 写 UART 寄存器（不含 GPIO；引脚 AF 配置在构造器内完成）。
///
/// 顺序对照 `py32e407_ll_uart.c` 的 `LL_UART_Init`：先 `CR1`/`CR3`（字符格式），
/// 再 `CR2`（中断使能），最后 `BRR`/`BRRF`（波特率）。不能颠倒——`BRR` 一旦非零
/// 就使 UART 进入"波特时钟工作"的 busy 语义，此后写 `CR1` 会置 `SR.BUSY_ERR`
/// （RM 38.4.3 / 38.4.4 均注明相关位"仅当 UART 不忙时可写"）。
fn init_regs<T: Instance>(config: &Config) -> Result<(), Error> {
    let regs = T::regs();

    // ---- CR1：字符格式（RM 38.4.4）----
    let m = config.data_bits.to_cr1_m();
    let (pce, ps) = config.parity.to_cr1();
    let stop = config.stop_bits.to_cr1();
    let msb_first = config.bit_order == BitOrder::MsbFirst;
    let swap = config.swap_tx_rx;
    regs.cr1().modify(|_, w| {
        // M[1:0]：数据位长度（SVD 中该字段无枚举值，按 RM 编码直接写）
        unsafe { w.m().bits(m) };
        w.stop().bit(stop);
        w.pce().bit(pce);
        // Note: PS=1 为偶校验（RM 38.4.4），与 STM32 USART_CR1.PS 相反
        w.ps().bit(ps);
        // 固定奇偶校验（SP）本驱动不使用，显式清零
        w.sp().clear_bit();
        // SBK（发送 break）保持 0：强制 sout 为低由上层按需另开
        w.sbk().clear_bit();
        w.swap().bit(swap);
        w.msbfirst().bit(msb_first);
        w
    });

    // ---- CR3：只用 M_E，本驱动恒为 0（不支持 9 位字长，RM 38.4.6）----
    regs.cr3().write(|w| w.m_e().clear_bit());

    // ---- CR2：同步轮询驱动，4 个中断使能位全部清零（RM 38.4.5）----
    // 注意 CR2 复位值为 0x0000_0008（BUSYERRIE = 1），不显式清零会让 busy 误操作
    // 触发中断请求。
    regs.cr2().write(|w| unsafe { w.bits(0) });

    // ---- BRR/BRRF：波特率（RM 38.4.2 + `UART_BRR_SAMPLING16`）----
    // UART1/2/3 均在 APB1 上，分频用的外设时钟是 PCLK1（`LL_UART_Init` 取
    // `rcc_clocks.PCLK1_Frequency`；`HAL_UART_Init` 亦取 `HAL_RCC_GetPCLK1Freq`）。
    let pclk1 = crate::rcc::clocks().apb1.0;
    let (brr, brrf) = baudrate_div(pclk1, config.baudrate)?;
    regs.brr().write(|w| unsafe { w.brr().bits(brr) });
    regs.brrf().write(|w| unsafe { w.brrf().bits(brrf) });

    // RM 38.4.2："一旦设置了 BRR，在发送或接收数据之前，应等待至少 8 个时钟周期"
    brr_settle_delay();
    Ok(())
}

/// 按 `UART_BRR_SAMPLING16`（`py32e407_hal_uart.h`）计算 `(BRR, BRRF)`。
///
/// 与 C 宏逐位一致：
/// ```text
/// div100 = PCLK * 25 / (4 * baud)      // 100 倍定标的除数
/// MANT   = div100 / 100
/// FRAQ   = ((div100 - MANT * 100) * 16 + 50) / 100
/// integer = (MANT << 4) + (FRAQ & 0xF0) + (FRAQ & 0x0F)
/// BRR  = (integer >> 4) & 0xFFFF
/// BRRF = integer & 0xF
/// ```
/// FRAQ 可能取到 16，此时进位自然落到整数部分（`& 0xF0` / `& 0x0F` 拆分的结果）。
///
/// `baudrate == 0` 或整数部分越界（LL 的 `IS_LL_UART_BRR_MIN/MAX`：1..=0xFFFF）时
/// 返回 [`Error::InvalidConfig`]。
fn baudrate_div(pclk: u32, baudrate: u32) -> Result<(u16, u8), Error> {
    if baudrate == 0 {
        return Err(Error::InvalidConfig);
    }

    // 用 u64 避免 PCLK * 25 在 85 MHz 下接近 u32 上限的问题
    let div100 = (pclk as u64) * 25 / (4 * baudrate as u64);
    let mant = div100 / 100;
    let fraq = ((div100 - mant * 100) * 16 + 50) / 100;

    let integer = (mant << 4) | fraq;
    let brr = integer >> 4;
    if brr == 0 || brr > 0xFFFF {
        return Err(Error::InvalidConfig);
    }

    Ok((brr as u16, (integer & 0xF) as u8))
}

/// 写 `BRR` 后按 RM 38.4.2 要求等待一小段时间（"至少 8 个时钟周期"）。
///
/// 该要求以波特时钟为量级（8 个波特时钟 = 8 × 16 个 PCLK 周期）；此处固定等待
/// 128 个循环，覆盖常见 PCLK（≥ 8 MHz）下的这一量级。极低波特率下若需严格
/// 等待 8 个完整位时间，请在使用侧另行延时。
#[inline]
fn brr_settle_delay() {
    for _ in 0..128u32 {
        core::hint::spin_loop();
    }
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

/// 读取并清零 `SR` 的线路状态错误位（RC_W1，RM 38.4.3）。
///
/// 优先级：`Overrun` > `Framing` > `Parity` > `Break`。无错误时返回 `Ok(())`。
#[inline]
fn take_line_error(regs: &pac::uart1::RegisterBlock) -> Result<(), Error> {
    let sr = regs.sr().read();
    let err = if sr.ore().bit_is_set() {
        Some(Error::Overrun)
    } else if sr.fe().bit_is_set() {
        Some(Error::Framing)
    } else if sr.pe().bit_is_set() {
        Some(Error::Parity)
    } else if sr.bri().bit_is_set() {
        Some(Error::Break)
    } else {
        None
    };

    if err.is_some() {
        // 四个位均为 RC_W1：写 1 清零
        regs.sr().write(|w| {
            w.ore().set_bit();
            w.pe().set_bit();
            w.fe().set_bit();
            w.bri().set_bit();
            w
        });
    }

    match err {
        Some(e) => Err(e),
        None => Ok(()),
    }
}
