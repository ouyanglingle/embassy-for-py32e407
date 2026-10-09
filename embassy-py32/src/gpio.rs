//! General-purpose Input/Output (GPIO)（同步 Blocking 驱动）
//!
//! 适配 PY32E407 系列（Cortex-M4F，Puya）。寄存器流程依据：
//! - HAL C: `py32e407_hal_gpio.c`（`HAL_GPIO_Init`）
//! - LL:    `py32e407_ll_gpio.h` / `py32e407_ll_bus.h`
//! - RM:    PY32E407 参考手册 V0.6 第 9 章（GPIO）
//!
//! 与 STM32 同名外设的差异（已对照 RM + SVD + PAC 交叉验证）：
//! - SVD/PAC 中输出速度寄存器名为 `OSPDDER`（RM 9.4.3 写作 `OSPEEDR`，同一寄存器，
//!   疑为 SVD 拼写缺陷）；AF 选择字段名为 `AFSELx`（PAC: `afsel{n}`）；
//! - MODER/OTYPER/PUPDR 位编码与 STM32 GPIO v2 一致；
//! - GPIO 时钟使能位在 `RCC_AHB2ENR.IOPxEN`（GPIOA 从 bit2 起，与 ST 不同），
//!   Puya LL 宏在置位后强制读回一次 RCC 寄存器，驱动已复刻该动作。
//!
//! 使用方式对齐 embassy-stm32：`Output::new(p.PA5, Level::Low, Speed::Low)`。
//! 仅实现同步（直接读写）驱动，不含 DMA/中断；预留后续扩展 Async。
//! 引脚所有权由 [`crate::Peri`] 保证：同一引脚同一时刻至多一个驱动句柄，
//! 因此模式配置寄存器（MODER 等）的读-改-写无需临界区。

use crate::pac;
use crate::{Peri, PeripheralType};

/// MODER 字段编码：输入（RM 9.4.1，复位后大部分引脚为模拟态）。
const MODE_INPUT: u8 = 0b00;
/// MODER 字段编码：通用输出。
const MODE_OUTPUT: u8 = 0b01;
/// MODER 字段编码：复用功能。
const MODE_ALTERNATE: u8 = 0b10;
/// MODER 字段编码：模拟（高阻，对应 embassy-stm32 的 disconnected）。
const MODE_ANALOG: u8 = 0b11;

/// 取 GPIOx 寄存器块。
///
/// # Safety（实现说明）
///
/// 指针均来自 PAC 的基址常量；六个端口的寄存器块布局完全一致
/// （同一 SVD 模板，偏移 0x00–0x28），故统一转换为 `gpioa::RegisterBlock` 访问。
#[inline]
fn block(port: u8) -> &'static pac::gpioa::RegisterBlock {
    let ptr: *const pac::gpioa::RegisterBlock = match port {
        0 => pac::Gpioa::PTR.cast(),
        1 => pac::Gpiob::PTR.cast(),
        2 => pac::Gpioc::PTR.cast(),
        3 => pac::Gpiod::PTR.cast(),
        4 => pac::Gpioe::PTR.cast(),
        5 => pac::Gpiof::PTR.cast(),
        _ => unreachable!("invalid GPIO port"),
    };
    unsafe { &*ptr }
}

/// 读-改-写 `GPIOx_MODER` 中某引脚的 2-bit 模式字段。
#[inline]
fn set_moder(port: u8, pin: u8, mode: u8) {
    block(port).moder().modify(|r, w| unsafe {
        w.bits((r.bits() & !(0b11 << (pin * 2))) | ((mode as u32) << (pin * 2)))
    });
}

/// 读-改-写 `GPIOx_PUPDR` 中某引脚的 2-bit 上/下拉字段（RM 9.4.4）。
#[inline]
fn set_pupdr(port: u8, pin: u8, pull: u8) {
    block(port).pupdr().modify(|r, w| unsafe {
        w.bits((r.bits() & !(0b11 << (pin * 2))) | ((pull as u32) << (pin * 2)))
    });
}

/// 读-改-写 `GPIOx_OSPEEDR`（PAC: OSPDDER）中某引脚的 2-bit 速度字段（RM 9.4.3）。
#[inline]
fn set_ospeedr(port: u8, pin: u8, speed: u8) {
    block(port).ospdder().modify(|r, w| unsafe {
        w.bits((r.bits() & !(0b11 << (pin * 2))) | ((speed as u32) << (pin * 2)))
    });
}

/// 读-改-写 `GPIOx_OTYPER` 中某引脚的 1-bit 输出类型字段（RM 9.4.2）。
#[inline]
fn set_otyper(port: u8, pin: u8, open_drain: bool) {
    block(port).otyper().modify(|r, w| unsafe {
        if open_drain {
            w.bits(r.bits() | (1 << pin))
        } else {
            w.bits(r.bits() & !(1 << pin))
        }
    });
}

/// 读-改-写 AF 选择字段：pin 0–7 走 `AFRL.AFSEx`，pin 8–15 走 `AFRH.AFSEx`（RM 9.4.8/9.4.9）。
///
/// 对应 `HAL_GPIO_Init` 中 `GPIOx->AFR[position >> 3]` 的 4-bit 字段写入。
#[inline]
fn set_afr(port: u8, pin: u8, af: u8) {
    debug_assert!(af <= 0xF, "alternate function number must be 0..=15");
    if pin < 8 {
        block(port).afrl().modify(|r, w| unsafe {
            w.bits((r.bits() & !(0xF << (pin * 4))) | ((af as u32) << (pin * 4)))
        });
    } else {
        let n = pin - 8;
        block(port).afrh().modify(|r, w| unsafe {
            w.bits((r.bits() & !(0xF << (n * 4))) | ((af as u32) << (n * 4)))
        });
    }
}

pub(crate) trait SealedPin {
    /// `pin_port` = 端口号 × 16 + 引脚号（与 embassy-stm32 的 `PinNumber` 编码一致）。
    fn pin_port(&self) -> u8;

    /// 引脚号（端口内 0..16）。
    #[inline]
    fn _pin(&self) -> u8 {
        self.pin_port() % 16
    }

    /// 端口号（0 = GPIOA … 5 = GPIOF）。
    #[inline]
    fn _port(&self) -> u8 {
        self.pin_port() / 16
    }

    /// 原子置高（BSRR.BSx 写 1，无需读-改-写，RM 9.4.7）。
    #[inline]
    fn set_high(&self) {
        let (port, n) = (self._port(), self._pin());
        block(port).bsrr().write(|w| unsafe { w.bits(1 << n) });
    }

    /// 原子置低（BSRR.BRx 写 1，无需读-改-写，RM 9.4.7）。
    #[inline]
    fn set_low(&self) {
        let (port, n) = (self._port(), self._pin());
        block(port)
            .bsrr()
            .write(|w| unsafe { w.bits(1 << (16 + n)) });
    }

    /// 把引脚配置为模拟（高阻）态。
    #[inline]
    fn set_as_analog(&self) {
        let (port, n) = (self._port(), self._pin());
        set_pupdr(port, n, Pull::None.to_pupdr());
        set_moder(port, n, MODE_ANALOG);
    }

    /// 把引脚置为"断开"（embassy-stm32 语义，等价模拟态）。
    ///
    /// 驱动在 Drop 时应调用本方法释放引脚。
    #[inline]
    fn set_as_disconnected(&self) {
        self.set_as_analog();
    }
}

/// GPIO 引脚 trait（对齐 embassy-stm32 的 `Pin`）。
#[allow(private_bounds)]
pub trait Pin: PeripheralType + Into<AnyPin> + SealedPin + Sized + 'static {
    /// 引脚号（端口内 0..16）。
    #[inline]
    fn pin(&self) -> u8 {
        self._pin()
    }

    /// 端口号（0 = GPIOA … 5 = GPIOF）。
    #[inline]
    fn port(&self) -> u8 {
        self._port()
    }
}

/// EXTI 引脚 trait（对齐 embassy-stm32 的 `ExtiPin`）。
///
/// 本 trait 只声明"这条引脚归哪条 EXTI 线"，实际的端口选择/触发/屏蔽配置在
/// [`crate::exti`]；EXTI 不在 [`Input`] 里内建，因为它要占用对应的 EXTI 线
/// ——线是稀缺资源（`PA5`/`PB5`/`PC5`… 共用 `EXTI5`），必须显式交出一份所有权。
#[allow(private_bounds)]
pub trait ExtiPin: PeripheralType + SealedPin {
    /// 该引脚对应的 EXTI 通道（如 `PC4` → `EXTI4`）。
    ///
    /// 由 [`crate::peripherals`] 为每个 `Pxy` 实现；同一线号只能有一个
    /// [`crate::Peri`] 通道句柄存在，故编译期即可拦住"两个引脚抢一条线"。
    type ExtiChannel: crate::exti::Channel;
}

/// 类型擦除的 GPIO 引脚（对齐 embassy-stm32 的 `AnyPin`）。
pub struct AnyPin {
    /// `pin_port` = 端口号 × 16 + 引脚号。
    pub(crate) pin_port: u8,
}

impl AnyPin {
    /// 从 pin_port 编号不安全地构造 `AnyPin`（对齐 embassy 的 `AnyPin::steal`）。
    ///
    /// `pin_port` = `port_num * 16 + pin_num`，port_num 0 对应 GPIOA。
    #[inline]
    pub const unsafe fn steal(pin_port: u8) -> Peri<'static, Self> {
        unsafe { Peri::new_unchecked(Self { pin_port }) }
    }
}

impl PeripheralType for AnyPin {}

// 类型擦除后零大小语义由 pin_port 值承载，Copy 安全（对齐 embassy 单例均 Copy）
impl Clone for AnyPin {
    fn clone(&self) -> Self {
        Self {
            pin_port: self.pin_port,
        }
    }
}
impl Copy for AnyPin {}

impl core::fmt::Display for AnyPin {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let port = char::from(b'A' + (self.pin_port / 16));
        let pin = self.pin_port % 16;
        write!(f, "P{port}{pin}")
    }
}

impl SealedPin for AnyPin {
    #[inline]
    fn pin_port(&self) -> u8 {
        self.pin_port
    }
}

impl Pin for AnyPin {}

/// 输入/输出电平。
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Level {
    /// 低电平。
    Low,
    /// 高电平。
    High,
}

impl From<bool> for Level {
    fn from(v: bool) -> Self {
        if v {
            Self::High
        } else {
            Self::Low
        }
    }
}

/// 内部上/下拉电阻设置（RM 9.4.4 PUPDR）。
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Pull {
    /// 无上/下拉。
    None,
    /// 上拉。
    Up,
    /// 下拉。
    Down,
}

impl Pull {
    /// 映射为 PUPDR 的 2-bit 编码（HAL `GPIO_PULLNOPULL/UP/DOWN` = 0/1/2）。
    #[inline]
    fn to_pupdr(self) -> u8 {
        match self {
            Self::None => 0b00,
            Self::Up => 0b01,
            Self::Down => 0b10,
        }
    }
}

/// 输出速度（RM 9.4.3 OSPEEDR，HAL `GPIO_SPEED_FREQ_*`）。
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Speed {
    /// 低速（00）。
    Low,
    /// 中速（01）。
    Medium,
    /// 高速（10）。
    High,
    /// 超高速（11）。
    VeryHigh,
}

impl Speed {
    /// 映射为 OSPEEDR 的 2-bit 编码。
    #[inline]
    fn to_ospeedr(self) -> u8 {
        match self {
            Self::Low => 0b00,
            Self::Medium => 0b01,
            Self::High => 0b10,
            Self::VeryHigh => 0b11,
        }
    }
}

/// 输出类型（RM 9.4.2 OTYPER）。
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OutputType {
    /// 推挽输出。
    PushPull,
    /// 开漏输出。
    OpenDrain,
}

impl OutputType {
    /// 映射为 OTYPER 的 1-bit 编码（0 = 推挽，1 = 开漏）。
    #[inline]
    fn to_otyper(self) -> bool {
        matches!(self, Self::OpenDrain)
    }
}

/// GPIO 灵活引脚：可在输入/输出/复用/模拟之间自由切换。
///
/// 输出锁存（ODR）电平在非输出模式下也会保持，切回输出模式时会"记住"之前的电平。
pub struct Flex<'d> {
    pin: Peri<'d, AnyPin>,
}

impl<'d> Flex<'d> {
    /// 包裹一个引脚（对齐 embassy-stm32 的 `Flex::new`）。
    ///
    /// 内部先使能该端口时钟（含读回等待）；此时引脚仍保持复位默认配置。
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>) -> Self {
        enable_pin_clock(pin.pin_port());
        Self { pin: pin.degrade() }
    }

    /// 把引脚配置为输入模式，并按 `pull` 启用内部上/下拉。
    ///
    /// 寄存器顺序：先写 PUPDR/OTYPER，最后切 MODER（HAL_GPIO_Init 同样最后
    /// 生效 MODER；先配附属寄存器可避免模式切换瞬间的毛刺）。
    #[inline]
    pub fn set_as_input(&mut self, pull: Pull) {
        let (port, n) = self.port_pin();
        set_pupdr(port, n, pull.to_pupdr());
        set_otyper(port, n, false);
        set_moder(port, n, MODE_INPUT);
    }

    /// 把引脚配置为推挽输出。
    ///
    /// 输出电平为之前设置（或复位默认低）；下拉关闭。
    #[inline]
    pub fn set_as_output(&mut self, speed: Speed) {
        let (port, n) = self.port_pin();
        set_pupdr(port, n, Pull::None.to_pupdr());
        set_otyper(port, n, false);
        set_ospeedr(port, n, speed.to_ospeedr());
        set_moder(port, n, MODE_OUTPUT);
    }

    /// 把引脚配置为开漏输出（同时保留输入读取能力）。
    ///
    /// 输出低时驱动拉低，输出高时释放总线（可读回实际线电平）。
    #[inline]
    pub fn set_as_input_output(&mut self, speed: Speed, pull: Pull) {
        let (port, n) = self.port_pin();
        set_pupdr(port, n, pull.to_pupdr());
        set_otyper(port, n, true);
        set_ospeedr(port, n, speed.to_ospeedr());
        set_moder(port, n, MODE_OUTPUT);
    }

    /// 把引脚配置为复用功能（Alternate Function）模式。
    ///
    /// * `af` — 复用功能编号 0–15，具体映射查数据手册 V0.13 的引脚 AF 表；
    /// * `output_type` — 该复用功能的输出类型（SPI/I2C 等外设各自要求不同）。
    ///
    /// 寄存器顺序对照 `HAL_GPIO_Init`：AFR → MODER(=10) → OSPEEDR → OTYPER → PUPDR。
    #[inline]
    pub fn set_as_alternate(&mut self, af: u8, speed: Speed, pull: Pull, output_type: OutputType) {
        let (port, n) = self.port_pin();
        set_afr(port, n, af);
        set_pupdr(port, n, pull.to_pupdr());
        set_otyper(port, n, output_type.to_otyper());
        set_ospeedr(port, n, speed.to_ospeedr());
        set_moder(port, n, MODE_ALTERNATE);
    }

    /// 把引脚配置为模拟模式（高阻）。
    ///
    /// 用作 ADC/COMP 输入，或作为"断开"状态（对应 embassy-stm32 的 disconnected）。
    #[inline]
    pub fn set_as_analog(&mut self) {
        self.pin.set_as_analog();
    }

    /// 读取引脚输入电平是否为高（IDR，RM 9.4.5）。
    #[inline]
    pub fn is_high(&self) -> bool {
        !self.is_low()
    }

    /// 读取引脚输入电平是否为低。
    #[inline]
    pub fn is_low(&self) -> bool {
        let (port, n) = self.port_pin();
        block(port).idr().read().bits() & (1 << n) == 0
    }

    /// 读取当前输入电平。
    #[inline]
    pub fn get_level(&self) -> Level {
        self.is_high().into()
    }

    /// 读取输出锁存是否为高（ODR，RM 9.4.6）。
    #[inline]
    pub fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    /// 读取输出锁存是否为低。
    #[inline]
    pub fn is_set_low(&self) -> bool {
        let (port, n) = self.port_pin();
        block(port).odr().read().bits() & (1 << n) == 0
    }

    /// 读取当前输出锁存电平。
    #[inline]
    pub fn get_output_level(&self) -> Level {
        self.is_set_high().into()
    }

    /// 原子置高。
    #[inline]
    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    /// 原子置低。
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    /// 设置输出电平。
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }

    /// 翻转输出电平。
    #[inline]
    pub fn toggle(&mut self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }

    /// 取 (端口号, 引脚号)。
    #[inline]
    pub(crate) fn port_pin(&self) -> (u8, u8) {
        (self.pin.pin_port() / 16, self.pin.pin_port() % 16)
    }
}

impl<'d> Drop for Flex<'d> {
    #[inline]
    fn drop(&mut self) {
        // 释放引脚时回到断开（高阻）态，对应 embassy-stm32 的 set_as_disconnected
        self.pin.set_as_disconnected();
    }
}

/// 输入模式引脚。
pub struct Input<'d> {
    flex: Flex<'d>,
}

impl<'d> Input<'d> {
    /// 创建输入驱动（对齐 embassy-stm32 的 `Input::new`）。
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>, pull: Pull) -> Self {
        let mut flex = Flex::new(pin);
        flex.set_as_input(pull);
        Self { flex }
    }

    /// 退回 [`Flex`]，便于重新配置模式。
    #[inline]
    pub fn into_flex(self) -> Flex<'d> {
        self.flex
    }

    /// 引脚号（端口内 0..16）。
    ///
    /// 只供 crate 内驱动使用：EXTI 驱动要靠它定位线号，而引脚被类型擦除成
    /// [`AnyPin`] 后，只有 `Peri` 句柄的调用方读不到这个值。
    #[inline]
    pub(crate) fn pin(&self) -> u8 {
        self.flex.port_pin().1
    }

    /// 端口号（0 = GPIOA … 5 = GPIOF）。见 [`Self::pin`]。
    #[inline]
    pub(crate) fn port(&self) -> u8 {
        self.flex.port_pin().0
    }

    /// 读取引脚电平是否为高。
    #[inline]
    pub fn is_high(&self) -> bool {
        self.flex.is_high()
    }

    /// 读取引脚电平是否为低。
    #[inline]
    pub fn is_low(&self) -> bool {
        self.flex.is_low()
    }

    /// 读取当前电平。
    #[inline]
    pub fn get_level(&self) -> Level {
        self.flex.get_level()
    }
}

/// 推挽输出引脚。
pub struct Output<'d> {
    flex: Flex<'d>,
}

impl<'d> Output<'d> {
    /// 创建推挽输出驱动（对齐 embassy-stm32 的 `Output::new`）。
    ///
    /// 先写输出锁存电平、再切输出模式，保证上电首电平无毛刺。
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>, initial_output: Level, speed: Speed) -> Self {
        let mut flex = Flex::new(pin);
        match initial_output {
            Level::High => flex.set_high(),
            Level::Low => flex.set_low(),
        }
        flex.set_as_output(speed);
        Self { flex }
    }

    /// 退回 [`Flex`]，便于重新配置模式。
    #[inline]
    pub fn into_flex(self) -> Flex<'d> {
        self.flex
    }

    /// 置高。
    #[inline]
    pub fn set_high(&mut self) {
        self.flex.set_high()
    }

    /// 置低。
    #[inline]
    pub fn set_low(&mut self) {
        self.flex.set_low()
    }

    /// 设置电平。
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.flex.set_level(level)
    }

    /// 翻转电平。
    #[inline]
    pub fn toggle(&mut self) {
        self.flex.toggle()
    }

    /// 读取输出锁存是否为高。
    #[inline]
    pub fn is_set_high(&self) -> bool {
        self.flex.is_set_high()
    }

    /// 读取输出锁存是否为低。
    #[inline]
    pub fn is_set_low(&self) -> bool {
        self.flex.is_set_low()
    }

    /// 读取当前输出锁存电平。
    #[inline]
    pub fn get_output_level(&self) -> Level {
        self.flex.get_output_level()
    }
}

/// 开漏输出引脚（保留输入读取，可回读实际线电平）。
pub struct OutputOpenDrain<'d> {
    flex: Flex<'d>,
}

impl<'d> OutputOpenDrain<'d> {
    /// 创建开漏输出驱动，无上/下拉（对齐 embassy-stm32 的 `OutputOpenDrain::new`）。
    ///
    /// 先写输出锁存电平、再切输出模式，保证上电首电平无毛刺。
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>, initial_output: Level, speed: Speed) -> Self {
        Self::new_pull(pin, initial_output, speed, Pull::None)
    }

    /// 创建开漏输出驱动并带上/下拉（对齐 embassy-stm32 的 `new_pull`）。
    #[inline]
    pub fn new_pull(
        pin: Peri<'d, impl Pin>,
        initial_output: Level,
        speed: Speed,
        pull: Pull,
    ) -> Self {
        let mut flex = Flex::new(pin);
        match initial_output {
            Level::High => flex.set_high(),
            Level::Low => flex.set_low(),
        }
        flex.set_as_input_output(speed, pull);
        Self { flex }
    }

    /// 退回 [`Flex`]，便于重新配置模式。
    #[inline]
    pub fn into_flex(self) -> Flex<'d> {
        self.flex
    }

    /// 置低（强驱动）。
    #[inline]
    pub fn set_low(&mut self) {
        self.flex.set_low()
    }

    /// 释放总线（置高 = 开漏释放，实际电平由外部上拉决定）。
    #[inline]
    pub fn set_high(&mut self) {
        self.flex.set_high()
    }

    /// 设置输出锁存电平。
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.flex.set_level(level)
    }

    /// 翻转输出锁存电平。
    #[inline]
    pub fn toggle(&mut self) {
        self.flex.toggle()
    }

    /// 读取线上实际电平是否为高（输入路径回读）。
    #[inline]
    pub fn is_high(&self) -> bool {
        self.flex.is_high()
    }

    /// 读取线上实际电平是否为低。
    #[inline]
    pub fn is_low(&self) -> bool {
        self.flex.is_low()
    }

    /// 读取线上实际电平。
    #[inline]
    pub fn get_level(&self) -> Level {
        self.flex.get_level()
    }
}

/// 使能引脚所在端口的 GPIO 时钟（含读回等待）。
///
/// 位号：`RCC_AHB2ENR.IOPxEN`，GPIOA 从 bit2 起（PAC `ahb2enr`：IOPAEN=bit2 …
/// IOPFEN=bit7，与 ST 的 bit0 起始不同）。经 [`crate::rcc`] 的总线助手执行，
/// 读回动作对应 `LL_AHB2_GRP1_EnableClock`。
#[inline]
fn enable_pin_clock(pin_port: u8) {
    let port = pin_port / 16;
    crate::rcc::enable_ahb2(1 << (2 + port));
}

/// 供 Flex 使用的私有便捷方法（degrade 到 AnyPin，对齐 embassy 的类型擦除）。
trait Degrade {
    fn degrade(self) -> Peri<'static, AnyPin>;
}

impl<P: Pin> Degrade for Peri<'_, P> {
    #[inline]
    fn degrade(self) -> Peri<'static, AnyPin> {
        // 丢弃原句柄、取 AnyPin 句柄：所有权语义仍由 Peri 类型系统保证（单例只能取出一次）
        unsafe { Peri::new_unchecked(AnyPin { pin_port: self.pin_port() }) }
    }
}

// ---------------------------------------------------------------------------
// embedded-hal 1.0 兼容实现（由 crate feature `embedded-hal-1` 开启）
// ---------------------------------------------------------------------------
//
// 做法对齐 embassy-stm32：trait impl 写在**类型所属的 crate 内部**，而不是让下游
// 去写 newtype 适配层。原因是孤儿规则——`embedded_hal::digital::OutputPin` 是外部
// trait，`crate::gpio::Output` 对下游 crate 也是外部类型，下游无法直接 impl。
// 写在这里之后，`embedded-hal` 生态的第三方驱动就能直接接收本 HAL 的
// [`Output`] / [`Input`] / [`Flex`] / [`OutputOpenDrain`]，**零包装开销**。
//
// GPIO 写电平不会失败，故 `Error` 统一取 [`core::convert::Infallible`]
// （`embedded-hal` 已为它提供 `digital::Error` 实现）。
#[cfg(feature = "embedded-hal-1")]
mod embedded_hal_impls {
    use super::{Flex, Input, Output, OutputOpenDrain};
    use core::convert::Infallible;
    use embedded_hal_1::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

    // ---- Input：只读输入 ----

    impl<'d> ErrorType for Input<'d> {
        type Error = Infallible;
    }

    impl<'d> InputPin for Input<'d> {
        #[inline]
        fn is_high(&mut self) -> Result<bool, Self::Error> {
            Ok(Input::is_high(self))
        }

        #[inline]
        fn is_low(&mut self) -> Result<bool, Self::Error> {
            Ok(Input::is_low(self))
        }
    }

    // ---- Output：推挽输出 ----

    impl<'d> ErrorType for Output<'d> {
        type Error = Infallible;
    }

    impl<'d> OutputPin for Output<'d> {
        #[inline]
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Output::set_high(self);
            Ok(())
        }

        #[inline]
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Output::set_low(self);
            Ok(())
        }
    }

    impl<'d> StatefulOutputPin for Output<'d> {
        #[inline]
        fn is_set_high(&mut self) -> Result<bool, Self::Error> {
            Ok(Output::is_set_high(self))
        }

        #[inline]
        fn is_set_low(&mut self) -> Result<bool, Self::Error> {
            Ok(Output::is_set_low(self))
        }

        #[inline]
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Output::toggle(self);
            Ok(())
        }
    }

    // ---- Flex：可自由切换方向的引脚，三套 trait 都能给 ----

    impl<'d> ErrorType for Flex<'d> {
        type Error = Infallible;
    }

    impl<'d> OutputPin for Flex<'d> {
        #[inline]
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Flex::set_high(self);
            Ok(())
        }

        #[inline]
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Flex::set_low(self);
            Ok(())
        }
    }

    impl<'d> StatefulOutputPin for Flex<'d> {
        #[inline]
        fn is_set_high(&mut self) -> Result<bool, Self::Error> {
            Ok(Flex::is_set_high(self))
        }

        #[inline]
        fn is_set_low(&mut self) -> Result<bool, Self::Error> {
            Ok(Flex::is_set_low(self))
        }

        #[inline]
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Flex::toggle(self);
            Ok(())
        }
    }

    impl<'d> InputPin for Flex<'d> {
        #[inline]
        fn is_high(&mut self) -> Result<bool, Self::Error> {
            Ok(Flex::is_high(self))
        }

        #[inline]
        fn is_low(&mut self) -> Result<bool, Self::Error> {
            Ok(Flex::is_low(self))
        }
    }

    // ---- OutputOpenDrain：开漏输出，同时可回读线电平（I2C 式总线常用）----

    impl<'d> ErrorType for OutputOpenDrain<'d> {
        type Error = Infallible;
    }

    impl<'d> OutputPin for OutputOpenDrain<'d> {
        #[inline]
        fn set_high(&mut self) -> Result<(), Self::Error> {
            OutputOpenDrain::set_high(self);
            Ok(())
        }

        #[inline]
        fn set_low(&mut self) -> Result<(), Self::Error> {
            OutputOpenDrain::set_low(self);
            Ok(())
        }
    }

    impl<'d> StatefulOutputPin for OutputOpenDrain<'d> {
        /// 读 `ODR` 输出锁存（即"本驱动想要输出什么"），不是 `IDR` 线电平。
        #[inline]
        fn is_set_high(&mut self) -> Result<bool, Self::Error> {
            Ok(self.flex.is_set_high())
        }

        /// 读 `ODR` 输出锁存。见 [`Self::is_set_high`]。
        #[inline]
        fn is_set_low(&mut self) -> Result<bool, Self::Error> {
            Ok(self.flex.is_set_low())
        }

        #[inline]
        fn toggle(&mut self) -> Result<(), Self::Error> {
            OutputOpenDrain::toggle(self);
            Ok(())
        }
    }

    impl<'d> InputPin for OutputOpenDrain<'d> {
        /// 读 `IDR` **线电平**：开漏释放后若外部无上拉会读到低，属预期。
        #[inline]
        fn is_high(&mut self) -> Result<bool, Self::Error> {
            Ok(OutputOpenDrain::is_high(self))
        }

        /// 读 `IDR` 线电平。见 [`Self::is_high`]。
        #[inline]
        fn is_low(&mut self) -> Result<bool, Self::Error> {
            Ok(OutputOpenDrain::is_low(self))
        }
    }
}
