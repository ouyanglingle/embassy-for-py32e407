//! # embassy-py32
//!
//! PY32E407 系列微控制器（Cortex-M4F，Puya）的 Rust 同步（Blocking）外设驱动库。
//!
//! 代码结构对齐 embassy-stm32 0.6.0：入口是 [`init`]，返回包含全部外设单例的
//! [`Peripherals`]；寄存器访问通过本工程生成的 [`py32e407-pac`](crate::pac) 完成。
//! 当前仅实现同步驱动（不含 DMA/中断/Async），后续将按外设逐步扩展 Async 能力。
//!
//! # 使用示例
//!
//! ```ignore
//! use embassy_py32::{init, gpio::{Level, Output, Speed}, rcc, Config};
//!
//! let p = init(Config::default()); // 默认 HSI16，不动时钟树
//! let mut led = Output::new(p.PA5, Level::Low, Speed::Low);
//! led.toggle();
//! ```
//!
//! # Cargo features
//!
//! - **`embedded-hal-1`**（可选，默认关闭）：为已有外设实现 `embedded-hal` 1.0 的
//!   trait，使 `embedded-hal` 生态的第三方驱动可以直接接收本 HAL 的类型。
//!   impl 全部写在**本 crate 内部**（对齐 embassy-stm32 的做法）——trait 与类型
//!   对下游都是外部项，下游受孤儿规则限制无法自行 impl，只能写 newtype 适配层。
//!   开启后可用：
//!
//!   | 本 HAL 类型 | 实现的 trait |
//!   |---|---|
//!   | [`gpio::Input`] | `digital::InputPin` |
//!   | [`gpio::Output`] | `digital::OutputPin` + `StatefulOutputPin` |
//!   | [`gpio::Flex`] | `digital::OutputPin` + `InputPin` + `StatefulOutputPin` |
//!   | [`gpio::OutputOpenDrain`] | `digital::OutputPin` + `InputPin` + `StatefulOutputPin` |
//!   | [`exti::ExtiInput`] | `digital::InputPin` |
//!   | [`spi::Spi`]（主机） | `spi::SpiBus<u8>` + `spi::SpiBus<u16>` |
//!   | [`i2c::I2c`] | `i2c::I2c` |
//!   | [`timer::pwm::PwmChannel`] | `pwm::SetDutyCycle` |
//!
//!   未覆盖的：UART（`embedded-hal` 1.0 已把串口 trait 移出核心，需 `embedded-io`
//!   或 `embedded-hal-nb`）、QEI（`embedded-hal` 无编码器 trait）、延时
//!   （`delay::DelayNs`，可用 `timer` 自行拼或忙等实现）。

#![no_std]

use core::marker::PhantomData;

pub mod cordic;
pub mod esmc;
pub mod exti;
pub mod gpio;
pub mod i2c;
pub mod lpuart;
pub mod peripherals;
pub mod rcc;
pub mod spi;
pub mod timer;
pub mod uart;
pub mod usart;

pub use peripherals::Peripherals;
pub use py32e407_pac as pac;

/// 频率类型（对齐 embassy 生态的 `Hertz`）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Hertz(
    /// 频率值（Hz）。
    pub u32,
);

/// 外设句柄（借用式所有权，仿 embassy-hal-internal 的 `Peri`）。
///
/// 内部携带零大小的外设单例值（如 [`peripherals::PA5`]），
/// 用于把"哪个外设被占用了"编码进类型系统。
pub struct Peri<'d, T> {
    inner: T,
    _lifetime: PhantomData<&'d ()>,
}

impl<'d, T: PeripheralType + Copy> Peri<'d, T> {
    /// 取出一个外设句柄并携带其单例值。
    ///
    /// # Safety
    ///
    /// 同一外设实例在程序中至多对应一个活跃句柄；
    /// 对 GPIO 引脚而言，同一物理引脚不得同时被两个驱动持有。
    #[inline]
    pub const unsafe fn new_unchecked(inner: T) -> Self {
        Self {
            inner,
            _lifetime: PhantomData,
        }
    }

    /// 短期借出一个子句柄（reborrow），`self` 在子句柄存活期间保持被借用。
    #[inline]
    pub fn reborrow(&mut self) -> Peri<'_, T> {
        unsafe { Self::new_unchecked(self.inner) }
    }
}

impl<'d, T: PeripheralType> core::ops::Deref for Peri<'d, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.inner
    }
}

/// 标记 trait：可被 [`Peri`] 包裹的外设类型。
pub trait PeripheralType {}

/// 全局 HAL 配置（对齐 embassy-stm32 的 `Config`）。
///
/// `#[non_exhaustive]`，外部构造请用 [`Config::default()`] 后逐字段赋值。
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub struct Config {
    /// RCC 时钟树配置。
    pub rcc: rcc::Config,
}

impl Default for Config {
    /// 默认配置：时钟树保持复位默认（HSI16，全 1 分频），只验证就绪不写寄存器。
    fn default() -> Self {
        Self {
            rcc: rcc::Config::default(),
        }
    }
}

/// 初始化整个 HAL（对齐 `embassy_stm32::init`）。
///
/// 依次完成：按 `config.rcc` 配置时钟树（阻塞等待各就绪标志）→
/// 取出全部外设单例。每个外设单例在程序中只能被取出一次。
///
/// # Panics
///
/// 时钟配置失败（振荡器就绪超时/配置非法）时 panic，携带具体错误。
pub fn init(config: Config) -> Peripherals {
    rcc::init(config.rcc).expect("rcc: clock init failed");
    Peripherals::take()
}
