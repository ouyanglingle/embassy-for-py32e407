# py32e407cbind-rs

把 PUYA 官方 **C LL 驱动**中**全部 23 个外设**（`py32e407_ll_*.c`）编译为 Rust 可调用的库。

核心思路（对应 esp-idf-sys 的 “raw 绑定 = bindgen + cc” 分层）：

1. 在 `build.rs` 里用 [`cc`](https://crates.io/crates/cc) crate 调用 **arm-none-eabi-gcc**，把 C 源
   交叉编译成静态库 `libpy32e407_ll.a`（`cc::Build::compile` 会自动让 cargo 以 `static` 方式链接它）。
   **自动扫描** `Src/` 下所有 `py32e407_ll_*.c`，仅排除依赖 HAL 层的 `usb` / `sdmmc`。
2. 在 `build.rs` 里用 [`bindgen`](https://crates.io/crates/bindgen) 解析**动态生成**的
   `OUT_DIR/wrapper_gen.h`（自动 include 全部 LL 头，排除 usb/sdmmc），自动生成 Rust FFI 绑定
   `OUT_DIR/bindings.rs`，由 `src/lib.rs` 通过 `include!` 引入。

本 crate 是 `no_std` 裸机库，最终的内存布局/链接脚本由依赖它的 `bin` 工程（如 `demo-py32`）提供。

---

## ✅ 构建状态与产物核对（已验证）

`cargo build --target thumbv7em-none-eabihf` **通过**（约 23s，0 error；仅有 vendor 固件自身的
`-Wunused-parameter` C 警告）。

用 `arm-none-eabi-nm` 对 `libpy32e407_ll.a` 做**大小写敏感**匹配，核对产物一致性：

| 指标 | 数量 |
| --- | --- |
| `.a` 中**全局可链接**的 `LL_*` 符号（`T`） | **104** |
| `bindings.rs` 中生成的 `pub fn LL_*` 声明 | **104** |
| 有声明但无对应符号（悬空） | **0** |
| 有符号但无对应声明（遗漏） | **0** |
| 生成的 `pub const LL_*` 常量 | 2432 |
| 接入的 LL 外设模块 | 23（排除依赖 HAL 的 USB/SDMMC） |

即：对 C 驱动**对外暴露的全部可链接 API** 是 100% 覆盖，双向零误差。

### 已接入的 23 个 LL 外设

ADC、COMP、CORDIC、CRC、CTC、DAC、DMA、EXTI、GPIO、I2C（含 I2S）、LCDC、LPTIM、LPUART、OPA、
PWR、RCC、RNG、RTC、SPI、TIM、UART、USART、utils（含 FLASH / SYSTICK / PLL 相关）。

> 排除的 2 个：`py32e407_ll_usb.c`、`py32e407_ll_sdmmc.c` —— 其 LL 驱动依赖 HAL 层类型
> （`HAL_StatusTypeDef` / `HAL_Delay`），无法脱离 HAL 单独编译。

---

## 环境要求

1. **交叉编译器**：GNU Arm Embedded Toolchain，提供 `arm-none-eabi-gcc`。
   - 下载：<https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads>
   - 安装后确认 `arm-none-eabi-gcc --version` 可运行（建议加入 `PATH`）。
   - 若你的编译器名不同，可用环境变量覆盖：`PY32_CC=/path/to/your-gcc cargo build ...`
2. **libclang**：`bindgen` 运行期依赖。本机 `LIBCLANG_PATH` 已指向 esp 工具链的 `libclang.dll`
   （LLVM 22.x）。Windows 上也可另行安装 LLVM 官方包并设置 `LIBCLANG_PATH`。
3. **Rust 裸机目标**：
   ```bash
   rustup target add thumbv7em-none-eabihf
   ```

---

## 构建

```bash
cd py32e407cbind-rs
cargo build --target thumbv7em-none-eabihf
```

（`.cargo/config.toml` 已默认设好 `target`，直接 `cargo build` 也可，但需先 `rustup target add`。）

`build.rs` 会：
- 探测交叉编译器，缺失则给出清晰错误而非晦涩报错；
- **第 1 步**：以 `-mcpu=cortex-m4 -mthumb -mfloat-abi=hard -mfpu=fpv4-sp-d16 -ffreestanding` 编译 C，
  定义宏 `PY32E407xE`（触发外设定义）与 `USE_FULL_LL_DRIVER`（否则 LL 的 `.c` 整文件被 `#ifdef` 吃掉），
  产出 `libpy32e407_ll.a` 并自动 `static` 链接；
- **第 2 步**：用同一交叉编译器动态探测 newlib 的 `<stdint.h>` 目录，调用 `bindgen` 解析动态生成的
  `wrapper_gen.h`（自动 include 全部 LL 头，排除 usb/sdmmc），`use_core()` + `layout_tests(false)` 以适配
  `no_std`，allowlist `LL_.*` + `ErrorStatus`，生成 `bindings.rs` 到 `OUT_DIR`。

---

## 在 bin 工程中使用

在最终应用的 `Cargo.toml` 里：

```toml
[dependencies]
py32e407cbind-rs = { path = "../py32e407cbind-rs" }
py32e407-pac = { path = "../py32e407-pac" }
```

```rust
use py32e407cbind_rs::{
    LlRccClocks, LlGpioInit, GpioType, ErrorStatus,
    LL_RCC_GetSystemClocksFreq, LL_GPIO_Init, LL_GPIO_StructInit,
};

// 读系统时钟频率（硬件时钟已配置后）
let clocks = LlRccClocks::read();

// 初始化 GPIO：把 PAC 的 GPIO 外设指针强转为 *mut GpioType 传入
let mut cfg = LlGpioInit::zeroed();
unsafe { LL_GPIO_StructInit(&mut cfg); }
cfg.Pin = 1 << 5;
cfg.Mode = 1; // LL_GPIO_MODE_OUTPUT
unsafe { LL_GPIO_Init(gpioa as *mut GpioType, &mut cfg); }
```

> `gpioa` 来自 `py32e407-pac`（PAC 的 GPIO 寄存器块布局与 C `GPIO_TypeDef` 一致，可安全互转）。
> `LlRccClocks::read()` 与 `LlGpioInit::zeroed()` 是 crate 提供的便捷方法，底层仍是 `extern "C"` 调用。

---

## 扩展

**目前已自动接入全部 LL 外设，新增官方外设无需改任何代码** —— `build.rs` 会自动扫描
`Src/` 下所有 `py32e407_ll_*.c`，并动态生成包含全部 LL 头的 `wrapper_gen.h`。

只有在需要**剔除**某个外设时才改 `build.rs` 的 `HAL_DEPENDENT` 常量。

---

## 注意事项 / 坑

1. **入口冲突**：带 `main()` 的示例 `main.c`、以及 `system_py32e4xx.c` 中的 `SystemInit` 等，
   在 Rust 裸机项目里通常由 `cortex-m-rt` / embassy 接管。本 crate **不**编译这些，避免符号冲突。
   若确需 `SystemInit`/`SystemCoreClock`，请从 `Pack/KEIL/Drivers/CMSIS/...` 单独引入并处理好与
   Rust 启动流程的衔接。
2. **static inline 边界（重要）**：LL 驱动大量函数是头内 `__STATIC_INLINE`（`static inline`），
   在 `.a` 里只留下**局部（`t`）**副本（本机实测 221 个），属内部链接，**无法 FFI 链接** ——
   例如 `LL_GPIO_SetPinMode`、`LL_TIM_SetPrescaler` 等，`.c` 中只有调用、没有外部定义。
   可调用的只有 `.c` 中定义且 `USE_FULL_LL_DRIVER` 开启时导出的**全局（`T`）**函数（104 个，
   如 `LL_RCC_GetSystemClocksFreq`、`LL_GPIO_Init`）。
   若某 LL 功能只有 inline 版本，应在 Rust 侧用 `py32e407-pac` 直接操作寄存器，而非 FFI；
   或用 bindgen 的 `generate_inline_functions` 让 bindgen 把函数体翻成 Rust。
   **不要**试图把它们改成 `extern` 声明（例如重定义 `__STATIC_INLINE`），会导致
   `undefined reference` 链接失败。
   > 判符号可见性时，`nm` 的 `T`（全局）与 `t`（局部）**必须区分大小写**；
   > PowerShell `Select-String` 默认大小写不敏感，会误把局部符号算成可链接。
3. **no_std 约束**：C 驱动不能依赖 libc（`printf`/`malloc` 等）。本 `build.rs` 已加 `-ffreestanding -fno-builtin`；
   bindgen 侧也用 `use_core()` + `layout_tests(false)`，生成的绑定不引用 `std`。
4. **bindgen 的 `<stdint.h>`**：裸机下 `stdint.h` 来自 newlib（`sysroot/include`，sysroot 本身已含
   `arm-none-eabi` 段，不要再叠一层）。`build.rs` 用 `arm-none-eabi-gcc --print-sysroot` 动态探测。
5. **中文路径**：C 源通过 `../参考设计/...` 相对引用。若你的工具链对中文路径敏感，可把
   `参考设计/PY32E407_Firmware_V0.7.0/Drivers` 复制为 crate 内的 `vendor/` 目录并相应修改 `build.rs` 的路径。
6. **改了 build.rs 却没生效？** build script 的编译产物与指纹在 `target/debug`（host 侧），
   只删 `target/thumbv7em-none-eabihf` 可能仍复用旧的 `bindings.rs`。强制重跑请删整个 `target`。
