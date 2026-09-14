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

use embassy_py32::gpio::{Level, Output, Speed};
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

use lcd::color::RGB565;
use lcd::display::Display;
use lcd::font::FONT_7X10;
use lcd::st7735::{Config as TftConfig, Orientation, St7735Driver};
use lcd::LcdDriver;
use lcd_hal::{Delay, OutputPinAdapter, SpiBusAdapter};

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

    // TIM2 正交编码器接口（QEI）：PA0 = TI1（CH1）、PA1 = TI2（CH2），AF1。
    // 默认模式 3（x4）、上升沿、不滤波。编码器 A/B 接 PA0/PA1 后转动即改变 count()。
    // 未接编码器时读数保持 0，属正常。这里读一次位置与方向并记录。
    let mut qei = embassy_py32::timer::qei::Qei::new(
        p.TIM2,
        p.PD3,
        p.PD4,
        embassy_py32::timer::qei::Config::default(),
    )
    .unwrap();
    let qei_up = matches!(
        qei.read_direction(),
        embassy_py32::timer::qei::Direction::Up
    );
    defmt::info!(
        "TIM2 QEI count={} max={} up={}",
        qei.count(),
        qei.max_count(),
        qei_up
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
        defmt::info!("Ciallo (TIM6 1Hz tick)");

        // LCD：每秒整屏换一种颜色并重绘标题，直观验证屏幕"亮着且在刷新"
        let bg = LCD_CYCLE[lcd_cycle];
        lcd_cycle = (lcd_cycle + 1) % LCD_CYCLE.len();
        display.fill_screen(bg);
        display.draw_string(4, 4, "PY32E407", FONT_7X10, RGB565::White, bg);
        display.draw_string(4, 18, "ST7735 SPI3 OK", FONT_7X10, RGB565::Yellow, bg);
        defmt::info!(
            "TIM7 IRQ ticks={}",
            TIM7_TICKS.load(core::sync::atomic::Ordering::Relaxed)
        );
        defmt::info!(
            "TIM2 QEI count={} max={} up={}",
            qei.count() as i16,
            qei.max_count(),
            qei_up
        );
        qei.reset();
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
