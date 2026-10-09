//! # py32e407cbind-rs
//!
//! 把 PUYA 官方 **C LL 驱动**（`py32e407_ll_rcc.c` / `py32e407_ll_gpio.c` / …）通过 `cc`
//! 交叉编译为静态库，再用 **bindgen** 从官方头自动生成 Rust FFI 绑定。编译、链接、代码生成
//! 全部由本 crate 的 `build.rs` 完成（对应 esp-idf-sys 的 “raw 绑定 = bindgen + cc” 分层）。
//!
//! ## 三层结构
//!
//! | 层 | 内容 | 是否需 `unsafe` |
//! |---|---|---|
//! | ① 原始绑定 | `include!(bindings.rs)`：`.c` 里定义的函数（`LL_GPIO_Init`、`LL_PLL_ConfigSystemClock_HSE`…） | 是（FFI 固有） |
//! | ② 外部化包装 | bindgen `wrap_static_fns` 把**全部** `__STATIC_INLINE` 函数外部化，编进 `libpy32e407_ll_ext.a`；Rust 侧名字**仍是官方名**（靠 `#[link_name]` 重定向） | 是 |
//! | ③ 便捷方法 | 本文件里的 `LlRccClocks::read()` / `LlGpioInit::zeroed()` 等 | 否 |
//!
//! 调用 FFI 一定需要 `unsafe`——这是 Rust 的语言规则，**无法靠绑定生成消除**。
//! 若希望调用方完全不写 `unsafe`，需要在本 crate 里继续加第 ③ 层的安全封装。
//!
//! ## 重要边界（务必先读）
//!
//! - LL 头里约 2000 个函数写成 **`__STATIC_INLINE`**。`static` 是内部链接：它们只会被内联进
//!   用到它们的 `.c`、最多留下**局部符号**（`nm` 里小写 `t`），**永远不产生全局符号**，
//!   因此 **bindgen 根本不会为它们生成声明**，手工声明也必然链接失败。
//!   本 crate 用 bindgen 的 **`wrap_static_fns`** 自动为**全部**这类函数生成外部化包装
//!   （符号名 `<原名>__ext`，用 `#[link_name]` 重定向）。
//!   **调用方直接写官方函数名即可，不需要知道 `__ext`，也不需要维护任何清单。**
//!   例：`LL_USART_Enable` / `LL_USART_TransmitData8` / `LL_GPIO_TogglePin` /
//!   `LL_AHB2_GRP1_EnableClock` / `LL_USART_IsActiveFlag_TXE` 全都可以直接用。
//! - 只用到的那部分才会进最终镜像：包装用 `-ffunction-sections` 编译，链接期 `--gc-sections`
//!   会裁掉没用到的（实测外部化约 2000 个函数、实际只用 7 个时，固件只涨约 750 字节）。
//! - 本 crate 是 `no_std` 裸机库，最终的内存布局（`link.x` + `memory.x`）由依赖它的
//!   `bin` 工程（例如 `cbind-demo`）提供，本 crate 自身不参与最终链接。
//! - 两个静态库都引用了**官方定义在 `system_py32e407.c` 里**的符号
//!   （`SystemCoreClock`、`AHBPrescTable[16]`、`APBPrescTable[8]`）。本 crate 不编译
//!   那个文件，**必须由最终的 bin 提供**，否则链接报 undefined reference。
//! - **切勿**把带 `main()` 的示例 `main.c` 编进来，否则会和 `cortex-m-rt` 的程序入口冲突。
//! - `GPIO_TypeDef` / `UART_TypeDef` / `DMA_TypeDef` 等在 Rust 侧为 opaque。外设指针由设备头的
//!   基址宏得到，**全部基址宏（`.*_BASE`）都已导出**：`GPIOC_BASE as *mut GpioType`、
//!   `UART2_BASE as *mut UartType`、`DMA1_BASE as *mut DmaType`。**不需要 py32e407-pac。**
//! - 设备头里的**全大写宏**（寄存器位定义 `DMA_CH_CTRLL_TT_0`、字段掩码、时钟值…）也已整批
//!   导出。这不是可有可无的：bindgen 求值宏时会拿替换表里的标识符查自己已生成的常量表，
//!   查不到就**静默丢弃整条宏**（见 `build.rs` 的说明）。少了那一层，
//!   `LL_DMA_DIRECTION_MEMORY_TO_PERIPH`、`LL_UART_DATAWIDTH_8B` 这类常量会在 Rust 侧凭空消失。
//!
//! ## 用法示例（在最终 bin 中）
//! ```ignore
//! use py32e407cbind_rs::{GpioType, LlGpioInit, LlRccClocks, GPIOB_BASE,
//!                        LL_AHB2_GRP1_EnableClock, LL_AHB2_GRP1_PERIPH_GPIOB,
//!                        LL_GPIO_Init, LL_GPIO_PIN_5, LL_GPIO_StructInit, LL_GPIO_TogglePin};
//!
//! // 读取系统时钟频率（需硬件时钟已配置后才有意义）
//! let clocks = LlRccClocks::read();
//!
//! // 使能 GPIOB 时钟（原本是 static inline，现由 wrap_static_fns 自动外部化）
//! unsafe { LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOB) };
//!
//! // 初始化 PB5 为输出：端口指针直接由基址宏得到，无需 PAC
//! let gpiob = GPIOB_BASE as *mut GpioType;
//! let mut cfg = LlGpioInit::zeroed();
//! unsafe {
//!     LL_GPIO_StructInit(&mut cfg);
//!     cfg.Pin = LL_GPIO_PIN_5;
//!     cfg.Mode = LL_GPIO_MODE_OUTPUT;
//!     LL_GPIO_Init(gpiob, &mut cfg);
//!
//!     LL_GPIO_TogglePin(gpiob, LL_GPIO_PIN_5);
//! }
//! ```
#![no_std]
// 自动生成的 C 绑定保留原始标识符（如 GPIO 字段 MODER、宏 LL_RCC_HSE_4_8MHz），
// 不符合 Rust 命名规范属常态，统一在 crate 顶层 allow 掉即可（bindings.rs 与自动生成的
// 别名文件都经 include! 并入本 crate，受此 allow 覆盖）。
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

// 由 build.rs 经 bindgen 自动生成的 FFI 绑定（类型 + extern "C" 函数声明）。
// 两个静态库的链接由 build.rs 里的 cc::Build::compile 自动完成
// （分别输出 `cargo:rustc-link-lib=static=py32e407_ll` 与 `...=py32e407_ll_ext`），
// 无需手写 `#[link]`。
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// 不需要额外的“官方名别名”层：bindgen 的 `wrap_static_fns` 为包装生成的是
//     #[link_name = "LL_GPIO_TogglePin__ext"]
//     pub fn LL_GPIO_TogglePin(...);
// —— **Rust 侧函数名就是官方名**，`__ext` 只出现在 `link_name` 属性里。
// 所以下面直接就能用 `LL_USART_TransmitData8` / `LL_AHB2_GRP1_EnableClock` 等全部官方 API。

// 少数 LL 常量写成 `((uint32_t)某位宏)`，bindgen 的宏解析器处理不了这种 C 风格强制转换，
// 会把整条宏**静默丢弃**（`LL_DMA_DIRECTION_MEMORY_TO_PERIPH`、`LL_UART_DATAWIDTH_8B` …）。
// `ll_consts` 按官方定义把它们补回，并重导出到 crate 根，调用方照官方名书写即可。
mod ll_consts;
pub use ll_consts::*;

/// 友好别名：C `LL_RCC_ClocksTypeDef`（系统时钟频率，4×`u32`）。
pub type LlRccClocks = LL_RCC_ClocksTypeDef;
/// 友好别名：C `LL_GPIO_InitTypeDef`（GPIO 初始化结构体）。
pub type LlGpioInit = LL_GPIO_InitTypeDef;
/// 友好别名：C `GPIO_TypeDef`（opaque 外设寄存器块）。
/// 端口指针可由 `GPIO[A-F]_BASE` 直接得到：`GPIOC_BASE as *mut GpioType`。
pub type GpioType = GPIO_TypeDef;
/// 友好别名：C `UART_TypeDef`（PY32 的 **UART1/2/3**，与 `USART` 是两套独立外设）。
/// 指针由基址宏得到：`UART2_BASE as *mut UartType`。
pub type UartType = UART_TypeDef;
/// 友好别名：C `USART_TypeDef`（PY32 的 **USART1/2/3**）。`USART2_BASE as *mut UsartType`。
pub type UsartType = USART_TypeDef;
/// 友好别名：C `DMA_TypeDef`（DMA1 / DMA2）。`DMA1_BASE as *mut DmaType`。
pub type DmaType = DMA_TypeDef;
/// 友好别名：C `LL_UART_InitTypeDef`（UART 初始化结构体）。
pub type LlUartInit = LL_UART_InitTypeDef;
/// 友好别名：C `LL_DMA_InitTypeDef`（DMA 通道初始化结构体）。
pub type LlDmaInit = LL_DMA_InitTypeDef;

impl LL_RCC_ClocksTypeDef {
    /// 读取当前系统时钟频率。硬件时钟已初始化后调用才有意义。
    ///
    /// # Safety
    /// 内部调用 `LL_RCC_GetSystemClocksFreq`，会读取硬件 RCC 寄存器；
    /// 需在裸机上下文、RCC 时钟树可访问时调用。
    pub fn read() -> Self {
        let mut clocks = unsafe { core::mem::zeroed::<Self>() };
        unsafe {
            LL_RCC_GetSystemClocksFreq(&mut clocks as *mut _);
        }
        clocks
    }
}

impl LL_GPIO_InitTypeDef {
    /// 全零初始化（等价于 `core::mem::zeroed`，供后续 `LL_GPIO_StructInit` 填充默认值）。
    pub fn zeroed() -> Self {
        unsafe { core::mem::zeroed::<Self>() }
    }
}

impl LL_UART_InitTypeDef {
    /// 全零初始化（供后续 `LL_UART_StructInit` / `LL_UART_Init` 填充）。
    pub fn zeroed() -> Self {
        unsafe { core::mem::zeroed::<Self>() }
    }
}

impl LL_DMA_InitTypeDef {
    /// 全零初始化（供后续 `LL_DMA_StructInit` / `LL_DMA_Init` 填充）。
    pub fn zeroed() -> Self {
        unsafe { core::mem::zeroed::<Self>() }
    }
}
