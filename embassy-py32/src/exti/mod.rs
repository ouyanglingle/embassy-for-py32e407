//! External Interrupts (EXTI) —— 外部扩展中断/事件控制器（同步 Blocking 驱动）
//!
//! 结构对齐 `embassy-stm32-0.6.0/src/exti/`：本文件是 [`ExtiInput`] 与 [`Channel`]，
//! 寄存器操作在 `low_level`，手工中断处理用的辅助类型在 [`blocking`]。
//!
//! 寄存器流程依据：
//! - HAL C: `py32e407_hal_exti.c`（`HAL_EXTI_SetConfigLine` / `GetPending` /
//!   `ClearPending` / `GenerateSWI` / `IRQHandler`）与 `py32e407_hal_gpio.c` 的
//!   `HAL_GPIO_Init` EXTI 模式段（SYSCFG 时钟 → EXTICR → IMR1/EMR1 → RTSR/FTSR）
//! - LL: `py32e407_ll_bus.h`（`LL_APB2_GRP1_PERIPH_SYSCFG`；EXTI **无**时钟门控位）
//! - RM: PY32E407 参考手册 V0.6 第 13 章（13.2 EXTI / 13.3 寄存器）、
//!   第 10 章 10.2.6–10.2.9（`SYS_EXTICR1–4`）、13.1.3（中断向量表）
//! - SVD/PAC: `py32e407-pac/src/exti/`、`src/syscfg/exticr{1,2,3,4}.rs`
//!
//! # 本芯片 EXTI 的关键事实（RM + HAL + PAC 交叉验证）
//!
//! - **线号 0–34**（RM 表 13-1）：0–15 是 GPIO 线（用 `SYSCFG_EXTICR` 选端口），
//!   16–21 是 PVD / RTC 闹钟 / COMP1–4，22–34 是 I2C/LPTIM/ETH/LPUART/USB 唤醒线。
//!   其中 0–21 是**可配置线**（有边沿选择器、挂起位、独立屏蔽位），22–34 是**直接输入线**
//!   （固定上升沿、无 `PR`，RM 13.2.1）。本驱动只覆盖 **GPIO 线 0–15**——与
//!   embassy-stm32 的覆盖范围一致（它同样只处理 0–15）。
//! - 寄存器分两组：`IMR1`/`EMR1` 管 0–31，`IMR2`/`EMR2` 管 32–34（只用 bit0–2）；
//!   而 `RTSR`/`FTSR`/`SWIER`/`PR` **只覆盖 0–21**（22 位有效，没有 `PR2`）。
//!   GPIO 线（0–15）全部落在第一组，故本驱动只碰 `IMR1`。
//! - **GPIO 端口选择在 SYSCFG，不在 EXTI**：`SYSCFG_EXTICR1–4`（偏移 0x14/0x18/0x1C/0x20）
//!   每线一个 4-bit 半字节，编码 0 = PA、1 = PB、2 = PC、3 = PD、4 = PE、5 = PF（RM 10.2.6）。
//!   RM 13.2.4 明确注："配置 GPIO 线上的外部中断/事件，**必须先使能 SYSCFG 时钟**"。
//!   [`crate::rcc::init`] 已置位过该位，驱动在写 EXTICR 前再断言一次（幂等）。
//! - **`IMR1` 复位值 `0xC3C0_0000`**（RM 13.3.1）：线 22–25（四路 I2C 唤醒）与 30/31
//!   （LPTIM1/ETH）复位就开着。驱动只动目标位、其余保持，不会误关这些唤醒线。
//! - **边沿置起 `PR` 与 `IMR` 无关**：`IMR` 只决定要不要向 NVIC 发出请求（RM 13.2.3）。
//!   `PR` 是 RC_W1（写 1 清除，RM 13.3.6）。
//! - EXTI 无独立时钟使能位（`py32e407_ll_bus.h` 里没有 `EXTIEN`，与 STM32F4 一致）。
//!
//! # 与 STM32 的差异（别照搬 ST 的习惯）
//!
//! - 中断向量**不是** STM32F4 的 `EXTI0_1`/`EXTI2_3`/`EXTI4_15`，而是 `EXTI0`(IRQ 6)、
//!   `EXTI1`(7)、`EXTI2`(8)、`EXTI3`(9)、`EXTI4`(10)、`EXTI9_5`(23)、`EXTI15_10`(40)
//!   （RM 13.1.3）。
//! - `IMR2`/`EMR2` 只到线 34，且 `EXTICR` 在 SYSCFG 里（EXTI 侧无 `PR2`/`RTSR2`）。
//!
//! # 相对 embassy-stm32 的同步裁剪
//!
//! **已对齐**：`ExtiInput`（`new_blocking` / 电平读取 / `set_edge_detection` /
//! `enable_interrupt` / `disable_interrupt` / `clear_pending` / `is_pending` /
//! 私有 `pin_mask`）、`Channel` + `impl_exti!`、`blocking::ExtiGroupMask`、
//! `low_level` 的函数划分与命名、`TriggerEdge` / `InterruptState` 的再导出位置、
//! `gpio::ExtiPin`（`type ExtiChannel`）、embedded-hal 1.0 的 `digital::InputPin`。
//!
//! **未实现**（属 Async 面，或依赖本工程尚未有的中断类型层）：
//!
//! | embassy-stm32 的项 | 本工程的处理 |
//! |---|---|
//! | `ExtiInput<'d, Mode: PeriMode>` 的 `Mode` 泛型与 `Async` / `Blocking` 标记 | 去掉：本库是纯同步库，`uart`/`i2c`/`timer`/`esmc`/`gpio` 也都没有 mode 泛型。本工程的 `ExtiInput<'d>` 等价于 embassy-stm32 的 `ExtiInput<'d, Blocking>` |
//! | `ExtiInput<Async>` 及 `wait_for_rising_edge` / `wait_for_any_edge` 等 waker 等待 | 待 Async 扩展 |
//! | `from_input` / `from_flex` | 只存在于 embassy-stm32 的 `ExtiInput<Async>`（签名多一个 `_irq: impl Binding`），属 async 面，随之一起推迟 |
//! | `ExtiInputFuture` | 同上 |
//! | `Channel::irq()` / `type IRQ: InterruptType`、`InterruptHandler`、`foreach_exti_irq!`、`init()` | 依赖 `crate::interrupt::typelevel`（本库尚无中断类型层）。**中断向量由用户自行实现**，见下 |
//! | `cpu_regs()` 与 `on_irq()` 里"掩蔽已触发线"的处理 | 属异步 IRQ 处理器的实现细节，随上一行一起推迟 |
//! | `EXTI_COUNT` / `EXTI_WAKERS` | waker 表，属 Async |
//! | `embedded_hal_02::digital::v2::InputPin` | 本工程未依赖 embedded-hal 0.2 |
//! | `critical_section::with(...)` 包住读-改-写 | 本工程未引入 `critical-section`；依据同 `gpio.rs`——每条线只有一个持有者（详见 `low_level::set_exti_interrupt_enabled` 的说明） |
//!
//! # 使用示例（轮询）
//!
//! ```ignore
//! use embassy_py32::exti::{ExtiInput, TriggerEdge};
//! use embassy_py32::gpio::Pull;
//! use embassy_py32::init;
//!
//! let p = init(Default::default());
//! // PD10 上升沿；`p.EXTI10` 是这条线的所有权句柄（`PD10`/`PE10` 同为 EXTI10，只能取一个）
//! let mut key = ExtiInput::new_blocking(p.PD10, p.EXTI10, Pull::None, TriggerEdge::Rising);
//!
//! if key.is_pending() {
//!     key.clear_pending();   // 不清掉 `PR`，下一次边沿会被漏掉
//!     // ... 处理一次上升沿
//! }
//! ```
//!
//! # 使用示例（中断：向量由用户实现）
//!
//! ```ignore
//! #[interrupt]
//! fn EXTI15_10() {                  // PD10 → EXTI10 → 向量 EXTI15_10（RM 13.1.3）
//!     let exti = unsafe { &*embassy_py32::pac::Exti::PTR };
//!     if exti.pr().read().bits() & (1 << 10) != 0 {
//!         exti.pr().write(|w| unsafe { w.bits(1 << 10) });   // 清挂起
//!         // ... 处理
//!     }
//! }
//! // 初始化后：NVIC::unmask(pac::Interrupt::EXTI15_10)
//! ```

mod low_level;

pub use low_level::{InterruptState, TriggerEdge};

pub mod blocking;

use crate::gpio::{ExtiPin, Input, Level, Pin as GpioPin, Pull};
use crate::{Peri, PeripheralType};

/// EXTI 输入驱动（对齐 embassy-stm32 的 `ExtiInput`）。
///
/// 在 GPIO [`Input`] 外挂一层 EXTI：既能读电平，也能查/清边沿挂起位。
/// EXTI 不内建在 `Input` 里，因为它要占用对应的 EXTI 线——线是稀缺资源
/// （`PA5`/`PB5`/`PC5`… 都走 `EXTI5`，RM 13.2.4），必须显式交出一份所有权。
pub struct ExtiInput<'d> {
    pin: Input<'d>,
}

impl<'d> ExtiInput<'d> {
    /// 读引脚是否为高。
    #[inline]
    pub fn is_high(&self) -> bool {
        self.pin.is_high()
    }

    /// 读引脚是否为低。
    #[inline]
    pub fn is_low(&self) -> bool {
        self.pin.is_low()
    }

    /// 读引脚电平。
    #[inline]
    pub fn get_level(&self) -> Level {
        self.pin.get_level()
    }
}

/// Blocking 侧的方法（对应 embassy-stm32 的 `impl<'d> ExtiInput<'d, Blocking>`）。
///
/// 本库没有 `Mode` 泛型，故与上面那个"所有模式共有"的 impl 分成两块、只是为了让
/// 结构与 embassy-stm32 一一对应（将来补 `Async` 时，新方法加在单独一块里即可）。
impl<'d> ExtiInput<'d> {
    /// 创建 EXTI 输入（对齐 embassy-stm32 的 `ExtiInput::new_blocking`）。
    ///
    /// * `pin` — 目标引脚（同时决定端口与 EXTI 线号）；
    /// * `_ch` — 该引脚的 EXTI 通道句柄（`p.EXTI<线号>`），只用于所有权追踪；
    /// * `pull` — 内部上/下拉；
    /// * `trigger_edge` — 触发沿。
    ///
    /// 顺序照 embassy-stm32：先 `Input::new`（GPIO 模式），再
    /// `configure_exti_pin`（SYSCFG 时钟 → EXTICR → RTSR/FTSR → 清 `PR`），
    /// 最后 `set_exti_interrupt_enabled` 打开 `IMR1`。
    ///
    /// 注意：置了 `IMR` 不等于会进中断——还得用户在向量表里实现对应向量并
    /// `NVIC::unmask`；只做轮询时保持 `IMR` 开着也无副作用。
    pub fn new_blocking<T: GpioPin + ExtiPin>(
        pin: Peri<'d, T>,
        _ch: Peri<'d, T::ExtiChannel>,
        pull: Pull,
        trigger_edge: TriggerEdge,
    ) -> Self {
        let pin = Input::new(pin, pull);
        low_level::configure_exti_pin(pin.pin(), pin.port(), trigger_edge);
        low_level::set_exti_interrupt_enabled(pin.pin(), InterruptState::Enabled);
        Self { pin }
    }

    /// 重新设置边沿触发方式（`RTSR`/`FTSR`）。
    ///
    /// 会一并清掉挂起位（`configure_exti_pin` 的结尾动作）：改完极性后旧挂起位的
    /// 含义已经变了，留着会在 `IMR` 使能时立刻变成一次伪请求。
    pub fn set_edge_detection(&mut self, trigger_edge: TriggerEdge) {
        let pin = self.pin.pin();
        let port = self.pin.port();
        low_level::configure_exti_pin(pin, port, trigger_edge);
    }

    /// 使能该线的中断请求（`IMR1` 置位）。
    pub fn enable_interrupt(&mut self) {
        low_level::set_exti_interrupt_enabled(self.pin.pin(), InterruptState::Enabled);
    }

    /// 关闭该线的中断请求。
    pub fn disable_interrupt(&mut self) {
        low_level::set_exti_interrupt_enabled(self.pin.pin(), InterruptState::Disabled);
    }

    /// 清掉该线的挂起位（`PR`，写 1 清零）。
    ///
    /// 通常在处理完一次边沿后调用；不清会让下一次边沿被"已经置着"的挂起位掩盖。
    pub fn clear_pending(&mut self) {
        low_level::clear_exti_pending(self.pin.pin());
    }

    /// 该线是否有未处理的边沿挂起（读 `PR`）。
    ///
    /// 与 `IMR` 是否使能无关：`PR` 由硬件在边沿处置起，`IMR` 只影响是否向 NVIC 发请求。
    pub fn is_pending(&self) -> bool {
        low_level::is_exti_pending(self.pin.pin())
    }

    /// 本输入对应的 EXTI 线掩码（`1 << 线号`），供 [`blocking::ExtiGroupMask`] 用。
    fn pin_mask(&self) -> u32 {
        1 << self.pin.pin()
    }
}

// ---------------------------------------------------------------------------
// embedded-hal 1.0 兼容实现（由 crate feature `embedded-hal-1` 开启）
// ---------------------------------------------------------------------------
//
// 与 `gpio.rs` 同样的理由：trait 与类型对下游都是外部项，impl 只能写在本 crate。
// `ExtiInput` 就是一个"能读电平的输入脚"，故只给 `digital::InputPin`
// （`embedded-hal` 1.0 里没有"等待边沿"的同步 trait，那个在 `embedded-hal-async`）。
#[cfg(feature = "embedded-hal-1")]
mod embedded_hal_impls {
    use super::ExtiInput;
    use core::convert::Infallible;
    use embedded_hal_1::digital::{ErrorType, InputPin};

    impl<'d> ErrorType for ExtiInput<'d> {
        type Error = Infallible;
    }

    impl<'d> InputPin for ExtiInput<'d> {
        #[inline]
        fn is_high(&mut self) -> Result<bool, Self::Error> {
            Ok(ExtiInput::is_high(self))
        }

        #[inline]
        fn is_low(&mut self) -> Result<bool, Self::Error> {
            Ok(ExtiInput::is_low(self))
        }
    }
}

/// 密封 trait：EXTI 通道只能由本 crate 的 `EXTI0..EXTI15` 实现。
trait SealedChannel {}

/// EXTI 通道 trait（对齐 embassy-stm32 的 `exti::Channel`）。
///
/// 通道句柄的存在意义是**编译期所有权**：`PA5`/`PB5`/`PC5`… 共用同一条 EXTI 线，
/// 而同一时刻只能有一个 [`Peri`] 句柄指向 `EXTI5`，因此不可能把两个引脚
/// 同时挂到一条线上。
///
/// 相对 embassy-stm32 少了 `irq()` 与 `type IRQ: InterruptType`——那两个依赖
/// `crate::interrupt::typelevel`，本库尚未有中断类型层。
#[allow(private_bounds)]
pub trait Channel: PeripheralType + SealedChannel + Sized {
    /// EXTI 线号（`EXTI5` → 5），同时也是引脚号。
    fn number(&self) -> u8;
}

/// 为 `EXTI0..EXTI15` 实现 [`Channel`]（对齐 embassy-stm32 的 `impl_exti!`）。
macro_rules! impl_exti {
    ($type:ident, $number:expr) => {
        impl SealedChannel for crate::peripherals::$type {}
        impl Channel for crate::peripherals::$type {
            #[inline]
            fn number(&self) -> u8 {
                $number
            }
        }
    };
}

impl_exti!(EXTI0, 0);
impl_exti!(EXTI1, 1);
impl_exti!(EXTI2, 2);
impl_exti!(EXTI3, 3);
impl_exti!(EXTI4, 4);
impl_exti!(EXTI5, 5);
impl_exti!(EXTI6, 6);
impl_exti!(EXTI7, 7);
impl_exti!(EXTI8, 8);
impl_exti!(EXTI9, 9);
impl_exti!(EXTI10, 10);
impl_exti!(EXTI11, 11);
impl_exti!(EXTI12, 12);
impl_exti!(EXTI13, 13);
impl_exti!(EXTI14, 14);
impl_exti!(EXTI15, 15);
