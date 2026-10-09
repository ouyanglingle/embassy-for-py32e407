//! # py32e407hal-rs
//!
//! 把 PUYA 官方 **C HAL 驱动**通过 `cc` 交叉编译为静态库，再用 **手写**的 Rust
//! extern 声明把 HAL API 暴露给 Rust。
//!
//! ## 与 `py32e407cbind-rs` 的关系
//!
//! HAL 是 LL 之上的**状态机/回调封装**。HAL 的 `.c` 文件会调 LL 的 `LL_*` 函数，
//! 所以链接时**必须**同时有 `py32e407cbind-rs` 编出来的两个静态库：
//!
//! ```toml
//! [dependencies]
//! py32e407cbind-rs = { path = "../py32e407cbind-rs" }  # 提供 LL_* 符号
//! py32e407hal-rs    = { path = "../py32e407hal-rs" }    # 本 crate：HAL_* 声明 + .a
//! ```
//!
//! ## 为什么不用 bindgen
//!
//! LL 头里**全是 `__STATIC_INLINE`**（约 2058 个），所以 cbind-rs 走 bindgen +
//! `wrap_static_fns` 路线。HAL 不一样：绝大多数函数**真的在 `.c` 里实现**
//! （`Init`/`DeInit`/`Transmit_DMA` 等），所以只需要 Rust 侧一份 extern 声明。
//! HAL 头层层嵌套（master `py32e4xx_hal.h` → conf → 各模块头 → def → 设备头）+ 宏开关
//! 选模块，让 bindgen 稳定工作要反复调 include 顺序；手写更可控。
//!
//! ## HAL 弱符号覆盖：HAL_GetTick / HAL_Delay
//!
//! HAL 自带一组 `__weak` 默认函数（`HAL_GetTick` / `HAL_Delay` 等），HAL 内部超时路径
//! 会调它。本文件用 Rust `#[no_mangle] pub extern "C" fn` 提供**可工作的实现**：
//! 维护一个静态 u32 tick 计数器，由 `bin` 在主循环 1 ms 节拍里调 [`bump_tick`] 推进。
//!
//! ## 不可用的 HAL 部分
//!
//! HAL 设计时假定有 `cortex-m` 标准 startup 调 `SystemInit` 后再 `HAL_Init`、
//! `HAL_MspInit`，并依赖 NVIC 向量表里的 DMA/UART 中断回调推进状态机。
//! 本工程**没有 NVIC 向量表**（不依赖 `py32e407-pac`），所以**完整 HAL 状态机不可用**。
//!
//! 实战做法（见 `hal-demo`）：
//! * `HAL_UART_Init` / `HAL_DMA_Init` 可以正常调（它们只操作寄存器，不依赖回调）；
//! * `HAL_UART_Transmit_DMA` / `HAL_UART_Receive_DMA` 可以正常**启动**传输，
//!   但 DMA TC 完成后** HAL 内部期待的 ISR 回调不会触发**，所以
//!   `HAL_UART_GetState()` 永远看不到 READY——本 demo 直接绕开它，**手动轮询
//!   DMA TC 标志**判断完成，然后把 `huart.gState` 复位回 `HAL_UART_STATE_READY`。
//!
//! ## 用法
//!
//! ```ignore
//! use py32e407hal_rs as hal;
//! use py32e407cbind_rs as ll;
//!
//! const UART2: *mut hal::UART_TypeDef = hal::UART2_BASE as *mut hal::UART_TypeDef;
//! const DMA1: *mut hal::DMA_TypeDef   = hal::DMA1_BASE as *mut hal::DMA_TypeDef;
//!
//! let mut huart = hal::UART_HandleTypeDef::zeroed();
//! unsafe {
//!     hal::__HAL_RESET_HANDLE_STATE(&mut huart);
//!     huart.Instance = UART2;
//!     huart.Init.BaudRate = 115_200;
//!     // ...其他字段
//!     hal::HAL_UART_Init(&mut huart);
//!
//!     // 启动一次 DMA 发送
//!     hal::HAL_UART_Transmit_DMA(&mut huart, buf.as_mut_ptr(), buf.len() as u32);
//! }
//! ```
#![no_std]
// 自动生成的 C 风格命名（HAL_StatusTypeDef / HAL_UART_StateTypeDef / 字段名 Init.* 等等）
// 不符合 Rust 命名规范是 HAL 的固有形态，与 cbind-rs 一样顶层 allow 掉。
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

// ===========================================================================
// ① HAL 公共类型
// ===========================================================================

/// HAL 通用返回值。
/// ⚠️ 注意：与经典 STM32 相反，**0 = 成功**，1/2/3 分别是 ERROR / BUSY / TIMEOUT。
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HAL_StatusTypeDef {
    Ok = 0,
    Error = 1,
    Busy = 2,
    Timeout = 3,
}

/// HAL 锁状态。
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HAL_LockTypeDef {
    Unlocked = 0,
    Locked = 1,
}

/// UART 状态机字段的可能取值。
/// 这些值不是连续的：READY=0x20, BUSY_TX=0x21, BUSY_RX=0x22, ... ERROR=0xE0。
/// HAL 内部按位组合（gState | RxState）来表达复合状态。
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HAL_UART_StateTypeDef {
    Reset = 0x00,
    Ready = 0x20,
    Busy = 0x24,
    BusyTx = 0x21,
    BusyRx = 0x22,
    BusyTxRx = 0x23,
    Timeout = 0xA0,
    Error = 0xE0,
}

// ===========================================================================
// ② HAL handle 与外设结构体（按 C 布局暴露 Rust 可见的字段）
// ===========================================================================
//
// HAL 的 `.c` 会按 C 头里的**字段偏移**读写 handle（比如 `huart->gState = ...`）。
// Rust 侧的 handle 必须用 `#[repr(C)]`、字段顺序与 C 一致、字段对齐也一致，
// HAL 才能正确读写。
//
// ⚠️ **不要新增 / 调整字段** —— 改了 `UART_HandleTypeDef` 布局，HAL 内部访问
// `gState`/`RxState`/`hdmatx`/`hdmarx` 的偏移会全部错位，行为不可预测。
//
// `USE_HAL_*_REGISTER_CALLBACKS = 0` 让 callback 指针表不出现（cbind-rs 的
// `py32e407_hal_conf.h` 已关），否则大小会更大。

/// HAL 的 `UART_InitTypeDef`（按官方头里的字段顺序）。
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UART_InitTypeDef {
    pub BaudRate: u32,
    pub WordLength: u32,
    pub StopBits: u32,
    pub Parity: u32,
    pub MsbFirst: u32,
}

/// HAL 的 `UART_HandleTypeDef` —— **与 C 头严格一致**。
///
/// ⚠️ 字段顺序与类型不能动。C 侧按此布局读写，Rust 改一处就全错。
/// `hdmatx`/`hdmarx` 是 `*mut DMA_HandleTypeDef`（外层 opaque struct 的指针）。
/// 末尾留 `_trail` 字节确保实际 struct（包含 C 那边还有一些 RxXfer* / pTxBuffPtr / Lock /
/// ErrorCode 等字段）不会越界。`UART_HandleTypeDef` 完整大小约 96 字节（无 register
/// callbacks），取 128 给 32 字节余量。
#[repr(C)]
pub struct UART_HandleTypeDef {
    pub Instance: *mut UART_TypeDef,
    pub Init: UART_InitTypeDef,
    pub pTxBuffPtr: *mut u8,
    pub TxXferSize: u32,
    pub TxXferCount: u32,
    pub pRxBuffPtr: *mut u8,
    pub RxXferSize: u32,
    pub RxXferCount: u32,
    pub hdmatx: *mut DMA_HandleTypeDef,
    pub hdmarx: *mut DMA_HandleTypeDef,
    pub Lock: HAL_LockTypeDef,
    pub gState: HAL_UART_StateTypeDef,
    pub RxState: HAL_UART_StateTypeDef,
    pub ErrorCode: u32,
    /// C 头里还有字段（`RxEventType` 等）；Rust 看不见但 HAL 会读。
    /// **不能用 `[u8; N]` 兜底**——`#[repr(C)]` 的尾字段得是数组，
    /// 且**对齐 = 元素对齐**（u8 = 1，OK），长度给到 64 字节足够覆盖所有 USE_*_REGISTER_CALLBACKS=0 的字段。
    pub _trail: [u8; 64],
}

/// UART 外设寄存器类型（来自 cbind-rs 的 `ll::UartType`，这里**重命名导入**
/// 避免与 `UART_HandleTypeDef::Instance` 类型推断混淆）。
pub type UART_TypeDef = crate::cbind_alias::UartType;

impl UART_HandleTypeDef {
    /// 全零初始化（HAL 文档要求：首次 Init 前 State 必须为 0）。
    pub fn zeroed() -> Self {
        unsafe { core::mem::zeroed::<Self>() }
    }
}

/// HAL 的 `DMA_InitTypeDef`（按 C 头字段顺序）。
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_InitTypeDef {
    pub Request: u32,
    pub Direction: u32,
    pub PeriphInc: u32,
    pub MemInc: u32,
    pub PeriphDataAlignment: u32,
    pub MemDataAlignment: u32,
    pub Mode: u32,
    pub Priority: u32,
}

/// HAL 的 `DMA_HandleTypeDef`。
#[repr(C)]
pub struct DMA_HandleTypeDef {
    pub Instance: *mut DMA_TypeDef,
    pub Init: DMA_InitTypeDef,
    pub pParent: *mut core::ffi::c_void, // HAL 把它当 `void*`（指向 UART_HandleTypeDef 等）
    /// C 头里还有更多字段（Xfer* / State / ErrorCode 等），HAL 会读。Rust 留尾缓冲。
    pub _trail: [u8; 96],
}

pub type DMA_TypeDef = crate::cbind_alias::DmaType;

impl DMA_HandleTypeDef {
    pub fn zeroed() -> Self {
        unsafe { core::mem::zeroed::<Self>() }
    }
}

/// cbind-rs 暴露的 UART/DMA 外设类型别名，避免循环依赖（hal-rs 不该依赖 cbind-rs 的代码）。
/// 实际定义在 `crate::cbind_alias` 模块里，**仅暴露类型别名**，不引入任何 cbind 函数。
mod cbind_alias {
    // 这里直接用 cbind-rs 的 bindgen 生成的类型名（`UART_TypeDef` / `DMA_TypeDef`）；
    // 通过 `use py32e407cbind_rs as ll;` 引入当前 crate 的使用方那里仍然叫 `ll::UartType`。
    // 本 crate 自己不依赖 cbind-rs —— 用户端依赖里把 cbind-rs 也加上即可。
    //
    // 由于 HAL 的 .a 引用了 cbind-rs 的 LL_* 符号，**链接期**仍依赖 cbind-rs；
    // 但**类型**层面只需要一个名字，引入独立 crate 会破坏 halo / cbind 隔离，
    // 所以这里复用 `ll::UartType` 的 C 类型名（cbind-rs 已通过 bindgen 把它命名为
    // `UART_TypeDef`）—— 直接转发。
    pub use py32e407cbind_rs::UartType as UartType;
    pub use py32e407cbind_rs::DmaType as DmaType;
}

// ===========================================================================
// ③ HAL 函数声明（手写 Rust 外部声明 + 函数指针表等）
// ===========================================================================
//
// 这里**只列实际用到的** HAL API（demo 用到 + cbind 配套用）。新增 HAL 模块时
// 按官方头里的原型照抄即可（名称、参数顺序、返回类型都不能改，否则 undefined
// symbol 或 ABI 不一致）。
//
// ⚠️ **绝对不能在这里改函数签名**——这是 Rust 与 C 的 ABI 契约，错了会**运行时**
// 行为异常而不是编译期报。修改后必须 `cargo run --release` 上板实测。

// ---------- UART ----------
extern "C" {
    pub fn HAL_UART_Init(huart: *mut UART_HandleTypeDef) -> HAL_StatusTypeDef;
    pub fn HAL_UART_DeInit(huart: *mut UART_HandleTypeDef) -> HAL_StatusTypeDef;
    pub fn HAL_UART_Transmit_DMA(
        huart: *mut UART_HandleTypeDef,
        pData: *mut u8,
        Size: u32,
    ) -> HAL_StatusTypeDef;
    pub fn HAL_UART_Receive_DMA(
        huart: *mut UART_HandleTypeDef,
        pData: *mut u8,
        Size: u32,
    ) -> HAL_StatusTypeDef;
    pub fn HAL_UART_GetState(huart: *mut UART_HandleTypeDef) -> HAL_UART_StateTypeDef;
    pub fn HAL_UART_DMAStop(huart: *mut UART_HandleTypeDef) -> HAL_StatusTypeDef;

    /// HAL 文档要求「在首次 Init 前**必须**用此宏清零 State 字段」——
    /// Rust 里写 `huart._priv = [0; 256]` 等价；这里用 `inline(never)` 包一层
    /// 表达「对结构体 memset」的语义。
    #[inline(never)]
    fn __HAL_RESET_HANDLE_STATE_impl(huart: *mut UART_HandleTypeDef);
}

/// HAL 文档要求的「Init 前清零 State 字段」宏的 Rust 版本。
/// 直接调 Rust 的 `zeroed()`，不要再调用任何 HAL 函数。
#[inline]
pub unsafe fn __HAL_RESET_HANDLE_STATE(h: *mut UART_HandleTypeDef) {
    core::ptr::write_bytes(h as *mut u8, 0u8, core::mem::size_of::<UART_HandleTypeDef>());
}

// ---------- DMA ----------
extern "C" {
    pub fn HAL_DMA_Init(hdma: *mut DMA_HandleTypeDef) -> HAL_StatusTypeDef;
    pub fn HAL_DMA_DeInit(hdma: *mut DMA_HandleTypeDef) -> HAL_StatusTypeDef;
}

// ---------- HAL 公共 ----------
extern "C" {
    /// HAL 初始化（复位 + 配置 SysTick）。**本工程不用**——HAL 的 SysTick 路径
    /// 依赖中断向量表；我们提供自己的 [`HAL_GetTick`] / [`HAL_Delay`] 替代。
    pub fn HAL_Init() -> HAL_StatusTypeDef;

    /// HAL 反初始化。
    pub fn HAL_DeInit() -> HAL_StatusTypeDef;

    // HAL_GetTick / HAL_Delay 由本文件下面用 Rust 强符号覆盖，不要在 extern 块里再声明。
}

// ===========================================================================
// ④ HAL 弱符号覆盖：HAL_GetTick / HAL_Delay
// ===========================================================================
//
// HAL 默认实现是空（`__STATIC_INLINE` / `__weak`），HAL_Delay 在 HAL 内部超时路径里被调。
// 这里用 Rust 提供**强实现**——链接器按"强覆盖弱"规则选我们的。
//
// **怎么用**：在 1 ms 节拍里调一次 [`bump_tick`]，`HAL_Delay` 就会按预期工作。
// 不调用 `bump_tick()` 时 `HAL_Delay` 会陷入死循环（这是预期：1 ms 节拍没跑起来就别用 HAL）。

/// HAL 全局 tick 计数（毫秒）。
#[no_mangle]
pub static mut HAL_TICK_COUNT: u32 = 0;

#[no_mangle]
pub extern "C" fn HAL_GetTick() -> u32 {
    unsafe { HAL_TICK_COUNT }
}

#[no_mangle]
pub extern "C" fn HAL_Delay(Delay: u32) {
    let start = unsafe { HAL_TICK_COUNT };
    while unsafe { HAL_TICK_COUNT }.wrapping_sub(start) < Delay {
        core::hint::spin_loop();
    }
}

/// 由 bin 在 1 ms 节拍里调用一次。
///
/// # Safety
/// 调用方负责在中断里也安全（Cortex-M 单核下没问题；多核需在临界段）。
#[inline]
pub unsafe fn bump_tick() {
    HAL_TICK_COUNT = HAL_TICK_COUNT.wrapping_add(1);
}

// ===========================================================================
// ⑤ 端口基址宏（HAL 需要 UART2 / DMA1 等指针）
// ===========================================================================
//
// HAL 的 `UART_HandleTypeDef::Instance` 字段是 `*mut UART_TypeDef`，
// 而 `UART_TypeDef` 在 cbind-rs 里（cbind-rs 的 build.rs 同时 allowlist 了 `.*_BASE`）。
// **本 crate 不重复定义基址** —— 用户直接 `use py32e407cbind_rs as ll; ll::UART2_BASE`
// 再 `as *mut UartType`，与 HAL 句柄对接。

// ===========================================================================
// ⑥ HAL handle 上常用宏的 Rust 版本
// ===========================================================================

/// `__HAL_LOCK` 的 Rust 版本：返回当前锁状态。
/// HAL_UART_Init 等函数在入口用 `__HAL_LOCK` 检查 ——
/// 已锁就返回 BUSY，未锁就置为锁定、返回"可以继续"。
///
/// Rust 里通常不需要手动调（HAL_UART_Init 内部处理），暴露出来供自定义同步使用。
#[inline]
pub unsafe fn __HAL_LOCK(h: *mut UART_HandleTypeDef) -> bool {
    // HAL 文档：`Lock` 字段在 handle 偏移的某处，存储 HAL_LockTypeDef (u32)。
    // 我们把 handle 当 256 字节 opaque —— 找不到精确 Lock 偏移。
    // 实际做法是 HAL 自己用宏展开，对 Rust 不直接可见，所以这里只是个 stub。
    // 用 HAL 时避开需要外部锁的场景即可。
    let _ = h;
    false
}
