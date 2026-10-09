//! EXTI 寄存器级操作（对齐 embassy-stm32 的 `exti/low_level.rs`）。
//!
//! 只做"对单条线的一次确定读写"，不含调用策略；寄存器分组与出处见 [`super`] 的模块文档。
//! 所有函数都只接受 **GPIO 线 0–15**（`pin` 既是引脚号也是线号），与 embassy-stm32 的
//! `configure_exti_pin` / `set_exti_interrupt_enabled` / `clear_exti_pending` /
//! `is_exti_pending` 一一对应。

use crate::pac;

/// 边沿触发方式（RM 13.3.3 `EXTI_RTSR` / 13.3.4 `EXTI_FTSR`）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TriggerEdge {
    /// 仅下降沿（高 → 低）。
    Falling,
    /// 仅上升沿（低 → 高）。
    Rising,
    /// 双边沿。
    Any,
}

/// 中断屏蔽状态（`EXTI_IMR1` 的单个位）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InterruptState {
    /// 使能（该线可以发出中断请求）。
    Enabled,
    /// 关闭。
    Disabled,
}

impl From<InterruptState> for bool {
    #[inline]
    fn from(state: InterruptState) -> bool {
        matches!(state, InterruptState::Enabled)
    }
}

/// 取 EXTI 寄存器块（基址 `0x4001_0400`，PAC [`pac::Exti`]）。
///
/// # Safety（实现说明）
///
/// 指针来自 PAC 的基址常量。"只有一个实例在操作这些寄存器"由类型系统保证：
/// 每条 EXTI 线只能被一个 [`super::Channel`] 句柄持有，而 [`super::ExtiInput`] 必须
/// 消费该句柄才能构造。与 `gpio.rs` / `rcc.rs` 取寄存器块的做法一致。
#[inline]
fn regs() -> &'static pac::exti::RegisterBlock {
    unsafe { &*pac::Exti::PTR }
}

/// 取 SYSCFG 寄存器块（基址 `0x4001_0000`，PAC [`pac::Syscfg`]）。
///
/// EXTI 的 GPIO 端口选择器（`SYS_EXTICR1–4`）在 SYSCFG 里，不在 EXTI 里（RM 10.2.6）。
#[inline]
fn syscfg() -> &'static pac::syscfg::RegisterBlock {
    unsafe { &*pac::Syscfg::PTR }
}

/// 把 `bits` 的第 `pin` 位置成 `set`，其余位原样保留。
#[inline]
fn with_bit(bits: u32, pin: u8, set: bool) -> u32 {
    let mask = 1u32 << pin;
    if set { bits | mask } else { bits & !mask }
}

/// 配置某条 GPIO 线的 EXTI 输入源与触发沿（对齐 embassy-stm32 的 `configure_exti_pin`）。
///
/// * `pin` — 引脚号（0–15），同时就是 EXTI 线号；
/// * `port` — 端口号（0 = GPIOA … 5 = GPIOF），即 [`crate::gpio::Pin::port`] 的取值；
/// * `trigger_edge` — 触发沿。
///
/// 寄存器顺序依据 `HAL_GPIO_Init` 的 EXTI 段（`py32e407_hal_gpio.c`）与 RM：
/// ① 使能 SYSCFG 时钟 → ② `SYSCFG_EXTICR1–4` 选端口（RM 10.2.6–10.2.9）→
/// ③ `EXTI_RTSR`/`EXTI_FTSR` 选边沿（RM 13.3.3 / 13.3.4）→ ④ 清 `PR`。
///
/// `IMRx`/`EMRx` 不在此处碰，那是 [`set_exti_interrupt_enabled`] 的职责——这个拆分
/// 与 embassy-stm32 一致。
///
/// 第 ④ 步是本驱动与 embassy-stm32 都比 HAL 多做的：`HAL_GPIO_Init` 不清 `PR`，
/// 但改完边沿极性后残留的挂起位会让 `IMR` 一使能就立刻发一次伪请求。
pub(super) fn configure_exti_pin(pin: u8, port: u8, trigger_edge: TriggerEdge) {
    debug_assert!(pin < 16, "only GPIO lines 0..=15 can select a port");
    debug_assert!(port < 6, "port index must be 0..=5 (GPIOA..GPIOF)");

    // ① RM 13.2.4 注："通过 SYSCFG_EXTICRx 配置 GPIO 线上的外部中断/事件，必须先使能
    //    SYSCFG 时钟"——时钟关闭时写 EXTICR 静默失效。
    //    `rcc::init` 已置过一次（SYSCFGEN = APB2ENR.bit0，见 rcc.rs），这里再断言一次，
    //    让 EXTI 驱动不依赖调用方是否走过 `init`；置位幂等，代价是一次读-改-写 + 读回。
    crate::rcc::enable_apb2(1 << 0);

    // ② 每线一个 4-bit 半字节：EXTI0 → bit0:3 … EXTI3 → bit12:15（即偏移 (pin % 4) * 4）；
    //    寄存器号 pin / 4 → EXTICR1–4（偏移 0x14/0x18/0x1C/0x20）。
    //    半字节编码 0 = PA、1 = PB、…、5 = PF（RM 10.2.6），与 `Pin::port()` 的取值一致。
    let shift = (pin % 4) * 4;
    let mask = 0xF_u32 << shift;
    let value = (port as u32) << shift;
    let write = |cur: u32| (cur & !mask) | value;
    let s = syscfg();
    match pin / 4 {
        0 => s.exticr1().modify(|cur, w| unsafe { w.bits(write(cur.bits())) }),
        1 => s.exticr2().modify(|cur, w| unsafe { w.bits(write(cur.bits())) }),
        2 => s.exticr3().modify(|cur, w| unsafe { w.bits(write(cur.bits())) }),
        _ => s.exticr4().modify(|cur, w| unsafe { w.bits(write(cur.bits())) }),
    };

    // ③ RTSR/FTSR 各一次读-改-写（HAL 与 embassy-stm32 都是两次独立写入）。
    let (rising, falling) = match trigger_edge {
        TriggerEdge::Falling => (false, true),
        TriggerEdge::Rising => (true, false),
        TriggerEdge::Any => (true, true),
    };
    let r = regs();
    r.rtsr()
        .modify(|cur, w| unsafe { w.bits(with_bit(cur.bits(), pin, rising)) });
    r.ftsr()
        .modify(|cur, w| unsafe { w.bits(with_bit(cur.bits(), pin, falling)) });

    // ④
    clear_exti_pending(pin);
}

/// 使能 / 关闭某条线的中断请求（对齐 embassy-stm32 的 `set_exti_interrupt_enabled`）。
///
/// 只写 `EXTI_IMR1`（RM 13.3.1）。注意 `IMR` 决定的是"要不要向 NVIC 发请求"，
/// **不影响 `PR` 是否被边沿置起**（RM 13.2.3），所以关掉 `IMR` 后仍可用
/// [`is_exti_pending`] 轮询。
///
/// 本库不注册中断向量、不动 NVIC：置位 `IMR` 后要真正进中断，还得用户在向量表里
/// 实现对应的 `EXTIx` / `EXTI9_5` / `EXTI15_10` 并 `NVIC::unmask`（见 [`super`] 模块文档）。
///
/// 这里不套临界区：与 `gpio.rs` 的理由相同——每条 EXTI 线在同一时刻只有一个
/// [`super::ExtiInput`] 持有者（[`super::Channel`] 句柄是单例），故读-改-写不会与
/// 同一位上的另一个驱动竞争（本工程也未引入 `critical-section` 依赖）。
pub(super) fn set_exti_interrupt_enabled(pin: u8, state: InterruptState) {
    debug_assert!(pin < 16, "only GPIO lines 0..=15");
    let set: bool = state.into();
    regs()
        .imr1()
        .modify(|cur, w| unsafe { w.bits(with_bit(cur.bits(), pin, set)) });
}

/// 按掩码清 `PR`（对齐 embassy-stm32 的 `clear_exti_pending_mask`）。
///
/// 供 [`super::blocking::ExtiGroupMask`] 在共享中断向量里一次清掉多条线用。
///
/// `PR` 是 RC_W1（RM 13.3.6）：整寄存器写 `mask`，只有 mask 里为 1 的位被清，
/// 写 0 的位无任何副作用。SVD 未给这些位标 `oneToClear`，故 PAC 只生成了 `BitWriter`
/// 而没有 `clear_bit_by_one()`，只能这样整写——语义与 `HAL_EXTI_ClearPending` 的
/// `EXTI->PR = mask` 完全相同。
pub(super) fn clear_exti_pending_mask(mask: u32) {
    regs().pr().write(|w| unsafe { w.bits(mask) });
}

/// 清某条线的挂起位（对齐 embassy-stm32 的 `clear_exti_pending`）。
pub(super) fn clear_exti_pending(pin: u8) {
    debug_assert!(pin < 16, "only GPIO lines 0..=15");
    clear_exti_pending_mask(1 << pin);
}

/// 读某条线的挂起位（对齐 embassy-stm32 的 `is_exti_pending`）。
///
/// 边沿由硬件置位 `PR`，与 `IMR` 无关；本函数是同步驱动"查询一次边沿"的入口。
pub(super) fn is_exti_pending(pin: u8) -> bool {
    debug_assert!(pin < 16, "only GPIO lines 0..=15");
    regs().pr().read().bits() & (1 << pin) != 0
}
