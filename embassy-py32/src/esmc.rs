//! 外部串行存储控制器 ESMC / QSPI（同步 Blocking 驱动）
//!
//! 适配 PY32E407。寄存器流程依据：
//! - HAL C: `py32e407_hal_esmc.c` 的 `HAL_ESMC_Init`、
//!   `ESMC_Config`（四种功能模式的寄存器装配）、`HAL_ESMC_Command`、
//!   `HAL_ESMC_Transmit` / `HAL_ESMC_Receive`、`HAL_ESMC_MemoryMapped` /
//!   `HAL_ESMC_MemoryWrite`、`ESMC_WriteBuff` / `ESMC_ReadBuff`、
//!   `ESMC_WaitFlagStateUntilTimeout`；配置结构体见 `py32e407_hal_esmc.h`
//!   的 `ESMC_InitTypeDef` / `ESMC_CommandTypeDef`
//! - LL:    `py32e407_ll_bus.h`（`LL_AHB1_GRP1_EnableClock`，ESMC 在 AHB1）
//! - RM:    PY32E407 参考手册 V0.6 第 30 章（30.3 功能描述、30.4~30.8 寄存器）
//! - DS:    PY32E407 数据手册 V0.13 §2.29、表 5-58/5-59（fQCK ≤ 85 MHz SDR、
//!   42.5 MHz DDR）与端口复用表 3-3…3-8
//! - 例程:  `Projects/PY32E407-STK/Example/ESMC/ESMC_ReadWrite_QSPI_Polling`、
//!   `ESMC_MemoryMapping_QSPI`、`ESMC_XIP_ReadWritePSRAM_4IO`（引脚与 AF 的硬证据）
//!
//! ESMC 是单实例外设（AHB3 基址 `0xA000_1000`，PAC [`crate::pac::Esmc`]），
//! 支持 Single/Dual/Quad/Octal SPI 与 SDR/DDR，工作模式分**间接模式**
//! （全程用寄存器收发）与**内存映射（XIP）模式**（外部存储映射进地址空间）。
//!
//! # 关键实现事实（已用 RM + SVD + PAC + HAL 交叉验证）
//!
//! - **写 `ADDR24` 才是"开火"动作**：24 位地址模式下指令（`ADDR24.INS1`）与
//!   24 位地址（`ADDR24.ADDR[23:0]`）同处一个 32 位寄存器，HAL 因此在
//!   `ESMC_Config` 里只把该值存进 `ContexAddr24`，留到 `Transmit`/`Receive`
//!   开头才写一次。本驱动用句柄字段 [`Esmc::addr24`] 复刻：**配置阶段绝不
//!   分开写 `ADDR24`**（先写 `INS1` 再写地址会中途发起一次错误命令）。
//! - **配置阶段用"读一次、整写一次"的影子变量**（HAL 的 `TmpCR3`/`TmpSOCR`/
//!   `TmpDCR`/`TmpADDR24`/`TmpADDR32` 风格），而不是逐字段 `modify`：
//!   一是 `ADDR24`/`ADDR32` 含动作位（`SS_SET`/`SS_CLR` 写 1 生效），
//!   二是保证写入顺序与 HAL 完全一致（`ADDR32 → SOCR → DCR → STRR → CR3 → ADDR24`）。
//! - **`CR` 用 `modify` 而非整写**：HAL 在 Init 里整写 `CR = SPIEN | 2xQUAD`，
//!   会把 RM 30.4.1 记载"复位值 1"的保留位 bit1 清 0；本驱动只改有定义的位，
//!   保留位维持复位值。
//! - **Init 的写入顺序按 RM 调整过**：RM 30.4.2 注明"启用模式 3 时，建议先使能
//!   CPOL 和 CPHA 寄存器，再配置波特率"，而 HAL 先写 `BAUD` 再写 `CR2`。
//!   本驱动按 RM 顺序（`CR2 → BAUD → TCR`）。
//! - **XIP 配置里 HAL 有一处笔误**：`ESMC_Config` 的内存映射分支写的是
//!   `CLEAR_BIT(TmpXDCR, ESMC_XCR3_NO_ADDR)`（把 `XCR3` 的掩码用在了 `XDCR` 上，
//!   既清不掉 `XCR3.NO_ADDR`、又动了 `XDCR` 的保留位）。本驱动改为清除
//!   `XCR3.NO_ADDR`（RM 30.5.4），`XDCR` 只写 `DUMMY[5:0]` 与 `NO_CMD`。
//! - **SVD/PAC 缺 `XCR3.NO_DATA`（bit7，RM 30.5.4 有）**：本驱动不写该位；
//!   其复位值为 0，恰好是 XIP 读/写都需要的"地址后进入数据阶段"。
//! - **只做同步**：不使用 DMA 与中断——Init 里显式清 `IER1`/`IER2`/`GIE`/`DMAEN`，
//!   `CR2` 的 DMA 使能位保持 0；收发全程轮询 `SR`/`IFR1` 标志并带超时。
//!   HAL 的 `TransmitFIFO`/`ReceiveFIFO`（半空/半满批次）与 `_IT`/`_DMA` 变体未实现。
//!
//! # 使用示例（间接模式读 JEDEC ID）
//!
//! ```ignore
//! use embassy_py32::esmc::{ChipSelect, ChipSelects, Command, Config, DataMode, Esmc, InstructionMode};
//!
//! // 本板接线：PB10 = CLK、PE12~PE15 = BK1_IO0~IO3、PE11 = CS0
//! let mut esmc = Esmc::new_blocking_with_cs(
//!     p.ESMC, p.PB10, p.PE12, p.PE13, p.PE14, p.PE15,
//!     ChipSelects::none().cs0(p.PE11),
//!     Config::default(),
//! )
//! .unwrap();
//!
//! let cmd = Command {
//!     instruction_mode: InstructionMode::SingleLine,
//!     instruction: 0x9F,          // JEDEC ID
//!     data_mode: DataMode::Read,
//!     nb_data: 3,
//!     chip_select: ChipSelect::Cs0,
//!     ..Default::default()
//! };
//! let mut id = [0u8; 3];
//! esmc.command_read(&cmd, &mut id).unwrap();
//! ```

use crate::gpio::{Flex, OutputType, Pin, Pull, Speed};
use crate::pac;
use crate::{Hertz, Peri};

/// 单次轮询等待的循环上限（次数）。
///
/// 量级依据：HSI16 下每次循环约 4~6 个周期（≈0.3 µs），`20_000_000` 次 ≈ 6~8 s。
/// 本驱动的每次等待都是「FIFO 交接」级别的短等待：最长约一个 FIFO 深度
/// （16 字节 = 128 个 SCLK 周期，最慢分频 BAUD = 255、HCLK 168 MHz → SCLK ≈ 659 kHz
/// 时约 195 µs），故上限留有 4 个数量级以上裕量；只有外部存储器无响应或总线
/// 挂死时才会返回 [`Error::Timeout`]。
const TIMEOUT_LOOPS: u32 = 20_000_000;

/// ESMC 内部 FIFO 深度（字节）。RM 30.3.5.9：TXFIFO 16 字节、RXFIFO 16 字节、
/// XIPFIFO 8 字节；收发路径用的是前两者，HAL 的 `HAL_ESMC_FIFO_SIZE` 亦为 0x10。
const FIFO_SIZE: usize = 16;

/// ESMC 的 AHB1 时钟使能位掩码（`RCC_AHB1ENR.ESMCEN`，bit 12）。
///
/// 依据：RM 6.10.9（AHB1ENR bit12 = ESMCEN，"ESMC(QSPI)模块时钟使能"）；
/// SVD/PAC 的 `rcc::ahb1enr::EsmcenW` 亦在 bit12。
const AHB1ENR_ESMCEN: u32 = 1 << 12;

// ---------------------------------------------------------------------------
// 寄存器位掩码（RM 30.4~30.8，按位序手工展开）
//
// 配置阶段刻意复刻 HAL 的 `MODIFY_REG`/`WRITE_REG` 语义：先把寄存器读进影子变量
// 再整体写回。svd2rust 的字段写入器作用于 `W`（不接受外部影子值），故此处使用
// 显式掩码——每个常量都对应 HAL `py32e407_hal_esmc.h` 中的同名宏，便于逐条对照。
// ---------------------------------------------------------------------------

// SOCR（RM 30.4.6）—— 与 HAL 的 ESMC_SOCR_* 一一对应
/// `SOCR.SPIMODE[1:0]`：00=Single、01=Dual、10=Quad、11=Octal（或 Dual-Flash）。
const SOCR_SPIMODE: u8 = 0x03;
/// `SOCR.SEND_M`（HAL 名 `SENM`）：地址后追加 1 个备用字节（`ADDR32.MREG`）。
const SOCR_SENM: u8 = 0x04;
/// `SOCR.MULTICMD`：指令按当前 SPI 模式（多位）发送。
const SOCR_MULTICMD: u8 = 0x08;
/// `SOCR.MULTIADDR`：地址按当前 SPI 模式（多位）发送。
const SOCR_MULTIADDR: u8 = 0x10;
/// `SOCR.DDRCMD`：指令阶段双沿发送。
const SOCR_DDRCMD: u8 = 0x20;
/// `SOCR.DDRADDR`：地址阶段双沿发送。
const SOCR_DDRADDR: u8 = 0x40;
/// `SOCR.DDRDATA`：数据阶段双沿收发。
const SOCR_DDRDATA: u8 = 0x80;

// DCR（RM 30.4.7）
/// `DCR.DUMMY[5:0]`：地址阶段后的 Dummy 周期数（0~63）。
const DCR_DUMMY: u8 = 0x3F;
/// `DCR.NO_CMD`：置 1 则省略指令阶段（从地址开始）。
const DCR_NO_CMD: u8 = 0x40;
/// `DCR.REC`：置 1 为间接**读**（接收数据），清 0 为间接**写**。
const DCR_REC: u8 = 0x80;

// CR3（RM 30.4.8）
/// `CR3.ADDR8BIT`：地址阶段 8 位。
const CR3_ADDR8BIT: u8 = 0x01;
/// `CR3.ADDR16BIT`：地址阶段 16 位。
const CR3_ADDR16BIT: u8 = 0x02;
/// `CR3.ADDR32BIT`：地址阶段 32 位。
const CR3_ADDR32BIT: u8 = 0x04;
/// `CR3.NO_ADDR`：置 1 则省略地址阶段。
const CR3_NO_ADDR: u8 = 0x08;
/// `CR3.xSPI_RWDS`：使能 xSPI 的 RWDS（DQS）引脚。
const CR3_XSPI_RWDS: u8 = 0x10;
/// `CR3.CMD_EXT`：指令为 2 字节（第 2 字节取自 `ADDR32.MREG`）。
const CR3_CMD_EXT: u8 = 0x20;
/// `CR3.NO_DATA`：置 1 则地址后不进入数据阶段（擦除等只发指令+地址）。
const CR3_NO_DATA: u8 = 0x80;

// ADDR32（RM 30.8.2）
/// `ADDR32.ADDR3[7:0]`：32 位地址模式的最高字节（`ADDR[31:24]`）。
const ADDR32_ADDR3: u32 = 0x0000_00FF;
/// `ADDR32.MREG[7:0]`：备用字节，或 2 字节指令的第 2 字节（`CR3.CMD_EXT = 1`）。
const ADDR32_MREG: u32 = 0x0000_FF00;
/// `ADDR32.SS[5:0]`：本次传输驱动哪个 nCS（bit0 = SS0 … bit5 = SS5）。
const ADDR32_SSXO: u32 = 0x003F_0000;
/// `ADDR32.MREG` 的位偏移（HAL 的 `ESMC_ADDR32_MREG_Pos`）。
const ADDR32_MREG_POS: u32 = 8;
/// `ADDR32.SS` 的位偏移（HAL 的 `ESMC_ADDR32_SSXO_Pos`）。
const ADDR32_SSXO_POS: u32 = 16;

// ADDR24（RM 30.8.1，只能按 32 位访问）
/// `ADDR24.INS1[7:0]`：1 字节指令（`CR3.CMD_EXT = 0` 时）。
const ADDR24_INS1: u32 = 0x0000_00FF;
/// `ADDR24.ADDR[23:0]`：24 位地址，位于 bit 31:8（HAL 用 `<< ESMC_ADDR24_ADDR0_Pos`）。
const ADDR24_ADDR: u32 = 0xFFFF_FF00;

// XCR3 / XCR3_WE（RM 30.5.4 / 30.7.4）
/// `XCR3.ADDR8BIT`。
const XCR3_ADDR8BIT: u8 = 0x01;
/// `XCR3.ADDR16BIT`。
const XCR3_ADDR16BIT: u8 = 0x02;
/// `XCR3.ADDR32BIT`。
const XCR3_ADDR32BIT: u8 = 0x04;
/// `XCR3.NO_ADDR`。
const XCR3_NO_ADDR: u8 = 0x08;
/// `XCR3.xSPI_RWDS`。
const XCR3_XSPI_RWDS: u8 = 0x10;
/// `XCR3.CMD_EXT`。
const XCR3_CMD_EXT: u8 = 0x20;

// XSOCR / XSOCR_WE（RM 30.5.2 / 30.7.2）
/// `XSOCR.SPI_MODE[1:0]`。
const XSOCR_SPIMODE: u8 = 0x03;
/// `XSOCR.SEND_M`。
const XSOCR_SEND_M: u8 = 0x04;
/// `XSOCR.MULTICMD`。
const XSOCR_MULTICMD: u8 = 0x08;
/// `XSOCR.MULTIADDR`。
const XSOCR_MULTIADDR: u8 = 0x10;
/// `XSOCR.DDRCMD`。
const XSOCR_DDRCMD: u8 = 0x20;
/// `XSOCR.DDRADDR`。
const XSOCR_DDRADDR: u8 = 0x40;
/// `XSOCR.DDRDATA`。
const XSOCR_DDRDATA: u8 = 0x80;

// XDCR / XDCR_WE（RM 30.5.3 / 30.7.3）
/// `XDCR.DUMMY[5:0]`（PAC/SVD 里的字段名是 `XDUMMY`）。
const XDCR_DUMMY: u8 = 0x3F;
/// `XDCR.NO_CMD`。
const XDCR_NO_CMD: u8 = 0x40;

// IFR2（RM 30.4.14）
/// `IFR2.TXFIFOOVIF`：TX FIFO 溢出标志（RC_W1）。
///
/// SVD/PAC 只生成了该位的读侧（`txfifoovif()`），没有字段写入器，故清标志时按位整写。
const IFR2_TXFIFOOVIF: u8 = 0x10;
/// `IFR2.TXWORDOVIF`：TX 数据未按字对齐标志（RC_W1，bit5）。
const IFR2_TXWORDOVIF: u8 = 0x20;

// XMODE / XMODE_WE（RM 30.6.1 / 30.6.2）
/// `XMODE.M[7:0]`：备用字节，或 2 字节指令的第 2 字节。
const XMODE_M: u8 = 0xFF;

/// ESMC 错误类型。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// 等待状态/中断标志位超过 [`TIMEOUT_LOOPS`] 仍未就绪（对照 HAL 的
    /// `HAL_ESMC_ERROR_TIMEOUT`，多因外部存储器无响应、nCS/CLK 未接或时钟未使能）。
    Timeout,
    /// 传入的配置或命令非法（对照 HAL 的 `HAL_ESMC_ERROR_INVALID_PARAM`
    /// 与其 `assert_param` 校验，另见本驱动新增的几项一致性检查）。
    InvalidConfig,
    /// TX FIFO 上溢（`IFR2.TXFIFOOVIF`）：写入速度超过外部存储器取数速度。
    TxFifoOverrun,
    /// TX 数据未按字对齐（`IFR2.TXWORDOVIF`，RM 30.4.14）：
    /// 先写若干单字节、又写整字，导致同一 4 字节空间被写 5 个字节。
    TxNotWordAligned,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            Self::Timeout => "Operation Timed Out",
            Self::InvalidConfig => "Invalid Configuration",
            Self::TxFifoOverrun => "TX FIFO Overrun",
            Self::TxNotWordAligned => "TX Data Not Word Aligned",
        };
        f.write_str(msg)
    }
}

impl core::error::Error for Error {}

/// 时钟极性/相位组合（`CR2.CPOL`/`CR2.CPHA`，RM 30.3.5.2）。
///
/// ESMC 只支持模式 0 与模式 3，不支持中间相位。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClockMode {
    /// 模式 0：空闲时 SCLK 为低，第一个时钟边沿采样（`CPOL = 0`、`CPHA = 0`，复位默认）。
    Mode0,
    /// 模式 3：空闲时 SCLK 为高，第二个时钟边沿采样（`CPOL = 1`、`CPHA = 1`）。
    Mode3,
}

impl ClockMode {
    /// 是否写入 `CPOL`/`CPHA`（模式 3 两者同时置 1）。
    #[inline]
    fn is_mode3(self) -> bool {
        matches!(self, Self::Mode3)
    }
}

/// SPI 传输格式（`SOCR.SPIMODE[1:0]`，RM 30.3.3）。
///
/// 决定**地址之后**的地址/备用字节/数据阶段走几根线；指令与地址是否走多线
/// 另由 [`InstructionMode`] / [`AddressMode`] 控制。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TransferFormat {
    /// 单线（`2'b00`）：IO0 发、IO1 收。
    Single,
    /// 双线（`2'b01`）：IO0/IO1 同时收发 2 位。
    Dual,
    /// 四线（`2'b10`）：IO0~IO3 同时收发 4 位。
    Quad,
    /// 八线（`2'b11`）：IO0~IO7 同时收发 8 位。
    ///
    /// 注意：八线信号（IO4~IO7、DQS）的引脚不在本驱动的引脚 trait 集合内，
    /// 需调用方自行配置为 AF（见模块内 `impl_*` 的说明）；
    /// 在 `Config::dual_flash` 为 `true` 时该档表示「双 4 线（Dual-Flash）模式」。
    Octal,
}

impl TransferFormat {
    /// 取 `SPIMODE[1:0]` 的字段值。
    #[inline]
    fn bits(self) -> u8 {
        match self {
            Self::Single => 0b00,
            Self::Dual => 0b01,
            Self::Quad => 0b10,
            Self::Octal => 0b11,
        }
    }
}

/// 指令阶段模式（`SOCR.MULTICMD` + `DCR.NO_CMD`，RM 30.3.5.5/30.3.5.6）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InstructionMode {
    /// 不发指令（`DCR.NO_CMD = 1`），命令序列从地址阶段开始。
    None,
    /// 指令按单线发送（`MULTICMD = 0`，复位默认）。
    SingleLine,
    /// 指令按当前 [`TransferFormat`] 的多线方式发送（`MULTICMD = 1`）。
    MultiLines,
}

/// 指令长度（`CR3.CMD_EXT`，RM 30.3.2）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InstructionLength {
    /// 1 字节指令，取值自 `ADDR24.INS1`（复位默认）。
    OneByte,
    /// 2 字节指令（`CMD_EXT = 1`）：第 1 字节在 `ADDR24.INS1`，第 2 字节在 `ADDR32.MREG`。
    TwoBytes,
}

/// 地址阶段模式（`SOCR.MULTIADDR` + `CR3.NO_ADDR`，RM 30.3.2）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AddressMode {
    /// 不发地址（`CR3.NO_ADDR = 1`）。
    None,
    /// 地址按单线发送（`MULTIADDR = 0`）。
    SingleLine,
    /// 地址按当前 [`TransferFormat`] 的多线方式发送（`MULTIADDR = 1`）。
    MultiLines,
}

/// 地址宽度（`CR3.ADDR8BIT/ADDR16BIT/ADDR32BIT`，RM 30.3.2）。
///
/// 三者全 0 即 24 位（复位默认）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AddressSize {
    /// 8 位地址（`ADDR8BIT = 1`）。
    Bits8,
    /// 16 位地址（`ADDR16BIT = 1`）。
    Bits16,
    /// 24 位地址（全 0，复位默认）。
    Bits24,
    /// 32 位地址（`ADDR32BIT = 1`）：额外发送 `ADDR32.ADDR3`（`ADDR[31:24]`）。
    Bits32,
}

impl AddressSize {
    /// 取 `CR3` 的地址宽度位组合。
    #[inline]
    fn cr3_bits(self) -> u8 {
        match self {
            Self::Bits8 => CR3_ADDR8BIT,
            Self::Bits16 => CR3_ADDR16BIT,
            Self::Bits24 => 0,
            Self::Bits32 => CR3_ADDR32BIT,
        }
    }
}

/// 备用字节阶段模式（`SOCR.SEND_M`，RM 30.3.2）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AlternateByteMode {
    /// 不发送备用字节（`SEND_M = 0`，复位默认）。
    Disabled,
    /// 地址后追加 1 个备用字节（`SEND_M = 1`，取值自 `ADDR32.MREG`，
    /// 常见于 4IO 读命令的模式位，如 `0xEB` 的 `M[7:0]`）。
    Enabled,
}

/// 数据阶段方向与长度（`DCR.REC` + `BCR`，RM 30.3.4.1/30.4.9）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataMode {
    /// 无数据阶段（`CR3.NO_DATA = 1`）。
    None,
    /// 间接写（`DCR.REC = 0`）：数据经 [`Esmc::write`] 写入 `DATA`。
    Write,
    /// 间接读（`DCR.REC = 1`）：数据经 [`Esmc::read`] 从 `DATA` 读出。
    Read,
}

/// DDR（双数据速率）模式（`SOCR.DDRCMD/DDRADDR/DDRDATA`，RM 30.3.3）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DdrMode {
    /// 全 SDR（复位默认）。
    Disabled,
    /// 仅数据阶段 DDR（`DDRDATA = 1`）。
    Data,
    /// 地址 + 数据阶段 DDR。
    AddressAndData,
    /// 指令 + 地址 + 数据阶段全部 DDR。
    CommandAddressAndData,
}

impl DdrMode {
    /// `DDRCMD` 位。
    #[inline]
    fn cmd_bit(self) -> bool {
        matches!(self, Self::CommandAddressAndData)
    }

    /// `DDRADDR` 位。
    #[inline]
    fn addr_bit(self) -> bool {
        matches!(self, Self::AddressAndData | Self::CommandAddressAndData)
    }

    /// `DDRDATA` 位。
    #[inline]
    fn data_bit(self) -> bool {
        !matches!(self, Self::Disabled)
    }

    /// 合成 `SOCR` 里的三个 DDR 位。
    #[inline]
    fn socr_bits(self) -> u8 {
        (if self.cmd_bit() { SOCR_DDRCMD } else { 0 })
            | (if self.addr_bit() { SOCR_DDRADDR } else { 0 })
            | (if self.data_bit() { SOCR_DDRDATA } else { 0 })
    }
}

/// 片选选择（`ADDR32.SS[5:0]` / `XSSOCR.SS[5:0]`，RM 30.6.3/30.8.2）。
///
/// 位序与硬件 `ESMC_CSn_NCS` 引脚一一对应：`Cs0` 即 `ESMC_CS0_NCS`。
/// 该位选中的 nCS 才会在传输期间被驱动，故必须与 [`ChipSelects`] 里实际接线的
/// 那个信号保持一致。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChipSelect {
    /// `ESMC_CS0_NCS`
    Cs0,
    /// `ESMC_CS1_NCS`
    Cs1,
    /// `ESMC_CS2_NCS`
    Cs2,
    /// `ESMC_CS3_NCS`
    Cs3,
    /// `ESMC_CS4_NCS`
    Cs4,
    /// `ESMC_CS5_NCS`
    Cs5,
}

impl ChipSelect {
    /// 取 `SS[5:0]` 的位掩码（bit0 = CS0）。
    #[inline]
    fn bits(self) -> u8 {
        1 << (self as u8)
    }
}

/// ESMC 初始化配置（对照 HAL 的 `ESMC_InitTypeDef`）。
///
/// `#[non_exhaustive]`，外部构造请用 [`Config::default()`] 后逐字段赋值。
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// 目标 SCLK 频率。实际值由 `BAUD` 分频得出（`SCLK = HCLK / BAUD`，RM 30.4.4），
    /// 取**不超过**该值的最快分频档；`BAUD < 2` 时硬件固定二分频，
    /// 故 SCLK 上限为 `HCLK / 2`。可用 [`Esmc::frequency`] 读回实际值。
    pub sclk: Hertz,
    /// 时钟极性/相位（只支持模式 0 与模式 3）。
    pub clock_mode: ClockMode,
    /// 双 Flash（双 4 线）模式（`CR.2xQUAD`，RM 30.3.3）：同时操作两个 Quad SPI 存储器，
    /// 每个字节的 `[7:4]` 位来自 Flash1、`[3:0]` 位来自 Flash2，`BCR` 必须为偶数。
    /// 数据/地址线的引脚集合需相应增加（本驱动只配置 Bank1 的 IO0~IO3）。
    pub dual_flash: bool,
    /// nCS 在两条命令之间保持**高**的最少 SCLK 周期数（`TCR.HIGH[3:0]`，RM 30.4.3）。
    ///
    /// 取值为**寄存器原始值**：`0` 与 `1` 都是 1 个周期，`n ≥ 2` 为 `n` 个周期。
    /// 合法范围 0~15。
    pub cs_high_time: u8,
    /// nCS 建立/保持时间，单位是 **HCLK** 周期（`TCR.HOLD[3:0]`，RM 30.4.3）。
    ///
    /// 取值为**寄存器原始值**：`0` 为禁用，`n ≥ 1` 插入 `n + 1` 个周期。
    /// 合法范围 0~15。（建立时间的精确值随 SDR/DDR 与模式 0/3 略有差异，见 RM 表。）
    pub cs_hold_time: u8,
    /// ESMC 各复用引脚的输出速度（`OSPEEDR`，RM 9.4.3）。
    pub gpio_speed: Speed,
}

impl Default for Config {
    /// 默认值对照两个官方例程（`ESMC_ReadWrite_QSPI_Polling` 与
    /// `ESMC_XIP_ReadWritePSRAM_4IO`）：模式 0、单 Flash、
    /// `CS_HIGH_TIME_1_CYCLE | CS_HOLD_TIME_2_CYCLE`（即 `TCR.HIGH = 1`、`TCR.HOLD = 1`）、
    /// GPIO 超高速档。SCLK 默认取保守的 1 MHz（DS 表 5-58 允许 85 MHz，按板上走线与
    /// 存储器型号逐步上调即可）。
    fn default() -> Self {
        Self {
            sclk: Hertz(1_000_000),
            clock_mode: ClockMode::Mode0,
            dual_flash: false,
            cs_high_time: 1,
            cs_hold_time: 1,
            gpio_speed: Speed::VeryHigh,
        }
    }
}

/// 一条 ESMC 命令（对照 HAL 的 `ESMC_CommandTypeDef`）。
///
/// 命令由 5 个阶段组成：指令 → 地址 → 备用字节 → Dummy → 数据，
/// 任一阶段都可跳过（RM 30.3.2）。构造时用 `..Default::default()` 补齐未用字段：
///
/// ```ignore
/// let cmd = Command {
///     instruction_mode: InstructionMode::SingleLine,
///     instruction: 0xEB,               // 4IO Fast Read
///     address_mode: AddressMode::SingleLine,
///     address: addr,
///     data_mode: DataMode::Read,
///     nb_data: len,
///     transfer_format: TransferFormat::Quad,
///     dummy_cycles: 6,
///     chip_select: ChipSelect::Cs0,
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Command {
    /// 地址/备用字节/数据阶段的传输格式。
    pub transfer_format: TransferFormat,
    /// 指令阶段模式。
    pub instruction_mode: InstructionMode,
    /// 指令长度（1 或 2 字节）。
    pub instruction_length: InstructionLength,
    /// 指令码。1 字节指令只用低 8 位；2 字节指令时低字节在前、
    /// 高字节（`>> 8`）作为第 2 字节。
    pub instruction: u16,
    /// 地址阶段模式。
    pub address_mode: AddressMode,
    /// 地址宽度。
    pub address_size: AddressSize,
    /// 地址值（按 [`AddressSize`] 取低 8/16/24/32 位）。
    pub address: u32,
    /// 备用字节阶段模式。
    pub alternate_byte_mode: AlternateByteMode,
    /// 备用字节内容（仅在 [`AlternateByteMode::Enabled`] 时发送）。
    pub alternate_byte: u8,
    /// 数据阶段方向。
    pub data_mode: DataMode,
    /// 数据阶段字节数。`1..=0xFFFF_FFFE` 为定长（写入 `BCR`）；
    /// `0xFFFF_FFFF` 表示不定长（`BCR = 0`，由 FIFO 状态机持续收发，
    /// 直到主机不再提供/取走数据，RM 30.3.5.1）。
    pub nb_data: u32,
    /// DDR 模式。
    pub ddr: DdrMode,
    /// Dummy 周期数，0~63（`DCR.DUMMY`，RM 30.3.5.7）。
    pub dummy_cycles: u8,
    /// 本次命令驱动哪个 nCS。
    pub chip_select: ChipSelect,
    /// 使能 xSPI 的 RWDS（DQS）引脚（`CR3.xSPI_RWDS`，RM 30.3.3）。
    pub dqs: bool,
    /// 数据阶段采样点相对 SCLK 边沿延伸的 **HCLK** 周期数，0~63
    /// （`STRR.SMP`，RM 30.4.16）。仅在 SDR 模式下有效，且不得超过半个 SCLK 周期。
    pub sample_shifting: u8,
    /// 是否启用 xSPI Dummy 双倍延迟（`CR2.XSPI_DUAL_LAT_OFF` 的**反相**，
    /// RM 30.4.2/表 30-3）：`false` 时写入 `XSPI_DUAL_LAT_OFF = 1`（关闭双倍延迟），
    /// `true` 时清零该位（Dummy 周期变为 `2N-1`）。只在 xSPI 八线模式下有意义。
    pub xspi_dual_latency: bool,
}

impl Default for Command {
    /// 默认值对照 HAL `ESMC_CommandTypeDef` 的"清零初始化 + 常见默认"：
    /// 单线格式、无指令/地址/备用字节/数据、24 位地址、无 Dummy、DDR 关闭、
    /// 片选 CS0、不使能 DQS、无采样延伸、关闭 xSPI 双倍延迟。
    fn default() -> Self {
        Self {
            transfer_format: TransferFormat::Single,
            instruction_mode: InstructionMode::None,
            instruction_length: InstructionLength::OneByte,
            instruction: 0,
            address_mode: AddressMode::None,
            address_size: AddressSize::Bits24,
            address: 0,
            alternate_byte_mode: AlternateByteMode::Disabled,
            alternate_byte: 0,
            data_mode: DataMode::None,
            nb_data: 0,
            ddr: DdrMode::Disabled,
            dummy_cycles: 0,
            chip_select: ChipSelect::Cs0,
            dqs: false,
            sample_shifting: 0,
            xspi_dual_latency: false,
        }
    }
}

// ---------------------------------------------------------------------------
// 引脚（密封 trait + 按信号分组的实现）
//
// 信号 → 引脚与 AF 号的依据：官方例程的 MSP 初始化（最硬的证据）+ DS V0.13
// 端口复用表 3-3…3-8 交叉核对。**只收录已有官方例程证实的引脚**——
// 复用表经 PDF 抽取后存在"连续空单元格被压缩"的问题，仅凭表格计数容易把 AF 号
// 数少 1（教训见 DEMO 文档），故宁缺毋滥；需要其它引脚时按同样证据补进下面的
// `impl_*` 宏即可。
// ---------------------------------------------------------------------------

mod sealed {
    /// 密封的时钟引脚查询。
    pub trait SealedClkPin {
        /// 该引脚作为 `ESMC_CLK` 时的复用功能编号。
        fn af_num(&self) -> u8;
    }

    /// 密封的 Bank1 IO0 引脚查询。
    pub trait SealedIo0Pin {
        /// 该引脚作为 `ESMC_BK1_IO0` 时的复用功能编号。
        fn af_num(&self) -> u8;
    }

    /// 密封的 Bank1 IO1 引脚查询。
    pub trait SealedIo1Pin {
        /// 该引脚作为 `ESMC_BK1_IO1` 时的复用功能编号。
        fn af_num(&self) -> u8;
    }

    /// 密封的 Bank1 IO2 引脚查询。
    pub trait SealedIo2Pin {
        /// 该引脚作为 `ESMC_BK1_IO2` 时的复用功能编号。
        fn af_num(&self) -> u8;
    }

    /// 密封的 Bank1 IO3 引脚查询。
    pub trait SealedIo3Pin {
        /// 该引脚作为 `ESMC_BK1_IO3` 时的复用功能编号。
        fn af_num(&self) -> u8;
    }

    /// 密封的 `ESMC_CS0_NCS` 引脚查询。
    pub trait SealedCs0Pin {
        /// 该引脚作为 `ESMC_CS0_NCS` 时的复用功能编号。
        fn af_num(&self) -> u8;
    }

    /// 密封的 `ESMC_CS1_NCS` 引脚查询。
    pub trait SealedCs1Pin {
        /// 该引脚作为 `ESMC_CS1_NCS` 时的复用功能编号。
        fn af_num(&self) -> u8;
    }

    /// 密封的 `ESMC_CS2_NCS` 引脚查询。
    pub trait SealedCs2Pin {
        /// 该引脚作为 `ESMC_CS2_NCS` 时的复用功能编号。
        fn af_num(&self) -> u8;
    }

    /// 密封的 `ESMC_CS3_NCS` 引脚查询。
    pub trait SealedCs3Pin {
        /// 该引脚作为 `ESMC_CS3_NCS` 时的复用功能编号。
        fn af_num(&self) -> u8;
    }

    /// 密封的 `ESMC_CS4_NCS` 引脚查询。
    pub trait SealedCs4Pin {
        /// 该引脚作为 `ESMC_CS4_NCS` 时的复用功能编号。
        fn af_num(&self) -> u8;
    }

    /// 密封的 `ESMC_CS5_NCS` 引脚查询。
    pub trait SealedCs5Pin {
        /// 该引脚作为 `ESMC_CS5_NCS` 时的复用功能编号。
        fn af_num(&self) -> u8;
    }
}

/// 可作为 `ESMC_CLK` 的引脚。
#[allow(private_bounds)]
pub trait ClkPin: sealed::SealedClkPin + Pin {}

/// 可作为 `ESMC_BK1_IO0` 的引脚。
#[allow(private_bounds)]
pub trait Io0Pin: sealed::SealedIo0Pin + Pin {}

/// 可作为 `ESMC_BK1_IO1` 的引脚。
#[allow(private_bounds)]
pub trait Io1Pin: sealed::SealedIo1Pin + Pin {}

/// 可作为 `ESMC_BK1_IO2` 的引脚。
#[allow(private_bounds)]
pub trait Io2Pin: sealed::SealedIo2Pin + Pin {}

/// 可作为 `ESMC_BK1_IO3` 的引脚。
#[allow(private_bounds)]
pub trait Io3Pin: sealed::SealedIo3Pin + Pin {}

/// 可作为 `ESMC_CS0_NCS` 的引脚。
#[allow(private_bounds)]
pub trait Cs0Pin: sealed::SealedCs0Pin + Pin {}

/// 可作为 `ESMC_CS1_NCS` 的引脚。
#[allow(private_bounds)]
pub trait Cs1Pin: sealed::SealedCs1Pin + Pin {}

/// 可作为 `ESMC_CS2_NCS` 的引脚。
#[allow(private_bounds)]
pub trait Cs2Pin: sealed::SealedCs2Pin + Pin {}

/// 可作为 `ESMC_CS3_NCS` 的引脚。
#[allow(private_bounds)]
pub trait Cs3Pin: sealed::SealedCs3Pin + Pin {}

/// 可作为 `ESMC_CS4_NCS` 的引脚。
#[allow(private_bounds)]
pub trait Cs4Pin: sealed::SealedCs4Pin + Pin {}

/// 可作为 `ESMC_CS5_NCS` 的引脚。
#[allow(private_bounds)]
pub trait Cs5Pin: sealed::SealedCs5Pin + Pin {}

/// 为引脚生成某信号的密封查询与公开 trait 实现（含 AF 号）。
macro_rules! impl_signal_pins {
    ($sealed:ident, $trait:ident, $($pin:ident = $af:literal),+ $(,)?) => {
        $(
            impl sealed::$sealed for crate::peripherals::$pin {
                #[inline]
                fn af_num(&self) -> u8 {
                    $af
                }
            }
            impl $trait for crate::peripherals::$pin {}
        )+
    };
}

// 依据 1（**本板接线**，来自 PY32Studio 生成工程 `参考设计/demo/Core/SRC/esmc.c`
// 的 `HAL_ESMC_MspInit`，与硬件完全一致，是最硬证据）：
//   PB10 = ESMC_CLK、PE11 = ESMC_CS0_NCS、PE12 = ESMC_BK1_IO0、PE13 = ESMC_BK1_IO1、
//   PE14 = ESMC_BK1_IO2、PE15 = ESMC_BK1_IO3 —— 全部 `GPIO_AF10_ESMC`。
// 依据 2（官方 STK 例程 `Example/ESMC/*/Src/py32e407_hal_msp.c`）：
//   ESMC_ReadWrite_QSPI_Polling / _IT / _DMA / _4IO_DTR / MemoryMapping_QSPI：
//   CLK = PA3、IO0 = PB1、IO1 = PB0、IO2 = PA7、IO3 = PA6、SS5 = PD2，全部 AF10；
//   ESMC_XIP_ReadWritePSRAM_4IO：CLK = PE10(AF10)、SS0 = PE11(AF10)、
//   IO0 = **PE15(AF9)**、IO1 = PE13(AF10)、IO2 = PE14(AF10)、IO3 = PE9(AF10)。
// 注：DS 表 3-4 里 PE15 同时占两列——AF9 = `ESMC_BK1_IO0`、AF10 = `ESMC_BK1_IO3`，
// 上面两组证据分别落到这两个功能上，故 PE15 同时实现 Io0Pin(9) 与 Io3Pin(10)。
impl_signal_pins!(SealedClkPin, ClkPin, PB10 = 10, PA3 = 10, PE10 = 10);
impl_signal_pins!(SealedIo0Pin, Io0Pin, PE12 = 10, PB1 = 10, PE15 = 9);
impl_signal_pins!(SealedIo1Pin, Io1Pin, PE13 = 10, PB0 = 10);
impl_signal_pins!(SealedIo2Pin, Io2Pin, PE14 = 10, PA7 = 10);
impl_signal_pins!(SealedIo3Pin, Io3Pin, PE15 = 10, PA6 = 10, PE9 = 10);
impl_signal_pins!(SealedCs0Pin, Cs0Pin, PE11 = 10);
impl_signal_pins!(SealedCs5Pin, Cs5Pin, PD2 = 10);

/// 单个片选引脚：`Flex` 句柄 + 该信号对应的 AF 号。
///
/// AF 号必须在引脚被 [`Flex`] 类型擦除（`Peri<_, AnyPin>`）之前取出来存下，
/// 否则配置引脚时就查不到该信号用哪一列复用了。
struct CsPinSlot<'d> {
    /// 引脚句柄。
    flex: Flex<'d>,
    /// 该引脚作为此片选信号时的复用功能编号。
    af: u8,
}

/// ESMC 片选（nCS）引脚集合，最多 6 个。
///
/// 只需填写**实际接线**的片选信号；未填写的 `nCS` 输出不会被配置，也不会被
/// [`Command::chip_select`] 选中。用 [`ChipSelects::none`] 起头再逐个挂上：
///
/// ```ignore
/// let cs = ChipSelects::none().cs0(p.PE11);
/// ```
pub struct ChipSelects<'d> {
    /// 下标 `n` 对应硬件信号 `ESMC_CSn_NCS`。
    pins: [Option<CsPinSlot<'d>>; 6],
}

impl<'d> ChipSelects<'d> {
    /// 空集合（不使用任何 ESMC 片选输出）。
    #[inline]
    pub const fn none() -> Self {
        Self {
            pins: [None, None, None, None, None, None],
        }
    }

    /// 挂上 `ESMC_CS0_NCS` 引脚。
    #[inline]
    pub fn cs0(mut self, pin: Peri<'d, impl Cs0Pin>) -> Self {
        let af = pin.af_num();
        self.pins[0] = Some(CsPinSlot {
            flex: Flex::new(pin),
            af,
        });
        self
    }

    /// 挂上 `ESMC_CS1_NCS` 引脚。
    #[inline]
    pub fn cs1(mut self, pin: Peri<'d, impl Cs1Pin>) -> Self {
        let af = pin.af_num();
        self.pins[1] = Some(CsPinSlot {
            flex: Flex::new(pin),
            af,
        });
        self
    }

    /// 挂上 `ESMC_CS2_NCS` 引脚。
    #[inline]
    pub fn cs2(mut self, pin: Peri<'d, impl Cs2Pin>) -> Self {
        let af = pin.af_num();
        self.pins[2] = Some(CsPinSlot {
            flex: Flex::new(pin),
            af,
        });
        self
    }

    /// 挂上 `ESMC_CS3_NCS` 引脚。
    #[inline]
    pub fn cs3(mut self, pin: Peri<'d, impl Cs3Pin>) -> Self {
        let af = pin.af_num();
        self.pins[3] = Some(CsPinSlot {
            flex: Flex::new(pin),
            af,
        });
        self
    }

    /// 挂上 `ESMC_CS4_NCS` 引脚。
    #[inline]
    pub fn cs4(mut self, pin: Peri<'d, impl Cs4Pin>) -> Self {
        let af = pin.af_num();
        self.pins[4] = Some(CsPinSlot {
            flex: Flex::new(pin),
            af,
        });
        self
    }

    /// 挂上 `ESMC_CS5_NCS` 引脚。
    #[inline]
    pub fn cs5(mut self, pin: Peri<'d, impl Cs5Pin>) -> Self {
        let af = pin.af_num();
        self.pins[5] = Some(CsPinSlot {
            flex: Flex::new(pin),
            af,
        });
        self
    }
}

// ---------------------------------------------------------------------------
// 驱动
// ---------------------------------------------------------------------------

/// ESMC 同步驱动。
///
/// 引脚所有权由 [`Peri`] 保证：同一物理引脚同一时刻至多被一个驱动持有。
/// 驱动被 Drop 时各引脚的 [`Flex`] 会恢复为模拟（高阻）态。
///
/// 典型用法见模块文档；命令的生命周期是「[`command`](Esmc::command) →
/// [`write`](Esmc::write) / [`read`](Esmc::read)」，或直接用组合后的
/// [`command_write`](Esmc::command_write) / [`command_read`](Esmc::command_read)。
pub struct Esmc<'d> {
    /// 外设单例（借用式所有权）。
    _peri: Peri<'d, crate::peripherals::ESMC>,
    /// `ESMC_CLK`
    _clk: Flex<'d>,
    /// `ESMC_BK1_IO0`~`IO3`
    _io: [Flex<'d>; 4],
    /// 片选输出
    _cs: ChipSelects<'d>,
    /// 待写入 `ADDR24` 的「指令 + 24 位地址」值。写它是**启动命令序列**的动作，
    /// 故配置阶段只缓存、由 `write`/`read` 写一次（对照 HAL 的 `ContexAddr24`）。
    addr24: u32,
    /// 上一条命令的数据阶段方向。
    data_mode: DataMode,
    /// 上一条命令是否为不定长（`BCR == 0`），决定收尾等待哪个标志。
    variable_length: bool,
}

impl<'d> Esmc<'d> {
    /// 创建驱动并完成初始化（阻塞），不占用任何片选输出。
    ///
    /// 适用于 CS 由外部 GPIO 手动控制，或本设计只用单个存储器的场景；
    /// 需要用硬件 nCS 时改用 [`Esmc::new_blocking_with_cs`]。
    ///
    /// # Errors
    ///
    /// 配置非法时返回 [`Error::InvalidConfig`]（`sclk` 为 0、`cs_*_time > 15`）。
    pub fn new_blocking(
        peri: Peri<'d, crate::peripherals::ESMC>,
        clk: Peri<'d, impl ClkPin>,
        io0: Peri<'d, impl Io0Pin>,
        io1: Peri<'d, impl Io1Pin>,
        io2: Peri<'d, impl Io2Pin>,
        io3: Peri<'d, impl Io3Pin>,
        config: Config,
    ) -> Result<Self, Error> {
        Self::new_blocking_with_cs(
            peri,
            clk,
            io0,
            io1,
            io2,
            io3,
            ChipSelects::none(),
            config,
        )
    }

    /// 创建驱动并完成初始化（阻塞），同时接管若干片选输出。
    ///
    /// 流程对照 `HAL_ESMC_Init`（+ 其 `HAL_ESMC_MspInit`）：
    /// 1. 使能 AHB1 上的 ESMC 时钟（含 LL 要求的读回等待）；
    /// 2. 把 CLK / IO0~IO3 / 各片选引脚配为复用功能（AF 号见各 `impl_signal_pins!`）；
    /// 3. 等 `SR.IDLE_STATE` 置位 → 写 `CR2`（CPOL/CPHA）→ `BAUD` → `TCR`
    ///    （HIGH/HOLD）→ 清空全部片选位 → 写 `CR`（使能 SPI、按需使能双 Flash）
    ///    → 清 `IER1`/`IER2`（本驱动不用中断）→ 清 FIFO 指针。
    ///
    /// 时钟使能是**幂等**的（置位型），且**不复位外设**——与 HAL 一致：
    /// HAL 只在 `HAL_ESMC_DeInit` 里做软复位。
    ///
    /// # Errors
    ///
    /// 配置非法时返回 [`Error::InvalidConfig`]；初始化等待 IDLE 超时返回 [`Error::Timeout`]。
    pub fn new_blocking_with_cs(
        peri: Peri<'d, crate::peripherals::ESMC>,
        clk: Peri<'d, impl ClkPin>,
        io0: Peri<'d, impl Io0Pin>,
        io1: Peri<'d, impl Io1Pin>,
        io2: Peri<'d, impl Io2Pin>,
        io3: Peri<'d, impl Io3Pin>,
        cs: ChipSelects<'d>,
        config: Config,
    ) -> Result<Self, Error> {
        if config.sclk.0 == 0 || config.cs_high_time > 0xF || config.cs_hold_time > 0xF {
            return Err(Error::InvalidConfig);
        }

        // 1. 使能外设时钟（RCC_AHB1ENR.ESMCEN）
        crate::rcc::enable_ahb1(AHB1ENR_ESMCEN);

        // AF 号要在引脚被 Flex 接管之前取出
        let clk_af = clk.af_num();
        let io0_af = io0.af_num();
        let io1_af = io1.af_num();
        let io2_af = io2.af_num();
        let io3_af = io3.af_num();

        let mut this = Self {
            _peri: peri,
            _clk: Flex::new(clk),
            _io: [
                Flex::new(io0),
                Flex::new(io1),
                Flex::new(io2),
                Flex::new(io3),
            ],
            _cs: cs,
            addr24: 0,
            data_mode: DataMode::None,
            variable_length: false,
        };
        this.configure_pins(
            clk_af,
            [io0_af, io1_af, io2_af, io3_af],
            config.gpio_speed,
        );
        init_regs(&config)?;
        Ok(this)
    }

    /// 配置 CLK / IO0~IO3 / 片选引脚为复用功能。
    ///
    /// 上下拉照抄官方例程的 MSP：CLK 与 IO0/IO1 无上下拉，IO2/IO3（`#WP`/`#HOLD`）与所有
    /// nCS 上拉——空闲时这两类线被悬空拉低会误改状态位/误选中器件。
    /// （PY32Studio 生成工程对全部 ESMC 引脚都用 `GPIO_NOPULL`，官方例程则对
    /// IO2/IO3/CS 用上拉；本驱动取后者，属更保守的一侧。）
    /// 复用模式下引脚方向由外设控制，故一律用推挽输出类型。
    fn configure_pins(&mut self, clk_af: u8, io_af: [u8; 4], speed: Speed) {
        self._clk
            .set_as_alternate(clk_af, speed, Pull::None, OutputType::PushPull);
        for (i, flex) in self._io.iter_mut().enumerate() {
            // IO0/IO1 为 SO/SI（双向单线），IO2/IO3 为 #WP/#HOLD，需要上拉
            let pull = if i < 2 { Pull::None } else { Pull::Up };
            flex.set_as_alternate(io_af[i], speed, pull, OutputType::PushPull);
        }
        for slot in self._cs.pins.iter_mut() {
            if let Some(slot) = slot.as_mut() {
                slot.flex
                    .set_as_alternate(slot.af, speed, Pull::Up, OutputType::PushPull);
            }
        }
    }

    /// 当前**实际** SCLK 频率——由 `BAUD` 与实际 HCLK 反推（RM 30.4.4：
    /// `BAUD < 2` 时二分频，否则 `SCLK = HCLK / BAUD`），不是回显 [`Config::sclk`]。
    #[inline]
    pub fn frequency(&self) -> Hertz {
        let baud = regs().baud().read().baud().bits();
        let hclk = crate::rcc::clocks().hclk.0;
        Hertz(if baud < 2 { hclk / 2 } else { hclk / u32::from(baud) })
    }

    /// 内存映射（XIP）当前是否已使能（`CR.XIPEN`，RM 30.4.1）。
    ///
    /// **解引用映射窗口（`0x0400_0000`）之前先查这一位**：RM 30.3.4.2 明确
    /// "如果 `XIP_EN` 位被清除，XIP 总线将设置为忙，用以指示访问错误"——
    /// 此时对映射区的 AHB 访问可能把总线挂住。
    #[inline]
    pub fn xip_enabled(&self) -> bool {
        regs().cr().read().xipen().bit_is_set()
    }

    /// 配置并启动一条命令（对照 `HAL_ESMC_Command` 的同步路径）。
    ///
    /// 命令分两类：
    /// - **只有指令**（无地址、无备用字节、无数据）→ 走 HAL 的
    ///   `ESMC_FUNCTIONAL_MODE_INDIRECT_INSTRUCT`，经 `SFCR` 发送，等待完成；
    /// - **其余** → 走 `ESMC_FUNCTIONAL_MODE_INDIRECT_READ_WRITE`，配好各阶段后：
    ///   - 无数据阶段 → 立即写 `ADDR24` 启动并等待完成；
    ///   - 有数据阶段 → 只缓存 `ADDR24`，由随后的 [`write`](Esmc::write) /
    ///     [`read`](Esmc::read) 写它来启动（HAL 同此设计）。
    ///
    /// # Errors
    ///
    /// 命令字段非法返回 [`Error::InvalidConfig`]；等待完成超时返回 [`Error::Timeout`]。
    pub fn command(&mut self, cmd: &Command) -> Result<(), Error> {
        validate_indirect(cmd)?;
        let regs = regs();
        wait_idle(regs)?;

        // 只有指令一条：用 SFCR 发送（RM 30.3.5.5：SFCR0 专用于单字节 Flash 命令）
        if cmd.instruction_mode != InstructionMode::None
            && cmd.address_mode == AddressMode::None
            && cmd.alternate_byte_mode == AlternateByteMode::Disabled
            && cmd.data_mode == DataMode::None
        {
            configure_indirect_instruct(regs, cmd);
            self.data_mode = DataMode::None;
            self.variable_length = false;
            return wait_idle_done(regs);
        }

        let addr24 = configure_indirect(regs, cmd);
        self.data_mode = cmd.data_mode;
        if cmd.data_mode == DataMode::None {
            self.addr24 = addr24;
            // 无数据阶段：写 ADDR24 即启动并结束整个命令序列
            unsafe { regs.addr24().write(|w| w.bits(addr24)) };
            return wait_idle_done(regs);
        }
        // 数据阶段由随后的 write/read 完成（此时才写 ADDR24 启动）
        self.addr24 = addr24;
        Ok(())
    }

    /// 阻塞发送数据阶段（间接写，对照 `HAL_ESMC_Transmit`）。
    ///
    /// 必须在 [`command`](Esmc::command) 配好 [`DataMode::Write`] 且 `nb_data`
    /// 与 `data.len()` 一致后调用；写 `ADDR24` 才真正启动命令序列。
    ///
    /// # Errors
    ///
    /// 上一条命令不是写、或长度为 0 时返回 [`Error::InvalidConfig`]；
    /// 等 FIFO 让出空间超时返回 [`Error::Timeout`]；
    /// 数据未按字对齐时返回 [`Error::TxNotWordAligned`]。
    pub fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        if self.data_mode != DataMode::Write || data.is_empty() {
            return Err(Error::InvalidConfig);
        }
        let regs = regs();

        // 启动命令序列
        unsafe { regs.addr24().write(|w| w.bits(self.addr24)) };

        let result = self.write_phase(regs, data);
        // 无论成败都要收尾（HAL 同样在循环外无条件关片选）
        disable_slave(regs);
        result
    }

    /// 阻塞接收数据阶段（间接读，对照 `HAL_ESMC_Receive`）。
    ///
    /// 必须在 [`command`](Esmc::command) 配好 [`DataMode::Read`] 且 `nb_data`
    /// 与 `data.len()` 一致后调用。
    ///
    /// # Errors
    ///
    /// 上一条命令不是读、或长度为 0 时返回 [`Error::InvalidConfig`]；
    /// 等数据/完成标志超时返回 [`Error::Timeout`]。
    pub fn read(&mut self, data: &mut [u8]) -> Result<(), Error> {
        if self.data_mode != DataMode::Read || data.is_empty() {
            return Err(Error::InvalidConfig);
        }
        let regs = regs();

        // 启动命令序列
        unsafe { regs.addr24().write(|w| w.bits(self.addr24)) };

        let result = self.read_phase(regs, data);
        disable_slave(regs);
        result
    }

    /// 发命令并立即写数据（[`command`](Esmc::command) + [`write`](Esmc::write)）。
    ///
    /// # Errors
    ///
    /// `cmd.nb_data` 与 `data.len()` 不一致时返回 [`Error::InvalidConfig`]；
    /// 其余同 `command` / `write`。
    pub fn command_write(&mut self, cmd: &Command, data: &[u8]) -> Result<(), Error> {
        if cmd.data_mode != DataMode::Write || cmd.nb_data as usize != data.len() {
            return Err(Error::InvalidConfig);
        }
        self.command(cmd)?;
        self.write(data)
    }

    /// 发命令并立即读数据（[`command`](Esmc::command) + [`read`](Esmc::read)）。
    ///
    /// # Errors
    ///
    /// `cmd.nb_data` 与 `data.len()` 不一致时返回 [`Error::InvalidConfig`]；
    /// 其余同 `command` / `read`。
    pub fn command_read(&mut self, cmd: &Command, data: &mut [u8]) -> Result<(), Error> {
        if cmd.data_mode != DataMode::Read || cmd.nb_data as usize != data.len() {
            return Err(Error::InvalidConfig);
        }
        self.command(cmd)?;
        self.read(data)
    }

    /// 进入内存映射（XIP）读模式（对照 `HAL_ESMC_MemoryMapped`）。
    ///
    /// 流程：等 IDLE → 清 `CR.XIPEN/DMAEN/GIE` → 装配 XIP 读寄存器组
    /// （`XSFCR`/`XSOCR`/`XDCR`/`XCR3`/`XMODE`/`XSSOCR`/`XSTRR`）→
    /// 置 `CR.XIPEN` → 置 `ADDR32.SS_SET` 驱动所选 nCS。
    /// 之后外部存储器被映射到系统地址空间，CPU 直接按地址读即可，无需再下命令。
    ///
    /// 清除 `CR.XIPEN` 可退出 XIP（见 [`abort`](Esmc::abort) 与
    /// [`Esmc::exit_xip`]）。注意 XIP 期间若要发新的大容量命令，
    /// 必须先退出 XIP / 避免访问该映射区间（RM 30.3.4.2）。
    ///
    /// 注意 `cmd.nb_data` 在 XIP 下必须为 0（数据量由 AHB 访问决定），
    /// `cmd.data_mode` 可留默认的 [`DataMode::None`]；地址宽度按外部存储器容量选
    /// （16 MiB 用 [`AddressSize::Bits24`] 刚好覆盖）。
    ///
    /// # Errors
    ///
    /// 命令字段非法返回 [`Error::InvalidConfig`]；等待 IDLE 超时返回 [`Error::Timeout`]。
    pub fn memory_mapped(&mut self, cmd: &Command) -> Result<(), Error> {
        validate_xip(cmd, DataMode::Read)?;
        let regs = regs();
        wait_idle(regs)?;

        // 关 XIP / DMA / 全局中断后再改配置
        regs.cr().modify(|_, w| {
            w.xipen().clear_bit();
            w.dmaen().clear_bit();
            w.gie().clear_bit();
            w
        });
        configure_xip_read(regs, cmd);

        regs.cr().modify(|_, w| w.xipen().set_bit());
        enable_slave(regs);
        self.data_mode = DataMode::None;
        self.variable_length = false;
        Ok(())
    }

    /// 配置内存映射（XIP）模式下的**写**命令（对照 `HAL_ESMC_MemoryWrite`）。
    ///
    /// 与 [`memory_mapped`](Esmc::memory_mapped) 的区别是装配 `XSFCR_WE`/`XSOCR_WE`/
    /// `XDCR_WE`/`XCR3_WE`/`XMODE_WE`/`XSSOCR_WE` 一套寄存器。RM 30.3.4 明确：
    /// **XIP 写仅支持八线以外的 PSRAM**，且模式 0 时需要 SCLK 低于 HCLK/2
    /// （即 `BAUD ≥ 2`，本驱动的分频下限恰好满足）。
    ///
    /// # Errors
    ///
    /// 同 [`memory_mapped`](Esmc::memory_mapped)。
    pub fn memory_write(&mut self, cmd: &Command) -> Result<(), Error> {
        validate_xip(cmd, DataMode::Write)?;
        let regs = regs();
        wait_idle(regs)?;

        regs.cr().modify(|_, w| {
            w.xipen().clear_bit();
            w.dmaen().clear_bit();
            w.gie().clear_bit();
            w
        });
        configure_xip_write(regs, cmd);

        regs.cr().modify(|_, w| w.xipen().set_bit());
        enable_slave(regs);
        self.data_mode = DataMode::None;
        self.variable_length = false;
        Ok(())
    }

    /// 退出内存映射模式并停止当前操作（清 `CR.XIPEN` 后关片选、等回 IDLE）。
    ///
    /// # Errors
    ///
    /// 等待回到 IDLE 超时返回 [`Error::Timeout`]。
    pub fn exit_xip(&mut self) -> Result<(), Error> {
        let regs = regs();
        regs.cr().modify(|_, w| w.xipen().clear_bit());
        self.abort()
    }

    /// 中止当前操作（对照 `HAL_ESMC_Abort` 的同步部分）：关片选并等回 IDLE。
    ///
    /// # Errors
    ///
    /// 等待回到 IDLE 超时返回 [`Error::Timeout`]。
    pub fn abort(&mut self) -> Result<(), Error> {
        let regs = regs();
        disable_slave(regs);
        wait_idle(regs)
    }

    /// 软件复位 ESMC（对照 `HAL_ESMC_DeInit` 的核心动作：`CR.SOFTRST`）。
    ///
    /// RM 30.4.1 注：软复位会清 FIFO 指针、其余寄存器回到默认值（含 `XIPEN = 1`、
    /// `SPIEN = 1`），故复位后需重新初始化。HAL 只置位该位、随后在 `MspDeInit`
    /// 里关掉外设时钟；本驱动保持时钟开启，因此**置位后再清零（脉冲）**，
    /// 避免外设停在复位态。
    pub fn software_reset(&mut self) {
        let regs = regs();
        disable_slave(regs);
        regs.cr().modify(|_, w| w.softrst().set_bit());
        regs.cr().modify(|_, w| w.softrst().clear_bit());
        self.data_mode = DataMode::None;
        self.variable_length = false;
        self.addr24 = 0;
    }

    /// 发送阶段：分批灌满 TFIFO 并按 FIFO 状态推进（对照 `HAL_ESMC_Transmit` 的循环）。
    fn write_phase(&self, regs: &pac::esmc::RegisterBlock, data: &[u8]) -> Result<(), Error> {
        // 首批之前先等状态机进入 DATAWAIT，确认已在等数据
        wait_data_wait(regs)?;

        let mut pos = 0;
        while pos < data.len() {
            if data.len() - pos > FIFO_SIZE {
                write_fifo(regs, &data[pos..pos + FIFO_SIZE]);
                pos += FIFO_SIZE;
                wait_data_wait(regs)?;
            } else {
                write_fifo(regs, &data[pos..]);
                pos = data.len();
                if self.variable_length {
                    wait_data_wait(regs)?;
                } else {
                    wait_idle_done(regs)?;
                }
            }
        }
        // TX FIFO 上溢/未对齐属于硬件告警位，收尾时顺带检查
        check_tx_flags(regs)
    }

    /// 接收阶段：分批取空 RXFIFO（对照 `HAL_ESMC_Receive` 的循环）。
    fn read_phase(
        &self,
        regs: &pac::esmc::RegisterBlock,
        data: &mut [u8],
    ) -> Result<(), Error> {
        let mut pos = 0;
        // 先按整 FIFO 批量取，尾部（1~16 字节）再单独处理
        while data.len() - pos > FIFO_SIZE {
            wait_data_wait(regs)?;
            read_fifo(regs, &mut data[pos..pos + FIFO_SIZE]);
            pos += FIFO_SIZE;
        }
        while pos < data.len() {
            if self.variable_length {
                wait_data_wait(regs)?;
            } else {
                wait_idle_done(regs)?;
            }
            read_fifo(regs, &mut data[pos..]);
            pos = data.len();
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 内部实现
// ---------------------------------------------------------------------------

/// 取 ESMC 的寄存器块。
///
/// ESMC 是单实例外设（基址 `0xA000_1000`，RM 30.4 开头），故不需要
/// `Instance` trait 那样的实例分派；基址与类型均来自 PAC。
#[inline]
fn regs() -> &'static pac::esmc::RegisterBlock {
    unsafe { &*pac::Esmc::PTR }
}

/// 由 `HCLK` 与目标 SCLK 换算 `BAUD`（RM 30.4.4）。
///
/// `BAUD = ceil(HCLK / sclk)`：保证实际 SCLK **不超过**请求值；
/// 硬件把 `BAUD < 2` 视作二分频，故下限钳到 2；字段 8 位，上限 255。
fn compute_prescaler(hclk: u32, sclk: u32) -> u8 {
    if sclk == 0 {
        return 2;
    }
    hclk.div_ceil(sclk).clamp(2, u8::MAX as u32) as u8
}

/// 写 ESMC 初始化寄存器（不含 GPIO；引脚 AF 配置在构造器内完成）。
///
/// 顺序：等 IDLE → `CR2`（CPOL/CPHA）→ `BAUD` → `TCR` → 清片选位 →
/// `CR`（SPIEN / 2xQUAD）→ 清 `IER1`/`IER2` → 清 FIFO。
///
/// Note: HAL v0.7.0 的 `HAL_ESMC_Init` 写 `BAUD` 在前、`CR2` 在后；
/// RM 30.4.2 注明模式 3 应"先使能 CPOL/CPHA，再配置波特率"，故此处按 RM 排序。
fn init_regs(config: &Config) -> Result<(), Error> {
    let regs = regs();

    // HAL_ESMC_Init: ESMC_WaitFlagStateUntilTimeout(ESMC_FLAG_IDLE, SET)
    wait_idle(regs)?;

    // CR2.CPOL/CPHA（RM 30.4.2）：只支持 0x0 与 0x3
    let mode3 = config.clock_mode.is_mode3();
    regs.cr2().modify(|_, w| {
        w.cpol().bit(mode3);
        w.cpha().bit(mode3);
        w
    });

    // 波特率分频（RM 30.4.4）
    let prescaler = compute_prescaler(
        crate::rcc::clocks().hclk.0,
        config.sclk.0,
    );
    regs.baud()
        .modify(|_, w| unsafe { w.baud().bits(prescaler) });

    // nCS 高/保持时间（RM 30.4.3）
    regs.tcr().modify(|_, w| unsafe {
        w.high().bits(config.cs_high_time);
        w.hold().bits(config.cs_hold_time);
        w
    });

    // 清空全部片选输出（HAL: CLEAR_BIT(ADDR32/XSSOCR/XSSOCR_WE, ..._SSXO)）
    regs.addr32().modify(|_, w| unsafe { w.ssxo().bits(0) });
    regs.xssocr().modify(|_, w| unsafe { w.ssxo().bits(0) });
    regs.xssocr_we().modify(|_, w| unsafe { w.ssxo().bits(0) });

    // CR：使能 SPI、按需使能双 Flash；其余有定义的位清零
    // （忽略 XIPEN 的复位默认 1——间接模式必须关掉 XIP）
    // Note: HAL 整写 CR = SPIEN | 2xQUAD，会连带清掉 RM 记为"复位值 1"的保留位 bit1；
    // 这里用 modify 只改有定义的位。
    regs.cr().modify(|_, w| {
        w.softrst().clear_bit();
        w._2qspi().bit(config.dual_flash);
        w.dmaen().clear_bit();
        w.gie().clear_bit();
        w.xipen().clear_bit();
        w.spien().set_bit();
        w
    });

    // 同步驱动不使用中断：关掉全部中断源（HAL 靠复位默认值，此处显式保证）
    regs.ier1().write(|w| unsafe { w.bits(0) });
    regs.ier2().write(|w| unsafe { w.bits(0) });

    // 清 FIFO 指针（HAL: __HAL_ESMC_CLEAR_FIFO）
    clear_fifo(regs);
    Ok(())
}

/// 装配"只有指令"的功能模式（HAL 的 `ESMC_FUNCTIONAL_MODE_INDIRECT_INSTRUCT`）。
///
/// 只发 `SFCR0` 里的 1 字节指令，地址/备用字节/Dummy/数据阶段全部跳过
/// （`CR3.NO_ADDR | CR3.NO_DATA`）。写 `SFCR` 即启动传输。
///
/// Note: 该分支下 HAL 不支持 2 字节指令（2 字节需要 `ADDR32.MREG`，而本模式不写
/// `ADDR32`），故本驱动对 2 字节指令直接返回 [`Error::InvalidConfig`]。
fn configure_indirect_instruct(regs: &pac::esmc::RegisterBlock, cmd: &Command) {
    disable_slave(regs);
    clear_fifo(regs);

    let mut cr3 = regs.cr3().read().bits();
    let mut socr = regs.socr().read().bits();
    let mut dcr = regs.dcr().read().bits();
    let mut addr32 = regs.addr32().read().bits();

    // 传输格式 + DDR
    socr = (socr & !(SOCR_SPIMODE | SOCR_DDRCMD | SOCR_DDRADDR | SOCR_DDRDATA))
        | cmd.transfer_format.bits()
        | cmd.ddr.socr_bits();
    // 指令多线
    socr = set_bit_u8(socr, SOCR_MULTICMD, cmd.instruction_mode == InstructionMode::MultiLines);
    // 有指令
    dcr &= !DCR_NO_CMD;
    // 跳过地址与数据阶段
    cr3 |= CR3_NO_ADDR | CR3_NO_DATA;
    // 片选输出
    addr32 = (addr32 & !ADDR32_SSXO) | (u32::from(cmd.chip_select.bits()) << ADDR32_SSXO_POS);
    // DQS
    cr3 = set_bit_u8(cr3, CR3_XSPI_RWDS, cmd.dqs);

    unsafe {
        regs.cr3().write(|w| w.bits(cr3));
        regs.socr().write(|w| w.bits(socr));
        regs.dcr().write(|w| w.bits(dcr));
        regs.addr32().write(|w| w.bits(addr32));
        // 写 SFCR 即发送指令（HAL: WRITE_REG(SFCR, Instruction & 0xFF)）
        regs.sfcr().write(|w| w.bits((cmd.instruction & 0xFF) as u8));
    }
}

/// 装配间接读/写功能模式（HAL 的 `ESMC_FUNCTIONAL_MODE_INDIRECT_READ_WRITE`），
/// 返回待写入 `ADDR24` 的「指令 + 24 位地址」值。
///
/// 写入顺序严格照抄 HAL：`ADDR32 → SOCR → DCR → STRR → CR3`。
/// 有无数据阶段决定 `CR3.NO_DATA`，并且**不写 `ADDR24`**——数据阶段要在
/// `write`/`read` 里写它来启动。
fn configure_indirect(regs: &pac::esmc::RegisterBlock, cmd: &Command) -> u32 {
    // xSPI Dummy 双倍延迟（RM 30.4.2；注意位名是"关闭"，故取值取反）
    regs.cr2().modify(|_, w| {
        w.xspi_dual_lat_off().bit(!cmd.xspi_dual_latency);
        w
    });

    disable_slave(regs);
    clear_fifo(regs);

    // 影子变量（HAL 的 TmpCR3/TmpSOCR/TmpDCR/TmpADDR24/TmpADDR32）
    let mut cr3 = regs.cr3().read().bits();
    let mut socr = regs.socr().read().bits();
    let mut dcr = regs.dcr().read().bits();
    let mut addr24 = regs.addr24().read().bits();
    let mut addr32 = regs.addr32().read().bits();

    // 传输格式 + DDR
    socr = (socr & !(SOCR_SPIMODE | SOCR_DDRCMD | SOCR_DDRADDR | SOCR_DDRDATA))
        | cmd.transfer_format.bits()
        | cmd.ddr.socr_bits();

    // —— 指令阶段 ——
    if cmd.instruction_mode == InstructionMode::None {
        dcr |= DCR_NO_CMD;
    } else {
        dcr &= !DCR_NO_CMD;
        socr = set_bit_u8(
            socr,
            SOCR_MULTICMD,
            cmd.instruction_mode == InstructionMode::MultiLines,
        );
        addr24 = (addr24 & !ADDR24_INS1) | u32::from((cmd.instruction & 0xFF) as u8);
    }

    // —— 指令长度：2 字节时第 2 字节落在 ADDR32.MREG ——
    if cmd.instruction_length == InstructionLength::TwoBytes {
        cr3 |= CR3_CMD_EXT;
        addr32 = (addr32 & !ADDR32_MREG)
            | (u32::from((cmd.instruction >> 8) as u8) << ADDR32_MREG_POS);
    } else {
        cr3 &= !CR3_CMD_EXT;
    }

    // —— 地址阶段 ——
    if cmd.address_mode == AddressMode::None {
        cr3 |= CR3_NO_ADDR;
    } else {
        cr3 &= !CR3_NO_ADDR;
        // HAL：地址走多线的前提是地址模式或指令模式为多线
        let multi = cmd.address_mode == AddressMode::MultiLines
            || cmd.instruction_mode == InstructionMode::MultiLines;
        socr = set_bit_u8(socr, SOCR_MULTIADDR, multi);
        // 地址宽度
        cr3 = (cr3 & !(CR3_ADDR8BIT | CR3_ADDR16BIT | CR3_ADDR32BIT)) | cmd.address_size.cr3_bits();
        // 32 位地址的最高字节放在 ADDR32.ADDR3
        if cmd.address_size == AddressSize::Bits32 {
            addr32 = (addr32 & !ADDR32_ADDR3) | u32::from((cmd.address >> 24) as u8);
        }
        // 低 24 位放在 ADDR24.ADDR[23:0]（寄存器 bit31:8）
        addr24 = (addr24 & !ADDR24_ADDR) | ((cmd.address & 0x00FF_FFFF) << 8);
    }

    // —— 备用字节阶段（与指令第 2 字节共用 ADDR32.MREG）——
    if cmd.alternate_byte_mode == AlternateByteMode::Enabled {
        addr32 = (addr32 & !ADDR32_MREG)
            | (u32::from(cmd.alternate_byte) << ADDR32_MREG_POS);
        socr |= SOCR_SENM;
    } else {
        socr &= !SOCR_SENM;
    }

    // —— Dummy 周期 ——
    dcr = (dcr & !DCR_DUMMY) | (cmd.dummy_cycles & DCR_DUMMY);

    // —— 片选输出 ——
    addr32 = (addr32 & !ADDR32_SSXO) | (u32::from(cmd.chip_select.bits()) << ADDR32_SSXO_POS);
    // —— DQS ——
    cr3 = set_bit_u8(cr3, CR3_XSPI_RWDS, cmd.dqs);

    // —— 数据阶段 ——
    let mut bcr = 0u32;
    match cmd.data_mode {
        DataMode::Write => dcr &= !DCR_REC,
        DataMode::Read => dcr |= DCR_REC,
        DataMode::None => dcr &= !DCR_REC,
    }
    if cmd.data_mode != DataMode::None {
        // nb_data == u32::MAX 表示不定长（HAL: >= HAL_ESMC_TCR_MAX_LENGTH）
        if cmd.nb_data == u32::MAX {
            bcr = 0;
        } else {
            bcr = cmd.nb_data;
        }
    }

    unsafe {
        regs.addr32().write(|w| w.bits(addr32));
        regs.socr().write(|w| w.bits(socr));
        regs.dcr().write(|w| w.bits(dcr));
        // 采样延伸（STRR.SMP≥0；HAL 是整写，故此处也整写）
        regs.strr()
            .write(|w| w.bits(cmd.sample_shifting & 0x3F));
        regs.bcr().write(|w| w.bits(bcr));
    }

    if cmd.data_mode == DataMode::None {
        cr3 |= CR3_NO_DATA;
    } else {
        cr3 &= !CR3_NO_DATA;
    }
    unsafe { regs.cr3().write(|w| w.bits(cr3)) };

    addr24
}

/// 装配 XIP 读寄存器组（HAL 的 `ESMC_FUNCTIONAL_MODE_MEMORY_MAPPED`）。
///
/// Note: HAL 在地址段写的是 `CLEAR_BIT(TmpXDCR, ESMC_XCR3_NO_ADDR)`——掩码属于
/// `XCR3` 却作用在 `XDCR` 上（笔误：`XCR3.NO_ADDR` 是 bit3，`XDCR` 的 bit3 是
/// 保留位）。本驱动改为清除 `XCR3.NO_ADDR`，`XDCR` 只写 `DUMMY`/`NO_CMD`。
fn configure_xip_read(regs: &pac::esmc::RegisterBlock, cmd: &Command) {
    disable_slave(regs);
    clear_fifo(regs);

    // 影子变量（HAL 的 TmpXSOCR/TmpXDCR/TmpXCR3/TmpXSSOCR 等）
    // XSFCR/XMODE 每次都会被整体重算，故不需要先读回旧值
    let mut xsocr = regs.xsocr().read().bits();
    let mut xdcr = regs.xdcr().read().bits();
    let mut xcr3 = regs.xcr3().read().bits();
    let mut xssocr = regs.xssocr().read().bits();

    // 传输格式 + DDR
    xsocr = (xsocr & !(XSOCR_SPIMODE | XSOCR_DDRCMD | XSOCR_DDRADDR | XSOCR_DDRDATA))
        | cmd.transfer_format.bits()
        | cmd.ddr.socr_bits();

    // 指令阶段
    if cmd.instruction_mode == InstructionMode::None {
        xdcr |= XDCR_NO_CMD;
        xsocr &= !XSOCR_MULTICMD;
    } else {
        xdcr &= !XDCR_NO_CMD;
        xsocr = set_bit_u8(
            xsocr,
            XSOCR_MULTICMD,
            cmd.instruction_mode == InstructionMode::MultiLines,
        );
    }

    // 指令长度：2 字节时第 2 字节落在 XMODE
    let mut xmode = if cmd.instruction_length == InstructionLength::TwoBytes {
        xcr3 |= XCR3_CMD_EXT;
        (cmd.instruction >> 8) as u8
    } else {
        xcr3 &= !XCR3_CMD_EXT;
        0
    };

    // 地址阶段（XIP 恒发地址，AHB 总线提供地址值）
    xcr3 &= !XCR3_NO_ADDR;
    let multi = cmd.address_mode == AddressMode::MultiLines
        || cmd.instruction_mode == InstructionMode::MultiLines;
    xsocr = set_bit_u8(xsocr, XSOCR_MULTIADDR, multi);
    xcr3 = (xcr3 & !(XCR3_ADDR8BIT | XCR3_ADDR16BIT | XCR3_ADDR32BIT)) | cmd.address_size.cr3_bits();

    // 备用字节阶段（与指令第 2 字节共用 XMODE）
    if cmd.alternate_byte_mode == AlternateByteMode::Enabled {
        xmode = cmd.alternate_byte;
        xsocr |= XSOCR_SEND_M;
    } else {
        xsocr &= !XSOCR_SEND_M;
    }

    // Dummy 周期
    xdcr = (xdcr & !XDCR_DUMMY) | (cmd.dummy_cycles & XDCR_DUMMY);
    // 片选（XSSOCR 的 SS[5:0] 在 bit5:0）
    xssocr = (xssocr & !0x3F) | cmd.chip_select.bits();
    // DQS
    xcr3 = set_bit_u8(xcr3, XCR3_XSPI_RWDS, cmd.dqs);

    unsafe {
        regs.xcr3().write(|w| w.bits(xcr3));
        regs.xssocr().write(|w| w.bits(xssocr));
        regs.xsocr().write(|w| w.bits(xsocr));
        regs.xdcr().write(|w| w.bits(xdcr));
        regs.xmode().write(|w| w.bits(xmode & XMODE_M));
        // XSTRR 的字段名/PAC 名为 XSMP，与 STRR 同为 6 位
        regs.xstrr().write(|w| w.bits(cmd.sample_shifting & 0x3F));
    }
    // 无指令时 HAL 不写 XSFCR（否则会把上一条指令写回去）
    if cmd.instruction_mode != InstructionMode::None {
        unsafe { regs.xsfcr().write(|w| w.bits((cmd.instruction & 0xFF) as u8)) };
    }
}

/// 装配 XIP 写寄存器组（HAL 的 `ESMC_FUNCTIONAL_MODE_MEMORY_WRITE`）。
///
/// 与读路径的差异（照抄 HAL）：
/// - 指令码写在 `XSFCR_WE`（在该分支内直接写寄存器，不经过影子变量）；
/// - `XSOCR_WE.MULTIADDR` 只依据 [`AddressMode`]（**不**连带指令模式）；
/// - 地址段同样修正了 HAL 把 `XCR3_WE_NO_ADDR` 作用在 `XDCR_WE` 上的笔误。
fn configure_xip_write(regs: &pac::esmc::RegisterBlock, cmd: &Command) {
    disable_slave(regs);
    clear_fifo(regs);

    // 影子变量（HAL 的 TmpXSOCR_WE/TmpXDCR_WE/TmpXCR3_WE/TmpXSSOCR_WE 等）
    // XMODE_WE 每次都会被整体重算，故不需要先读回旧值
    let mut xssocr_we = regs.xssocr_we().read().bits();
    let mut xsocr_we = regs.xsocr_we().read().bits();
    let mut xdcr_we = regs.xdcr_we().read().bits();
    let mut xcr3_we = regs.xcr3_we().read().bits();

    // 传输格式 + DDR
    xsocr_we = (xsocr_we & !(XSOCR_SPIMODE | XSOCR_DDRCMD | XSOCR_DDRADDR | XSOCR_DDRDATA))
        | cmd.transfer_format.bits()
        | cmd.ddr.socr_bits();

    // 指令阶段
    if cmd.instruction_mode == InstructionMode::None {
        xdcr_we |= XDCR_NO_CMD;
    } else {
        unsafe {
            regs.xsfcr_we()
                .write(|w| w.bits((cmd.instruction & 0xFF) as u8))
        };
        xdcr_we &= !XDCR_NO_CMD;
        xsocr_we = set_bit_u8(
            xsocr_we,
            XSOCR_MULTICMD,
            cmd.instruction_mode == InstructionMode::MultiLines,
        );
    }

    // 指令长度：2 字节时第 2 字节落在 XMODE_WE
    let mut xmode_we = if cmd.instruction_length == InstructionLength::TwoBytes {
        xcr3_we |= XCR3_CMD_EXT;
        (cmd.instruction >> 8) as u8
    } else {
        xcr3_we &= !XCR3_CMD_EXT;
        0
    };

    // 地址阶段
    xcr3_we &= !XCR3_NO_ADDR;
    xsocr_we = set_bit_u8(
        xsocr_we,
        XSOCR_MULTIADDR,
        cmd.address_mode == AddressMode::MultiLines,
    );
    xcr3_we =
        (xcr3_we & !(XCR3_ADDR8BIT | XCR3_ADDR16BIT | XCR3_ADDR32BIT)) | cmd.address_size.cr3_bits();

    // 备用字节阶段（与指令第 2 字节共用 XMODE_WE）
    if cmd.alternate_byte_mode == AlternateByteMode::Enabled {
        xmode_we = cmd.alternate_byte;
        xsocr_we |= XSOCR_SEND_M;
    } else {
        xsocr_we &= !XSOCR_SEND_M;
    }

    // Dummy 周期
    xdcr_we = (xdcr_we & !XDCR_DUMMY) | (cmd.dummy_cycles & XDCR_DUMMY);
    // 片选（XSSOCR_WE 的 SS[5:0] 在 bit5:0）
    xssocr_we = (xssocr_we & !0x3F) | cmd.chip_select.bits();
    // DQS
    xcr3_we = set_bit_u8(xcr3_we, XCR3_XSPI_RWDS, cmd.dqs);

    unsafe {
        regs.xsocr_we().write(|w| w.bits(xsocr_we));
        regs.xdcr_we().write(|w| w.bits(xdcr_we));
        regs.xmode_we().write(|w| w.bits(xmode_we & XMODE_M));
        regs.xcr3_we().write(|w| w.bits(xcr3_we));
        regs.xssocr_we().write(|w| w.bits(xssocr_we));
    }
}

/// 与工作模式无关的命令字段校验（对照 HAL `HAL_ESMC_Command` 里的 `assert_param`）。
///
/// 注意**不校验** `instruction`（指令阶段关闭时该值被硬件忽略，官方例程也常在
/// 关闭阶段时留着上一次的值）与 `alternate_byte`（同理，官方 `ESMC_ReadWrite_QSPI_Polling`
/// 就在 `AlternateByteMode = DISABLE` 的同时把 `AlternateByte` 填成 `0xff`），
/// 以免把合法写法判成错误。
fn validate_common(cmd: &Command) -> Result<(), Error> {
    // HAL: IS_ESMC_DUMMY_CYCLES / IS_ESMC_SAMPLESHIFTING
    if cmd.dummy_cycles > 63 || cmd.sample_shifting > 63 {
        return Err(Error::InvalidConfig);
    }
    // 无指令阶段时不可能有 2 字节指令：第 2 字节落在 MREG、第 1 字节却无处可取
    // （RM 30.3.2：2 字节指令的第 1 字节固定来自 ADDR24.INS1）
    if cmd.instruction_mode == InstructionMode::None
        && cmd.instruction_length == InstructionLength::TwoBytes
    {
        return Err(Error::InvalidConfig);
    }
    Ok(())
}

/// 间接模式（[`Esmc::command`]）的校验：数据阶段必须给出确定的长度。
///
/// HAL 对 `data_mode != None && nb_data == 0` 会静默降级成"无数据"、对
/// `data_mode == None && nb_data != 0` 会静默把 `BCR` 写 0——两种都容易掩盖误配，
/// 这里一律判错。
fn validate_indirect(cmd: &Command) -> Result<(), Error> {
    validate_common(cmd)?;
    if cmd.data_mode != DataMode::None && cmd.nb_data == 0 {
        return Err(Error::InvalidConfig);
    }
    if cmd.data_mode == DataMode::None && cmd.nb_data != 0 {
        return Err(Error::InvalidConfig);
    }
    Ok(())
}

/// XIP（内存映射）模式的校验。
///
/// XIP 下**数据量由 AHB 访问决定**、数据方向由调用的函数决定
/// （[`Esmc::memory_mapped`] = 读、[`Esmc::memory_write`] = 写），
/// 因此 `nb_data` 必须为 0（填了说明把 XIP 当成间接模式在用了），
/// `data_mode` 允许留默认的 [`DataMode::None`]（官方 XIP 例程的写法）或与
/// `direction` 一致，反向填写判错。
fn validate_xip(cmd: &Command, direction: DataMode) -> Result<(), Error> {
    validate_common(cmd)?;
    if cmd.nb_data != 0 {
        return Err(Error::InvalidConfig);
    }
    if cmd.data_mode != DataMode::None && cmd.data_mode != direction {
        return Err(Error::InvalidConfig);
    }
    Ok(())
}

/// 只置位/清位一个字节位（省去显式 if/else，供影子变量使用）。
#[inline]
fn set_bit_u8(value: u8, mask: u8, on: bool) -> u8 {
    if on { value | mask } else { value & !mask }
}

/// 关闭片选输出（HAL 的 `__HAL_ESMC_DISABLE_SLAVE`：置 `ADDR32.SS_CLR`）。
///
/// 置位后当前操作立即终止，ESMC 回到 IDLE，nCS 拉高；该位由硬件自动清零。
#[inline]
fn disable_slave(regs: &pac::esmc::RegisterBlock) {
    regs.addr32().modify(|_, w| w.ss_clr().set_bit());
}

/// 驱动所选片选输出（HAL 的 `__HAL_ESMC_ENABLE_SLAVE`：置 `ADDR32.SS_SET`）。
///
/// 供 XIP 模式常驻拉低 nCS 使用；nCS 激活后该位自动清零。
#[inline]
fn enable_slave(regs: &pac::esmc::RegisterBlock) {
    regs.addr32().modify(|_, w| w.ss_set().set_bit());
}

/// 清空 TX/RX FIFO 指针（HAL 的 `__HAL_ESMC_CLEAR_FIFO`）。
#[inline]
fn clear_fifo(regs: &pac::esmc::RegisterBlock) {
    regs.cr2()
        .modify(|_, w| w.txfifo_clr().set_bit().rxfifo_clr().set_bit());
}

/// 等 `SR.IDLE_STATE` 置位（HAL 的 `ESMC_FLAG_IDLE`，RM 30.4.9）。
fn wait_idle(regs: &pac::esmc::RegisterBlock) -> Result<(), Error> {
    for _ in 0..TIMEOUT_LOOPS {
        if regs.sr().read().idle_state().bit_is_set() {
            return Ok(());
        }
    }
    Err(Error::Timeout)
}

/// 等 `IFR1.IDLEDONEIF` 置位（HAL 的 `ESMC_FLAG_IDLEDONEIF`，RM 30.4.12）。
fn wait_idle_done(regs: &pac::esmc::RegisterBlock) -> Result<(), Error> {
    for _ in 0..TIMEOUT_LOOPS {
        if regs.ifr1().read().idledoneif().bit_is_set() {
            return Ok(());
        }
    }
    Err(Error::Timeout)
}

/// 等 `IFR1.DATAWAITIF` 置位（HAL 的 `ESMC_FLAG_DATAWAITIF`，RM 30.4.12）。
///
/// 写方向表示"FIFO 空了、等主机补数据"，读方向表示"FIFO 满了、等主机取走"。
fn wait_data_wait(regs: &pac::esmc::RegisterBlock) -> Result<(), Error> {
    for _ in 0..TIMEOUT_LOOPS {
        if regs.ifr1().read().datawaitif().bit_is_set() {
            return Ok(());
        }
    }
    Err(Error::Timeout)
}

/// 检查 TX 侧两个告警位（RM 30.4.14；HAL 未检查，属本驱动新增的健壮性检查）。
///
/// 两位都是 RC_W1（**写 1 清零**），而 SVD/PAC 只给了 `TXWORDOVIF` 的写入器，
/// 且该位在 SVD 里是普通读写（未标 `modifiedWriteValues`），故两者都按 RM 的
/// 位序整写单个位来清除——写 0 对其它标志没有影响。
fn check_tx_flags(regs: &pac::esmc::RegisterBlock) -> Result<(), Error> {
    let ifr2 = regs.ifr2().read();
    if ifr2.txfifoovif().bit_is_set() {
        unsafe { regs.ifr2().write(|w| w.bits(IFR2_TXFIFOOVIF)) };
        return Err(Error::TxFifoOverrun);
    }
    if ifr2.txwordovif().bit_is_set() {
        unsafe { regs.ifr2().write(|w| w.bits(IFR2_TXWORDOVIF)) };
        return Err(Error::TxNotWordAligned);
    }
    Ok(())
}

/// 向 `DATA` 灌入 `buf`（HAL 的 `ESMC_WriteBuff`）。
///
/// 整字部分按 **32 位**写入，尾部不足 4 字节按**字节**写入——与 HAL 一致：
/// `DATA` 是 TXFIFO 的顶端，字节写入同样会推入 FIFO。
///
/// **顺序不能反过来**（先写尾字节、再写整字会出错）。依据 RM 30.4.14
/// `IFR2.TXWORDOVIF`："此位表示要写入 TX FIFO 的数据没有按照字对齐……当软件试图将
/// 超过 4 个字节的数据写入到 4 个字节的内存空间时，该位就会被置位（例如先写入单个字节，
/// 然后写入 4 字节的数据）；注：**加载到 TX FIFO 的数据应该始终与 word 对齐；
/// 不支持字和字节交叉写入的方法**"。
///
/// 即 TXFIFO 内部按 4 字节槽位记账：先写满所有整字（每个字正好占满一个槽），
/// 最后剩下的 1~3 字节落在**一个全新的空槽**里，属于合法用法；反过来先塞 1 个字节
/// 再用整字去填同一个槽，就会触发 `TXWORDOVIF`。
///
/// 实测（PY32E407 + GD25LQ128，2 MHz）1 / 2 / 13 字节的数据阶段均正常，
/// 无 `TXWORDOVIF` / `TXFIFOOVIF`。
fn write_fifo(regs: &pac::esmc::RegisterBlock, buf: &[u8]) {
    let full_words = buf.len() / 4;
    for i in 0..full_words {
        let b = &buf[i * 4..i * 4 + 4];
        let word = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
        unsafe { regs.data().write(|w| w.bits(word)) };
    }
    for &byte in &buf[full_words * 4..] {
        // 尾部按字节宽度访问 DATA（HAL: (uint8_t *)&DATA）
        unsafe { core::ptr::write_volatile(regs.data().as_ptr() as *mut u8, byte) };
    }
}

/// 从 `DATA` 取出 `buf.len()` 字节（HAL 的 `ESMC_ReadBuff`）。
///
/// 注意尾部不足 4 字节时 HAL 仍是**一次 32 位读**再取低字节，而不是逐字节读
/// （`DATA` 是 RXFIFO 顶端，逐字节读会多弹出一个字），本驱动照做。
fn read_fifo(regs: &pac::esmc::RegisterBlock, buf: &mut [u8]) {
    let full_words = buf.len() / 4;
    for i in 0..full_words {
        let word = regs.data().read().bits();
        buf[i * 4..i * 4 + 4].copy_from_slice(&word.to_le_bytes());
    }
    let tail = buf.len() % 4;
    if tail != 0 {
        let bytes = regs.data().read().bits().to_le_bytes();
        buf[full_words * 4..].copy_from_slice(&bytes[..tail]);
    }
}
