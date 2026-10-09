# 项目长期笔记（embassy-for-py32e407）

> 只放**跨会话仍然成立**的事实与约定；逐次工作过程记在同目录的 `YYYY-MM-DD.md`。

## 工作区结构

| 目录 | 用途 |
|---|---|
| `py32e407cbind-rs/` | PUYA 官方 **C LL 驱动**的 Rust FFI 绑定（cc 编 `.a` + bindgen 生成绑定） |
| `cbind-demo/` | 直接用 cbind-rs 的独立 bin 示例（**不依赖 `py32e407-pac`**） |
| `py32e407hal-rs/` | PUYA 官方 **C HAL 驱动**的 Rust FFI 绑定（cc 编 `.a` + Rust 手写 extern 声明） |
| `hal-demo/` | 用 HAL 状态机封装 + LL 轮询绕开 ISR 的示例（**必须同时依赖 cbind-rs**） |
| `embassy-py32/` | 手写的安全 Rust HAL（另一条路线） |
| `demo-py32/` | 基于 embassy-py32 的固件工程；`PY32E4_Series.yaml`（probe-rs 芯片描述）也在这里 |
| `py32e407-pac/` | svd2rust 生成的 PAC（cbind-demo 已不用） |
| `temp_pac_demo/` | 仅用 PAC（不开 critical-section feature）做 GPIO + RCC，最小翻转 PC3 示例 |
| `参考设计/` | 官方固件包 V0.7.0（**一手资料**）+ 参考手册 V0.6 / 数据手册 V0.13 |

## cbind-rs 的三层 + 两条「坑」

1. `.c` 里定义的函数 → 直接 FFI。
2. LL 头里的 `__STATIC_INLINE`（约 2058 个）→ bindgen `wrap_static_fns` 自动外部化成
   `<官方名>__ext`，编进 `libpy32e407_ll_ext.a`；**Rust 侧名字就是官方名**（靠 `#[link_name]`）。
3. `ll_consts.rs` 补回**被 bindgen 静默丢弃的常量**（带 `((uint32_t)…)` 的宏）。
   ⚠️ 子模块，顶部必须 `use crate::*;`；补的时候用设备头位宏，**不写魔数**。

- **坑 1**：`static inline` 完全不产生全局符号，bindgen 也**不会**为其生成声明 →
  只能靠 `wrap_static_fns`（手写 `extern "C"` 必然 undefined symbol）。
- **坑 2**：带强制转换的**宏常量**被整条丢弃，不报错。build.rs 现有对账警告。
  基线缺口 **199 个**（ADC / CORDIC / COMP 等，本项目不用；DMA 与 UART 已补齐）。
- 删掉 PAC 后必须 `use cortex_m as _;`（否则 `_critical_section_1_0_acquire` undefined）。

## py32e407-pac（svd2rust 0.37）API 易错点

- **字段名小写**：`Peripherals::take()/steal()` 的字段是 `rcc` / `gpioc` / `gpioa` …
  （不是 `RCC` / `GPIOC`）。编译器错误会建议改名。
- **`FieldWriter::bits()` 是 unsafe**：写多 bit 字段（MODER 2-bit、CFGR 4-bit 等）
  必须 `unsafe { w.field().bits(value) }`，svd2rust 0.37 把它标成 unsafe 是因为无法
  静态保证写入值合法（无对应枚举）。`BitWriter::set_bit()` / `clear_bit()` **不是** unsafe。
- **不开 `critical-section` feature 也能用**：`Peripherals::take()` 是
  `#[cfg(feature = "critical-section")]`。启动期只取一次就改用
  `unsafe { Peripherals::steal() }`，可以完全避开 `cortex-m` 的
  `critical-section-single-core` 依赖 —— 这就是 `temp_pac_demo` 的做法。
- **GPIO 时钟 = AHB2ENR**：字段名是 `iopcen` / `iopaen` / `iopben` …
  （位序从 bit2 起，**不是** STM32 风格的 bit0）。svd2rust 已逐位拆字段，
  直接 `.set_bit()` 即可，**别手写掩码**。
- **PC3 原子翻转 = BSRR**：`bs3()` 写低 16 位 = 拉高；`br3()` 写高 16 位 = 拉低。
  比 ODR 读-改-写安全（不会被 ISR 抢断）。

## embassy 在 PAC-only 工程上的最小骨架（已实测可 build）

- **cargo deps**（已实测通过 build + 烧录量 3 228 B）：
  - `embassy-executor = { version = "0.10", features = ["platform-cortex-m", "executor-thread"] }`
  - `embassy-time = { version = "0.5", features = ["tick-hz-1_000", "generic-queue-8"] }`
  - `embassy-time-driver = "0.2"` + `embassy-time-queue-utils = "0.3"` + `heapless = "0.8"`
  - `cortex-m = { version = "0.7", features = ["critical-section-single-core"] }`
  - `critical-section = "1.1"` —— **必须显式列**（即使它是传递依赖，`use critical_section::...` 也拿不到）
  - PAC 不开 `critical-section` feature（继续用 `steal()`）
- **main 入口**：`#[embassy_executor::main] async fn main(spawner: Spawner) { ... }`
  - 不要 `#[entry]`，宏展开时会自动加 `#[cortex_m_rt::entry]`
  - 函数体做初始化 + spawn 子任务 + return `()`；**不能**写 `loop {}`，否则 executor 不派工
- **子任务**：`#[embassy_executor::task] async fn task() { ... }` 默认 pool_size=1
  - `spawner.spawn(task().unwrap())` —— `task()` 先返回 `Result<SpawnToken, SpawnError>`，
    `.unwrap()` 后再喂给 `spawner.spawn(token)`（**spawner.spawn 不返回 Result**）
- **time driver 必写**：cortex-m-rt 默认 SysTick handler 是个 spinloop —— 没 ISR 推进队列
  `Timer::after().await` 永远不醒。最小实现：`#[cortex_m_rt::exception] fn SysTick()` 推
  `TICK_COUNT` + 在 critical_section 里调 `queue.next_expiration(now)` 唤醒到期 waker
- **`AtomicU64` 在 thumbv7em-none-eabihf 上不可用**：Cortex-M4F 最大 32-bit atomic，
  `LDREXD/STREXD` 不通用。`TICK_COUNT` 用 `Mutex<RefCell<u64>>` 串行化读 / 写
- 完整示例见 `temp_pac_demo/`（main.rs + time_driver.rs）

## cbind-demo 的硬件约定（已固定）

| 信号 | 引脚 | 说明 |
|---|---|---|
| LED / 方波输出 | **PC3** | 推挽输出，1 ms 翻转 = 500 Hz |
| UART2_TX | **PC12** | AF5（与官方 PY32E407-STK 例程一致） |
| UART2_RX | **PD2** | AF5 |
| UART2_TX DMA | `DMA1` **CH1** | 请求号 1；M2P；目的端 HARD 握手 |
| UART2_RX DMA | `DMA1` **CH2** | 请求号 2；P2M；源端 HARD 握手 |

串口 115200 8N1。RX 为**定长块**（16 B 收满才回显）。

## py32e407hal-rs 路线选择

- **不走 bindgen**：HAL 头层层嵌套 + 宏开关选模块，bindgen/libclang 反复调
  include 顺序都失败。改用 Rust 侧**手写 extern 声明**（HAL 函数 ~200 个，组别清晰）。
- **Handle 必须按 C 布局暴露字段**：`#[repr(C)]` + 字段顺序与官方头一致 + `_trail: [u8; N]` 尾缓冲。
  HAL 内部按偏移读写 handle，Rust 改了字段顺序就全错。
- **必须配 cbind-rs**：HAL 的 .c 引用 LL_* 符号；hal-rs 的 Cargo.toml 直接列 `py32e407cbind-rs`。
- **HAL 中断状态机被绕过**：HAL 状态机靠 DMA/UART ISR 推进；本工程没有 NVIC 向量表，
  ISR 不会触发 → 用 **LL 轮询 DMA TC 标志**判断完成，再手动把 `huart.gState` 复位。
- **`HAL_GetTick` / `HAL_Delay` 必须自己提供**：hal-rs 提供 Rust 强符号实现；
  bin 在主循环 1ms 节拍里调 `hal::bump_tick()`。

## 本机环境

- 工具链：`thumbv7em-none-eabihf`；`arm-none-eabi-gcc`（GNU Arm Embedded，cargo 通过 rustup
  toolchain 找到，**但不在 PATH 上** —— `where arm-none-eabi-size` 空）+ `libclang`
  （`LIBCLANG_PATH` 指向 esp 工具链）。
- 烧录：probe-rs（`--chip PY32E407xE`，芯片描述用 `demo-py32/PY32E4_Series.yaml`）。CLI 不接探针。
- **Rust binutils 全套已预装在 `~/.cargo/bin/`**（cargo-binutils）：
  `cargo-size` / `cargo-bloat` / `cargo-objcopy` / `cargo-objdump` / `cargo-strip` / `rust-size` 等。
  **看代码大小用 `cargo size --release --target thumbv7em-none-eabihf`**；
  看每个 section 加 `-- -A`。`arm-none-eabi-size` 没装，用 `rust-size` 替。
  `probe-rs 0.31.0` 没有 `size` 子命令，别试 `probe-rs size`。
- **Git Bash 缺 coreutils**：`ls/grep/tail/head/find/rm` 全不可用，**命令不要加管道**；
  列文件用 Glob/Read/Grep，查符号用 `rust-nm` / `rust-objdump`（在 `~/.cargo/bin/`）；
  PowerShell 输出不回显；`cmd //c` 内建命令输出会丢。
  文件操作：`rm`/`mkdir`/`ls`/`cat`/`grep` 用
  `/c/Users/ouyan/.workbuddy/binaries/PortableGit/versions/1.2.0/usr/bin/` 下的 Git 工具绝对路径调用。

## 硬件事实速查（易错）

- `UART` 与 `USART` 是**两套独立外设**：`UART.DR` @**0x00**、`USART.DR` @**0x04**。
- PY32 的 **UART 没有使能位**（无 `LL_UART_Enable`）。
- `ErrorStatus`：**`SUCCESS = 0`**（与经典 STM32 相反）。
- `PWR` 挂 **APB1**（`APB1ENR1.PWREN`）；`PLLMul` 是**原始倍频数 2~257**（硬件写 `-2`）。
- GPIO 挂 **AHB2 且位序从 bit2 起**（`IOPCEN = bit4`）。
- DMA 是「通道 → 请求号 → 外设」两段式映射：`Set*PeriphMap` + `SYSCFG_SetDMARemap`
  必须成对，且**先开 SYSCFG 时钟**（`APB2ENR`）。
