# MPU6050 六轴传感器 Rust 驱动

基于逐飞科技（SeekFree）C 语言版本驱动 `zf_device_mpu6050.c/h` 移植的 Rust 驱动，使用 `embedded-hal` 阻塞式 I2C trait。

## 概述

MPU6050 是 InvenSense 公司生产的六轴惯性测量单元（IMU），集成了：
- **三轴加速度计**：可配置量程 ±2g / ±4g / ±8g / ±16g
- **三轴陀螺仪**：可配置量程 ±250°/s / ±500°/s / ±1000°/s / ±2000°/s
- **温度传感器**：精度 ±1°C

## 硬件接线

| 模块引脚 | MCU 引脚 | 说明 |
|---------|---------|------|
| VCC | 3.3V | 电源 |
| GND | GND | 地 |
| SCL | I2C SCL | I2C 时钟线 |
| SDA | I2C SDA | I2C 数据线 |
| AD0 | GND/VCC | 地址选择（GND=0x68, VCC=0x69）|

## 快速开始

### 1. 在 `main.rs` 中声明模块

```rust
mod mpu6050;
```

### 2. 初始化 I2C 和 MPU6050

```rust
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use crate::mpu6050::{Mpu6050, Address, AccelRange, GyroRange, Config};

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

// 初始化 I2C（以 I2C1 为例，PB6=SCL, PB7=SDA）
let i2c = i2c::I2c::new(
    p.I2C1,
    p.PB6,
    p.PB7,
    Irqs,
    i2c::Config::default(),
);

// 创建驱动实例（默认地址 0x68）
let mut mpu = Mpu6050::new(i2c, Address::default());

// 初始化传感器
mpu.init().unwrap();
```

### 3. 读取传感器数据

```rust
// 读取原始数据
let accel_raw = mpu.read_accel_raw().unwrap();
let gyro_raw = mpu.read_gyro_raw().unwrap();
defmt::info!("Accel raw: x={}, y={}, z={}", accel_raw.x, accel_raw.y, accel_raw.z);
defmt::info!("Gyro raw:  x={}, y={}, z={}", gyro_raw.x, gyro_raw.y, gyro_raw.z);

// 读取物理单位数据（g 和 °/s，与 C 驱动一致）
let accel_g = mpu.read_accel_g().unwrap();
let gyro_dps = mpu.read_gyro_dps().unwrap();
defmt::info!("Accel: x={:.2}g, y={:.2}g, z={:.2}g", accel_g.x, accel_g.y, accel_g.z);
defmt::info!("Gyro:  x={:.1}°/s, y={:.1}°/s, z={:.1}°/s", gyro_dps.x, gyro_dps.y, gyro_dps.z);

// 读取温度
let temp = mpu.read_temperature().unwrap();
defmt::info!("Temperature: {}", temp);

// 一次性读取所有数据（C 驱动单位）
let (accel, gyro, temp) = mpu.read_all_c_units().unwrap();
```

### 4. 自定义配置

```rust
// 使用自定义配置创建驱动
let config = Config {
    accel_range: AccelRange::G2,      // ±2g
    gyro_range: GyroRange::Dps250,    // ±250°/s
    dlpf: DlpfBandwidth::Accel94Gyro98, // 94Hz/98Hz 低通滤波
    sample_rate_div: 0x07,            // 125Hz 采样率
    clock_source: ClockSource::Internal8Mhz, // 内部 8MHz 振荡器（与 C 驱动一致）
};
let mut mpu = Mpu6050::new_with_config(i2c, Address::default(), config);
mpu.init().unwrap();

// 运行时修改量程
mpu.set_accel_range(AccelRange::G4).unwrap();
mpu.set_gyro_range(GyroRange::Dps500).unwrap();
```

## API 参考

### 核心类型

| 类型 | 说明 |
|------|------|
| `Mpu6050<I2C>` | 驱动主结构体，泛型于 `embedded-hal` I2C 实现 |
| `Address` | I2C 地址选择（`Low`=0x68, `High`=0x69） |
| `Config` | 驱动配置（量程、滤波、采样率等） |
| `AccelRange` | 加速度计量程枚举 |
| `GyroRange` | 陀螺仪量程枚举 |
| `DlpfBandwidth` | 数字低通滤波器带宽枚举 |
| `ClockSource` | 时钟源选择枚举 |

### 数据类型

| 类型 | 单位 | 说明 |
|------|------|------|
| `RawData` | i16 | 原始三轴数据 |
| `AccelData` | m/s² | 加速度计数据（SI 单位） |
| `AccelDataG` | g | 加速度计数据（重力单位，与 C 驱动一致） |
| `GyroData` | rad/s | 陀螺仪数据（SI 单位） |
| `GyroDataDps` | °/s | 陀螺仪数据（度/秒，与 C 驱动一致） |
| `Temperature` | °C | 温度数据 |
| `AllRawData` | - | 一次读取的所有原始数据 |
| `AllData` | g + °/s + °C | 一次读取的所有物理单位数据 |

### 主要方法

| 方法 | 说明 |
|------|------|
| `new(i2c, address)` | 创建驱动实例 |
| `new_with_config(i2c, address, config)` | 使用自定义配置创建 |
| `init()` | 初始化传感器（与 C 驱动序列一致） |
| `self_check()` | WHO_AM_I 自检 |
| `read_accel_raw()` | 读取加速度计原始数据 |
| `read_accel()` | 读取加速度计数据（m/s²） |
| `read_accel_g()` | 读取加速度计数据（g） |
| `read_gyro_raw()` | 读取陀螺仪原始数据 |
| `read_gyro()` | 读取陀螺仪数据（rad/s） |
| `read_gyro_dps()` | 读取陀螺仪数据（°/s） |
| `read_temperature_raw()` | 读取温度原始数据 |
| `read_temperature()` | 读取温度（°C） |
| `read_all()` | 读取所有数据（SI 单位） |
| `read_all_c_units()` | 读取所有数据（C 驱动单位：g + °/s + °C） |
| `read_all_raw()` | 单次 I2C 事务读取所有原始数据（14 字节） |
| `set_accel_range()` | 设置加速度计量程 |
| `set_gyro_range()` | 设置陀螺仪量程 |
| `set_sample_rate_div()` | 设置采样率分频 |
| `set_dlpf()` | 设置数字低通滤波器 |
| `set_clock_source()` | 设置时钟源 |
| `sleep()` | 进入/退出睡眠模式 |
| `release()` | 释放 I2C 总线 |

### 工具函数

| 函数 | 说明 |
|------|------|
| `accel_to_g(raw, range)` | 原始值转 g（对应 C 驱动 `mpu6050_acc_transition`） |
| `gyro_to_dps(raw, range)` | 原始值转 °/s（对应 C 驱动 `mpu6050_gyro_transition`） |

### 错误类型

| 变体 | 说明 |
|------|------|
| `Error::Bus(I2C::Error)` | I2C 总线通信错误 |
| `Error::WhoAmI { expected, got }` | WHO_AM_I 校验失败（设备 ID 不匹配） |
| `Error::Timeout` | 自检超时 |

## 量程与灵敏度对照表

### 加速度计

| 量程 | 枚举值 | 寄存器值 | 灵敏度 (LSB/g) | C 驱动转换因子 |
|------|--------|---------|---------------|--------------|
| ±2g | `AccelRange::G2` | 0x00 | 16384 | /16384 |
| ±4g | `AccelRange::G4` | 0x08 | 8192 | /8192 |
| ±8g | `AccelRange::G8` | 0x10 | 4096 | /4096 |
| ±16g | `AccelRange::G16` | 0x18 | 2048 | /2048 |

### 陀螺仪

| 量程 | 枚举值 | 寄存器值 | 灵敏度 (LSB/(°/s)) | C 驱动转换因子 |
|------|--------|---------|-------------------|--------------|
| ±250°/s | `GyroRange::Dps250` | 0x00 | 131.0 | /131.0 |
| ±500°/s | `GyroRange::Dps500` | 0x08 | 65.5 | /65.5 |
| ±1000°/s | `GyroRange::Dps1000` | 0x10 | 32.8 | /32.8 |
| ±2000°/s | `GyroRange::Dps2000` | 0x18 | 16.4 | /16.4 |

### 采样率计算

采样率由 `sample_rate_div` 决定：

```
Sample_Rate = 1kHz / (1 + SMPLRT_DIV)
```

| 分频值 | 采样率 |
|--------|--------|
| 0 | 1000 Hz |
| 7 | 125 Hz（C 驱动默认） |
| 9 | 100 Hz |
| 19 | 50 Hz |
| 99 | 10 Hz |

### 时钟源

| 枚举值 | CLKSEL | 说明 |
|--------|--------|------|
| `ClockSource::Internal8Mhz` | 0 | 内部 8MHz 振荡器（**C 驱动默认**） |
| `ClockSource::PllXGyro` | 1 | PLL + X 轴陀螺仪参考 |
| `ClockSource::PllYGyro` | 2 | PLL + Y 轴陀螺仪参考 |
| `ClockSource::PllZGyro` | 3 | PLL + Z 轴陀螺仪参考 |
| `ClockSource::PllExt32k` | 4 | PLL + 外部 32.768kHz |
| `ClockSource::PllExt19m` | 5 | PLL + 外部 19.2MHz |

> **注意**：C 驱动写入 `PWR_MGMT_1 = 0x00`，即使用内部 8MHz 振荡器。本驱动默认值与之保持一致。若需更高时钟精度，建议使用 PLL 陀螺仪参考时钟。

## C 驱动与 Rust 驱动对照

| C 驱动 | Rust 驱动 | 说明 |
|--------|----------|------|
| `mpu6050_init()` | `mpu.init()` | 初始化 |
| `mpu6050_get_acc()` | `mpu.read_accel_raw()` | 读取加速度计原始值 |
| `mpu6050_get_gyro()` | `mpu.read_gyro_raw()` | 读取陀螺仪原始值 |
| `mpu6050_acc_transition(val)` | `mpu.read_accel_g()` 或 `accel_to_g(val, range)` | 原始值转 g |
| `mpu6050_gyro_transition(val)` | `mpu.read_gyro_dps()` 或 `gyro_to_dps(val, range)` | 原始值转 °/s |
| `mpu6050_gyro_x/y/z` | `RawData { x, y, z }` | 全局变量 → 结构体字段 |
| `mpu6050_acc_x/y/z` | `RawData { x, y, z }` | 全局变量 → 结构体字段 |
| `MPU6050_ACC_SAMPLE` | `Config.accel_range` | 编译时常量 → 运行时配置 |
| `MPU6050_GYR_SAMPLE` | `Config.gyro_range` | 编译时常量 → 运行时配置 |
| `MPU6050_DEV_ADDR` | `Address::Low/High` | 宏定义 → 枚举 |
| `MPU6050_USE_SOFT_IIC` | N/A | Rust 版本使用硬件 I2C |
| 无 | `mpu.read_temperature()` | C 驱动未提供温度读取 |
| 无 | `mpu.read_all_raw()` | 单次 I2C 事务读取 14 字节 |

## 与 C 驱动的主要差异

1. **阻塞式 I2C**：使用 `embedded-hal` 的 `I2c` trait，方法均为同步阻塞调用（无需 `async/await`）
2. **类型安全**：使用枚举替代宏定义，编译期保证配置值合法
3. **无全局变量**：C 驱动使用全局变量存储数据，Rust 驱动返回结构体
4. **错误处理**：使用 `Result` 类型替代返回值检查，包含详细的错误信息
5. **硬件 I2C**：Rust 版本使用 `embedded-hal` I2C trait，不提供软件 I2C 支持
6. **批量读取**：提供 `read_all_raw()` 方法，单次 I2C 事务读取 14 字节数据，效率更高
7. **温度读取**：C 驱动未提供温度读取，Rust 驱动增加了温度传感器支持
8. **运行时配置**：C 驱动的量程等参数为编译时常量，Rust 驱动支持运行时修改

## 默认配置对照

| 参数 | C 驱动 | Rust 驱动 `Config::default()` |
|------|--------|------------------------------|
| 加速度计量程 | ±8g (`ACCEL_CONFIG=0x10`) | `AccelRange::G8` |
| 陀螺仪量程 | ±2000°/s (`GYRO_CONFIG=0x18`) | `GyroRange::Dps2000` |
| DLPF | 21Hz/20Hz (`CONFIG=0x04`) | `DlpfBandwidth::Accel21Gyro20` |
| 采样率分频 | 7 (`SMPLRT_DIV=0x07` → 125Hz) | `0x07` |
| 时钟源 | 内部 8MHz (`PWR_MGMT_1=0x00`) | `ClockSource::Internal8Mhz` |

## 寄存器地址映射

| 寄存器 | 地址 | 说明 |
|--------|------|------|
| SMPLRT_DIV | 0x19 | 采样率分频 |
| CONFIG | 0x1A | 数字低通滤波器 |
| GYRO_CONFIG | 0x1B | 陀螺仪配置 |
| ACCEL_CONFIG | 0x1C | 加速度计配置 |
| INT_PIN_CFG | 0x37 | 中断引脚配置 |
| ACCEL_XOUT_H | 0x3B | 加速度计 X 高字节 |
| TEMP_OUT_H | 0x41 | 温度高字节 |
| GYRO_XOUT_H | 0x43 | 陀螺仪 X 高字节 |
| USER_CONTROL | 0x6A | 用户控制 |
| PWR_MGMT_1 | 0x6B | 电源管理 1 |
| WHO_AM_I | 0x75 | 设备 ID（默认 0x68） |

## 许可证

与原 C 驱动保持一致，使用 GPL-3.0 许可证。
