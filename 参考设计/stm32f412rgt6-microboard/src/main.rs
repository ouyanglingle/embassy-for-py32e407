#![no_std]
#![no_main]
use crate::{
    kvdb::{FlashErase, FlashWrite},
    lcd::{LcdDriver, display::BufferDisplay, font::FONT_7X10},
    mpu6050::Mpu6050Ahrs,
};
use cortex_m::prelude::_embedded_hal_Pwm;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{
    self, Config as BoardClockConfig, bind_interrupts, dma,
    gpio::{Input, Level, Output, OutputType, Pull, Speed},
    i2c::{self, Config as I2cConfig, I2c},
    peripherals,
    qspi::{Config as QspiConfig, Qspi, TransferConfig},
    spi::{Config as SpiConfig, Spi},
    time::{Hertz, khz, mhz},
    timer::{
        Channel,
        low_level::CountingMode,
        simple_pwm::{PwmPin, SimplePwm},
    },
    usart::{self, Config as UartConfig, Uart},
};

use defmt_rtt as _;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel as MpscChannel;
use panic_probe as _;

mod kvdb;
use kvdb::{Kvdb, MyFlash};
mod lcd;
use lcd::{
    color::RGB565,
    st7735::{Config as TftConfig, St7735Driver},
};
mod mpu6050;
use mpu6050::{AccelRange, Address, ClockSource, Config, DlpfBandwidth, GyroRange, Mpu6050};

bind_interrupts!(struct Irqs{
    DMA2_STREAM5 => dma::InterruptHandler<peripherals::DMA2_CH5>;
    USART1 => usart::InterruptHandler<peripherals::USART1>;
    DMA2_STREAM7 => dma::InterruptHandler<peripherals::DMA2_CH7>;
    DMA2_STREAM2 => dma::InterruptHandler<peripherals::DMA2_CH2>;
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
    DMA1_STREAM6 => dma::InterruptHandler<peripherals::DMA1_CH6>;
    DMA1_STREAM5 => dma::InterruptHandler<peripherals::DMA1_CH5>;
});

/// 将 u32 格式化为十进制 ASCII 写入 buf，返回写入的字节数。
fn format_u32(mut n: u32, buf: &mut [u8]) -> usize {
    if n == 0 {
        if !buf.is_empty() {
            buf[0] = b'0';
        }
        return 1;
    }
    let start = 0;
    let mut pos = 0;
    while n > 0 {
        buf[pos] = b'0' + (n % 10) as u8;
        pos += 1;
        n /= 10;
    }
    buf[start..pos].reverse();
    pos
}

/// 将 i32 格式化为带符号十进制 ASCII，返回写入的字节数。
/// 最小值 i32::MIN 会回绕，但实际角度值不会到那个量级。
fn format_i32(mut n: i32, buf: &mut [u8]) -> usize {
    if n == 0 {
        if !buf.is_empty() {
            buf[0] = b'0';
        }
        return 1;
    }
    let start = 0;
    let mut pos = 0;
    let negative = n < 0;
    if negative {
        n = -n;
    }
    while n > 0 {
        buf[pos] = b'0' + (n % 10) as u8;
        pos += 1;
        n /= 10;
    }
    if negative {
        buf[pos] = b'-';
        pos += 1;
    }
    buf[start..pos].reverse();
    pos
}

/// 将 f32 格式化为定点小数 ASCII（xx.yy 格式），返回写入的字节数。
/// value 范围有限（角度 ±180，温度 ±50），小数部分保留 2 位。
fn format_f32(value: f32, buf: &mut [u8]) -> usize {
    let abs = if value < 0.0 { -value } else { value };
    let int_part = abs as i32;
    let frac_part = ((abs - int_part as f32) * 100.0 + 0.5) as i32; // 四舍五入到 2 位小数
    let mut pos = 0;

    // 符号
    if value < 0.0 {
        buf[pos] = b'-';
        pos += 1;
    }

    // 整数部分
    let n_int = format_i32(int_part, &mut buf[pos..]);
    pos += n_int;

    // 小数点
    buf[pos] = b'.';
    pos += 1;

    // 小数部分（保证 2 位）
    buf[pos] = b'0' + (frac_part / 10) as u8;
    buf[pos + 1] = b'0' + (frac_part % 10) as u8;
    pos += 2;

    pos
}

// 单点维护，不然太长了，要维护6个泛型
type MyKvdb = Kvdb<MyFlash, 4096, 8, 32, 32, 64>;

/// UART 发送通道：主循环 → uart_sender 任务
/// 缓冲区大小 48 字节（足以容纳一帧姿态数据），深度 4 帧
type UartBuf = [u8; 48];
static UART_CHANNEL: MpscChannel<CriticalSectionRawMutex, UartBuf, 4> = MpscChannel::new();

/// 从通道取数据并发送的任务
#[embassy_executor::task]
async fn uart_sender(mut tx: embassy_stm32::usart::UartTx<'static, embassy_stm32::mode::Async>) {
    let rx = UART_CHANNEL.receiver();
    loop {
        let buf = rx.receive().await;
        let _ = tx.write(&buf[..]).await;
    }
}
static mut uart_buf: [u8; 48] = [0u8; 48];
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(enable100_mhz());
    embassy_time::Timer::after_millis(1000).await; // 等待系统稳定
    let delay = embassy_time::Delay;
    let mut led = Output::new(p.PC15, Level::Low, Speed::VeryHigh);

    let mut uart1_cfg = UartConfig::default();
    uart1_cfg.baudrate = 115200;
    let uart1 = Uart::new(
        p.USART1, p.PA10, p.PA9, p.DMA2_CH7, p.DMA2_CH2, Irqs, uart1_cfg,
    )
    .unwrap();
    let (uart1_tx, _uart1_rx) = uart1.split();

    // ── 按钮配置：PC0（左按钮）、PC1（右按钮），上拉输入 ──
    let _btn_left = Input::new(p.PC0, Pull::Up);
    let _btn_right = Input::new(p.PC1, Pull::Up);

    // 128Mbit = 16MiB QSPI Flash 配置
    let mut qspi_cfg = QspiConfig::default();
    qspi_cfg.memory_size = embassy_stm32::qspi::enums::MemorySize::_16MiB;
    qspi_cfg.address_size = embassy_stm32::qspi::enums::AddressSize::_24bit;
    qspi_cfg.prescaler = 1; // 20MHz  QSPI_CLK = HCLK / (prescaler + 1)

    let qspi = Qspi::new_blocking_bank1(
        p.QUADSPI, p.PC9, p.PC10, p.PC8, p.PA1, p.PB1, p.PB6, qspi_cfg,
    );
    let mut gd25_flash = MyFlash::new(qspi);
    // 读取 JEDEC ID 获取制造商信息
    let mut jedec_id = [0u8; 3];
    let jedec_cmd = TransferConfig {
        iwidth: embassy_stm32::qspi::enums::QspiWidth::SING,
        awidth: embassy_stm32::qspi::enums::QspiWidth::NONE,
        dwidth: embassy_stm32::qspi::enums::QspiWidth::SING,
        instruction: 0x9F,
        address: None,
        dummy: embassy_stm32::qspi::enums::DummyCycles::_0,
    };
    gd25_flash.qspi.blocking_read(&mut jedec_id, jedec_cmd);
    defmt::info!(
        "JEDEC ID: {:02X} {:02X} {:02X}",
        jedec_id[0],
        jedec_id[1],
        jedec_id[2]
    );
    defmt::info!("Flash capacity: 128 Mbit / 16 MiB");

    // 示例：读取 Flash 前 256 字节（0x00 地址处的数据）
    let mut buf = [0u8; 256];
    let read_cmd = TransferConfig {
        iwidth: embassy_stm32::qspi::enums::QspiWidth::SING,
        awidth: embassy_stm32::qspi::enums::QspiWidth::SING,
        dwidth: embassy_stm32::qspi::enums::QspiWidth::SING,
        instruction: 0x03,       // 标准 Read Data 命令
        address: Some(0x000000), // 从 0 地址开始读
        dummy: embassy_stm32::qspi::enums::DummyCycles::_0,
    };
    gd25_flash.qspi.blocking_read(&mut buf, read_cmd);
    defmt::info!("First 16 bytes: {:a}", &buf[..16]);

    // 3. 初始化
    let mut db = MyKvdb::new(gd25_flash, 0x0000_0000).unwrap();
    // 4. 写入数据
    // db.set(b"count0", &4550u32.to_le_bytes()).unwrap();
    // db.set(b"count1", &999u32.to_le_bytes()).unwrap();
    // db.set(b"count2", &192u32.to_le_bytes()).unwrap();

    // 5. 读取数据
    let mut buf = [0u8; 4];
    db.get(b"count0", &mut buf).unwrap();
    defmt::info!("count0的值是: {}", u32::from_le_bytes(buf));
    db.get(b"count1", &mut buf).unwrap();
    defmt::info!("count1的值是: {}", u32::from_le_bytes(buf));
    db.get(b"count2", &mut buf).unwrap();
    defmt::info!("count2的值是: {}", u32::from_le_bytes(buf));
    // db.delete(b"count1").unwrap();
    let mut spi1_config = SpiConfig::default();
    spi1_config.frequency = mhz(50);
    let spi1 = Spi::new_txonly(p.SPI1, p.PA5, p.PA7, p.DMA2_CH5, Irqs, spi1_config);
    let blk_pin = Output::new(p.PA4, Level::High, Speed::Low);
    let dc_pin = Output::new(p.PC4, Level::Low, Speed::VeryHigh);
    let rst_pin = Output::new(p.PC5, Level::Low, Speed::VeryHigh);

    let tft_config = TftConfig {
        width: 160,
        height: 80,
        orientation: lcd::st7735::Orientation::Landscape,
        rgb: false,
        inverted: true,
        x_offset: 26,
        y_offset: 1,
    };
    // 硬件上CS直接接地了，所以这里不需要CS
    let mut st7735 = St7735Driver::new(
        spi1,
        None,
        dc_pin,
        Some(rst_pin),
        Some(blk_pin),
        delay,
        tft_config,
    );

    st7735.init();
    // ── 使用 BufferDisplay 全帧缓冲 ──
    // 屏幕 160×80 = 12800 像素，缓冲区自动分配
    let mut bdis = BufferDisplay::<12800>::new(160, 80);
    bdis.fill_screen(RGB565::Black);

    bdis.draw_string(
        0,
        0,
        "16MiB KVDB!",
        FONT_7X10,
        RGB565::Yellow,
        RGB565::Black,
    );

    // 查询所有键与值
    for i in 0..db.len() {
        // 先把键复制出来，避免借用冲突
        let key = db.key_at(i).unwrap();
        let mut key_buf = [0u8; 32]; // MAX_K = 32
        let klen = key.len();
        key_buf[..klen].copy_from_slice(key);
        let key_str = core::str::from_utf8(&key_buf[..klen]).unwrap();

        // 读取对应的值（现在 db 的不可变借用已释放）
        let mut val_buf = [0u8; 32]; // MAX_V = 32
        let _vlen = db.get(&key_buf[..klen], &mut val_buf).unwrap();

        // 构造 "key: VALUE" 显示字符串
        let mut display_buf = [0u8; 64];
        let mut pos = 0;

        // 复制键
        for &b in key_str.as_bytes() {
            display_buf[pos] = b;
            pos += 1;
        }
        display_buf[pos] = b':';
        pos += 1;
        display_buf[pos] = b' ';
        pos += 1;

        // 将值解析为 u32 并格式化为十进制
        let value = u32::from_le_bytes(val_buf[..4].try_into().unwrap());
        let n = format_u32(value, &mut display_buf[pos..]);
        pos += n;

        let display_str = core::str::from_utf8(&display_buf[..pos]).unwrap();
        bdis.draw_string(
            0,
            (10 + i * 10) as u16,
            display_str,
            FONT_7X10,
            RGB565::Purple,
            RGB565::Black,
        );
    }

    // 将初始画面一次性刷新到 LCD
    bdis.flush(&mut st7735);

    let tim2_ch1: PwmPin<'_, peripherals::TIM2, _> = PwmPin::new(p.PA0, OutputType::PushPull);
    let mut tim2 = SimplePwm::new(
        p.TIM2,
        Some(tim2_ch1),
        None,
        None,
        None,
        khz(2),
        CountingMode::EdgeAlignedUp,
    );
    tim2.enable(Channel::Ch1);
    info!("Max duty: {}", tim2.get_max_duty());
    tim2.set_duty(Channel::Ch1, 500);
    tim2.set_frequency(Hertz(4000));
    tim2.set_duty(Channel::Ch1, 0);
    let i2c1 = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB7,
        p.DMA1_CH6,
        p.DMA1_CH5,
        Irqs,
        I2cConfig::default(),
    );
    // 创建驱动实例（默认地址 0x68），使用 ±250°/s 陀螺仪量程（灵敏度 8 倍于 ±2000°/s）
    let mpu_config = Config {
        accel_range: AccelRange::G2,
        gyro_range: GyroRange::Dps250,
        dlpf: DlpfBandwidth::Accel44Gyro42,
        sample_rate_div: 0x07, // 125Hz
        clock_source: ClockSource::PllXGyro,
    };
    // 初始化传感器（最多重试 3 次）
    let mut mpu = Mpu6050::new_with_config(i2c1, Address::default(), mpu_config);
    if mpu.init().is_err() {
        bdis.draw_string(
            0,
            50,
            "MPU Init Failed",
            FONT_7X10,
            RGB565::Red,
            RGB565::Black,
        );
        bdis.flush(&mut st7735);
        loop {
            // embassy_time::Timer::after_millis(1000).await;
            cortex_m::asm::nop(); // 纯忙等待，不进入睡眠
        }
    } else {
        bdis.draw_string(
            0,
            50,
            "MPU Init Success",
            FONT_7X10,
            RGB565::Green,
            RGB565::Black,
        );
        bdis.flush(&mut st7735);
    }
    embassy_time::Timer::after_millis(800).await; // 显示结果 1 秒
    bdis.fill_screen(RGB565::Black);
    // ── 启动 UART 发送任务 ──
    spawner.spawn(uart_sender(uart1_tx)).unwrap();
    let uart_tx_sender = UART_CHANNEL.sender();

    // ─── UART1 AT 指令测试 ─────────────────────────────────────────
    // 发送 AT 指令
    // let at_cmd = b"AT+BDSPPMiniBoard-V1.1\r\n";
    // uart1_tx.write(at_cmd).await.unwrap();
    // info!("UART1 已发送: AT");

    // // 等待并接收对方的回传数据（read_until_idle 会在数据停顿后返回）
    // let mut rx_buf = [0u8; 256];
    // match uart1_rx.read_until_idle(&mut rx_buf).await {
    //     Ok(n) => {
    //         info!("UART1 收到 ({} 字节): {:a}", n, &rx_buf[..n]);
    //     }
    //     Err(e) => {
    //         info!("UART1 接收错误: {}", e);
    //     }
    // }

    // ── 陀螺仪零偏校准（紧贴 AHRS 循环，减小温漂影响）──
    let gyro_bias = mpu.calibrate_gyro_bias(200).unwrap();
    info!(
        "Gyro bias: x={} y={} z={} °/s",
        gyro_bias.x, gyro_bias.y, gyro_bias.z
    );

    let mut ahrs = Mpu6050Ahrs::new();
    ahrs.set_gyro_bias(gyro_bias);

    // 用初始加速度计初始化四元数
    {
        let (accel, _, _) = mpu.read_all_c_units().unwrap();
        ahrs.init_with_accel(accel.x, accel.y, accel.z);
        embassy_time::Timer::after_millis(200).await; // 等待稳定
        ahrs.set_attitude_zero();
    }

    // 首次时间戳
    let mut prev_time = embassy_time::Instant::now();

    // ── UART 发送缓冲区 ──
    // 格式: "R:xx.xx P:yy.yy Y:zz.zz T:tt.t\n" 约 36 字节
    // let mut uart_buf = [0u8; 48];

    // ── 主循环用显示变量 ──
    let display_font = FONT_7X10;
    let display_bg = RGB565::Black;
    // 每行文字 7×10，最大 "R:-180.00" 约 63px 宽
    let clear_w = 75;
    let clear_h = 10;
    let row_positions = [10, 20, 30, 40]; // Roll, Pitch, Yaw, Temp
    let col_x = 0;

    // 屏幕显示用字符串缓冲区
    let mut display_buf = [0u8; 32];

    loop {
        led.toggle();

        // 实测 dt
        let now = embassy_time::Instant::now();
        let dt_s = (now - prev_time).as_micros() as f32 / 1_000_000.0;
        prev_time = now;

        // 一次性读取所有数据
        let (accel, gyro, temp) = mpu.read_all_c_units().unwrap();

        // 根据陀螺仪角速度模值判断是否静态（阈值放宽到 5°/s，避免噪声误判）
        let gyro_sq = gyro.x * gyro.x + gyro.y * gyro.y + gyro.z * gyro.z;
        let is_static = gyro_sq < 25.0; // 模值² < 25 → 模值 < 5 °/s

        ahrs.update(
            accel.x, accel.y, accel.z, gyro.x, gyro.y, gyro.z, is_static, dt_s,
        );
        let angle = ahrs.get_euler_deg();
        // defmt::info!(
        //     "Roll: {}  Pitch: {}  Yaw: {} T:{}",
        //     angle.roll,
        //     angle.pitch,
        //     angle.yaw,
        //     temp
        // );

        // ── 屏幕显示（每帧都更新缓冲区，但只在缓冲区满时 flush）──
        // 先清除旧数据（4 行区域）
        for &row_y in &row_positions {
            bdis.fill_rect(col_x, row_y, clear_w, clear_h, display_bg);
        }

        // 绘制 Roll
        {
            let mut p = 0;
            display_buf[p..p + 2].copy_from_slice(b"R:");
            p += 2;
            p += format_f32(angle.roll, &mut display_buf[p..]);
            let s = core::str::from_utf8(&display_buf[..p]).unwrap();
            bdis.draw_string(
                col_x,
                row_positions[0],
                s,
                display_font,
                RGB565::Yellow,
                display_bg,
            );
        }

        // 绘制 Pitch
        {
            let mut p = 0;
            display_buf[p..p + 2].copy_from_slice(b"P:");
            p += 2;
            p += format_f32(angle.pitch, &mut display_buf[p..]);
            let s = core::str::from_utf8(&display_buf[..p]).unwrap();
            bdis.draw_string(
                col_x,
                row_positions[1],
                s,
                display_font,
                RGB565::Green,
                display_bg,
            );
        }

        // 绘制 Yaw
        {
            let mut p = 0;
            display_buf[p..p + 2].copy_from_slice(b"Y:");
            p += 2;
            p += format_f32(angle.yaw, &mut display_buf[p..]);
            let s = core::str::from_utf8(&display_buf[..p]).unwrap();
            bdis.draw_string(
                col_x,
                row_positions[2],
                s,
                display_font,
                RGB565::Cyan,
                display_bg,
            );
        }

        // 绘制温度
        {
            let mut p = 0;
            display_buf[p..p + 2].copy_from_slice(b"T:");
            p += 2;
            p += format_f32(temp.0, &mut display_buf[p..]);
            let s = core::str::from_utf8(&display_buf[..p]).unwrap();
            bdis.draw_string(
                col_x,
                row_positions[3],
                s,
                display_font,
                RGB565::Orange,
                display_bg,
            );
        }

        // 将姿态角区域（4 行，最大宽度 75px）刷新到 LCD
        bdis.flush(&mut st7735);

        // ── 通过通道发送到 UART 任务 ──
        // 格式: "R:xx.xx P:yy.yy Y:zz.zz T:tt.t\n"
        let mut p = 0;
        unsafe {
            uart_buf[p] = b'R';
            p += 1;
            uart_buf[p] = b':';
            p += 1;
            p += format_f32(angle.roll, &mut uart_buf[p..]);
            uart_buf[p] = b' ';
            p += 1;
            uart_buf[p] = b'P';
            p += 1;
            uart_buf[p] = b':';
            p += 1;
            p += format_f32(angle.pitch, &mut uart_buf[p..]);
            uart_buf[p] = b' ';
            p += 1;
            uart_buf[p] = b'Y';
            p += 1;
            uart_buf[p] = b':';
            p += 1;
            p += format_f32(angle.yaw, &mut uart_buf[p..]);
            uart_buf[p] = b' ';
            p += 1;
            uart_buf[p] = b'T';
            p += 1;
            uart_buf[p] = b':';
            p += 1;
            p += format_f32(temp.0, &mut uart_buf[p..]);
            uart_buf[p] = b'\n';
            p += 1;

            // 发送到通道（通道满时等待，但 4 帧深度通常不会满）
            let _ = uart_tx_sender.send(uart_buf).await;
        }
    }
}

fn enable100_mhz() -> BoardClockConfig {
    let mut config = BoardClockConfig::default();
    use embassy_stm32::rcc::*;
    config.rcc.hse = Some(Hse {
        freq: Hertz(8_000_000),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll_src = PllSource::HSE;
    config.rcc.pll = Some(Pll {
        prediv: PllPreDiv::DIV4,
        mul: PllMul::MUL100,
        divp: Some(PllPDiv::DIV2),
        divq: Some(PllQDiv::DIV4),
        divr: Some(PllRDiv::DIV2),
    });
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV2;
    config.rcc.apb2_pre = APBPrescaler::DIV1;
    config.rcc.sys = Sysclk::PLL1_P;
    config
}
