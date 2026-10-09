//! 兜底类型与零值构造函数。
//!
//! HAL 用 OOP 风格，每个外设都有自己的 `HandleTypeDef` 结构体（含状态字段、回调指针）。
//! Rust 侧拿到的都是 opaque 类型，只能 `zeroed()` 后再调 `HAL_PPP_Init`。
//!
//! 这里提供统一的入口与别名，调用方照官方 HAL 文档写即可：
//!
//! ```ignore
//! use py32e407hal_rs as hal;
//!
//! static mut UART2_HANDLE: hal::UART_HandleTypeDef = hal::UART_HandleTypeDef::zeroed();
//! ```
