//! # demo-py32 — embassy-py32 最小示例（对齐 embassy-stm32 用法）
//!
//! 用法与 embassy-stm32 完全同构：
//! 1. `embassy_py32::init(enable80mhz())` 返回 `Peripherals`；
//! 2. `Output::new(p.PB1, Level::Low, Speed::Low)` 创建 GPIO 驱动；
//! 3. `Uart::new_blocking(p.UART2, p.PC12, p.PD2, cfg)` 创建同步 UART 驱动（PC12=TX / PD2=RX）；
//! 4. `Lpuart::new_blocking(p.LPUART1, p.PC1, p.PC0, cfg)` 创建同步 LPUART1 驱动（PC1=TX / PC0=RX）；
//! 5. `Usart::new_blocking(p.USART1, p.PA9, p.PA10, cfg)` 创建同步 USART1 驱动（PA9=TX / PA10=RX）；
//! 6. `Spi::new_blocking(p.SPI1, p.PA5, p.PA7, p.PA6, cfg)` 创建同步 SPI1 主机驱动（PA5=SCK / PA7=MOSI / PA6=MISO）；
//! 7. `Timer::new_blocking(p.TIM6, cfg)` 创建 TIM 时基驱动（更新事件节拍）；
//! 8. `Pwm::new(p.TIM1, Some(PwmPin::new_ch1(p.PA8)), .., cfg)` 创建 TIM1_CH1 PWM 输出；
//! 9. `rcc::clocks()` 查询配置后的实际频率，供 SysTick 换算。
//! 10. `esmc::Esmc::new_blocking_with_cs(...)` 创建 ESMC（QSPI）驱动
//!     （PB10=CLK、PE11=CS0、PE12~PE15=IO0~IO3，16MiB NOR Flash），
//!     然后读 ID/状态、置 QE、在草稿扇区做一次**擦写自检**，再单线/4 线各走一次 XIP 读。
//! 11. `ExtiInput::new_blocking(p.PD10, p.EXTI10, Pull::None, TriggerEdge::Rising)`
//!     创建带 EXTI 的输入（PD10 上升沿，与本板 PY32Studio 工程一致），
//!     并用 `is_pending()` / `clear_pending()` 轮询边沿挂起位。
//!
//! 时钟链路：HSI16 → PLL(HSI/2 × 10) = 80 MHz；PPRE /2 保证 PCLK ≤ 85 MHz；
//! Flash 等待周期由驱动按 DS 5.3.4 自动设置（75~100 MHz → 4 WS）。
//! 因此 PCLK1 = 40 MHz，UART 驱动按该频率算 BRR/BRRF 分频。
//! 若想不动时钟树：`init(Config::default())` 即可（复位默认 HSI16）。

#![no_std]
#![no_main]

use cortex_m::peripheral::{syst::SystClkSource, SYST};
use cortex_m_rt::{entry, interrupt};
// `#[interrupt]` 宏展开后会插入 `interrupt::TIM7;` 以校验向量名，该路径需解析到
// PAC 的 `Interrupt` 枚举（与上面的宏同名但属不同类型命名空间，可共存）。
use embassy_py32::pac::Interrupt as interrupt;
// 声明 defmt-rtt 为全局 logger。必须写成 `use ... as _;`：
// 不这样写该 crate 不会被链接进来、日志发不出去；同时它也让 `.defmt` 段的版本标记
// 进入符号表，probe-rs 才能识别并解码 RTT 日志。
use defmt_rtt as _;

use embassy_py32::esmc;
use embassy_py32::exti::{ExtiInput, TriggerEdge};
use embassy_py32::gpio::{Level, Output, Pull, Speed};
use embassy_py32::lpuart;
use embassy_py32::spi;
use embassy_py32::timer::pwm::{Channel, Pwm, PwmPin};
use embassy_py32::timer::{self, RoundTo, Timer};
use embassy_py32::uart;
use embassy_py32::usart;
use embassy_py32::{rcc, Config as BoardConfig, Hertz};

// ---- LCD：参考设计的 ST7735 驱动（原样拷贝）+ embedded-hal 兼容适配层 ----
// `lcd` 里的 `display.rs` 还带整帧缓冲 BufferDisplay 等未全部用到的绘图 API，故允许 dead_code。
#[allow(dead_code)]
mod lcd;
mod lcd_hal;
// 电机速度闭环（embassy-py32 移植版）：尚未接入主循环，先允许 dead_code
#[allow(dead_code)]
mod motor;
// PID 控制器（motor.rs 依赖）
#[allow(dead_code)]
mod pid;

use lcd::color::RGB565;
use lcd::display::Display;
use lcd::font::FONT_7X10;
use lcd::st7735::{Config as TftConfig, Orientation, St7735Driver};
use lcd::LcdDriver;
use lcd_hal::{Delay, OutputPinAdapter, SpiBusAdapter};

/// CORDIC 硬件协处理器最小示例：算 √2 与 cos(π/4)（尚未接入主循环）。
///
/// 数据均为 Q1.31 定点（±1.0 ↔ ±2³¹）；SCALE 因子对应的补缩放在打印前完成。
/// 用法：在 `main` 里调 `cordic_demo(p.CORDIC)`。
#[allow(dead_code)]
fn cordic_demo(cordic: embassy_py32::Peri<'static, embassy_py32::peripherals::CORDIC>) {
    use embassy_py32::cordic::{Config, Cordic, Function, NbWrite, Precision, Scale};

    // √2：SCALE n=2 → 输入 2.0×2⁻² = 0.5，硬件回 √2×2⁻² = 0.3536（RM 表 43-13 n=2 行）
    let mut cfg = Config::default();
    cfg.function = Function::SquareRoot;
    cfg.precision = Precision::Cycles3; // sqrt 残差 2⁻¹⁹（表 43-14）
    cfg.scale = Scale::N2;
    let mut cordic = Cordic::new_blocking(cordic, cfg).expect("cordic init");

    let input = [0x4000_0000_i32]; // 0.5（Q1.31）
    let mut output = [0_i32; 1];
    cordic.calculate(&input, &mut output).expect("sqrt");
    let sqrt2 = output[0] as f32 / 2_147_483_648.0 * 4.0; // 结果 ×2ⁿ 补缩放
    defmt::info!("CORDIC sqrt(2) = {}", sqrt2);

    // cos(π/4)：角度以"圈"为单位，π/4 = 0.125 圈；不写 ARG2，模长按默认 1.0
    let mut cfg = Config::default();
    cfg.function = Function::Cosine;
    cfg.precision = Precision::Cycles6; // cos/sin 残差 2⁻¹⁹（表 43-14）
    cfg.nb_write = NbWrite::One;
    cordic.configure(cfg);

    let input = [0x1000_0000_i32]; // 0.125 圈（Q1.31）
    let mut output = [0_i32; 1];
    cordic.calculate(&input, &mut output).expect("cos");
    let v = output[0] as f32 / 2_147_483_648.0;
    defmt::info!("CORDIC cos(pi/4) = {}", v);
}

/// EXTI 外部中断最小示例：**PD10 上升沿**。
///
/// 引脚取自本板 PY32Studio 生成工程：`参考设计/demo/Core/SRC/gpio.c` 里
/// `GPIO_InitStruct.Mode = GPIO_MODE_IT_RISING`（`Pull = GPIO_NOPULL`）配的就是 PD10，
/// 对应向量 `EXTI15_10`（IRQ 40）、服务函数 `HAL_GPIO_EXTI_IRQHandler(GPIO_PIN_10)`，
/// 见 `参考设计/demo/Core/SRC/py32e407_it.c`。故这里照抄同一引脚、同一触发沿。
///
/// 使用流程（对齐 embassy-stm32 的阻塞用法）：
/// 1. [`ExtiInput::new_blocking`]：GPIO 输入 + EXTI 链路（SYSCFG 时钟 → EXTICR 端口选择
///    → `RTSR` 上升沿 → 清 `PR`）+ `IMR1` 使能；
/// 2. `is_pending()` 查询挂起位 → `clear_pending()` 清掉（`PR` 是 RC_W1，写 1 清零）；
/// 3. 自检：直接写 PAC 的 `SWIER` 造一次挂起，验证第 2 步的读/清通路真的可分辨
///    （不接外部信号时 `PR` 恒为 0，"读到 false"本身什么都证明不了；HAL 面按
///    embassy-stm32 的裁剪不含软件触发 API，故这一步在 demo 侧点寄存器）。
///
/// 想改用中断（本同步库只提供挂起位操作，不注册向量、不动 NVIC）：
///
/// ```ignore
/// #[interrupt]
/// fn EXTI15_10() {                    // 向量名取自 RM 13.1.3 / PAC 的 Interrupt 枚举
///     let exti = unsafe { &*embassy_py32::pac::Exti::PTR };
///     if exti.pr().read().bits() & (1 << 10) != 0 {
///         exti.pr().write(|w| unsafe { w.bits(1 << 10) });   // 必须先清挂起，否则漏下一次
///         // ... 处理
///     }
/// }
/// ```
/// 并在初始化后 `unsafe { cortex_m::peripheral::NVIC::unmask(embassy_py32::pac::Interrupt::EXTI15_10) }`。
/// 这与本文件里 TIM7 更新中断的写法完全同构。
fn exti_demo(
    pin: embassy_py32::Peri<'static, embassy_py32::peripherals::PD10>,
    ch: embassy_py32::Peri<'static, embassy_py32::peripherals::EXTI10>,
) {
    // 与生成工程一致：NOPULL（PD10 已由板上电路决定静态电平）、上升沿触发。
    // `p.EXTI10` 是这条线的所有权句柄——`PD10`/`PE10` 共用 EXTI10，同时只能取一个。
    let mut key = ExtiInput::new_blocking(pin, ch, Pull::None, TriggerEdge::Rising);
    defmt::info!("EXTI ready: PD10 rising edge");

    // ---- 自检：造一次挂起 → 查 → 清 → 再查 ----
    // 正常空跑时 PR 恒为 0，"读到 false"没有判别力，所以先经 PAC 的 SWIER 置起一次挂起
    // （RM 13.3.5：写 1 即置起对应 PR 位；`IMR1` 已使能但 NVIC 未 unmask，不会真的进中断）。
    key.clear_pending(); // 起点先清一次，避免残留值干扰判断
    let exti = unsafe { &*embassy_py32::pac::Exti::PTR };
    exti.swier().write(|w| unsafe { w.bits(1 << 10) });
    let after_sw = key.is_pending();
    key.clear_pending(); // 写 1 清零
    let after_clear = key.is_pending();
    defmt::info!(
        "EXTI self-test: pending after SWIER = {} -> after clear = {}",
        after_sw,
        after_clear
    );
    if !after_sw || after_clear {
        defmt::error!("EXTI self-test FAILED (PR read/clear path)");
    }

    // 读一次电平：PD10 未接信号时电平不定，这里只证明 IDR 可读。
    defmt::info!("EXTI PD10 level high = {}", key.is_high());

    // 实际用法是把 `key` 留在主循环里轮询：
    //     if key.is_pending() { key.clear_pending(); /* 处理一次上升沿 */ }
    // 本 demo 只做初始化 + 自检，故 `key` 在此析构（EXTI 侧配置保留在寄存器里，
    // 驱动不会隐式改硬件）。
}

/// 板载 QSPI NOR Flash 的容量：**16 MiB**（128 Mbit）。
///
/// 实测 JEDEC ID = `C8 60 18` → 厂商 `0xC8` = GigaDevice，容量码 `0x18` = 128 Mbit ✓；
/// 中段 `0x60` 是 **LQ 系列（1.8V 档）**（`0x40` 才是 3.3V 的 Q 系列）。
/// 16 MiB = 2²⁴，恰好被 **24 位地址**（[`esmc::AddressSize::Bits24`]，ESMC 默认档）
/// 完全覆盖，不需要 32 位地址模式。
const FLASH_SIZE: usize = 16 * 1024 * 1024;

/// ESMC **XIP（内存映射）窗口的基址**。
///
/// ⚠️ 依据 RM 表 3-1「存储器地址」——`0x9000_0000` 这一行标的就是 **ESMC Bank1**，
/// 而厂商 `Example/ESMC/ESMC_MemoryMapping_QSPI` 里的
/// `ApplicationAddress = 0x90000000` 正是这个值，两边对得上。
///
/// **别踩的坑**：RM 表 3-5「物理重映射地址」里那个 `0x0400_0000–0x07FF_FFFF`
/// 是**启动重映射视图**（把 ESMC 当启动源、别名到 0 地址时才用），
/// 在正常从内部 Flash 启动的配置下 `0x0008_0000–0x07FF_FFFF` 属于**保留区间**——
/// RM 表 3-1 明确写着"CPU 读写该空间时产生 Response error，进而进入 HardFault"。
/// 第一版 demo 就写了 `0x0400_0000`，于是上板读到 **Precise data access error@0x04000000**。
const ESMC_XIP_BASE: usize = 0x9000_0000;

/// 读写自检用的"草稿扇区"：**最后一颗 4 KB 扇区**（`0x00FF_F000`）。
///
/// 全片实测为空，用末尾扇区不会覆盖任何原有内容；但这一步是**真的擦写外部 Flash**
/// （非易失），不想要就把 [`esmc_demo`] 里的自检段落整体删掉。
const SCRATCH_ADDR: u32 = (FLASH_SIZE as u32) - 4096;

/// 常见 SPI-NOR 命令码 —— 按 **GD25LQ128 手册 Table 2（Standard/Dual/Quad SPI）**
/// 逐条核对过，不是照抄别的型号。
const CMD_WRITE_ENABLE: u16 = 0x06; // 写使能（置 WEL）
const CMD_READ_STATUS1: u16 = 0x05; // 状态寄存器 1（S7-S0）：bit0 = BUSY、bit1 = WEL
const CMD_WRITE_STATUS1: u16 = 0x01; // 写状态寄存器：**只有这一条，且必须发满 2 字节**（见下）
const CMD_READ_STATUS2: u16 = 0x35; // 状态寄存器 2（S15-S8）：bit1 = QE
const CMD_READ_ID: u16 = 0x9F; // JEDEC ID（3 字节）
const CMD_NORMAL_READ: u16 = 0x03; // 普通读（单线指令/地址、无 Dummy）
const CMD_PAGE_PROGRAM: u16 = 0x02; // 页编程（单线指令 + 24 位地址 + N 字节数据）
const CMD_SECTOR_ERASE_4K: u16 = 0x20; // 4KB 扇区擦除（只有指令 + 24 位地址，无数据阶段）
const CMD_FAST_READ_4O: u16 = 0x6B; // 1-1-4 Quad Output 快速读（Dummy 恒为 8）

// ── 为什么写状态寄存器只能用 `0x01` 且必须 2 字节 ────────────────────────────
//
// 手册 Table 2 里写状态寄存器只有一行：`Write Status Register  01H  S7-S0  S15-S8`
// —— 指令表里**没有 31H**。第 7.4 节另给了两条硬约束：
//
//   1. "CS# must be driven high after the eighth or sixteen bit of the data byte has been
//      latched in. Otherwise, the Write Status Register (WRSR) command is not executed."
//      → 数据阶段只能是 1 或 2 字节，多一个字节就不执行。
//   2. "**If CS# is driven high after eighth bit of the data byte, the QE and CMP bits will
//      be cleared to 0 in SPI mode**" → 只发 8 bit（只写 SR1）反而**把 QE 清掉**。
//      所以置 QE 必须发满 16 bit（SR1 + SR2）。
//
// **本项目踩过这个坑**：厂商例程 `ESMC_ReadWrite_QSPI_Polling` 的 QE 使能段用的是
// `P25Q64_WRITE_STATUS_REG_1_CMD = 0x31`（单字节）—— 那是 **P25Q64** 的指令集。
// 换到本板这颗 GigaDevice 后 `0x31` 完全无效：实测 **WEL 不消失、SR2 不变**
// （WEL 只在写类命令真正执行后才由硬件清零，所以这是"Flash 根本没执行这条命令"的
// 铁证），退回 `0x01` 发 2 字节立刻生效（SR2 = 0x02）。**换料必须对着该型号手册的
// 指令表核一遍，不能照抄别的型号的例程。**

/// 状态寄存器 1 里的块保护位（`BP4..BP0`，手册 Table1 的 S6..S2）。
///
/// 只要其中任何一位非 0，被保护的地址区**擦除/编程会静默失效**（读还正常），
/// 是"写了没反应"的经典陷阱，所以自检前先看一眼。
const SR1_BP_MASK: u8 = 0x7C;
#[allow(dead_code)]
const CMD_FAST_READ_4IO: u16 = 0xEB; // 1-4-4 Quad I/O 快速读（Dummy/模式字节按型号）

/// `0x6B` 的 Dummy 周期数：**恒为 8**（W25Q/GD25Q/GD25LQ/P25Q 一致）。
///
/// 厂商 XIP 例程用的就是 `P25Q64_1I4O_FAST_READ_CMD`(0x6B) + 8 Dummy + **单线地址** +
/// **无备用字节**，是三家里最不容易搞错的一组，故 demo 的 XIP 首选它。
const DUMMY_READ_4O: u8 = 8;

/// `0xEB` 的 Dummy 周期数：**GD25Q128 / GD25LQ128 都是 4**。
///
/// 依据 GD25 手册 §7.11 "Quad I/O Fast Read (EBH)"："the 3-byte address (A23-0)
/// and a 'Continuous Read Mode' byte and **4-dummy clock**"。别的型号不一样
/// （W25Q 系列常见 6，MX25L 的 QPI 模式可由 C0h 配成 4/6/8），换料要查手册。
/// 另注意 `0xEB` 比 `0x6B` 多一个模式字节（M[7:0]），写 `0x00` 才不进入连续读模式。
#[allow(dead_code)]
const DUMMY_FAST_READ_4IO: u8 = 4;

/// 状态寄存器 1 的 WEL（Write Enable Latch）位。
///
/// 写状态寄存器/编程/擦除前必须先置位，命令**完成后硬件自动清零**——所以读它的
/// 变化可以判断"命令到底有没有发到 Flash 上"。
const SR1_WEL: u8 = 0x02;

/// 状态寄存器 2 的 QE（Quad Enable）位。
///
/// 4 线读（`0x6B`/`0xEB`/`0x32`）**必须先置 QE**，否则 IO2/IO3 仍是 `#WP`/`#HOLD`
/// 输入、数据线上只有 2 根在动，读回的就是垃圾。官方例程
/// `ESMC_ReadWrite_QSPI_Polling` 在 4IO 读写前也做这一步
/// （读 0x35 → `|= 0x02` → 写使能 → 写 0x31）。
const SR2_QE: u8 = 0x02;

/// 读一个 Flash 状态寄存器（`cmd` 取 0x05 或 0x35）。
fn read_status_reg(qspi: &mut esmc::Esmc<'_>, cmd: u16) -> Result<u8, esmc::Error> {
    let read = esmc::Command {
        instruction_mode: esmc::InstructionMode::SingleLine,
        instruction: cmd,
        data_mode: esmc::DataMode::Read,
        nb_data: 1,
        chip_select: esmc::ChipSelect::Cs0,
        ..Default::default()
    };
    let mut b = [0u8; 1];
    qspi.command_read(&read, &mut b)?;
    Ok(b[0])
}

/// 发一条只有指令、无数据阶段的命令（写使能等）。
fn send_cmd(qspi: &mut esmc::Esmc<'_>, cmd: u16) -> Result<(), esmc::Error> {
    let c = esmc::Command {
        instruction_mode: esmc::InstructionMode::SingleLine,
        instruction: cmd,
        chip_select: esmc::ChipSelect::Cs0,
        ..Default::default()
    };
    qspi.command(&c)
}

/// 写 `bytes` 到 Flash 状态寄存器。
///
/// 本片只能用 `0x01`（[`CMD_WRITE_STATUS1`]）并一次发 2 字节（SR1 + SR2）；
/// 指令表里没有 `0x31`，只发 1 字节还会把 QE 清掉 —— 详见文件顶部的命令码说明。
fn write_status_reg(
    qspi: &mut esmc::Esmc<'_>,
    cmd: u16,
    bytes: &[u8],
) -> Result<(), esmc::Error> {
    let w = esmc::Command {
        instruction_mode: esmc::InstructionMode::SingleLine,
        instruction: cmd,
        data_mode: esmc::DataMode::Write,
        nb_data: bytes.len() as u32,
        chip_select: esmc::ChipSelect::Cs0,
        ..Default::default()
    };
    qspi.command_write(&w, bytes)
}

/// 轮询状态寄存器 1 的 `BUSY`（bit0）直到清零（写/擦除完成）。
///
/// 每次读状态都要走一整条 ESMC 命令（2 MHz 下约 10 µs），故这个循环本身就带延时；
/// 上限按"最慢的 4KB 扇区擦除"给足（手册 tSE 典型 30 ms / 最大 400 ms，
/// 折算约 4 万次轮询，这里留 10 倍余量）。
fn wait_not_busy(qspi: &mut esmc::Esmc<'_>) -> Result<(), esmc::Error> {
    for _ in 0..500_000 {
        if read_status_reg(qspi, CMD_READ_STATUS1)? & 0x01 == 0 {
            return Ok(());
        }
    }
    Err(esmc::Error::Timeout)
}

/// 从 Flash 读 `buf.len()` 字节到 `buf`（`0x03` 普通读：单线指令 + 24 位单线地址）。
fn flash_read(qspi: &mut esmc::Esmc<'_>, addr: u32, buf: &mut [u8]) -> Result<(), esmc::Error> {
    let cmd = esmc::Command {
        instruction_mode: esmc::InstructionMode::SingleLine,
        instruction: CMD_NORMAL_READ,
        address_mode: esmc::AddressMode::SingleLine,
        address_size: esmc::AddressSize::Bits24,
        address: addr,
        data_mode: esmc::DataMode::Read,
        nb_data: buf.len() as u32,
        chip_select: esmc::ChipSelect::Cs0,
        ..Default::default()
    };
    qspi.command_read(&cmd, buf)
}

/// 向 Flash 写 `data`（`0x02` 页编程：单线指令 + 24 位单线地址 + 数据）。
///
/// 注意 NOR 的编程只能把 1 变成 0，所以调用前该地址必须已擦除过；
/// 且**不能跨 256 字节页边界**（跨页会回到页首覆盖）。
fn flash_program(qspi: &mut esmc::Esmc<'_>, addr: u32, data: &[u8]) -> Result<(), esmc::Error> {
    send_cmd(qspi, CMD_WRITE_ENABLE)?;
    let cmd = esmc::Command {
        instruction_mode: esmc::InstructionMode::SingleLine,
        instruction: CMD_PAGE_PROGRAM,
        address_mode: esmc::AddressMode::SingleLine,
        address_size: esmc::AddressSize::Bits24,
        address: addr,
        data_mode: esmc::DataMode::Write,
        nb_data: data.len() as u32,
        chip_select: esmc::ChipSelect::Cs0,
        ..Default::default()
    };
    qspi.command_write(&cmd, data)?;
    wait_not_busy(qspi)
}

/// 擦除一个 4KB 扇区（`0x20`：只有指令 + 24 位地址，**没有数据阶段**）。
fn flash_erase_sector(qspi: &mut esmc::Esmc<'_>, addr: u32) -> Result<(), esmc::Error> {
    send_cmd(qspi, CMD_WRITE_ENABLE)?;
    let cmd = esmc::Command {
        instruction_mode: esmc::InstructionMode::SingleLine,
        instruction: CMD_SECTOR_ERASE_4K,
        address_mode: esmc::AddressMode::SingleLine,
        address_size: esmc::AddressSize::Bits24,
        address: addr,
        chip_select: esmc::ChipSelect::Cs0,
        ..Default::default()
    };
    qspi.command(&cmd)?;
    wait_not_busy(qspi)
}

/// 用 `cmd` 配置一次 XIP、回读 `XIPEN`、peek `ESMC_XIP_BASE + offset` 处的 u32，然后退出 XIP。
///
/// `0x9000_0000` 是 ESMC 的 XIP 窗口（RM 表 3-1「ESMC Bank1」）。返回值就是 peek 到的
/// 内容（`None` = 配置失败或 `XIPEN` 没置上），便于调用方把单线/4 线两组结果放一起比。
fn xip_peek(
    qspi: &mut esmc::Esmc<'_>,
    cmd: &esmc::Command,
    label: &str,
    offset: usize,
) -> Option<u32> {
    let addr = ESMC_XIP_BASE + offset;
    let mut out = None;
    match qspi.memory_mapped(cmd) {
        Ok(()) => {
            if qspi.xip_enabled() {
                // XIPEN 回读确认后再解引用：XIPEN=0 时访问映射区会把总线锁成 busy
                // （RM 30.3.4.2）。第一版没查，读 0x0400_0000 直接 BusFault。
                let w = unsafe { core::ptr::read_volatile(addr as *const u32) };
                defmt::info!("ESMC XIP({=str}) @{=usize:#x} = {=u32:#x}", label, addr, w);
                out = Some(w);
            } else {
                defmt::error!("ESMC XIP({=str}): XIPEN 没置上", label);
            }
        }
        Err(e) => defmt::error!(
            "ESMC XIP({=str}) 配置失败: {}",
            label,
            defmt::Display2Format(&e)
        ),
    }
    // 退出 XIP 回到间接模式，便于下一轮重新配置。
    if let Err(e) = qspi.exit_xip() {
        defmt::error!("ESMC exit XIP 失败: {}", defmt::Display2Format(&e));
    }
    out
}

/// ESMC（QSPI）示例：读 ID/状态 → 置 QE → 草稿扇区擦写自检 → 单线/4 线 XIP 读对比。
///
/// 引脚为**本板实际接线**（与 PY32Studio 生成工程的 `HAL_ESMC_MspInit` 逐字一致）：
/// `PB10 = ESMC_CLK`、`PE12~PE15 = ESMC_BK1_IO0~IO3`、`PE11 = ESMC_CS0_NCS`，
/// 全部 `AF10`。这组引脚与 demo 其它外设（PB1/PB2、PB3~PB5、PB12~PB15、
/// UART2 的 PC12/PD2、PE2/PE3 …）都不冲突。
///
/// 本片实测：Mode0 + 2 MHz + 24 位地址下，间接读写（4KB 擦除 / 页编程）、单线 XIP 与
/// **4 线 XIP** 均已跑通。
fn esmc_demo(
    peri: embassy_py32::Peri<'static, embassy_py32::peripherals::ESMC>,
    clk: embassy_py32::Peri<'static, impl esmc::ClkPin>,
    io0: embassy_py32::Peri<'static, impl esmc::Io0Pin>,
    io1: embassy_py32::Peri<'static, impl esmc::Io1Pin>,
    io2: embassy_py32::Peri<'static, impl esmc::Io2Pin>,
    io3: embassy_py32::Peri<'static, impl esmc::Io3Pin>,
    cs: esmc::ChipSelects<'static>,
) {
    let mut config = esmc::Config::default();
    // HCLK = 168MHz 时请求 2MHz → BAUD = 84（DS 表 5-58 允许 85MHz）。
    // 想跑快些：0x03 普通读上限约 50MHz，Quad 读可到 85MHz（本芯片实测 ID 读得通即可）；
    // 实际值用 `qspi.frequency()` 读回（PY32Studio 的默认档是 HCLK/2）。
    config.sclk = Hertz(2_000_000);

    let mut qspi = match esmc::Esmc::new_blocking_with_cs(peri, clk, io0, io1, io2, io3, cs, config)
    {
        Ok(v) => v,
        Err(e) => {
            defmt::error!("ESMC init failed: {}", defmt::Display2Format(&e));
            return;
        }
    };
    defmt::info!(
        "ESMC ready: SCLK = {} Hz, flash = {} MiB (24-bit addr)",
        qspi.frequency().0,
        FLASH_SIZE / 1024 / 1024
    );

    // 1) JEDEC ID（0x9F）：单线指令、无地址、读 3 字节。
    //    本板实测 [0xC8, 0x60, 0x18] → GigaDevice / LQ 系列 / 128Mbit。
    let read_id = esmc::Command {
        instruction_mode: esmc::InstructionMode::SingleLine,
        instruction: CMD_READ_ID,
        data_mode: esmc::DataMode::Read,
        nb_data: 3,
        chip_select: esmc::ChipSelect::Cs0,
        ..Default::default()
    };
    let mut id = [0u8; 3];
    match qspi.command_read(&read_id, &mut id) {
        Ok(()) => defmt::info!("ESMC JEDEC ID = {=[u8]:#x}", id),
        Err(e) => defmt::error!("ESMC read id failed: {}", defmt::Display2Format(&e)),
    }

    // 2) 状态寄存器 1（0x05）：bit0 = BUSY、bit1 = WEL。
    match read_status_reg(&mut qspi, CMD_READ_STATUS1) {
        Ok(s) => defmt::info!("ESMC SR1 = {=u8:#x}", s),
        Err(e) => defmt::error!("ESMC read SR1 failed: {}", defmt::Display2Format(&e)),
    }

    // 3) 打开 QE 位（4 线读的前提），顺带把**写入通路**验证清楚。
    //
    //    诊断链（关键在 WEL 的变化，它是判断"命令到底有没有到 Flash"的唯一抓手）：
    //      a. WREN(0x06) 后立刻读 SR1 —— WEL 置位说明"只有指令、无数据"的 SFCR 通路 OK；
    //      b. 写 SR2 之后 WEL 被硬件自动清零 —— 说明 Flash 确实执行完了一条写状态寄存器命令
    //         （即带数据阶段的间接写通路 OK）；
    //      c. 最后回读 SR2 看 QE 有没有真的落下去。
    //    第 1 版就是只看了 c，QE 没置上时无法区分是"写也没发出去"还是"Flash 拒绝接受"。
    //
    //    注意这一步是**写 Flash 的非易失状态位**（与官方例程一致）；只想读 ID 的话整段可删。
    match read_status_reg(&mut qspi, CMD_READ_STATUS2) {
        Ok(sr2) if sr2 & SR2_QE != 0 => defmt::info!("ESMC SR2 = {=u8:#x} (QE 已置位)", sr2),
        Ok(sr2) => {
            defmt::info!("ESMC SR2 = {=u8:#x}, 尝试置 QE…", sr2);
            // 回写 SR1 时要把 WEL 位掩掉：它是硬件锁存位，不该由软件写。
            let sr1v = read_status_reg(&mut qspi, CMD_READ_STATUS1).unwrap_or(0) & !SR1_WEL;
            // (a) 写使能 + 查 WEL
            let mut wel_ok = false;
            match send_cmd(&mut qspi, CMD_WRITE_ENABLE)
                .and_then(|()| read_status_reg(&mut qspi, CMD_READ_STATUS1))
            {
                Ok(sr1) => {
                    wel_ok = sr1 & SR1_WEL != 0;
                    defmt::info!("ESMC WREN 后 SR1 = {=u8:#x} → WEL={}", sr1, wel_ok);
                }
                Err(e) => defmt::error!("ESMC WREN 失败: {}", defmt::Display2Format(&e)),
            }
            if !wel_ok {
                defmt::warn!("ESMC: WEL 没置上，后续写状态寄存器会被 Flash 忽略");
            }

            // (b)(c) 用 `0x01` 一次发满 2 字节（SR1 + SR2）。
            // **别用 `0x31`**：本片指令表里没有这条，实测发出去毫无反应（WEL 不消失、
            // SR2 不变）；**也别只发 1 字节**：手册 7.4 明确说那样会把 QE 清掉。
            // 详见文件顶部命令码旁的说明。
            let r = send_cmd(&mut qspi, CMD_WRITE_ENABLE)
                .and_then(|()| {
                    write_status_reg(&mut qspi, CMD_WRITE_STATUS1, &[sr1v, sr2 | SR2_QE])
                })
                .and_then(|()| wait_not_busy(&mut qspi))
                .and_then(|()| read_status_reg(&mut qspi, CMD_READ_STATUS1))
                .and_then(|sr1| {
                    defmt::info!(
                        "ESMC 0x01(2B) 后 SR1 = {=u8:#x} → WEL 已清={}",
                        sr1,
                        sr1 & SR1_WEL == 0
                    );
                    read_status_reg(&mut qspi, CMD_READ_STATUS2)
                });
            match r {
                Ok(v) => defmt::info!(
                    "ESMC 0x01(2B) 后 SR2 = {=u8:#x} (QE 已置={})",
                    v,
                    v & SR2_QE != 0
                ),
                Err(e) => defmt::error!(
                    "ESMC 写状态寄存器失败: {}",
                    defmt::Display2Format(&e)
                ),
            }
        }
        Err(e) => defmt::error!("ESMC read SR2 failed: {}", defmt::Display2Format(&e)),
    }
    let quad_ready = matches!(read_status_reg(&mut qspi, CMD_READ_STATUS2), Ok(v) if v & SR2_QE != 0);
    defmt::info!("ESMC QE 是否就绪 = {}", quad_ready);

    // 4) 首 16 字节（0x03）：本片实测为空（全 0xFF）。
    let mut head = [0u8; 16];
    match flash_read(&mut qspi, 0, &mut head) {
        Ok(()) => defmt::info!("ESMC flash[0..16] = {=[u8]:#x}", head),
        Err(e) => defmt::error!("ESMC read flash failed: {}", defmt::Display2Format(&e)),
    }

    // 5) **擦写自检** —— 验证"写通路 + 4 线读"的唯一可靠手段。
    //
    //    空的 Flash 本来就全读成 0xFF，所以**即使 QE 没置、Dummy 数错、数据线全断，
    //    "空片读出 0xFF" 也照样成立**——前几轮 demo 看到的 0xFFFFFFFF 其实什么都没证明。
    //    必须先在草稿扇区写进非 0xFF 的图案，再比对读回值，才能把"链路正确"和
    //    "恰好都是空的"区分开。
    //
    //    两种长度都测：1 字节（最短的 TX 数据阶段）与 13 字节（"整字 + 1 字节尾"的
    //    混合情形——RM 30.4.14 要求 TX FIFO 数据字对齐、且明确"不支持字和字节交叉
    //    写入"，正好压这条边界）。
    //
    //    ⚠️ 会真的擦写外部 Flash（非易失）。不想要就删掉本段与 `SCRATCH_ADDR`。
    const PAT1: u8 = 0xA5;
    const PAT2_OFF: u32 = 0x10;
    let mut pat2 = [0u8; 13];
    for (i, b) in pat2.iter_mut().enumerate() {
        *b = i as u8;
    }

    // 先看 SR1 的块保护位：被保护区块的擦/写会静默失效（读仍正常）。
    if let Ok(sr1) = read_status_reg(&mut qspi, CMD_READ_STATUS1) {
        if sr1 & SR1_BP_MASK != 0 {
            defmt::warn!(
                "ESMC SR1 = {=u8:#x} 有块保护位（BP）置位，草稿扇区可能擦/写不动",
                sr1
            );
        }
    }

    // (a) 草稿扇区得是空的：NOR 编程只能把 1 变 0，旧数据会"与"进来。
    let mut pre = [0u8; 16];
    match flash_read(&mut qspi, SCRATCH_ADDR, &mut pre) {
        Ok(()) if pre.iter().all(|b| *b == 0xFF) => defmt::info!(
            "ESMC 草稿扇区 {=u32:#x} 已擦除（全 0xFF）",
            SCRATCH_ADDR
        ),
        Ok(()) => defmt::warn!(
            "ESMC 草稿扇区 {=u32:#x} 非空，编程结果会按位与到旧值上: {=[u8]:#x}",
            SCRATCH_ADDR,
            pre
        ),
        Err(e) => defmt::error!("ESMC 读草稿扇区失败: {}", defmt::Display2Format(&e)),
    }

    // (b) 擦 4KB 扇区 → (c) 写 1 字节与 13 字节两段 → (d) 读回比对。
    match flash_erase_sector(&mut qspi, SCRATCH_ADDR)
        .and_then(|()| flash_program(&mut qspi, SCRATCH_ADDR, &[PAT1]))
        .and_then(|()| flash_program(&mut qspi, SCRATCH_ADDR + PAT2_OFF, &pat2))
    {
        Ok(()) => {
            let mut rb1 = [0u8; 1];
            let mut rb2 = [0u8; 13];
            let read1 = flash_read(&mut qspi, SCRATCH_ADDR, &mut rb1);
            let read2 = flash_read(&mut qspi, SCRATCH_ADDR + PAT2_OFF, &mut rb2);
            match read1 {
                Ok(()) if rb1 == [PAT1] => defmt::info!("ESMC 擦写自检 1B: {=[u8]:#x} → OK", rb1),
                Ok(()) => defmt::error!(
                    "ESMC 擦写自检 1B: 写 {=[u8]:#x} 读回 {=[u8]:#x} → MISMATCH",
                    [PAT1],
                    rb1
                ),
                Err(e) => defmt::error!("ESMC 擦写自检 1B 读回失败: {}", defmt::Display2Format(&e)),
            }
            match read2 {
                Ok(()) if rb2 == pat2 => defmt::info!("ESMC 擦写自检 13B: {=[u8]:#x} → OK", rb2),
                Ok(()) => defmt::error!(
                    "ESMC 擦写自检 13B: 写 {=[u8]:#x} 读回 {=[u8]:#x} → MISMATCH",
                    pat2,
                    rb2
                ),
                Err(e) => defmt::error!(
                    "ESMC 擦写自检 13B 读回失败: {}",
                    defmt::Display2Format(&e)
                ),
            }
        }
        Err(e) => defmt::error!("ESMC 擦写自检失败: {}", defmt::Display2Format(&e)),
    }

    // (e) XIP 读：单线（0x03）与 4 线（0x6B）读**同一个已编程地址**再比对期望值。
    //     外部 Flash 映射到 **0x9000_0000**（RM 表 3-1「ESMC Bank1」），XIP 命令不带
    //     nb_data（数据量由 AHB 访问决定）。这一组对比是"4 线到底通不通"的判据。
    let xip_single = esmc::Command {
        instruction_mode: esmc::InstructionMode::SingleLine,
        instruction: CMD_NORMAL_READ,
        address_mode: esmc::AddressMode::SingleLine,
        address_size: esmc::AddressSize::Bits24,
        chip_select: esmc::ChipSelect::Cs0,
        ..Default::default()
    };
    // 厂商 XIP 例程用的就是这套：0x6B(1-1-4) + 单线地址 + 无备用字节 + 8 Dummy。
    let xip_quad = esmc::Command {
        transfer_format: esmc::TransferFormat::Quad,
        instruction_mode: esmc::InstructionMode::SingleLine,
        instruction: CMD_FAST_READ_4O,
        address_mode: esmc::AddressMode::SingleLine,
        address_size: esmc::AddressSize::Bits24,
        dummy_cycles: DUMMY_READ_4O,
        chip_select: esmc::ChipSelect::Cs0,
        ..Default::default()
    };
    // pat2 以 00 01 02 03 开头，按小端读出的首字就是 0x0302_0100。
    let off2 = (SCRATCH_ADDR + PAT2_OFF) as usize;
    let expect2: u32 = 0x0302_0100;
    let s = xip_peek(&mut qspi, &xip_single, "0x03 单线", off2);
    let q = if quad_ready {
        xip_peek(&mut qspi, &xip_quad, "0x6B 4线", off2)
    } else {
        defmt::warn!("ESMC: QE 未就绪，跳过 0x6B 4线 XIP");
        None
    };
    let verdict = |v: Option<u32>| match v {
        Some(x) if x == expect2 => "OK",
        Some(_) => "MISMATCH",
        None => "FAIL",
    };
    defmt::info!(
        "ESMC XIP 自检 @{=usize:#x}: 期望 {=u32:#x} → 单线 {} / 4线 {}",
        off2,
        expect2,
        verdict(s),
        verdict(q)
    );
}

/// LCD 每秒轮换的填充色，用来直观验证屏幕确实在刷新（而不只是上电花屏）。
const LCD_CYCLE: [RGB565; 5] = [
    RGB565::Red,
    RGB565::Green,
    RGB565::Blue,
    RGB565::White,
    RGB565::Black,
];

/// SysTick 毫秒延时：reload 设为 1ms 对应的计数值，逐毫秒等 `COUNTFLAG`。
///
///（保留作对照；主循环节拍已改由 TIM6 更新事件驱动。）
#[allow(dead_code)]
///
/// * `syst` — 内核级 SysTick 外设（从 `CorePeripherals` 取出）
/// * `hclk_hz` — 当前 HCLK 频率（来自 `rcc::clocks()`）
/// * `ms`   — 延时毫秒数
fn delay_ms(syst: &mut SYST, hclk_hz: u32, ms: u32) {
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(hclk_hz / 1_000 - 1);
    syst.clear_current();
    syst.enable_counter();
    for _ in 0..ms {
        while !syst.has_wrapped() {}
    }
    syst.disable_counter();
}

/// 板级时钟预设（两套并列，切换只改 `main` 里传的枚举值）。
// 两个预设都保留供选择，当前只用了 Default2，故允许未构造变体。
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClockPreset {
    /// **第一类默认**：芯片复位默认时钟树 —— HSI16 = 16MHz，AHB / APB1 / APB2 全 1 分频。
    /// （即 `BoardConfig::default()`：不动任何寄存器就能用。）
    Default1,
    /// **第二类默认**：HSE 8MHz → PLL `÷1 ×21` = 168MHz；AHB/1、APB1=/2、APB2=/2
    /// → HCLK 168MHz、PCLK1 = PCLK2 = 84MHz（APB 上限 85MHz，DS 时钟树）。
    Default2,
}

/// 按预设构造板级配置。
///
/// 第二类默认的关键约束（实测得出，详见 `rcc::Error` 与上文注释）：
/// - **PLL 参考频率必须 ≥ 2MHz**（RM §6.7.4：PLL 输入 2~32MHz）。
///   故 `PREDIV` 用 `/1`（参考 8MHz）而**不是** `/8`（那会得到 1MHz，PLL 永不锁定）。
/// - HSE 驱动档用 `High`（16~32MHz）：实测该档下 PLL 可锁。
fn board_config(preset: ClockPreset) -> BoardConfig {
    match preset {
        ClockPreset::Default1 => BoardConfig::default(),

        ClockPreset::Default2 => {
            let mut config = BoardConfig::default();
            use embassy_py32::rcc::*;
            config.rcc.hse = Some(Hse {
                freq: Hertz(8_000_000),
                mode: HseMode::Oscillator,
                drive: HseDrive::High, // 16~32 MHz 档
            });
            config.rcc.pll_src = PllSource::HSE;
            // PLL = 8 MHz / 1 × 21 / 1 = 168 MHz（参考 8MHz ∈ 2~32MHz ✓）
            config.rcc.pll = Some(Pll {
                prediv: PllPreDiv::DIV1,
                mul: 21,
                postdiv: PllPostDiv::DIV1,
            });
            config.rcc.sys = Sysclk::PLL;
            config.rcc.ahb_pre = AHBPrescaler::DIV1;
            config.rcc.apb1_pre = APBPrescaler::DIV2;
            config.rcc.apb2_pre = APBPrescaler::DIV2;
            config
        }
    }
}

/// 主循环：init 配置时钟树后，用 UART2 回显收到的字节，同时每 100ms 翻转一次 PB1 / PB2。
#[entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();

    // 配置时钟树：这里选**第二类默认**（HSE 8MHz → PLL 168MHz、PCLK=84MHz）。
    // 想回到"第一类默认"（HSI16 复位默认）只需把这个参数改成 `ClockPreset::Default1`。
    let boot_clocks = match embassy_py32::rcc::init(board_config(ClockPreset::Default2).rcc) {
        Ok(c) => c,
        Err(e) => {
            let r = unsafe { &*embassy_py32::pac::Rcc::PTR };
            defmt::error!(
                "rcc init failed: {} | CFGR={:08x} CFGR1={:08x} CFGR2={:08x} CFGR3={:08x} CR={:08x} PWR_CR1={:08x}",
                defmt::Debug2Format(&e),
                r.cfgr().read().bits(),
                r.cfgr1().read().bits(),
                r.cfgr2().read().bits(),
                r.cfgr3().read().bits(),
                r.cr().read().bits(),
                unsafe { (&*embassy_py32::pac::Pwr::PTR).cr1().read().bits() },
            );
            loop {
                core::hint::spin_loop();
            }
        }
    };
    defmt::info!(
        "clocks: sys={} hclk={} pclk1={} pclk2={}",
        boot_clocks.sys.0,
        boot_clocks.hclk.0,
        boot_clocks.apb1.0,
        boot_clocks.apb2.0
    );
    let p = embassy_py32::Peripherals::take();

    cordic_demo(p.CORDIC);

    // EXTI：PD10 上升沿输入（引脚与触发沿来自 PY32Studio 生成工程的 GPIO_MODE_IT_RISING）
    exti_demo(p.PD10, p.EXTI10);

    // ESMC（QSPI，16MiB NOR Flash）：PB10 = CLK、PE12~PE15 = BK1_IO0~IO3、PE11 = CS0。
    // 这组接线与 PY32Studio 生成工程的 `HAL_ESMC_MspInit` 一致，也与本 demo 其它外设不冲突。
    esmc_demo(
        p.ESMC,
        p.PB10,
        p.PE12,
        p.PE13,
        p.PE14,
        p.PE15,
        esmc::ChipSelects::none().cs0(p.PE11),
    );

    // GPIO 驱动：PB1 / PB2 两路输出，初电平都是低
    // （驱动内部自动使能 GPIOB 时钟并等待生效）
    // 注意：PC3 不在此处占用——它留给下面的 TIM1_CH4 做 PWM 输出。
    let mut pb1 = Output::new(p.PB1, Level::Low, Speed::Low);
    let mut pb2 = Output::new(p.PB2, Level::Low, Speed::Low);

    // 以实际 HCLK 驱动 SysTick
    let clocks = rcc::clocks();

    // 一条 RTT 日志：probe-rs run 会直接打印在终端里，用来确认程序真的跑起来了
    // （UART 还要额外接 USB-TTL，RTT 走 SWD 就能看）
    defmt::info!("boot ok, hclk={} Hz", clocks.hclk.0);

    // UART2：PC12 = TX、PD2 = RX，AF5，115200-8N1。
    // 引脚与官方 STK 例程 Example_LL/UART/UART_HyperTerminal_Polling 完全一致，
    // 接上 USB-TTL 即可用串口终端看到输出。
    let mut uart_config = uart::Config::default();
    uart_config.baudrate = 115_200;
    let mut uart = uart::Uart::new_blocking(p.UART2, p.PC12, p.PD2, uart_config).unwrap();

    uart.write(b"\r\nembassy-py32 UART2 ready @115200-8N1\r\n")
        .unwrap();

    // LPUART1：PC1 = TX、PC0 = RX，AF8，115200-8N1。
    // 引脚与官方 STK 例程 Example_LL/LPUART/LPUART_HyperTerminal_Polling 完全一致。
    // 这里只演示初始化与发送；回显循环放在 UART2 上（同步驱动的 read 会阻塞到超时，
    // 两条通道同时轮询会互相拖慢，实际项目按需选一条）。
    let mut lpuart_config = lpuart::Config::default();
    lpuart_config.baudrate = 115_200;
    let mut lpuart = lpuart::Lpuart::new_blocking(p.LPUART1, p.PC1, p.PC0, lpuart_config).unwrap();
    lpuart
        .write(b"\r\nembassy-py32 LPUART1 ready @115200-8N1\r\n")
        .unwrap();

    // USART1：PA9 = TX、PA10 = RX，AF7，115200-8N1。
    // 引脚与官方 STK 例程 Example_LL/USART/SCI_HyperTerminal_Polling 完全一致。
    // PA9/PA10 与上面的 UART2/LPUART1 引脚不冲突。
    let mut usart_config = usart::Config::default();
    usart_config.baudrate = 115_200;
    let mut usart = usart::Usart::new_blocking(p.USART1, p.PA9, p.PA10, usart_config).unwrap();
    usart
        .write(b"\r\nembassy-py32 USART1 ready @115200-8N1\r\n")
        .unwrap();

    // SPI1 主机：PA5 = SCK、PA7 = MOSI、PA6 = MISO（AF5），模式 0、MSB 优先、1 MHz。
    // 引脚与官方 STK 例程 Example_LL/SPI/SPI_TwoBoards_FullDuplex_Polling 完全一致。
    // 软件 NSS（SSM=1、SSI=1），故不占用 NSS 引脚。这里只演示初始化和一次原地全双工
    // 收发（未接从机时 MISO 悬空，读回值无意义，仅验证 API 通路）。
    let mut spi_config = spi::Config::default();
    spi_config.frequency = Hertz(1_000_000);
    let mut spi = spi::Spi::new_blocking(p.SPI1, p.PA5, p.PA7, p.PA6, spi_config).unwrap();
    let mut spi_buf = [0xAAu8, 0x55];
    spi.blocking_transfer_in_place(&mut spi_buf).unwrap();
    defmt::info!(
        "SPI1 ready (请求 {} Hz → 实际 {} Hz), loopback={=[u8]:#x}",
        spi_config.frequency.0,
        spi.frequency().0,
        spi_buf
    );
    // 16 位帧路径：切 DFF 后发送两个半字（验证 u16 字长可用）。
    spi.blocking_write(&[0x1234u16, 0xABCD]).unwrap();

    // SPI2 从机（硬件 NSS）：PB13=SCK、PB15=MOSI、PB14=MISO、PB12=NSS（AF5）。
    // 仅演示从机构造（无主机时不发起传输，避免阻塞等待），构造后临时值立即析构释放引脚。
    let mut spi3_cfg = spi::Config::default();
    spi3_cfg.frequency = Hertz(3_000_000);
    spi::Spi::new_blocking_slave(p.SPI2, p.PB13, p.PB15, p.PB14, p.PB12, spi3_cfg).unwrap();

    // ---- ST7735 TFT LCD（SPI3 主机，来自参考设计的 st7735 驱动）----
    // 接线：SCK=PB3(AF6)、MOSI=PB5(AF6)；DC=PD1、RST=PD0；
    // CS 直接接地（固定选中）、BLK 直接接 3V3（常亮）→ 二者无需 GPIO，传 None。
    // 注意：ST7735 只写、模块无 MISO；而 embassy-py32 的 SPI 驱动构造时必须给一个 MISO 脚，
    // 这里借空闲的 PB4 当"假 MISO"（不接线）。若 PB4(NJTRST) 影响烧录，改成 p.PC11 即可。
    let mut lcd_spi_config = spi::Config::default(); // 模式 0、MSB 优先
    lcd_spi_config.frequency = Hertz(30_000_000); // 请求 30MHz（实际由 PCLK1 分频决定，见下）
    let lcd_spi = spi::Spi::new_blocking(p.SPI3, p.PB3, p.PB5, p.PB4, lcd_spi_config).unwrap();
    // 实际 SCK：读 CR1.BR 反推（不是请求值）。PCLK1=42MHz 时 30MHz 请求 → 分频档 /2 = 21MHz。
    let lcd_sck = lcd_spi.frequency();

    let lcd_dc = Output::new(p.PD1, Level::Low, Speed::VeryHigh);
    let lcd_rst = Output::new(p.PD0, Level::Low, Speed::VeryHigh);

    // 160×80 横屏（对齐参考工程 stm32f412rgt6-microboard 的 ST7735 配置）
    let tft_config = TftConfig {
        width: 160,
        height: 80,
        orientation: Orientation::Landscape,
        rgb: false,
        inverted: true,
        x_offset: 26,
        y_offset: 1,
    };

    let mut lcd = St7735Driver::new(
        SpiBusAdapter::new(lcd_spi),
        None, // CS 接地，无需片选
        OutputPinAdapter::new(lcd_dc),
        Some(OutputPinAdapter::new(lcd_rst)),
        None, // BLK 接 3V3，常亮
        Delay::new(clocks.hclk.0),
        tft_config,
    );
    lcd.init();
    defmt::info!(
        "ST7735 init ok (SPI3 请求 {} Hz → 实际 {} Hz)",
        lcd_spi_config.frequency.0,
        lcd_sck.0
    );

    // 用 display 层填色 + 写字，直观验证点亮（同时也让该模块不被判为 dead_code）
    let mut display = Display::new(lcd);
    display.fill_screen(RGB565::Black);
    display.draw_string(4, 4, "PY32E407", FONT_7X10, RGB565::Green, RGB565::Black);
    display.draw_string(
        4,
        18,
        "ST7735 SPI3 OK",
        FONT_7X10,
        RGB565::Yellow,
        RGB565::Black,
    );
    defmt::info!("ST7735 drawn");

    // TIM6：基本定时器，配置为 10 Hz 更新事件，作为主循环节拍。
    // APB1 预分频 = /2，因此定时器内核时钟 CK_INT = 2 × PCLK1 = 80 MHz
    // （RM 第 6 章时钟树 "x1/x2" 分支；可对照 tim6.timer_clk() 的返回值）。
    let tim6 = Timer::new_blocking(p.TIM6, timer::Config::default()).unwrap();
    defmt::info!("TIM6 CK_INT = {} Hz", tim6.timer_clk().0);
    let beat = timer::Config::from_frequency(tim6.timer_clk(), Hertz(10)).unwrap();
    tim6.set_prescaler(beat.prescaler);
    tim6.set_reload(beat.period).unwrap();
    tim6.generate_update(); // 立即装载 PSC/ARR 影子寄存器并清 UIF
    tim6.start();

    // TIM7：更新中断示例（对齐 embassy-stm32 `timer::low_level` 的用法）。
    // 10 ms 更新周期（100 Hz），使能 DIER.UIE 后在中断里清 SR.UIF；
    // 需要先 unmask NVIC 的 TIM7 向量，否则中断请求不会被 CPU 响应。
    // TIM7 与 TIM6 同属基本定时器、同在 APB1，两者互不影响。
    let tim7 = Timer::new_blocking(p.TIM7, timer::Config::default()).unwrap();
    tim7.set_period_ms(10, RoundTo::Slower);
    tim7.set_autoreload_preload(true);
    tim7.enable_update_interrupt(true);
    tim7.start();
    unsafe { cortex_m::peripheral::NVIC::unmask(embassy_py32::pac::Interrupt::TIM7) };
    defmt::info!("TIM7 update IRQ @ {} Hz", tim7.get_frequency().0);

    // TIM1_CH4 PWM 输出：PC3（TIM1_CH4 在 PC3 上的复用列为 AF2）。
    // 500 Hz、50% 占空比、高电平有效；PC3 那颗 LED 会以 PWM 调光方式亮起。
    // 构造时计数器已启动但通道输出关闭，需显式 enable(Channel::Ch4)。
    let ch4 = PwmPin::new_ch4(p.PC3);
    let mut pwm_config = embassy_py32::timer::pwm::Config::default();
    pwm_config.frequency = Hertz(500);
    let mut pwm = Pwm::new(p.TIM1, None, None, None, Some(ch4), pwm_config).unwrap();
    pwm.enable(Channel::Ch4).unwrap();
    pwm.set_duty_cycle_percent(Channel::Ch4, 50).unwrap();
    defmt::info!("TIM1_CH4 PWM {} Hz", pwm.frequency().0);

    // ---- 三路正交编码器接口（QEI）----
    // 编码器 A/B 相即该定时器的 CH1/CH2 引脚（编码器模式下作 TI1/TI2 输入），
    // 引脚与 AF 号取自 DS V0.13 端口复用表，均复用 pwm 的通道引脚表：
    //   TIM2：PD3 = CH1（AF2）、PD4 = CH2（AF2）
    //   TIM3：PE2 = CH1（AF2）、PE3 = CH2（AF2）
    //   TIM4：PD12 = CH1（AF2）、PD13 = CH2（AF2）
    // 三路都用默认配置：模式 3（x4）、上升沿、不滤波、ARR = 0xFFFF。
    // 未接编码器时 CNT 保持 0，属正常。
    let mut qei2 = embassy_py32::timer::qei::Qei::new(
        p.TIM2,
        p.PD3,
        p.PD4,
        embassy_py32::timer::qei::Config::default(),
    )
    .unwrap();
    let mut qei3 = embassy_py32::timer::qei::Qei::new(
        p.TIM3,
        p.PE2,
        p.PE3,
        embassy_py32::timer::qei::Config::default(),
    )
    .unwrap();
    let mut qei4 = embassy_py32::timer::qei::Qei::new(
        p.TIM4,
        p.PD12,
        p.PD13,
        embassy_py32::timer::qei::Config::default(),
    )
    .unwrap();
    defmt::info!(
        "QEI ready: TIM2(PD3/PD4) max={} TIM3(PE2/PE3) max={} TIM4(PD12/PD13) max={}",
        qei2.max_count(),
        qei3.max_count(),
        qei4.max_count()
    );

    // I2C3 主机：PC8 = SCL、PC9 = SDA（AF8），与官方例程
    // Example_LL/I2C/I2C_TwoBoards_Com_Pollig_Init 一致；100 kHz 标准模式。
    // 注意：I2C 需要总线上有外部上拉电阻。这里对 7 位地址 0x50（常见 EEPROM 地址）
    // 读一个寄存器，总线上无从机时会返回 Err(Nack) 属正常，故用 is_ok() 记录而非 unwrap。
    let mut i2c = embassy_py32::i2c::I2c::new_blocking(
        p.I2C3,
        p.PC8,
        p.PC9,
        embassy_py32::i2c::Config::default(),
    )
    .unwrap();
    let mut i2c_buf = [0u8; 2];
    if i2c.blocking_read_reg(0x50, 0x00, &mut i2c_buf).is_ok() {
        defmt::info!("I2C3 read ok: {=[u8]:#x}", i2c_buf);
    } else {
        // 总线上无从机时 NACK 属正常（也可进 RTT 看具体 err code，这里只报结果）
        defmt::info!("I2C3: no device acked on bus (expected without a slave)");
    }

    let mut buf = [0u8; 1];
    let mut lcd_cycle = 0usize;
    loop {
        // 回显收到的字节。没有数据时 read() 会等到超时后返回 Err(Timeout)，
        // 这里直接忽略，继续闪灯——同步驱动没有"就绪查询"接口，属预期行为。
        if uart.read(&mut buf).is_ok() {
            let _ = uart.write(&buf);
        }

        pb1.toggle();
        pb2.toggle();
        // 注意：PC3 已交给 TIM1_CH4 做 PWM，不再在循环里 toggle（同一引脚只能有一个持有者）。
        // 等待 TIM6 的一次更新事件（1 Hz）。单次 wait_for_update 的轮询上限
        // 约 0.3~0.4 s，小于 1 s 的更新周期，超时返回 Err(Timeout) 属预期，循环重试。
        while tim6.wait_for_update().is_err() {}
        // defmt::info!("Ciallo (TIM6 1Hz tick)");

        // LCD：每秒整屏换一种颜色并重绘标题，直观验证屏幕"亮着且在刷新"
        let bg = LCD_CYCLE[lcd_cycle];
        lcd_cycle = (lcd_cycle + 1) % LCD_CYCLE.len();
        display.fill_screen(bg);
        display.draw_string(4, 4, "PY32E407", FONT_7X10, RGB565::White, bg);
        display.draw_string(4, 18, "ST7735 SPI3 OK", FONT_7X10, RGB565::Yellow, bg);
        // defmt::info!(
        //     "TIM7 IRQ ticks={}",
        //     TIM7_TICKS.load(core::sync::atomic::Ordering::Relaxed)
        // );
        // 三个编码器在刚过去的 1s 内的增量：本拍读到的 CNT 就是自上一拍清零以来的
        // 计数，读完立即 reset()，于是下一拍读到的又是新一段增量。
        // `as i16` 把 16 位回绕 / 反向计数还原为带符号值（反转时 CNT 会接近 0xFFFF）。
        defmt::info!(
            "QEI inc 1s: TIM2={} up={} | TIM3={} up={} | TIM4={} up={}",
            { u32::MAX - qei2.count() + 1 } as i16,
            matches!(
                qei2.read_direction(),
                embassy_py32::timer::qei::Direction::Up
            ),
            qei3.count() as i16,
            matches!(
                qei3.read_direction(),
                embassy_py32::timer::qei::Direction::Up
            ),
            qei4.count() as i16,
            matches!(
                qei4.read_direction(),
                embassy_py32::timer::qei::Direction::Up
            )
        );
        qei2.reset();
        qei3.reset();
        qei4.reset();
    }
}

/// TIM7 更新中断计数（中断里自增，主循环可读）。
///
/// 用 `AtomicU32` 而非 `static mut`：中断与主循环可能同时访问，原子访问避免数据竞争，
/// 也无需 `unsafe`（对齐 embassy-stm32 示例的共享计数写法）。
static TIM7_TICKS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

/// TIM7 更新中断服务函数（`#[interrupt]` 来自 cortex-m-rt；向量名取自 PAC）。
///
/// 每次更新事件硬件置 `SR.UIF` 并触发本中断；**必须写 0 清 UIF**，否则不会收到
/// 下一次中断。这里直接经 PAC 清位，等价于 `Timer::clear_update_interrupt()`。
#[interrupt]
fn TIM7() {
    let tim7 = unsafe { &*embassy_py32::pac::Tim7::PTR };
    tim7.sr().write(|w| w.uif().clear_bit());
    TIM7_TICKS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
}

/// 极简 panic 处理：先用 RTT 打一条带位置的日志（否则只能看到程序"静默卡死"，
/// 分不清是没跑起来还是 panic 了），再挂死原地打转等调试器接管。
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", defmt::Display2Format(info));
    loop {
        core::hint::spin_loop();
    }
}
