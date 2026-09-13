//! # py32e407cbind-rs
//!
//! 把 PUYA 官方 **C LL 驱动**（`py32e407_ll_rcc.c` / `py32e407_ll_gpio.c`）通过 `cc` 交叉编译为
//! 静态库，再用 **bindgen** 从官方头自动生成 Rust FFI 绑定。编译、链接、代码生成全部由
//! 本 crate 的 `build.rs` 完成（对应 esp-idf-sys 的 “raw 绑定 = bindgen + cc” 分层）。
//!
//! 生成的原始绑定位于 `OUT_DIR/bindings.rs`，本文件通过 `include!` 引入，并额外提供
//! 一组友好类型别名与便捷方法。
//!
//! ## 重要边界（务必先读）
//! - LL 驱动里**大量函数是头文件中的 `static inline`**，它们不会进 `.a`，无法 FFI 链接；
//!   真正可链接的函数只有在 `.c` 中定义、且 `USE_FULL_LL_DRIVER` 开启时才会导出的那些
//!   （例如 `LL_RCC_GetSystemClocksFreq`、`LL_GPIO_Init`）。
//! - 本 crate 是 `no_std` 裸机库，最终的内存布局（`link.x` + `memory.x`）由依赖它的
//!   `bin` 工程（例如 `demo-py32`）提供，本 crate 自身不参与最终链接。
//! - **切勿**把带 `main()` 的示例 `main.c` 编进来，否则会和 `cortex-m-rt` 的程序入口冲突。
//! - `GPIO_TypeDef` 在 Rust 侧为 opaque：其布局与 `py32e407-pac` 的 GPIO 寄存器块一致，
//!   调用时把 PAC 的 `&mut pac::GPIOA` 强转为 `*mut GpioType` 传入即可。
//!
//! ## 用法示例（在最终 bin 中）
//! ```ignore
//! use py32e407cbind_rs::{LlRccClocks, LlGpioInit, GpioType, ErrorStatus,
//!                        LL_RCC_GetSystemClocksFreq, LL_GPIO_Init, LL_GPIO_StructInit};
//!
//! // 读取系统时钟频率（需硬件时钟已配置后才有意义）
//! let clocks = LlRccClocks::read();
//!
//! // 初始化某个 GPIO（把 PAC 的 GPIO 外设指针传入）
//! let mut cfg = LlGpioInit::zeroed();
//! unsafe { LL_GPIO_StructInit(&mut cfg); }
//! cfg.Pin = 1 << 5;   // LL_GPIO_PIN_5
//! cfg.Mode = 1;        // LL_GPIO_MODE_OUTPUT
//! unsafe { LL_GPIO_Init(gpioa as *mut GpioType, &mut cfg); }
//! ```
#![no_std]
// 自动生成的 C 绑定保留原始标识符（如 GPIO 字段 MODER、宏 LL_RCC_HSE_4_8MHz），
// 不符合 Rust 命名规范属常态，统一在 crate 顶层 allow 掉即可（bindings.rs 经 include!
// 并入本 crate，受此 allow 覆盖）。
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

// 由 build.rs 经 bindgen 自动生成的 FFI 绑定（类型 + extern "C" 函数声明）。
// 静态库 libpy32e407_ll.a 的链接由 build.rs 中 cc::Build::compile 自动完成
// （它向 cargo 输出 `cargo:rustc-link-lib=static=py32e407_ll`），无需手写 `#[link]`。
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// 友好别名：C `LL_RCC_ClocksTypeDef`（系统时钟频率，4×`u32`）。
pub type LlRccClocks = LL_RCC_ClocksTypeDef;
/// 友好别名：C `LL_GPIO_InitTypeDef`（GPIO 初始化结构体）。
pub type LlGpioInit = LL_GPIO_InitTypeDef;
/// 友好别名：C `GPIO_TypeDef`（opaque 外设寄存器块）。
/// 布局与 `py32e407-pac` 的 GPIO 相同，调用时把 PAC 的 `&mut pac::GPIOA` 强转传入即可。
pub type GpioType = GPIO_TypeDef;

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
