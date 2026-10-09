//! 官方 LL 头里**被 bindgen 丢弃**的那批常量，按官方定义在这里补回。
//!
//! # 为什么要单独一个文件
//!
//! `build.rs` 已经把设备头里**全大写宏**（`DMA_CH_CTRLL_TT_0`、`UART_CR1_M_0` 这类
//! 寄存器位定义）整批导出，所以下面这些常量的**组成部件**在 Rust 侧是有的。
//! 但 bindgen 的宏解析器认不出 C 风格强制转换：只要一条宏的替换表里出现 `(uint32_t)`，
//! 整条宏就会被**静默丢掉**（既不报错，也不在 `bindings.rs` 里留下任何痕迹）。
//! 实测规律非常干净 —— 同一组里
//!
//! ```c
//! #define LL_DMA_SRC_ADDR_INC   0x00000000U                     // 纯字面量 → 在
//! #define LL_DMA_SRC_ADDR_FIX   ((uint32_t)DMA_CH_CTRLL_SINC_1)  // 带转换   → 丢
//! ```
//!
//! 只有后者消失。`LL_DMA_DIRECTION_*`、`LL_DMA_*_ADDR_FIX/DEC`、`LL_DMA_PRIORITY_1..5`、
//! `LL_DMA_*_PERIPH_MAP_2..6`、`LL_UART_DATAWIDTH_8B` 等都属于后者。
//!
//! `build.rs` 会在每次构建时统计并 `cargo:warning` 报出「头里有、绑定里没有」的
//! `LL_*` 宏数量，所以缺口不会静默扩大。
//!
//! # 这里的写法
//!
//! 每条都是**官方 `#define` 的逐字转写**，用设备头的位宏表示 ——
//! **没有任何魔数**，值可以直接和 `参考设计/.../Inc/py32e407_ll_*.h` 对照。
//! 头部注释里给出的 `#define` 原文来自 `PY32E407_Firmware_V0.7.0`。
//!
//! # 覆盖范围（截至 2026-09-21）
//!
//! 本文件覆盖 `py32e407_ll_dma.h` 的**全部**此类常量，外加 `py32e407_ll_uart.h` 的
//! 数据位宽 / TX 模式 / 地址匹配一组 —— 也就是 UART + DMA 这两条路径**完全齐了**。
//!
//! 其余外设目前仍有缺口：构建时 `build.rs` 报 **199 个**，其中绝大多数属于本项目
//! 用不到的外设（`LL_ADC_AWD_CHANNEL_*`、`LL_CORDIC_*`、`LL_COMP_*` 等）。
//! 需要用哪个就照抄本节格式补一条；想一次性消灭缺口，可以让 `build.rs` 扫描这些宏的
//! 替换表并自动翻译成 `pub const`（形态非常规整，全是 `((uint32_t)若干位宏的或)`），
//! 但那属于代码生成，建议等确有需要时再做。

// 绑定由 crate 根用 `include!(bindings.rs)` 引入，所以设备头的位宏（`DMA_CH_CTRLL_TT_0` 等）
// 都在 **crate 根**，本子模块里要显式引入才能引用。
use crate::*;

// ===========================================================================
// DMA —— 传输方向（py32e407_ll_dma.h 268~277 行）
// ===========================================================================

/// `#define LL_DMA_DIRECTION_MEMORY_TO_PERIPH ((uint32_t)DMA_CH_CTRLL_TT_0)`
pub const LL_DMA_DIRECTION_MEMORY_TO_PERIPH: u32 = DMA_CH_CTRLL_TT_0;
/// `#define LL_DMA_DIRECTION_PERIPH_TO_MEMORY ((uint32_t)DMA_CH_CTRLL_TT_1)`
pub const LL_DMA_DIRECTION_PERIPH_TO_MEMORY: u32 = DMA_CH_CTRLL_TT_1;
/// `#define LL_DMA_DIRECTION_PERIPH_TO_PERIPH ((uint32_t)DMA_CH_CTRLL_TT)`
pub const LL_DMA_DIRECTION_PERIPH_TO_PERIPH: u32 = DMA_CH_CTRLL_TT;

// ===========================================================================
// DMA —— 传输宽度（279~297 行）
// ===========================================================================

/// `#define LL_DMA_SRC_WIDTH_HALFWORD ((uint32_t)DMA_CH_CTRLL_SRC_TR_WIDTH_0)`
pub const LL_DMA_SRC_WIDTH_HALFWORD: u32 = DMA_CH_CTRLL_SRC_TR_WIDTH_0;
/// `#define LL_DMA_SRC_WIDTH_WORD ((uint32_t)DMA_CH_CTRLL_SRC_TR_WIDTH_1)`
pub const LL_DMA_SRC_WIDTH_WORD: u32 = DMA_CH_CTRLL_SRC_TR_WIDTH_1;
/// `#define LL_DMA_DST_WIDTH_HALFWORD ((uint32_t)DMA_CH_CTRLL_DST_TR_WIDTH_0)`
pub const LL_DMA_DST_WIDTH_HALFWORD: u32 = DMA_CH_CTRLL_DST_TR_WIDTH_0;
/// `#define LL_DMA_DST_WIDTH_WORD ((uint32_t)DMA_CH_CTRLL_DST_TR_WIDTH_1)`
pub const LL_DMA_DST_WIDTH_WORD: u32 = DMA_CH_CTRLL_DST_TR_WIDTH_1;

// ===========================================================================
// DMA —— 突发长度（299~325 行）
// ===========================================================================

/// `#define LL_DMA_SRC_BURST_LEN_4 ((uint32_t)DMA_CH_CTRLL_SRC_MSIZE_0)`
pub const LL_DMA_SRC_BURST_LEN_4: u32 = DMA_CH_CTRLL_SRC_MSIZE_0;
/// `#define LL_DMA_SRC_BURST_LEN_8 ((uint32_t)DMA_CH_CTRLL_SRC_MSIZE_1)`
pub const LL_DMA_SRC_BURST_LEN_8: u32 = DMA_CH_CTRLL_SRC_MSIZE_1;
/// `#define LL_DMA_SRC_BURST_LEN_16 ((uint32_t)(DMA_CH_CTRLL_SRC_MSIZE_1 | DMA_CH_CTRLL_SRC_MSIZE_0))`
pub const LL_DMA_SRC_BURST_LEN_16: u32 = DMA_CH_CTRLL_SRC_MSIZE_1 | DMA_CH_CTRLL_SRC_MSIZE_0;
/// `#define LL_DMA_SRC_BURST_LEN_32 ((uint32_t)DMA_CH_CTRLL_SRC_MSIZE_2)`
pub const LL_DMA_SRC_BURST_LEN_32: u32 = DMA_CH_CTRLL_SRC_MSIZE_2;
/// `#define LL_DMA_SRC_BURST_LEN_64 ((uint32_t)(DMA_CH_CTRLL_SRC_MSIZE_2 | DMA_CH_CTRLL_SRC_MSIZE_0))`
pub const LL_DMA_SRC_BURST_LEN_64: u32 = DMA_CH_CTRLL_SRC_MSIZE_2 | DMA_CH_CTRLL_SRC_MSIZE_0;
/// `#define LL_DMA_SRC_BURST_LEN_128 ((uint32_t)(DMA_CH_CTRLL_SRC_MSIZE_2 | DMA_CH_CTRLL_SRC_MSIZE_1))`
pub const LL_DMA_SRC_BURST_LEN_128: u32 = DMA_CH_CTRLL_SRC_MSIZE_2 | DMA_CH_CTRLL_SRC_MSIZE_1;

/// `#define LL_DMA_DST_BURST_LEN_4 ((uint32_t)DMA_CH_CTRLL_DST_MSIZE_0)`
pub const LL_DMA_DST_BURST_LEN_4: u32 = DMA_CH_CTRLL_DST_MSIZE_0;
/// `#define LL_DMA_DST_BURST_LEN_8 ((uint32_t)DMA_CH_CTRLL_DST_MSIZE_1)`
pub const LL_DMA_DST_BURST_LEN_8: u32 = DMA_CH_CTRLL_DST_MSIZE_1;
/// `#define LL_DMA_DST_BURST_LEN_16 ((uint32_t)(DMA_CH_CTRLL_DST_MSIZE_1 | DMA_CH_CTRLL_DST_MSIZE_0))`
pub const LL_DMA_DST_BURST_LEN_16: u32 = DMA_CH_CTRLL_DST_MSIZE_1 | DMA_CH_CTRLL_DST_MSIZE_0;
/// `#define LL_DMA_DST_BURST_LEN_32 ((uint32_t)DMA_CH_CTRLL_DST_MSIZE_2)`
pub const LL_DMA_DST_BURST_LEN_32: u32 = DMA_CH_CTRLL_DST_MSIZE_2;
/// `#define LL_DMA_DST_BURST_LEN_64 ((uint32_t)(DMA_CH_CTRLL_DST_MSIZE_2 | DMA_CH_CTRLL_DST_MSIZE_0))`
pub const LL_DMA_DST_BURST_LEN_64: u32 = DMA_CH_CTRLL_DST_MSIZE_2 | DMA_CH_CTRLL_DST_MSIZE_0;
/// `#define LL_DMA_DST_BURST_LEN_128 ((uint32_t)(DMA_CH_CTRLL_DST_MSIZE_2 | DMA_CH_CTRLL_DST_MSIZE_1))`
pub const LL_DMA_DST_BURST_LEN_128: u32 = DMA_CH_CTRLL_DST_MSIZE_2 | DMA_CH_CTRLL_DST_MSIZE_1;

// ===========================================================================
// DMA —— 地址增减模式（327~345 行）
// ===========================================================================

/// `#define LL_DMA_SRC_ADDR_DEC ((uint32_t)DMA_CH_CTRLL_SINC_0)`
pub const LL_DMA_SRC_ADDR_DEC: u32 = DMA_CH_CTRLL_SINC_0;
/// `#define LL_DMA_SRC_ADDR_FIX ((uint32_t)DMA_CH_CTRLL_SINC_1)`
pub const LL_DMA_SRC_ADDR_FIX: u32 = DMA_CH_CTRLL_SINC_1;
/// `#define LL_DMA_DST_ADDR_DEC ((uint32_t)DMA_CH_CTRLL_DINC_0)`
pub const LL_DMA_DST_ADDR_DEC: u32 = DMA_CH_CTRLL_DINC_0;
/// `#define LL_DMA_DST_ADDR_FIX ((uint32_t)DMA_CH_CTRLL_DINC_1)`
pub const LL_DMA_DST_ADDR_FIX: u32 = DMA_CH_CTRLL_DINC_1;

// ===========================================================================
// DMA —— 握手方式（347~363 行）
// ===========================================================================

/// `#define LL_DMA_SRC_HANDSHAKE_TYPE_SOFT ((uint32_t)DMA_CH_CFGL_HS_SEL_SRC)`
pub const LL_DMA_SRC_HANDSHAKE_TYPE_SOFT: u32 = DMA_CH_CFGL_HS_SEL_SRC;
/// `#define LL_DMA_DST_HANDSHAKE_TYPE_SOFT ((uint32_t)DMA_CH_CFGL_HS_SEL_DST)`
pub const LL_DMA_DST_HANDSHAKE_TYPE_SOFT: u32 = DMA_CH_CFGL_HS_SEL_DST;

// ===========================================================================
// DMA —— 通道优先级（365~376 行）
// ===========================================================================

/// `#define LL_DMA_PRIORITY_1 ((uint32_t)DMA_CH_CFGL_CH_PRIOR_0)`
pub const LL_DMA_PRIORITY_1: u32 = DMA_CH_CFGL_CH_PRIOR_0;
/// `#define LL_DMA_PRIORITY_2 ((uint32_t)DMA_CH_CFGL_CH_PRIOR_1)`
pub const LL_DMA_PRIORITY_2: u32 = DMA_CH_CFGL_CH_PRIOR_1;
/// `#define LL_DMA_PRIORITY_3 ((uint32_t)(DMA_CH_CFGL_CH_PRIOR_1 | DMA_CH_CFGL_CH_PRIOR_0))`
pub const LL_DMA_PRIORITY_3: u32 = DMA_CH_CFGL_CH_PRIOR_1 | DMA_CH_CFGL_CH_PRIOR_0;
/// `#define LL_DMA_PRIORITY_4 ((uint32_t)DMA_CH_CFGL_CH_PRIOR_2)`
pub const LL_DMA_PRIORITY_4: u32 = DMA_CH_CFGL_CH_PRIOR_2;
/// `#define LL_DMA_PRIORITY_5 ((uint32_t)(DMA_CH_CFGL_CH_PRIOR_2 | DMA_CH_CFGL_CH_PRIOR_0))`
pub const LL_DMA_PRIORITY_5: u32 = DMA_CH_CFGL_CH_PRIOR_2 | DMA_CH_CFGL_CH_PRIOR_0;

// ===========================================================================
// DMA —— 目的 / 源请求号（378~402 行）
// ===========================================================================
//
// 这些值要和 `LL_SYSCFG_SetDMARemap(DMAx, LL_SYSCFG_DMA_PERIPH_MAP_n, ...)` 的
// `n` 一一对应：通道的 `DST_PER`/`SRC_PER` 写成几，SYSCFG 里就把第几个请求
// 映射到目标外设。只写一半，通道永远等不到硬件请求。

/// `#define LL_DMA_DST_PERIPH_MAP_2 ((uint32_t)DMA_CH_CFGH_DST_PER_0)`
pub const LL_DMA_DST_PERIPH_MAP_2: u32 = DMA_CH_CFGH_DST_PER_0;
/// `#define LL_DMA_DST_PERIPH_MAP_3 ((uint32_t)DMA_CH_CFGH_DST_PER_1)`
pub const LL_DMA_DST_PERIPH_MAP_3: u32 = DMA_CH_CFGH_DST_PER_1;
/// `#define LL_DMA_DST_PERIPH_MAP_4 ((uint32_t)(DMA_CH_CFGH_DST_PER_1 | DMA_CH_CFGH_DST_PER_0))`
pub const LL_DMA_DST_PERIPH_MAP_4: u32 = DMA_CH_CFGH_DST_PER_1 | DMA_CH_CFGH_DST_PER_0;
/// `#define LL_DMA_DST_PERIPH_MAP_5 ((uint32_t)DMA_CH_CFGH_DST_PER_2)`
pub const LL_DMA_DST_PERIPH_MAP_5: u32 = DMA_CH_CFGH_DST_PER_2;
/// `#define LL_DMA_DST_PERIPH_MAP_6 ((uint32_t)(DMA_CH_CFGH_DST_PER_2 | DMA_CH_CFGH_DST_PER_0))`
pub const LL_DMA_DST_PERIPH_MAP_6: u32 = DMA_CH_CFGH_DST_PER_2 | DMA_CH_CFGH_DST_PER_0;

/// `#define LL_DMA_SRC_PERIPH_MAP_2 ((uint32_t)DMA_CH_CFGH_SRC_PER_0)`
pub const LL_DMA_SRC_PERIPH_MAP_2: u32 = DMA_CH_CFGH_SRC_PER_0;
/// `#define LL_DMA_SRC_PERIPH_MAP_3 ((uint32_t)DMA_CH_CFGH_SRC_PER_1)`
pub const LL_DMA_SRC_PERIPH_MAP_3: u32 = DMA_CH_CFGH_SRC_PER_1;
/// `#define LL_DMA_SRC_PERIPH_MAP_4 ((uint32_t)(DMA_CH_CFGH_SRC_PER_1 | DMA_CH_CFGH_SRC_PER_0))`
pub const LL_DMA_SRC_PERIPH_MAP_4: u32 = DMA_CH_CFGH_SRC_PER_1 | DMA_CH_CFGH_SRC_PER_0;
/// `#define LL_DMA_SRC_PERIPH_MAP_5 ((uint32_t)DMA_CH_CFGH_SRC_PER_2)`
pub const LL_DMA_SRC_PERIPH_MAP_5: u32 = DMA_CH_CFGH_SRC_PER_2;
/// `#define LL_DMA_SRC_PERIPH_MAP_6 ((uint32_t)(DMA_CH_CFGH_SRC_PER_2 | DMA_CH_CFGH_SRC_PER_0))`
pub const LL_DMA_SRC_PERIPH_MAP_6: u32 = DMA_CH_CFGH_SRC_PER_2 | DMA_CH_CFGH_SRC_PER_0;

// ===========================================================================
// DMA —— FIFO 模式（406~417 行）
// ===========================================================================

/// `#define LL_DMA_FIFO_MODE_1 ((uint32_t)DMA_CH_CFGH_FIFO_MODE)`
pub const LL_DMA_FIFO_MODE_1: u32 = DMA_CH_CFGH_FIFO_MODE;

// ===========================================================================
// UART —— 数据位宽（py32e407_ll_uart.h 143~147 行）
// ===========================================================================
//
// 注意 **`UART` 与 `USART` 是两套独立外设**：`LL_UART_*` 只用于 UART1/2/3，
// `LL_USART_*` 用于 USART1/2/3，两者的寄存器布局都不同（`UART.DR` 在偏移 0x00，
// `USART.DR` 在 0x04），别串用。

/// `#define LL_UART_DATAWIDTH_6B ((uint32_t)UART_CR1_M_0)`
pub const LL_UART_DATAWIDTH_6B: u32 = UART_CR1_M_0;
/// `#define LL_UART_DATAWIDTH_7B ((uint32_t)UART_CR1_M_1)`
pub const LL_UART_DATAWIDTH_7B: u32 = UART_CR1_M_1;
/// `#define LL_UART_DATAWIDTH_8B ((uint32_t)(UART_CR1_M_1) | (UART_CR1_M_0))`
pub const LL_UART_DATAWIDTH_8B: u32 = UART_CR1_M_1 | UART_CR1_M_0;
/// `#define LL_UART_TXMODE_1 ((uint32_t)UART_CR3_TX_MODE)`
pub const LL_UART_TXMODE_1: u32 = UART_CR3_TX_MODE;
/// `#define LL_UART_RX_ADDR_MATCH_HARD ((uint32_t)UART_CR3_ADDR_MATCH)`
pub const LL_UART_RX_ADDR_MATCH_HARD: u32 = UART_CR3_ADDR_MATCH;
