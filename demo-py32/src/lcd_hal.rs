//! embedded-hal 1.0 兼容层
//!
//! 参考设计的 ST7735 驱动（[`crate::lcd::st7735`]）是泛型于 `embedded_hal` 1.0 的
//! `SpiBus` / `OutputPin` / `DelayNs` 写的；而 embassy-py32 是自定义的同步外设 API，
//! 并未实现这些 trait。本模块用薄适配器（newtype）把二者桥接起来，从而可以**原样**
//! 使用参考驱动，而不改动 embassy-py32 与参考驱动本身。
//!
//! 三个适配器：
//! - [`SpiBusAdapter`]：包装 `embassy_py32::spi::Spi`（主机）→ `embedded_hal::spi::SpiBus`
//! - [`OutputPinAdapter`]：包装 `embassy_py32::gpio::Output` → `embedded_hal::digital::OutputPin`
//! - [`Delay`]：基于 `cortex_m::asm::delay` → `embedded_hal::delay::DelayNs`

use core::convert::Infallible;

use embassy_py32::gpio::Output;
use embassy_py32::spi::{Error as SpiError, Instance, Master, Spi};

// ---------------------------------------------------------------------------
// SPI
// ---------------------------------------------------------------------------

/// 包装 embassy-py32 的阻塞 SPI 主机，实现 [`embedded_hal::spi::SpiBus`]。
pub struct SpiBusAdapter<'d, T: Instance> {
    spi: Spi<'d, T, Master>,
}

impl<'d, T: Instance> SpiBusAdapter<'d, T> {
    /// 包裹一个已构造好的 SPI 主机驱动。
    pub fn new(spi: Spi<'d, T, Master>) -> Self {
        Self { spi }
    }
}

/// SPI 错误包装。
///
/// `embedded_hal::spi::Error` 是外部 crate 的 trait，`embassy_py32::spi::Error` 也是
/// 外部类型，受孤儿规则限制无法直接 `impl embedded_hal::spi::Error for embassy_py32::spi::Error`；
/// 故用本地 newtype 包一层，并在 `kind()` 里做错误分类映射。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpiErrorAdapter(pub SpiError);

impl embedded_hal::spi::Error for SpiErrorAdapter {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        use embedded_hal::spi::ErrorKind;
        match self.0 {
            SpiError::Overrun => ErrorKind::Overrun,
            SpiError::ModeFault => ErrorKind::ModeFault,
            SpiError::Framing => ErrorKind::FrameFormat,
            // CRC 未启用、超时/配置错误归入通用类
            SpiError::Crc | SpiError::Timeout | SpiError::InvalidConfig => ErrorKind::Other,
        }
    }
}

impl<T: Instance> embedded_hal::spi::ErrorType for SpiBusAdapter<'_, T> {
    type Error = SpiErrorAdapter;
}

impl<T: Instance> embedded_hal::spi::SpiBus for SpiBusAdapter<'_, T> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.spi.blocking_read::<u8>(words).map_err(SpiErrorAdapter)
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.spi.blocking_write::<u8>(words).map_err(SpiErrorAdapter)
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.spi
            .blocking_transfer::<u8>(read, write)
            .map_err(SpiErrorAdapter)
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.spi
            .blocking_transfer_in_place::<u8>(words)
            .map_err(SpiErrorAdapter)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Spi::flush(&mut self.spi).map_err(SpiErrorAdapter)
    }
}

// ---------------------------------------------------------------------------
// GPIO 输出
// ---------------------------------------------------------------------------

/// 包装 embassy-py32 的推挽输出，实现 [`embedded_hal::digital::OutputPin`]。
pub struct OutputPinAdapter<'d> {
    pin: Output<'d>,
}

impl<'d> OutputPinAdapter<'d> {
    /// 包裹一个已构造好的输出驱动。
    pub fn new(pin: Output<'d>) -> Self {
        Self { pin }
    }
}

impl embedded_hal::digital::ErrorType for OutputPinAdapter<'_> {
    // GPIO 写电平不会失败，用 Infallible（embedded-hal 已为其实现 digital::Error）
    type Error = Infallible;
}

impl embedded_hal::digital::OutputPin for OutputPinAdapter<'_> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low();
        Ok(())
    }

    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high();
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 延时
// ---------------------------------------------------------------------------

/// 基于 `cortex_m::asm::delay` 的忙等延时，实现 [`embedded_hal::delay::DelayNs`]。
///
/// `hclk_hz` 用于把纳秒换算成 CPU 周期数。LCD 的复位/上电时序只要求量级正确、
/// 偏长无妨，故用最简的忙等即可。
pub struct Delay {
    hclk_hz: u32,
}

impl Delay {
    /// 以当前 HCLK 频率（Hz，来自 `rcc::clocks()`）构造延时器。
    pub fn new(hclk_hz: u32) -> Self {
        Self { hclk_hz }
    }
}

impl embedded_hal::delay::DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        // cycles = ns × HCLK / 1e9，用 u64 防溢出
        let cycles = (u64::from(ns) * u64::from(self.hclk_hz) / 1_000_000_000) as u32;
        cortex_m::asm::delay(cycles.max(1));
    }
}
