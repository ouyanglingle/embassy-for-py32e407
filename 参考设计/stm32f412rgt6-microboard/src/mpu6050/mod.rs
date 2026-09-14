//! MPU6050 六轴惯性测量单元驱动（加速度计 + 陀螺仪）
//!
//! InvenSense MPU6050（MPU-6050）惯性模块驱动，通过 `embedded-hal` trait 支持 I2C 接口。
//!
//! 本驱动移植自已验证的 C 参考驱动（zf_device_mpu6050.c/h），
//! 遵循相同的初始化序列和寄存器配置。
//!
//! # 功能特性
//! - 加速度计，可配置满量程范围（±2g 至 ±16g）
//! - 陀螺仪，可配置满量程范围（±250°/s 至 ±2000°/s）
//! - 采样率分频器配置
//! - 数字低通滤波器（DLPF）带宽配置
//! - 原始数据与换算数据读取（g 和 °/s 单位）
//! - Mahony 姿态解算算法（Roll/Pitch/Yaw 输出）
//!
//! # 快速入门
//! ```ignore
//! let mut mpu = Mpu6050::new(i2c, Address::default());
//! mpu.init()?;
//! let accel = mpu.read_accel()?;
//! let gyro = mpu.read_gyro()?;
//! ```
//!
//! # 姿态解算
//! ```ignore
//! let mut ahrs = Mpu6050Ahrs::new();
//! loop {
//!     let (accel, gyro, _) = mpu.read_all_c_units()?;
//!     ahrs.update(accel.x, accel.y, accel.z, gyro.x, gyro.y, gyro.z, false, 0.01);
//!     let (roll, pitch, yaw) = ahrs.get_euler_deg();
//! }
//! ```
//!
//! # 接线说明
//! - VCC：3.3V 电源
//! - GND：接地
//! - SCL：I2C 时钟线
//! - SDA：I2C 数据线
//! - AD0：I2C 地址选择（接 GND = 0x68，接 VCC = 0x69）

#![no_std]

use core::fmt;
use embedded_hal::i2c::I2c;

// ── 寄存器映射表（来自数据手册，与 C 驱动 zf_device_mpu6050.h 对齐）──

mod reg {
    /// 采样率分频器 (0x19)
    pub const SMPLRT_DIV: u8 = 0x19;
    /// 配置寄存器 (0x1A) - DLPF 和外部同步
    pub const CONFIG: u8 = 0x1A;
    /// 陀螺仪配置 (0x1B)
    pub const GYRO_CONFIG: u8 = 0x1B;
    /// 加速度计配置 (0x1C)
    pub const ACCEL_CONFIG: u8 = 0x1C;
    /// 中断引脚配置 (0x37)
    pub const INT_PIN_CFG: u8 = 0x37;
    /// 加速度计 X 轴输出（高字节）
    pub const ACCEL_XOUT_H: u8 = 0x3B;
    /// 加速度计 X 轴输出（低字节）
    pub const ACCEL_XOUT_L: u8 = 0x3C;
    /// 加速度计 Y 轴输出（高字节）
    pub const ACCEL_YOUT_H: u8 = 0x3D;
    /// 加速度计 Y 轴输出（低字节）
    pub const ACCEL_YOUT_L: u8 = 0x3E;
    /// 加速度计 Z 轴输出（高字节）
    pub const ACCEL_ZOUT_H: u8 = 0x3F;
    /// 加速度计 Z 轴输出（低字节）
    pub const ACCEL_ZOUT_L: u8 = 0x40;
    /// 温度输出（高字节）
    pub const TEMP_OUT_H: u8 = 0x41;
    /// 温度输出（低字节）
    pub const TEMP_OUT_L: u8 = 0x42;
    /// 陀螺仪 X 轴输出（高字节）
    pub const GYRO_XOUT_H: u8 = 0x43;
    /// 陀螺仪 X 轴输出（低字节）
    pub const GYRO_XOUT_L: u8 = 0x44;
    /// 陀螺仪 Y 轴输出（高字节）
    pub const GYRO_YOUT_H: u8 = 0x45;
    /// 陀螺仪 Y 轴输出（低字节）
    pub const GYRO_YOUT_L: u8 = 0x46;
    /// 陀螺仪 Z 轴输出（高字节）
    pub const GYRO_ZOUT_H: u8 = 0x47;
    /// 陀螺仪 Z 轴输出（低字节）
    pub const GYRO_ZOUT_L: u8 = 0x48;
    /// 用户控制寄存器 (0x6A)
    pub const USER_CONTROL: u8 = 0x6A;
    /// 电源管理 1 (0x6B)
    pub const PWR_MGMT_1: u8 = 0x6B;
    /// WHO_AM_I 寄存器 (0x75)
    pub const WHO_AM_I: u8 = 0x75;
}

/// WHO_AM_I 默认值（MPU6050）
const WHO_AM_I_VAL: u8 = 0x68;

/// 自检超时计数（与 C 驱动一致）
const TIMEOUT_COUNT: u16 = 0x00FF;

// ── 配置寄存器位掩码 ───────────────────────────────────────────────

mod bits {
    // PWR_MGMT_1
    pub const DEVICE_RESET_BIT: u8 = 0x80;
    pub const SLEEP_BIT: u8 = 0x40;
    pub const CYCLE_BIT: u8 = 0x20;
    pub const TEMP_DIS_BIT: u8 = 0x08;
    pub const CLKSEL_MASK: u8 = 0x07;

    // GYRO_CONFIG
    pub const FS_G_MASK: u8 = 0x18;

    // ACCEL_CONFIG
    pub const FS_A_MASK: u8 = 0x18;

    // CONFIG (DLPF)
    pub const DLPF_CFG_MASK: u8 = 0x07;

    // INT_PIN_CFG
    pub const I2C_BYPASS_EN_BIT: u8 = 0x02;
}

// ── 枚举 / 配置 ────────────────────────────────────────────────────

/// I2C 地址选择。
///
/// AD0 引脚决定 I2C 地址的最低位。
/// - AD0 接 GND：地址 = 0x68（默认）
/// - AD0 接 VCC：地址 = 0x69
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Address {
    /// AD0 引脚接 GND (0x68)
    Low = 0x68,
    /// AD0 引脚接 VCC (0x69)
    High = 0x69,
}

impl Default for Address {
    fn default() -> Self {
        Self::Low
    }
}

impl Address {
    /// 获取 7 位 I2C 地址。
    pub const fn addr(self) -> u8 {
        self as u8
    }
}

/// 加速度计满量程范围。
///
/// 编码于 ACCEL_CONFIG 寄存器的 [4:3] 位。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AccelRange {
    /// ±2 g（默认）
    G2 = 0x00,
    /// ±4 g
    G4 = 0x08,
    /// ±8 g
    G8 = 0x10,
    /// ±16 g
    G16 = 0x18,
}

impl Default for AccelRange {
    fn default() -> Self {
        Self::G2
    }
}

impl AccelRange {
    /// 灵敏度，单位 LSB/g（来自数据手册）。
    /// 原始值 / 灵敏度 = g 单位值
    pub const fn sensitivity_lsb_g(self) -> f32 {
        match self {
            Self::G2 => 16384.0,
            Self::G4 => 8192.0,
            Self::G8 => 4096.0,
            Self::G16 => 2048.0,
        }
    }

    /// 换算因子（除数），与 C 驱动约定一致。
    /// 原始值 / 换算因子 = g 单位值
    pub const fn transition_factor(self) -> f32 {
        self.sensitivity_lsb_g()
    }

    /// 灵敏度，单位 **m/s²/LSB**。
    pub const fn sensitivity_ms2(self) -> f32 {
        9.80665 / self.sensitivity_lsb_g()
    }

    /// ACCEL_CONFIG 寄存器 [4:3] 位的原始值。
    pub const fn bits(self) -> u8 {
        self as u8
    }
}

/// 陀螺仪满量程范围。
///
/// 编码于 GYRO_CONFIG 寄存器的 [4:3] 位。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GyroRange {
    /// ±250 °/s（默认）
    Dps250 = 0x00,
    /// ±500 °/s
    Dps500 = 0x08,
    /// ±1000 °/s
    Dps1000 = 0x10,
    /// ±2000 °/s
    Dps2000 = 0x18,
}

impl Default for GyroRange {
    fn default() -> Self {
        Self::Dps250
    }
}

impl GyroRange {
    /// 灵敏度，单位 LSB/(°/s)（来自数据手册）。
    /// 原始值 / 灵敏度 = °/s 单位值
    pub const fn sensitivity_lsb_dps(self) -> f32 {
        match self {
            Self::Dps250 => 131.0,
            Self::Dps500 => 65.5,
            Self::Dps1000 => 32.8,
            Self::Dps2000 => 16.4,
        }
    }

    /// 换算因子（除数），与 C 驱动约定一致。
    /// 原始值 / 换算因子 = °/s 单位值
    pub const fn transition_factor(self) -> f32 {
        self.sensitivity_lsb_dps()
    }

    /// 灵敏度，单位 **rad/s/LSB**。
    pub const fn sensitivity_rads(self) -> f32 {
        (core::f32::consts::PI / 180.0) / self.sensitivity_lsb_dps()
    }

    /// GYRO_CONFIG 寄存器 [4:3] 位的原始值。
    pub const fn bits(self) -> u8 {
        self as u8
    }
}

/// 数字低通滤波器（DLPF）带宽配置。
///
/// 编码于 CONFIG 寄存器的 [2:0] 位。
/// DLPF 同时影响加速度计和陀螺仪。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DlpfBandwidth {
    /// 加速度计：260Hz，陀螺仪：256Hz（默认）
    Accel260Gyro256 = 0x00,
    /// 加速度计：184Hz，陀螺仪：188Hz
    Accel184Gyro188 = 0x01,
    /// 加速度计：94Hz，陀螺仪：98Hz
    Accel94Gyro98 = 0x02,
    /// 加速度计：44Hz，陀螺仪：42Hz
    Accel44Gyro42 = 0x03,
    /// 加速度计：21Hz，陀螺仪：20Hz
    Accel21Gyro20 = 0x04,
    /// 加速度计：10Hz，陀螺仪：10Hz
    Accel10Gyro10 = 0x05,
    /// 加速度计：5Hz，陀螺仪：5Hz
    Accel5Gyro5 = 0x06,
}

impl Default for DlpfBandwidth {
    fn default() -> Self {
        Self::Accel21Gyro20 // C 驱动默认值：0x04
    }
}

impl DlpfBandwidth {
    pub const fn bits(self) -> u8 {
        self as u8
    }
}

/// 时钟源选择。
///
/// 编码于 PWR_MGMT_1 寄存器的 [2:0] 位。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockSource {
    /// 内部 8MHz 振荡器
    Internal8Mhz = 0x00,
    /// PLL，以 X 轴陀螺仪为参考
    PllXGyro = 0x01,
    /// PLL，以 Y 轴陀螺仪为参考
    PllYGyro = 0x02,
    /// PLL，以 Z 轴陀螺仪为参考
    PllZGyro = 0x03,
    /// PLL，以外部 32.768kHz 为参考
    PllExt32k = 0x04,
    /// PLL，以外部 19.2MHz 为参考
    PllExt19m = 0x05,
}

impl Default for ClockSource {
    fn default() -> Self {
        Self::Internal8Mhz // 与 C 驱动一致：PWR_MGMT_1 = 0x00 → CLKSEL = 0
    }
}

impl ClockSource {
    pub const fn bits(self) -> u8 {
        self as u8
    }
}

// ── 数据类型 ───────────────────────────────────────────────────────

/// 三轴加速度计数据，单位 **m/s²**。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AccelData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// 三轴加速度计数据，单位 **g**（重力单位，与 C 驱动约定一致）。
#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub struct AccelDataG {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// 三轴陀螺仪数据，单位 **rad/s**。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GyroData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// 三轴陀螺仪数据，单位 **°/s**（与 C 驱动约定一致）。
#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub struct GyroDataDps {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// 三轴原始数据（i16）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// 温度，单位 **°C**。
#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub struct Temperature(pub f32);

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} °C", self.0)
    }
}

// ── 错误类型 ───────────────────────────────────────────────────────

/// 驱动错误。
#[derive(Debug)]
pub enum Error<I2C> {
    /// 底层 I2C 总线错误。
    Bus(I2C),
    /// WHO_AM_I 不匹配——设备错误或接线异常。
    WhoAmI { expected: u8, got: u8 },
    /// 等待条件超时。
    Timeout,
}

impl<I2C: fmt::Debug> fmt::Display for Error<I2C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bus(e) => write!(f, "I2C bus error: {e:?}"),
            Self::WhoAmI { expected, got } => {
                write!(
                    f,
                    "WHO_AM_I mismatch: expected 0x{expected:02X}, got 0x{got:02X}"
                )
            }
            Self::Timeout => write!(f, "timeout"),
        }
    }
}

// ── 驱动配置 ───────────────────────────────────────────────────────

/// MPU6050 驱动配置。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// 加速度计满量程范围。
    pub accel_range: AccelRange,
    /// 陀螺仪满量程范围。
    pub gyro_range: GyroRange,
    /// 数字低通滤波器带宽。
    pub dlpf: DlpfBandwidth,
    /// 采样率分频器（采样率 = 1kHz / (1 + smplrt_div)）。
    pub sample_rate_div: u8,
    /// 时钟源。
    pub clock_source: ClockSource,
}

impl Default for Config {
    fn default() -> Self {
        // 与 C 驱动默认值一致：
        // - ACCEL_CONFIG = 0x10 → ±8g
        // - GYRO_CONFIG = 0x18 → ±2000°/s
        // - CONFIG = 0x04 → DLPF 21Hz/20Hz
        // - SMPLRT_DIV = 0x07 → 125Hz 采样率
        Self {
            accel_range: AccelRange::G8,
            gyro_range: GyroRange::Dps2000,
            dlpf: DlpfBandwidth::Accel21Gyro20,
            sample_rate_div: 0x07,               // 125Hz
            clock_source: ClockSource::PllXGyro, // 与 C 驱动一致：PWR_MGMT_1 = 0x00 → CLKSEL = 0
        }
    }
}

// ── 驱动 ───────────────────────────────────────────────────────────

/// 泛型 MPU6050 驱动，支持任意 `I2c` 实现。
///
/// 通过 `embedded-hal` trait 同时支持阻塞式和异步 I2C。
pub struct Mpu6050<I2C> {
    i2c: I2C,
    address: u8,
    config: Config,
}

impl<I2C> Mpu6050<I2C>
where
    I2C: I2c,
{
    /// 使用默认地址 (0x68) 创建驱动实例。
    ///
    /// 传感器**尚未初始化**——需先调用 [`init`](Self::init)。
    pub fn new(i2c: I2C, address: Address) -> Self {
        Self {
            i2c,
            address: address.addr(),
            config: Config::default(),
        }
    }

    /// 使用自定义配置创建驱动实例。
    pub fn new_with_config(i2c: I2C, address: Address, config: Config) -> Self {
        Self {
            i2c,
            address: address.addr(),
            config,
        }
    }

    /// 释放底层 I2C 总线。
    pub fn release(self) -> I2C {
        self.i2c
    }

    /// 获取当前配置。
    pub fn config(&self) -> &Config {
        &self.config
    }

    // ── 底层寄存器访问 ───────────────────────────────────────────────

    /// 读取单个寄存器。
    fn read_reg(&mut self, reg: u8) -> Result<u8, Error<I2C::Error>> {
        let mut buf = [0u8; 1];
        self.i2c
            .write_read(self.address, &[reg], &mut buf)
            .map_err(Error::Bus)?;
        Ok(buf[0])
    }

    /// 写入单个寄存器。
    fn write_reg(&mut self, reg: u8, val: u8) -> Result<(), Error<I2C::Error>> {
        self.i2c
            .write(self.address, &[reg, val])
            .map_err(Error::Bus)
    }

    /// 读取多个连续寄存器。
    fn read_regs(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), Error<I2C::Error>> {
        self.i2c
            .write_read(self.address, &[reg], buf)
            .map_err(Error::Bus)
    }

    // ── 初始化（与 C 驱动序列一致）───────────────────────────────────

    /// 按照已验证的 C 驱动序列初始化 MPU6050。
    ///
    /// 步骤（来自 zf_device_mpu6050.c `mpu6050_init`）：
    /// 1. 上电延时
    /// 2. 自检（WHO_AM_I 带重试）
    /// 3. 唤醒（退出休眠模式）
    /// 4. 配置采样率分频器
    /// 5. 配置 DLPF
    /// 6. 配置陀螺仪量程
    /// 7. 配置加速度计量程
    /// 8. 配置用户控制和中断引脚
    // pub fn init(&mut self) -> Result<(), Error<I2C::Error>> {
    //     // 1) 上电延时（加长到 300ms，MPU6050 上电后 I2C 总线有时不稳定）
    //     // embassy_time::block_for(embassy_time::Duration::from_millis(300));

    //     // 2. 软件复位 MPU6050（即使总线刚恢复也可能需要）
    //     self.write_reg(reg::PWR_MGMT_1, bits::DEVICE_RESET_BIT)?;
    //     embassy_time::block_for(embassy_time::Duration::from_millis(100));

    //     // 3) 唤醒（退出休眠模式）并设置时钟源
    //     //    C 驱动写入 PWR_MGMT_1 = 0x00（sleep=0, CLKSEL=0=内部8MHz）
    //     //    此处应用 self.config.clock_source 中的配置
    //     self.write_reg(reg::PWR_MGMT_1, self.config.clock_source.bits())?;

    //     // 4) 配置采样率分频器（与 C 驱动一致：SMPLRT_DIV = 0x07 → 125Hz）
    //     self.write_reg(reg::SMPLRT_DIV, self.config.sample_rate_div)?;
    //     // 5.1) WHO_AM_I 自检（只读，不重复写）
    //     self.self_check()?; // self_check 内部不再写唤醒和分频器
    //     // 5.2) 配置 DLPF（与 C 驱动一致：CONFIG = 0x04）
    //     self.write_reg(reg::CONFIG, self.config.dlpf.bits())?;

    //     // 6) 配置陀螺仪量程（与 C 驱动一致：GYRO_CONFIG = 0x18 → ±2000°/s）
    //     self.write_reg(reg::GYRO_CONFIG, self.config.gyro_range.bits())?;

    //     // 7) 配置加速度计量程（与 C 驱动一致：ACCEL_CONFIG = 0x10 → ±8g）
    //     self.write_reg(reg::ACCEL_CONFIG, self.config.accel_range.bits())?;

    //     // 8) 配置用户控制和中断引脚（与 C 驱动一致）
    //     self.write_reg(reg::USER_CONTROL, 0x00)?;
    //     self.write_reg(reg::INT_PIN_CFG, bits::I2C_BYPASS_EN_BIT)?;

    //     Ok(())
    // }

    pub fn init(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_reg(reg::PWR_MGMT_1, bits::DEVICE_RESET_BIT)?;
        embassy_time::block_for(embassy_time::Duration::from_millis(100));
        self.write_reg(reg::PWR_MGMT_1, self.config.clock_source.bits())?;
        self.write_reg(reg::SMPLRT_DIV, self.config.sample_rate_div)?;
        self.self_check()?;
        self.write_reg(reg::CONFIG, self.config.dlpf.bits())?;
        self.write_reg(reg::GYRO_CONFIG, self.config.gyro_range.bits())?;
        self.write_reg(reg::ACCEL_CONFIG, self.config.accel_range.bits())?;
        self.write_reg(reg::USER_CONTROL, 0x00)?;
        self.write_reg(reg::INT_PIN_CFG, bits::I2C_BYPASS_EN_BIT)?;
        Ok(())
    }

    /// 自检：验证 WHO_AM_I，超时重试。
    ///
    /// 设备返回预期的 WHO_AM_I 值则返回 `Ok(())`。
    /// 若设备在超时时间内未正确响应，返回 `Err(Error::WhoAmI)`。
    pub fn self_check(&mut self) -> Result<(), Error<I2C::Error>> {
        for _ in 0..TIMEOUT_COUNT {
            match self.read_reg(reg::WHO_AM_I) {
                Ok(id) if id == WHO_AM_I_VAL => return Ok(()),
                Ok(id) => {} // 继续重试
                Err(_) => {} // I2C 错误也继续重试
            }
            embassy_time::block_for(embassy_time::Duration::from_millis(10));
        }
        // 超时，读取最后一次的值用于报错
        Err(Error::WhoAmI {
            expected: WHO_AM_I_VAL,
            got: self.read_reg(reg::WHO_AM_I).unwrap_or(0xFF),
        })
    }

    // ── 配置 ─────────────────────────────────────────────────────────

    /// 设置加速度计满量程范围。
    pub fn set_accel_range(&mut self, range: AccelRange) -> Result<(), Error<I2C::Error>> {
        self.config.accel_range = range;
        self.write_reg(reg::ACCEL_CONFIG, range.bits())
    }

    /// 设置陀螺仪满量程范围。
    pub fn set_gyro_range(&mut self, range: GyroRange) -> Result<(), Error<I2C::Error>> {
        self.config.gyro_range = range;
        self.write_reg(reg::GYRO_CONFIG, range.bits())
    }

    /// 设置采样率分频器。
    ///
    /// 采样率 = 1kHz / (1 + SMPLRT_DIV)
    /// 125Hz：SMPLRT_DIV = 7
    /// 100Hz：SMPLRT_DIV = 9
    /// 50Hz：SMPLRT_DIV = 19
    pub fn set_sample_rate_div(&mut self, div: u8) -> Result<(), Error<I2C::Error>> {
        self.config.sample_rate_div = div;
        self.write_reg(reg::SMPLRT_DIV, div)
    }

    /// 设置数字低通滤波器带宽。
    pub fn set_dlpf(&mut self, dlpf: DlpfBandwidth) -> Result<(), Error<I2C::Error>> {
        self.config.dlpf = dlpf;
        self.write_reg(reg::CONFIG, dlpf.bits())
    }

    /// 设置时钟源。
    pub fn set_clock_source(&mut self, source: ClockSource) -> Result<(), Error<I2C::Error>> {
        let pwr = self.read_reg(reg::PWR_MGMT_1)?;
        self.write_reg(reg::PWR_MGMT_1, (pwr & !bits::CLKSEL_MASK) | source.bits())
    }

    /// 使设备进入休眠模式。
    pub fn sleep(&mut self, enable: bool) -> Result<(), Error<I2C::Error>> {
        let pwr = self.read_reg(reg::PWR_MGMT_1)?;
        if enable {
            self.write_reg(reg::PWR_MGMT_1, pwr | bits::SLEEP_BIT)
        } else {
            self.write_reg(reg::PWR_MGMT_1, pwr & !bits::SLEEP_BIT)
        }
    }

    // ── 数据读取 ─────────────────────────────────────────────────────

    /// 读取加速度计原始输出（3 × i16）。
    pub fn read_accel_raw(&mut self) -> Result<RawData, Error<I2C::Error>> {
        let mut buf = [0u8; 6];
        self.read_regs(reg::ACCEL_XOUT_H, &mut buf)?;
        Ok(RawData {
            x: i16::from_be_bytes([buf[0], buf[1]]),
            y: i16::from_be_bytes([buf[2], buf[3]]),
            z: i16::from_be_bytes([buf[4], buf[5]]),
        })
    }

    /// 读取加速度计输出，单位 **m/s²**。
    pub fn read_accel(&mut self) -> Result<AccelData, Error<I2C::Error>> {
        let raw = self.read_accel_raw()?;
        let s = self.config.accel_range.sensitivity_ms2();
        Ok(AccelData {
            x: raw.x as f32 * s,
            y: raw.y as f32 * s,
            z: raw.z as f32 * s,
        })
    }

    /// 读取加速度计输出，单位 **g**（重力单位，与 C 驱动约定一致）。
    /// 使用与 C 驱动相同的换算因子：`原始值 / 换算因子`。
    pub fn read_accel_g(&mut self) -> Result<AccelDataG, Error<I2C::Error>> {
        let raw = self.read_accel_raw()?;
        let factor = self.config.accel_range.transition_factor();
        Ok(AccelDataG {
            x: raw.x as f32 / factor,
            y: raw.y as f32 / factor,
            z: raw.z as f32 / factor,
        })
    }

    /// 读取陀螺仪原始输出（3 × i16）。
    pub fn read_gyro_raw(&mut self) -> Result<RawData, Error<I2C::Error>> {
        let mut buf = [0u8; 6];
        self.read_regs(reg::GYRO_XOUT_H, &mut buf)?;
        Ok(RawData {
            x: i16::from_be_bytes([buf[0], buf[1]]),
            y: i16::from_be_bytes([buf[2], buf[3]]),
            z: i16::from_be_bytes([buf[4], buf[5]]),
        })
    }

    /// 读取陀螺仪输出，单位 **rad/s**。
    pub fn read_gyro(&mut self) -> Result<GyroData, Error<I2C::Error>> {
        let raw = self.read_gyro_raw()?;
        let s = self.config.gyro_range.sensitivity_rads();
        Ok(GyroData {
            x: raw.x as f32 * s,
            y: raw.y as f32 * s,
            z: raw.z as f32 * s,
        })
    }

    /// 读取陀螺仪输出，单位 **°/s**（与 C 驱动约定一致）。
    /// 使用与 C 驱动相同的换算因子：`原始值 / 换算因子`。
    pub fn read_gyro_dps(&mut self) -> Result<GyroDataDps, Error<I2C::Error>> {
        let raw = self.read_gyro_raw()?;
        let factor = self.config.gyro_range.transition_factor();
        Ok(GyroDataDps {
            x: raw.x as f32 / factor,
            y: raw.y as f32 / factor,
            z: raw.z as f32 / factor,
        })
    }

    /// 读取温度原始输出。
    pub fn read_temperature_raw(&mut self) -> Result<i16, Error<I2C::Error>> {
        let mut buf = [0u8; 2];
        self.read_regs(reg::TEMP_OUT_H, &mut buf)?;
        Ok(i16::from_be_bytes([buf[0], buf[1]]))
    }

    /// 读取温度，单位 **°C**。
    ///
    /// 数据手册公式：T(°C) = 原始值 / 340 + 36.53
    pub fn read_temperature(&mut self) -> Result<Temperature, Error<I2C::Error>> {
        let raw = self.read_temperature_raw()?;
        Ok(Temperature(raw as f32 / 340.0 + 36.53))
    }

    /// 便捷方法：一次性读取加速度计 + 陀螺仪 + 温度。
    pub fn read_all(&mut self) -> Result<(AccelData, GyroData, Temperature), Error<I2C::Error>> {
        let accel = self.read_accel()?;
        let gyro = self.read_gyro()?;
        let temp = self.read_temperature()?;
        Ok((accel, gyro, temp))
    }

    /// 便捷方法：一次性读取加速度计（g）+ 陀螺仪（°/s）+ 温度。
    /// 返回与 C 驱动相同单位的数据。
    pub fn read_all_c_units(
        &mut self,
    ) -> Result<(AccelDataG, GyroDataDps, Temperature), Error<I2C::Error>> {
        let accel = self.read_accel_g()?;
        let gyro = self.read_gyro_dps()?;
        let temp = self.read_temperature()?;
        Ok((accel, gyro, temp))
    }

    /// 在单次 I2C 事务中读取所有传感器原始数据（14 字节）。
    ///
    /// 当需要全部数据时，比分别读取更高效。
    pub fn read_all_raw(&mut self) -> Result<AllRawData, Error<I2C::Error>> {
        let mut buf = [0u8; 14];
        self.read_regs(reg::ACCEL_XOUT_H, &mut buf)?;
        Ok(AllRawData {
            accel: RawData {
                x: i16::from_be_bytes([buf[0], buf[1]]),
                y: i16::from_be_bytes([buf[2], buf[3]]),
                z: i16::from_be_bytes([buf[4], buf[5]]),
            },
            temp: i16::from_be_bytes([buf[6], buf[7]]),
            gyro: RawData {
                x: i16::from_be_bytes([buf[8], buf[9]]),
                y: i16::from_be_bytes([buf[10], buf[11]]),
                z: i16::from_be_bytes([buf[12], buf[13]]),
            },
        })
    }

    /// 校准陀螺仪零偏。
    ///
    /// 在传感器**静止**状态下调用，采集 `num_samples` 次陀螺仪数据取平均值作为零偏。
    /// 返回零偏值（°/s），可传给 `Mpu6050Ahrs::set_gyro_bias`。
    ///
    /// # 用法
    /// ```ignore
    /// let bias = mpu.calibrate_gyro_bias(200)?; // 静止时采样 200 次
    /// ahrs.set_gyro_bias(bias);
    /// ```
    ///
    /// # 参数
    /// - `num_samples`: 采样次数，建议 200~500（越多越准但越慢）
    pub fn calibrate_gyro_bias(
        &mut self,
        num_samples: u16,
    ) -> Result<GyroDataDps, Error<I2C::Error>> {
        let mut sum_x: f32 = 0.0;
        let mut sum_y: f32 = 0.0;
        let mut sum_z: f32 = 0.0;

        for _ in 0..num_samples {
            let gyro = self.read_gyro_dps()?;
            sum_x += gyro.x;
            sum_y += gyro.y;
            sum_z += gyro.z;
        }

        let inv = 1.0 / num_samples as f32;
        Ok(GyroDataDps {
            x: sum_x * inv,
            y: sum_y * inv,
            z: sum_z * inv,
        })
    }
}

/// 单次读取的所有原始传感器数据。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllRawData {
    /// 加速度计原始数据（X, Y, Z）
    pub accel: RawData,
    /// 温度原始数据
    pub temp: i16,
    /// 陀螺仪原始数据（X, Y, Z）
    pub gyro: RawData,
}

impl AllRawData {
    /// 根据提供的配置转换为物理单位。
    pub fn to_physical(&self, config: &Config) -> AllData {
        let accel_factor = config.accel_range.transition_factor();
        let gyro_factor = config.gyro_range.transition_factor();
        AllData {
            accel: AccelDataG {
                x: self.accel.x as f32 / accel_factor,
                y: self.accel.y as f32 / accel_factor,
                z: self.accel.z as f32 / accel_factor,
            },
            temp: Temperature(self.temp as f32 / 340.0 + 36.53),
            gyro: GyroDataDps {
                x: self.gyro.x as f32 / gyro_factor,
                y: self.gyro.y as f32 / gyro_factor,
                z: self.gyro.z as f32 / gyro_factor,
            },
        }
    }
}

/// 物理单位的所有传感器数据。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AllData {
    /// 加速度计数据（g）
    pub accel: AccelDataG,
    /// 温度（°C）
    pub temp: Temperature,
    /// 陀螺仪数据（°/s）
    pub gyro: GyroDataDps,
}

/// 欧拉角数据，单位 **度（°）**。
#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub struct EulerAngle {
    /// 横滚角（绕 X 轴旋转），单位 °
    pub roll: f32,
    /// 俯仰角（绕 Y 轴旋转），单位 °
    pub pitch: f32,
    /// 航向角（绕 Z 轴旋转），单位 °
    pub yaw: f32,
}

// ── Mahony 姿态解算（移植自 app_imu_ahrs.c）─────────────────────────

/// Mahony 互补滤波姿态解算参数。
const MAHONY_KP_STATIC: f32 = 4.0;
const MAHONY_KP_DYNAMIC: f32 = 0.01;
const MAHONY_KI_STATIC: f32 = 0.002;
const MAHONY_KI_DYNAMIC: f32 = 0.0;
const MAHONY_INTEGRAL_MAX: f32 = 0.05;
const MAHONY_INTEGRAL_MIN: f32 = -0.05;
const MAHONY_YAW_BIAS_MAX: f32 = 0.1;
const MAHONY_YAW_BIAS_KP_SLOW: f32 = 0.015;
const MAHONY_YAW_BIAS_KP_FAST: f32 = 0.02;

/// Mahony 互补滤波姿态解算器。
///
/// 基于四元数的 Mahony 滤波算法，融合加速度计和陀螺仪数据
/// 输出稳定的欧拉角（Roll/Pitch/Yaw）。
///
/// 算法移植自 `app_imu_ahrs.c`，采用 RK2（中点法）积分，
/// 并包含静态/动态自适应增益和航向零偏估计。
///
/// # 用法
/// ```ignore
/// let mut ahrs = Mpu6050Ahrs::new();
/// // 可选：使用初始加速度计数据初始化四元数
/// ahrs.init_with_accel(ax_g, ay_g, az_g);
///
/// loop {
///     let dt_s = 0.01; // 采样周期（秒）
///     ahrs.update(ax_g, ay_g, az_g, gx_dps, gy_dps, gz_dps, false, dt_s);
///     let euler = ahrs.get_euler_deg();
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Mpu6050Ahrs {
    /// 四元数分量 q0（实部）
    q0: f32,
    /// 四元数分量 q1
    q1: f32,
    /// 四元数分量 q2
    q2: f32,
    /// 四元数分量 q3
    q3: f32,
    /// 误差积分项 X
    ex_int: f32,
    /// 误差积分项 Y
    ey_int: f32,
    /// 误差积分项 Z
    ez_int: f32,
    /// 航向零偏估计（rad/s）
    yaw_bias_rad_s: f32,
    /// 陀螺仪零偏（°/s），由校准得到
    gyro_bias: GyroDataDps,
    /// 是否已完成零偏校准
    bias_calibrated: bool,
    /// 角度偏置（°）：get_euler_deg() 返回时会减去此值
    angle_offset: EulerAngle,
}

impl Default for Mpu6050Ahrs {
    fn default() -> Self {
        Self::new()
    }
}

impl Mpu6050Ahrs {
    /// 创建新的姿态解算器，四元数初始化为 [1, 0, 0, 0]（无旋转）。
    pub fn new() -> Self {
        Self {
            q0: 1.0,
            q1: 0.0,
            q2: 0.0,
            q3: 0.0,
            ex_int: 0.0,
            ey_int: 0.0,
            ez_int: 0.0,
            yaw_bias_rad_s: 0.0,
            gyro_bias: GyroDataDps {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            bias_calibrated: false,
            angle_offset: EulerAngle {
                roll: 0.0,
                pitch: 0.0,
                yaw: 0.0,
            },
        }
    }

    /// 使用初始加速度计数据初始化四元数。
    ///
    /// 根据静态加速度计方向推算初始 Roll/Pitch 角度，
    /// 生成对应的初始四元数。Yaw 角初始为 0。
    ///
    /// 若加速度计向量过小（≤ 0.0001），则退化为单位四元数。
    pub fn init_with_accel(&mut self, ax_g: f32, ay_g: f32, az_g: f32) {
        *self = Self::new();

        let norm = sqrtf((ax_g * ax_g) + (ay_g * ay_g) + (az_g * az_g));
        if norm <= 0.0001 {
            return;
        }

        let ax = ax_g / norm;
        let ay = ay_g / norm;
        let az = az_g / norm;

        let roll = atan2f(ay, az);
        let pitch = -asinf(clampf(ax, -1.0, 1.0));

        let cr = cosf(roll * 0.5);
        let sr = sinf(roll * 0.5);
        let cp = cosf(pitch * 0.5);
        let sp = sinf(pitch * 0.5);

        self.q0 = cr * cp;
        self.q1 = sr * cp;
        self.q2 = cr * sp;
        self.q3 = -sr * sp;
    }

    /// Mahony 姿态更新（核心算法）。
    ///
    /// 输入加速度计（g）和陀螺仪（°/s）数据，融合更新四元数。
    /// 采用 RK2（中点法）数值积分，静态/动态自适应增益。
    ///
    /// 若已通过 `calibrate_gyro_bias` 完成零偏校准，会自动扣除陀螺仪零偏。
    ///
    /// # 参数
    /// - `ax_g, ay_g, az_g`: 加速度计三轴数据，单位 g
    /// - `gx_dps, gy_dps, gz_dps`: 陀螺仪三轴数据，单位 °/s（未扣零偏的原始值）
    /// - `is_static`: 是否为静态（静止状态使用更强的修正增益）
    /// - `dt_s`: 采样周期，单位秒
    pub fn update(
        &mut self,
        ax_g: f32,
        ay_g: f32,
        az_g: f32,
        gx_dps: f32,
        gy_dps: f32,
        gz_dps: f32,
        is_static: bool,
        dt_s: f32,
    ) {
        if dt_s <= 0.0 {
            return;
        }

        // 扣除陀螺仪零偏（校准后生效）
        let gx_dps = if self.bias_calibrated {
            gx_dps - self.gyro_bias.x
        } else {
            gx_dps
        };
        let gy_dps = if self.bias_calibrated {
            gy_dps - self.gyro_bias.y
        } else {
            gy_dps
        };
        let gz_dps = if self.bias_calibrated {
            gz_dps - self.gyro_bias.z
        } else {
            gz_dps
        };

        // 自适应增益选择
        let kp = if is_static {
            MAHONY_KP_STATIC
        } else {
            MAHONY_KP_DYNAMIC
        };
        let ki = if is_static {
            MAHONY_KI_STATIC
        } else {
            MAHONY_KI_DYNAMIC
        };

        // 陀螺仪 °/s → rad/s
        let mut gx = gx_dps * DEG2RAD;
        let mut gy = gy_dps * DEG2RAD;
        let mut gz = gz_dps * DEG2RAD;

        // 静态时进行航向零偏估计
        if is_static {
            let gz_corrected = gz - self.yaw_bias_rad_s;
            let yaw_kp = if fabsf(gz_corrected) > 0.005 {
                MAHONY_YAW_BIAS_KP_FAST
            } else {
                MAHONY_YAW_BIAS_KP_SLOW
            };
            self.yaw_bias_rad_s += yaw_kp * gz_corrected * dt_s;
            self.yaw_bias_rad_s = clampf(
                self.yaw_bias_rad_s,
                -MAHONY_YAW_BIAS_MAX,
                MAHONY_YAW_BIAS_MAX,
            );
        }
        gz -= self.yaw_bias_rad_s;

        // 归一化加速度计
        let norm = sqrtf((ax_g * ax_g) + (ay_g * ay_g) + (az_g * az_g));
        if norm <= 0.0001 {
            return;
        }
        let ax = ax_g / norm;
        let ay = ay_g / norm;
        let az = az_g / norm;

        // 四元数推算的重力方向（半角）
        let halfvx = self.q1 * self.q3 - self.q0 * self.q2;
        let halfvy = self.q0 * self.q1 + self.q2 * self.q3;
        let halfvz = self.q0 * self.q0 - 0.5 + self.q3 * self.q3;

        // 加速度计与推算重力方向的误差（叉积）
        let halfex = ay * halfvz - az * halfvy;
        let halfey = az * halfvx - ax * halfvz;
        let halfez = ax * halfvy - ay * halfvx;

        // 误差积分 + PI 修正
        if ki > 0.0 {
            self.ex_int += ki * halfex * dt_s;
            self.ey_int += ki * halfey * dt_s;
            self.ez_int += ki * halfez * dt_s;

            self.ex_int = clampf(self.ex_int, MAHONY_INTEGRAL_MIN, MAHONY_INTEGRAL_MAX);
            self.ey_int = clampf(self.ey_int, MAHONY_INTEGRAL_MIN, MAHONY_INTEGRAL_MAX);
            self.ez_int = clampf(self.ez_int, MAHONY_INTEGRAL_MIN, MAHONY_INTEGRAL_MAX);

            gx += self.ex_int;
            gy += self.ey_int;
            gz += self.ez_int;
        } else {
            self.ex_int = 0.0;
            self.ey_int = 0.0;
            self.ez_int = 0.0;
        }

        // 比例修正
        gx += kp * halfex;
        gy += kp * halfey;
        gz += kp * halfez;

        // RK2（中点法）四元数积分
        let q0 = self.q0;
        let q1 = self.q1;
        let q2 = self.q2;
        let q3 = self.q3;
        let half_gx = 0.5 * gx;
        let half_gy = 0.5 * gy;
        let half_gz = 0.5 * gz;

        // k1 = f(t, q)
        let k1_q0 = -q1 * half_gx - q2 * half_gy - q3 * half_gz;
        let k1_q1 = q0 * half_gx + q2 * half_gz - q3 * half_gy;
        let k1_q2 = q0 * half_gy - q1 * half_gz + q3 * half_gx;
        let k1_q3 = q0 * half_gz + q1 * half_gy - q2 * half_gx;

        // 中点
        let half_dt = 0.5 * dt_s;
        let mut q_mid0 = q0 + k1_q0 * half_dt;
        let mut q_mid1 = q1 + k1_q1 * half_dt;
        let mut q_mid2 = q2 + k1_q2 * half_dt;
        let mut q_mid3 = q3 + k1_q3 * half_dt;

        // 归一化中点四元数
        let norm =
            sqrtf((q_mid0 * q_mid0) + (q_mid1 * q_mid1) + (q_mid2 * q_mid2) + (q_mid3 * q_mid3));
        if norm <= 0.0001 {
            return;
        }
        q_mid0 /= norm;
        q_mid1 /= norm;
        q_mid2 /= norm;
        q_mid3 /= norm;

        // k2 = f(t + dt/2, q_mid)
        let k2_q0 = -q_mid1 * half_gx - q_mid2 * half_gy - q_mid3 * half_gz;
        let k2_q1 = q_mid0 * half_gx + q_mid2 * half_gz - q_mid3 * half_gy;
        let k2_q2 = q_mid0 * half_gy - q_mid1 * half_gz + q_mid3 * half_gx;
        let k2_q3 = q_mid0 * half_gz + q_mid1 * half_gy - q_mid2 * half_gx;

        // 更新四元数
        self.q0 = q0 + k2_q0 * dt_s;
        self.q1 = q1 + k2_q1 * dt_s;
        self.q2 = q2 + k2_q2 * dt_s;
        self.q3 = q3 + k2_q3 * dt_s;

        // 归一化
        let norm = sqrtf(
            (self.q0 * self.q0) + (self.q1 * self.q1) + (self.q2 * self.q2) + (self.q3 * self.q3),
        );
        if norm > 0.0001 {
            self.q0 /= norm;
            self.q1 /= norm;
            self.q2 /= norm;
            self.q3 /= norm;
        }
    }

    /// 从当前四元数计算欧拉角，单位 **度（°）**。
    ///
    /// 返回 `(roll, pitch, yaw)`：
    /// - Roll：绕 X 轴旋转角，范围 [-180°, 180°]
    /// - Pitch：绕 Y 轴旋转角，范围 [-90°, 90°]
    /// - Yaw：绕 Z 轴旋转角，范围 [-180°, 180°]
    ///
    /// 返回值已自动扣除 `set_attitude_zero()` 设定的角度偏置，
    /// 并正确处理 ±180° 边界环绕。
    pub fn get_euler_deg(&self) -> EulerAngle {
        let raw = self.get_euler_deg_raw();
        // 扣除角度偏置，并归一化到 [-180, 180] 以处理 ±180° 边界环绕
        EulerAngle {
            roll: normalize_angle_180(raw.roll - self.angle_offset.roll),
            pitch: raw.pitch - self.angle_offset.pitch, // Pitch 范围 [-90,90] 无环绕
            yaw: normalize_angle_180(raw.yaw - self.angle_offset.yaw),
        }
    }

    /// 获取未扣除偏置的原始欧拉角（内部使用）。
    fn get_euler_deg_raw(&self) -> EulerAngle {
        let roll_rad = atan2f(
            2.0 * ((self.q0 * self.q1) + (self.q2 * self.q3)),
            1.0 - 2.0 * ((self.q1 * self.q1) + (self.q2 * self.q2)),
        );
        let pitch_sin = 2.0 * ((self.q0 * self.q2) - (self.q3 * self.q1));
        let pitch_rad = asinf(clampf(pitch_sin, -1.0, 1.0));
        let yaw_rad = atan2f(
            2.0 * ((self.q0 * self.q3) + (self.q1 * self.q2)),
            1.0 - 2.0 * ((self.q2 * self.q2) + (self.q3 * self.q3)),
        );

        EulerAngle {
            roll: roll_rad * RAD2DEG,
            pitch: pitch_rad * RAD2DEG,
            yaw: yaw_rad * RAD2DEG,
        }
    }

    /// 将**当前姿态设为 0**。
    ///
    /// 调用后 `get_euler_deg()` 返回的 Roll/Pitch/Yaw 都会以当前方向为基准。
    /// 建议在初始化完成（`init_with_accel` + 几帧 `update` 稳定后）调用。
    ///
    /// # 示例
    ///
    /// ```ignore
    /// ahrs.init_with_accel(ax, ay, az);
    /// // 运行几帧让滤波器稳定
    /// for _ in 0..10 {
    ///     ahrs.update(ax, ay, az, gx, gy, gz, true, dt_s);
    /// }
    /// ahrs.set_attitude_zero(); // 当前方向成为新的零点
    /// ```
    pub fn set_attitude_zero(&mut self) {
        self.angle_offset = self.get_euler_deg_raw();
    }

    /// 获取当前四元数 `(q0, q1, q2, q3)`。
    pub fn get_quaternion(&self) -> (f32, f32, f32, f32) {
        (self.q0, self.q1, self.q2, self.q3)
    }

    /// 重置姿态解算器（四元数恢复为 [1, 0, 0, 0]）。
    ///
    /// 注意：零偏校准结果不会被清除。如需重新校准，请调用 `set_gyro_bias`。
    pub fn reset(&mut self) {
        let bias = self.gyro_bias;
        let calibrated = self.bias_calibrated;
        *self = Self::new();
        self.gyro_bias = bias;
        self.bias_calibrated = calibrated;
    }

    /// 获取当前陀螺仪零偏（°/s）。
    pub fn get_gyro_bias(&self) -> GyroDataDps {
        self.gyro_bias
    }

    /// 手动设置陀螺仪零偏（°/s）。
    ///
    /// 通常配合 `Mpu6050::calibrate_gyro_bias` 使用：
    /// ```ignore
    /// let bias = mpu.calibrate_gyro_bias(200)?; // 静止时采样 200 次
    /// ahrs.set_gyro_bias(bias);
    /// ```
    pub fn set_gyro_bias(&mut self, bias: GyroDataDps) {
        self.gyro_bias = bias;
        self.bias_calibrated = true;
    }
}

// ── AHRS 辅助数学函数（使用 micromath，no_std 兼容）─────────────────

const DEG2RAD: f32 = core::f32::consts::PI / 180.0;
const RAD2DEG: f32 = 180.0 / core::f32::consts::PI;

/// 将角度归一化到 [-180°, 180°] 范围。
///
/// 解决欧拉角 ±180° 边界环绕问题。
/// 例如 358° → -2°（翻转 180° 时 raw=179°, offset=-179°, 差值为 358° → 正确值为 -2°）
#[inline]
fn normalize_angle_180(angle: f32) -> f32 {
    let mut a = angle % 360.0;
    if a > 180.0 {
        a -= 360.0;
    } else if a < -180.0 {
        a += 360.0;
    }
    a
}

#[inline]
fn clampf(value: f32, min_val: f32, max_val: f32) -> f32 {
    if value > max_val {
        max_val
    } else if value < min_val {
        min_val
    } else {
        value
    }
}

use micromath::F32Ext;

#[inline]
fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}

#[inline]
fn atan2f(y: f32, x: f32) -> f32 {
    y.atan2(x)
}

#[inline]
fn asinf(x: f32) -> f32 {
    x.asin()
}

#[inline]
fn cosf(x: f32) -> f32 {
    x.cos()
}

#[inline]
fn sinf(x: f32) -> f32 {
    x.sin()
}

#[inline]
fn fabsf(x: f32) -> f32 {
    if x < 0.0 { -x } else { x }
}

// ── 工具函数（与 C 驱动一致）───────────────────────────────────────

/// 将加速度计原始值转换为 g 单位。
///
/// 本函数与 C 驱动的 `mpu6050_acc_transition` 一致。
pub fn accel_to_g(raw: i16, range: AccelRange) -> f32 {
    raw as f32 / range.transition_factor()
}

/// 将陀螺仪原始值转换为 °/s 单位。
///
/// 本函数与 C 驱动的 `mpu6050_gyro_transition` 一致。
pub fn gyro_to_dps(raw: i16, range: GyroRange) -> f32 {
    raw as f32 / range.transition_factor()
}
