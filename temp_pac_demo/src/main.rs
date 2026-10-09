//! # temp_pac_demo —— 让 embassy-executor 接管 PAC-only 工程的启动流程
//!
//! 这是 embassy-for-py32e407 仓库里**第四条**「翻转 PC3」的路线，与已有示例对比：
//!
//! | 工程 | 外设操作方式 | 运行期 / 调度 | 关键依赖 |
//! |---|---|---|---|
//! | `demo-py32` | `embassy_py32::gpio::Output`（手写 HAL） | **同步阻塞** | embassy-py32 + PAC + cortex-m |
//! | `cbind-demo` | `py32e407cbind_rs::LL_GPIO_*`（C LL FFI） | 同步阻塞 | py32e407cbind-rs + cortex-m |
//! | `temp_pac_demo` | `Peripherals::steal()` + 直读寄存器 | **同步忙等** | PAC + cortex-m-rt |
//! | **`temp_pac_demo` (本版)** | PAC + 直读寄存器 | **embassy-executor 异步** | PAC + embassy-executor + embassy-time |
//!
//! ## 与旧版的差异
//!
//! * `#[entry] fn main() -> ! { loop { ... } }` → `#[embassy_executor::main] async fn main(spawner: Spawner)`
//!   * embassy 的 main macro 会自动在生成的 `main()` 上加 `#[cortex_m_rt::entry]`，
//!     创建 thread-mode `Executor`，把我们的 async body 当作**第一个 task** spawn 进去；
//!     之后 `Executor::run` 进入 WFE 循环，由 SEV / SysTick 唤醒后再 poll 任务。
//! * 翻转 PC3 的循环挪进一个独立的 `#[embassy_executor::task]`，从 `spawner` 里 spawn。
//!   * `Timer::after(Duration::from_millis(500)).await` 真正让出 CPU（`WFI/WFE`），
//!     由 SysTick 中断到期后唤醒。不再是 spin-loop 占用全部核时。
//! * 复用 PAC：**不**引入 `embassy-py32` HAL。GPIO/RCC 还是直读寄存器。
//!
//! ## 时钟与计时
//!
//! * HCLK = HSI16 = 16 MHz（复位默认）；不分频、不开 PLL。
//! * SysTick = 16_000 cycle / 中断 = **1 kHz**。
//! * `TICK_COUNT` 软计数 + 8 槽 [`embassy_time_queue_utils::Queue`] 当软定时器；
//!   实现见 [`time_driver`]。
//!
//! ## 启动流程图
//!
//! ```text
//! Reset (cortex_m_rt::Reset)
//!   └─> __embassy_main(spawner)        ← 我们的 main，被 executor 自动 spawn 成 task #0
//!         ├ Peripherals::steal()        ← 不开 critical-section feature
//!         ├ 开 GPIOC 时钟 / 配 PC3 输出
//!         ├ time_driver::init()         ← 起 SysTick @ 1 kHz
//!         ├ spawner.spawn(blink())      ← 排上 task #1
//!         └ return  ()                  ← body 结束
//!       Executor::run
//!         loop { poll(); wfe(); }        ← WFE 等 SysTick 或 SEV 唤醒
//!             ├ SysTick ISR 唤
//!             │   ├─ TICK_COUNT += 1
//!             │   └─ Queue::next_expiration(now) 唤醒到期 waker
//!             └ SEV 唤（其他任务 wake 另一个任务时）
//! ```

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use py32e407_pac::Peripherals;

mod time_driver;

/// 主任务：初始化外设和 embassy-time 后，spawn blink 任务，然后返回。
///
/// 注意：embassy 的 `main` macro 会把我们整个 async body **自动包装成第一个 task**
/// (`__embassy_main_task`)，由生成的 `fn main()` spawn 进 thread-mode Executor。
/// 所以这里不能写 `loop {}` —— 写 `loop` 会让这个任务永远不返回，Executor 也无从
/// 调度其它任务。正确的写法是「做初始化、spawn 子任务、返回」。
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // 用 `steal()` 而不是 `take()`：PAC 没开 `critical-section` feature，`take()`
    // 不会被编译进来；启动期也只取一次，不需要互斥。
    let dp = unsafe { Peripherals::steal() };

    // ---- 1. 给 GPIOC 开时钟 ----
    //
    // PY32 的 GPIO 挂在 AHB2，位序从 bit2 起（`IOPCEN = bit4`）。
    // svd2rust 已经把每一位单独做成 PAC 字段，直接 `.set_bit()` 即可。
    dp.rcc.ahb2enr().modify(|_, w| w.iopcen().set_bit());

    // ---- 2. PC3 配成通用推挽输出 ----
    //
    // MODER 每个引脚占 2 bit；PC3 对应字段 `moder3`（bits 6:7）。
    // `bits()` 在 svd2rust 0.37 是 unsafe（要调用者保证写入值合法）。
    dp.gpioc.moder().modify(|_, w| unsafe { w.moder3().bits(0b01) });

    // ---- 3. 启动 embassy-time 的 SysTick 驱动 ----
    //
    // 必须在任何 `Timer::after(...).await` 之前；之后 `Instant::now()` 才有意义。
    time_driver::init();

    // ---- 4. 把 blink 任务 spawn 到当前 Executor ----
    //
    // `blink()` 本身返回 `Result<SpawnToken, SpawnError>`（检查任务池是否还有空位，
    // 默认 `#[task]` 的 pool_size=1，已经 spawn 过一次就会冲突）；
    // `spawner.spawn` 接收 `SpawnToken` 并真正排队（**不**返回 Result）。
    spawner.spawn(blink().unwrap());
}

/// 后台 blink 任务：通过 BSRR 原子翻转 PC3，高/低各 500 ms → 1 Hz 方波。
///
/// 注意几点：
/// * `async fn` 必须返回 `()` 或 `!`；这里无限循环、永远不返回，等价于 `!`。
/// * `Timer::after(...).await` 真正让出 CPU：Executor 进入 WFE 等待 SysTick
///   ISR 在 500 ms 后唤醒；不会被 spin-loop 抢占。
/// * 不能在这个任务里调任何会让 Executor 退出的东西（比如 `loop {}` 在 main 里）。
#[embassy_executor::task]
async fn blink() {
    let dp = unsafe { Peripherals::steal() };
    loop {
        // 拉高
        dp.gpioc.bsrr().write(|w| w.bs3().set_bit());
        Timer::after(Duration::from_millis(500)).await;

        // 拉低
        dp.gpioc.bsrr().write(|w| w.br3().set_bit());
        Timer::after(Duration::from_millis(500)).await;
    }
}

/// 默认 ISR：所有未注册向量都走这里，原地挂死等调试器。
///
/// 与上一版的 `cortex_m_rt::exception unsafe fn DefaultHandler(_irqn: i16)` 一致；
/// SysTick 已经在 `time_driver` 里被覆盖，这里只兜底其它向量。
#[cortex_m_rt::exception]
unsafe fn DefaultHandler(_irqn: i16) {
    loop {
        cortex_m::asm::nop();
    }
}

/// 极简 panic 处理：原地挂死，等调试器接管。
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::nop();
    }
}