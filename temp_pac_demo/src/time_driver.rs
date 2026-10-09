//! # embassy-time 驱动 —— SysTick 1 kHz
//!
//! embassy-time 的时间基准（`Instant::now()` / `Timer::after()`）由本模块提供。
//! 因为本工程只用 PAC，不引入 embassy-py32 HAL，所以 HAL 配套的 time driver 没有；
//! 自己写一个最小的 SysTick-based 驱动。
//!
//! ## 设计取舍
//!
//! * **周期模式**（不是 tickless）：SysTick 每 1 ms 触发一次，每 tick 自增计数 + 检查队列
//!   把到期的 waker 唤醒。实现简单、便于排查，代价是 jitter 最多 1 ms。
//! * **软队列**：用 [`embassy_time_queue_utils::Queue`]（底层 `heapless::Vec`，容量 8），
//!   每次 `Timer::after()` 把「唤醒时刻 + Waker」入队。SysTick 触发时调
//!   [`Queue::next_expiration`] 把过期的弹出并 `wake`。
//!
//! 想做到 tickless（按下一个到期时刻反向设置 SysTick 重载值）就要换 SysTick 的"一次性"
//! 模式，本示例先不上。

use core::cell::RefCell;
use core::task::Waker;

use cortex_m::peripheral::syst::SystClkSource;
use critical_section::Mutex;
use embassy_time_driver::{Driver, TICK_HZ};
use embassy_time_queue_utils::Queue;

/// 自启动以来的毫秒数（每 SysTick 触发 +1）。
///
/// 注意：**不能用 `AtomicU64`**。`thumbv7em-none-eabihf` 的目标特性里没有
/// 64-bit 原子的支持（Cortex-M4F 最大支持 32-bit atomic，LDREXD/STREXD 并不通用），
/// `core::sync::atomic::AtomicU64` 在这个 target 上根本没定义。
///
/// 用 `Mutex<RefCell<u64>>` 替代：写入（SysTick ISR）和读取（thread mode）
/// 都在 `critical_section::with` 里串行化，不会出现读撕裂。
static TICK_COUNT: Mutex<RefCell<u64>> = Mutex::new(RefCell::new(0));

/// 软定时器队列 + 自身的 [`Driver`] 实现。
///
/// `Mutex<RefCell<Queue>>` 是 embassy 官方示例的常见组合：
/// * `Mutex` 用 `critical_section::with` 锁（无原子操作）；
/// * `RefCell` 提供内部可变性，让 `&self` 方法能拿到 `&mut Queue`。
struct Py32TimeDriver {
    queue: Mutex<RefCell<Queue>>,
}

// 把 driver 注册成全局实例：macro 会展开成 `#[unsafe(no_mangle)]` 的两个
// `_embassy_time_now` / `_embassy_time_schedule_wake` 函数，embassy-time 内部
// 用 `extern "Rust"` 链到这里。
embassy_time_driver::time_driver_impl!(static DRIVER: Py32TimeDriver = Py32TimeDriver {
    queue: Mutex::new(RefCell::new(Queue::new())),
});

impl Driver for Py32TimeDriver {
    fn now(&self) -> u64 {
        critical_section::with(|cs| *TICK_COUNT.borrow(cs).borrow())
    }

    fn schedule_wake(&self, at: u64, waker: &Waker) {
        critical_section::with(|cs| {
            let mut q = self.queue.borrow(cs).borrow_mut();
            q.schedule_wake(at, waker);
        });
    }
}

/// SysTick 中断：自增 tick + 把队列里过期的 waker 唤醒。
///
/// 由 [`cortex_m_rt::exception`] 装到 `.vector_table` 的第 15 项（异常号 15 = SysTick）。
/// 这是 embassy-time 唯一的"硬"定时器 —— 软队列本身不产生中断。
#[cortex_m_rt::exception]
fn SysTick() {
    // 读-改-写要在临界区里串行化（避免和 `now()` 读端撕裂 8 字节 u64）。
    let now = critical_section::with(|cs| {
        let mut tc = TICK_COUNT.borrow(cs).borrow_mut();
        *tc = tc.wrapping_add(1);
        *tc
    });

    critical_section::with(|cs| {
        // `next_expiration(now)` 会把所有 `at <= now` 的条目弹出并 wake，
        // 同时返回下一个未到期的时间戳（本轮没用到，因为我们是周期扫描）。
        let _next_at = DRIVER.queue.borrow(cs).borrow_mut().next_expiration(now);
    });
}

/// 启动 SysTick，按 [`TICK_HZ`] 的频率触发 [`SysTick`]。
///
/// 必须在 `main` 里**早于任何 `Timer::after(...).await` 之前**调用一次；
/// 之后 `TICK_COUNT` 才会单调递增，`Instant::now()` 才有意义。
pub fn init() {
    assert_eq!(TICK_HZ, 1_000, "本驱动硬编码 1 kHz，请同步 embassy-time 的 tick-hz feature");

    // `Peripherals::steal()` 不动 PAC 的 critical-section feature；
    // 这里只是为了拿到 SYST 句柄。
    let p = unsafe { cortex_m::Peripherals::steal() };
    let mut syst = p.SYST;

    // 系统时钟：保持复位默认 HSI16 = 16 MHz。
    // 1 ms = 16_000 cycle。
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(16_000 - 1); // 倒数到 0 时触发 SysTick
    syst.clear_current();        // 清掉写在 reload 里的初值
    syst.enable_counter();       // 开计数
    syst.enable_interrupt();     // 允许触发 SysTick 中断（异常 15）
}