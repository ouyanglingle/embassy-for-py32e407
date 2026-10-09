//! # cbind-demo — PY32E407：UART2 + DMA 收发，同时 PC3 每 1 ms 翻转
//!
//! 演示如何用 [`py32e407cbind_rs`]（把 PUYA 官方 **C LL 驱动**编译成静态库、再用 bindgen
//! 生成 FFI 绑定的薄封装）走一遍正常启动流程，然后：
//!
//! * **UART2 用 DMA 收发**：`DMA1` 通道 1 负责发送（内存 → `UART2.DR`），
//!   通道 2 负责接收（`UART2.DR` → 内存）；收满一块就原样回显，另每秒发一条心跳；
//! * **PC3 每 1 ms 翻转电平**：高/低各 1 ms，即 **2 ms 周期、500 Hz** 的方波。
//!
//! 移植依据是官方 LL 例程
//! `参考设计/PY32E407_Firmware_V0.7.0/Projects/PY32E407-STK/Example_LL/UART/UART_HyperTerminal_DMA`
//! —— 通道号、请求号映射、引脚与 AF 全部照抄，只把「中断驱动」换成了「轮询」（原因见下）。
//!
//! ## 一、启动顺序（对照官方 C 的 `Reset_Handler` / `SystemInit()` / `main()`）
//!
//! | 阶段 | 官方 C 工程 | 本示例 |
//! |---|---|---|
//! | 复位 | `Reset_Handler` 取 SP、`SystemInit()` | `cortex-m-rt` 的 `Reset`（SP/VTOR/`.data`/`.bss`/FPU） |
//! | 电压档 | `SystemInit()` 清 `PWR_CR1.VOS` / `SRAM_RETV` | [`pwr_high_voltage`] |
//! | 时钟树 | `APP_SystemClockConfig()` → `LL_PLL_ConfigSystemClock_HSE()` | [`configure_system_clock`] |
//! | 时基 | `LL_Init1msTick(SystemCoreClock)` | [`setup_1ms_tick`] |
//! | GPIO | `LL_AHB2_GRP1_EnableClock()` + `LL_GPIO_Init()` | [`pc3_output_init`] |
//! | 串口 | `LL_UART_Init(UART2, …)` | [`uart2_init`] |
//! | DMA | `LL_DMA_ConfigTransfer()` + `LL_SYSCFG_SetDMARemap()` | [`uart2_dma_init`] |
//! | 主循环 | 中断里搬数据 | 主循环里轮询 + 翻转 PC3 |
//!
//! ## 二、硬件
//!
//! | 信号 | 引脚 | 说明 |
//! |---|---|---|
//! | LED / 方波输出 | **PC3** | 推挽输出，每 1 ms 翻转（示波器接这里） |
//! | UART2_TX | **PC12** | AF5，推挽 + 上拉（与官方 STK 例程一致） |
//! | UART2_RX | **PD2** | AF5，推挽 + 上拉（同上） |
//!
//! 串口参数 **115200 8N1，无流控**。PC12/PD2 是官方 PY32E407-STK 板接板载 USB 转串口的位置；
//! 若你的板子走别的引脚，改 [`UART2_TX_PORT`] / [`UART2_TX_PIN`] 等常量与 [`UART2_AF`] 即可，
//! 其余逻辑不用动。
//!
//! ## 三、DMA 怎么接上 UART（PY32 与 STM32 在这里差别很大）
//!
//! PY32E407 的 DMA 是「**每条通道都能映射到任意一个硬件请求号**」的结构
//! （类似 STM32L4/G0 的 DMAMUX），而不是 STM32F4 那种「通道 ↔ 外设」固定死。
//! 所以每条通道要**成对**配置两处，缺一处通道就永远等不到请求：
//!
//! ```text
//! ① 通道自己的请求号：  LL_DMA_SetDstPeriphMap(DMA1, CH1, LL_DMA_DST_PERIPH_MAP_1)
//! ② 请求号指向谁：      LL_SYSCFG_SetDMARemap (DMA1, LL_SYSCFG_DMA_PERIPH_MAP_1, LL_SYSCFG_DMA_MAP_UART2_TX)
//! ```
//!
//! ⚠️ ②写的是 `SYSCFG->CFGR`，所以**必须先开 SYSCFG 时钟**（`LL_APB2_GRP1_EnableClock`），
//! 否则 remap 静默失效。本示例在 [`uart2_dma_init`] 里开了。
//!
//! 本示例沿用官方例程的分配：
//!
//! | 用途 | 通道 | 请求号 | 方向 | 握手 |
//! |---|---|---|---|---|
//! | UART2_TX | `DMA1` CH1 | 1（`..._MAP_1`） | 内存 → 外设 | 目的端硬件握手 |
//! | UART2_RX | `DMA1` CH2 | 2（`..._MAP_2`） | 外设 → 内存 | 源端硬件握手 |
//!
//! ## 四、为什么用轮询而不是中断
//!
//! 官方例程用 `NVIC_EnableIRQ(DMA1_Channel1_IRQn)` + 中断回调。本工程**没有中断向量表**：
//! 向量表来自 `cortex-m-rt`，而中断项要由 linker 的 `device.x` 里的 `__INTERRUPTS` 提供，
//! 本工程已不依赖 `py32e407-pac`（它才带 `device.x`）。
//!
//! 与其为了一个 demo 造一份向量表，这里改成**在主循环里轮询完成标志**，行为等价、依赖更少：
//! 仍然把 DMA 的中断屏蔽位打开（`LL_DMA_EnableIT_TFR`）但**不碰 NVIC**，
//! 保证 `STATUSTFR` 的完成标志可靠置位；判断完成只看标志，不看中断。
//!
//! 代价是没有「立即响应」：最坏晚 1 ms 才处理（主循环节拍就是 1 ms），
//! 对 115200 波特率下 16 字节 ≈ 1.4 ms 的传输来说完全够用。
//!
//! ## 五、`unsafe` 与「不碰寄存器」
//!
//! 调用 `extern "C"` 函数在 Rust 里必须 `unsafe`，这是 FFI 的固有成本，**无法靠绑定生成消除**。
//! 但除此之外，本文件**没有任何寄存器操作**：外设指针全部来自设备头的基址宏
//! （`UART2_BASE` / `DMA1_BASE` / `GPIOC_BASE`…），配置与读写全部走官方 C LL 函数。
//!
//! 少数 LL 常量（`LL_DMA_DIRECTION_MEMORY_TO_PERIPH`、`LL_UART_DATAWIDTH_8B` …）官方写成
//! `((uint32_t)某位宏)`，bindgen 解析不了这种 C 风格转换会**静默丢弃**整条宏；
//! `py32e407cbind-rs` 已在 `src/ll_consts.rs` 按官方定义补回，所以这里照样写官方名字。
//!
//! ## 六、怎么验证
//!
//! 1. `cargo run --release`（probe-rs 烧录）——RTT 里应看到启动日志与每收到的字节数；
//! 2. 串口终端连 **115200 8N1**：能看到每秒一条 `cbind-demo alive`；**发满 16 个字符**后
//!    会原样回显（16 字节一块，与官方例程同一模型）；
//! 3. 示波器 / 逻辑分析仪接 **PC3**：500 Hz 方波。
//!
//! ⚠️ PC3 的 1 ms 是 `LL_mDelay` **忙等**出来的。RTT 日志、以及回显时的内存拷贝
//! 都会打断它，所以方波会有毫秒级抖动 —— 要纯净波形就把日志去掉。

#![no_std]
#![no_main]
// 由 bindings.rs 生成的 C 风格命名（`SYSCLK_Frequency`、`Pin`、`LL_RCC_*` …）
// 不符合 Rust 命名规范属常态，与 py32e407cbind-rs 一样在顶层 allow 掉。
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

use cortex_m_rt::entry;
// 把 defmt-rtt 注册为全局 logger。必须写成 `use ... as _;`：不这样写该 crate
// 不会被链接进来、日志发不出去，且 `.defmt` 段缺少版本标记，probe-rs 无法解码。
use defmt_rtt as _;
// 同理必须显式"用一下" `cortex-m`：defmt-rtt 内部用 `critical-section` 做日志互斥，
// 而 `_critical_section_1_0_acquire/_release` 这两个符号由 `cortex-m` 的
// `critical-section-single-core` feature 提供。本示例没有任何代码直接引用 `cortex-m`，
// 若不写这一行，链接器会认为该 crate 未被使用而不把它拉进来，链接报
// `undefined symbol: _critical_section_1_0_acquire`。
use cortex_m as _;

use py32e407cbind_rs as ll;

// ===========================================================================
// 一、链接期符号：官方 C LL 驱动依赖，但不随 libpy32e407_ll.a 提供
// ===========================================================================
//
// `arm-none-eabi-nm -u` 显示 `py32e407_ll_rcc.o` / `py32e407_ll_utils.o` 引用了这三个
// 符号，而官方把它们定义在 `system_py32e407.c` 里 —— 本 crate 只编译 `py32e407_ll_*.c`，
// 所以必须由最终 bin 提供，否则链接报 undefined reference。
// （数值与 `system_py32e407.c` 中定义逐字一致。）

/// 芯片复位默认主频：HSI16 = 16 MHz。
const HSI_HZ: u32 = 16_000_000;

/// 本板 HSE 晶振频率：8 MHz。
const HSE_HZ: u32 = 8_000_000;

/// CMSIS 全局变量 `SystemCoreClock`（HCLK 频率，Hz）。
///
/// `LL_RCC_GetSystemClocksFreq` 读它、`LL_PLL_ConfigSystemClock_HSE` 内部也用它判断
/// 「是否需要先升 Flash 等待周期」，`LL_SetSystemCoreClock` 则写它。
/// 复位默认时钟是 HSI16，故初值与官方 `system_py32e407.c` 一致取 [`HSI_HZ`]。
#[no_mangle]
pub static mut SystemCoreClock: u32 = HSI_HZ;

/// CMSIS AHB 预分频表：`AHBPrescTable[HPRE]` 即实际分频比。
/// `py32e407_ll_rcc.o` / `py32e407_ll_utils.o` 引用（由 `LL_RCC_GetSystemClocksFreq` /
/// `LL_PLL_ConfigSystemClock_HSE` 计算 HCLK 用）。
#[no_mangle]
pub static AHBPrescTable: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 6, 7, 8, 9];

/// CMSIS APB 预分频表：`APBPrescTable[PPRE]` 即实际分频比。
/// `py32e407_ll_rcc.o` 引用（算 PCLK1/PCLK2 用）。
#[no_mangle]
pub static APBPrescTable: [u8; 8] = [0, 0, 0, 0, 1, 2, 3, 4];

// ===========================================================================
// 二、启动流程
// ===========================================================================

/// 选用哪套启动时钟（只改下面的 [`BOOT_CLOCK`] 即可整体切换）。
// 两个变体都保留供选择，当前只用了 HsePll168，故允许未构造变体。
#[allow(dead_code)]
#[derive(Clone, Copy)]
enum BootClock {
    /// **本板正常档**：HSE 8 MHz → PLL `÷1 ×21 ÷1` = 168 MHz；AHB /1、APB1&APB2 /2
    /// → PCLK1 = PCLK2 = 84 MHz（DS：HCLK ≤ 170 MHz、PCLK ≤ 85 MHz）。
    /// 与同工作区 `demo-py32` 的第二类默认预设一致。
    HsePll168,
    /// **无晶振兜底**：PLL 源改用 `HSI/2` = 8 MHz，同样 `×21` = 168 MHz，分频同上。
    HsiPll168,
}

/// 启动时钟选择。
const BOOT_CLOCK: BootClock = BootClock::HsePll168;

/// 官方 `SystemInit()` 里的关键一步：把 VDDD 的 LDO 切到**高性能电压档**。
///
/// `PWR_CR1.VOS` / `SRAM_RETV` 属**备份域，系统复位不会清零**——复位后可能停在低压档，
/// 此时 HCLK 上不去（高频下 PLL 根本不锁定，表现为一直等 `PLLRDY`）。
///
/// 官方 C 的 `SystemInit()` 用**裸寄存器写**做这三步：
/// `SET_BIT(RCC->APB1ENR1, PWREN)` → `CLEAR_BIT(PWR->CR1, VOS)` →
/// `CLEAR_BIT(PWR->CR1, SRAM_RETV)` → `CLEAR_BIT(RCC->APB1ENR1, PWREN)`。
///
/// 本示例改用 **LL 的等价函数**：
/// * 开/关 PWR 时钟 → [`ll::LL_APB1_GRP1_EnableClock`] / `DisableClock`；
/// * 清 `VOS` → [`ll::LL_PWR_SetRegulVoltageScaling`]（`LL_PWR_REGU_VOLTAGE_HIGH` == 0）；
/// * 清 `SRAM_RETV` → [`ll::LL_PWR_SetSramVoltageInStandby`]（`LL_PWR_SRAM2_VOLTAGE_HIGH` == 0）。
///
/// 注意 PY32 的 PWR 挂在 **APB1**（`APB1ENR1.PWREN`），与 STM32F4 的 `APB1ENR.PWREN` 不同名。
fn pwr_high_voltage() {
    unsafe {
        // 1. 使能 PWR 时钟（APB1ENR1.PWREN）
        ll::LL_APB1_GRP1_EnableClock(ll::LL_APB1_GRP1_PERIPH_PWR);

        // 2. VOS = 高性能档（=0）、SRAM_RETV = 0 —— 与官方 SystemInit 的两句 CLEAR_BIT 等价
        ll::LL_PWR_SetRegulVoltageScaling(ll::LL_PWR_REGU_VOLTAGE_HIGH);
        ll::LL_PWR_SetSramVoltageInStandby(ll::LL_PWR_SRAM2_VOLTAGE_HIGH);

        // 3. 关掉 PWR 时钟（对齐官方 SystemInit；后续不再用 PWR）
        ll::LL_APB1_GRP1_DisableClock(ll::LL_APB1_GRP1_PERIPH_PWR);
    }
}

/// 配置系统时钟树（对照官方 `main()` 的 `APP_SystemClockConfig()` +
/// `LL_PLL_ConfigSystemClock_HSE()`）。
///
/// `LL_PLL_ConfigSystemClock_HSE` 一条调用内部就完成了整套流程：
/// 使能 HSE 并等 `HSERDY` → 按 `HSEPrediv` 给 PLL 输入分频 → 配 `Prediv` / `PLLMul`
/// / `Postdiv` → 升 Flash 等待周期 → （目标 > 80 MHz 时先经 HPRE=/2 过渡）
/// 写 `CFGR.SW` 选 PLL 并轮询 `SWS` 确认 → 设 `PPRE1` / `PPRE2` → 调
/// `LL_SetSystemCoreClock()` 更新全局 `SystemCoreClock`。
///
/// # 注意
/// * `LL_UTILS_PLLInitTypeDef` 的 `PLLMul` 是**原始倍频数（2~257）**，不是枚举位域；
///   硬件写入的是 `PLLMul - 2`，这是官方 LL 的约定。
/// * ⚠️ 本芯片头文件里 `ErrorStatus` 是 **`SUCCESS = 0, ERROR = 1`**（与经典 STM32 的
///   `ERROR = 0, SUCCESS = 1` **相反**），务必按 `== 0` 判成功。
/// * ⚠️ HSE 若不起振，官方实现会在 `while (LL_RCC_HSE_IsReady() != 1U)` 里**死等**。
///   板子没有晶振时请把 [`BOOT_CLOCK`] 改成 [`BootClock::HsiPll168`]。
fn configure_system_clock() {
    // PLL 参数：(HSEPrediv, PLLMul, Prediv, Postdiv)
    let mut pll = ll::LL_UTILS_PLLInitTypeDef {
        HSEPrediv: ll::LL_RCC_HSE_PREDIV_DIV_1, // HSE /1 = 8 MHz（LL 要求 PLL 输入 2~32 MHz）
        PLLMul: 21,                             // ×21 → 168 MHz
        Prediv: ll::LL_RCC_PREDIV_DIV_1,        // PLL 前分频 /1
        Postdiv: ll::LL_RCC_POSTDIV_DIV_1,      // PLL 后分频 /1
    };
    // 总线分频：HCLK = 168 MHz；PCLK1 = PCLK2 = 168/2 = 84 MHz（APB 上限 85 MHz）
    let mut clk = ll::LL_UTILS_ClkInitTypeDef {
        AHBCLKDivider: ll::LL_RCC_SYSCLK_DIV_1,
        APB1CLKDivider: ll::LL_RCC_APB1_DIV_2,
        APB2CLKDivider: ll::LL_RCC_APB2_DIV_2,
    };

    let status = unsafe {
        match BOOT_CLOCK {
            // HSEBYPASS_OFF：用外部晶体（非旁路输入）
            BootClock::HsePll168 => ll::LL_PLL_ConfigSystemClock_HSE(
                HSE_HZ,
                ll::LL_UTILS_HSEBYPASS_OFF,
                &mut pll,
                &mut clk,
            ),
            BootClock::HsiPll168 => ll::LL_PLL_ConfigSystemClock_HSI(&mut pll, &mut clk),
        }
    };

    if status != 0 {
        defmt::error!("clock config FAILED (ErrorStatus = {})", status);
        spin();
    }
}

/// 配置 SysTick 为 **1 ms** 时基（官方 `LL_Init1msTick(SystemCoreClock)`）。
///
/// [`ll::LL_mDelay`] 的实现是「每等一个 `SysTick.CTRL.COUNTFLAG` 就减 1」，
/// 所以**必须先调用本函数、再调 `LL_mDelay`**，否则延时长度完全不对。
///
/// 这里用 `LL_RCC_GetSystemClocksFreq`（`.c` 里定义的、可 FFI 的真实实现）
/// **读回硬件实际频率**，而不是按配置推算的纸面值 —— 分频写错时能立刻从日志看出来。
fn setup_1ms_tick() {
    let clocks = ll::LlRccClocks::read();
    defmt::info!(
        "clocks: sys={} Hz, hclk={} Hz, pclk1={} Hz, pclk2={} Hz",
        clocks.SYSCLK_Frequency,
        clocks.HCLK_Frequency,
        clocks.PCLK1_Frequency,
        clocks.PCLK2_Frequency
    );

    // SysTick 的 reload 按 HCLK 算；顺手把 `SystemCoreClock` 与硬件对齐
    // （HSE 路径里 `LL_PLL_ConfigSystemClock_HSE` 已写过一次，这里对两条路径都显式写）
    unsafe {
        ll::LL_SetSystemCoreClock(clocks.HCLK_Frequency);
        ll::LL_Init1msTick(clocks.HCLK_Frequency);
    }
}

// ===========================================================================
// 三、GPIO：PC3 翻转
// ===========================================================================

/// GPIOC 端口指针。
///
/// `GPIOC_BASE` 是设备头（`py32e407xE.h`）里的宏 `#define GPIOC_BASE (0x48000800UL)`，
/// 由 bindgen 导出。直接当成指针用即可 —— 这样不需要引入 PAC 就能拿到 `*mut GpioType`。
const GPIOC: *mut ll::GpioType = ll::GPIOC_BASE as *mut ll::GpioType;

/// 把 **PC3** 配成推挽输出。
///
/// 官方 C 是两步：
/// 1. `LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOC)` —— 使能 GPIOC 时钟。
///    PY32 的 GPIO 挂 **AHB2**，且位序与 STM32 不同：STM32 从 bit0 起（IOPAEN=bit0），
///    PY32 从 bit2 起（IOPAEN=bit2 → IOPCEN=bit4）。**传常量即可，别手写掩码。**
/// 2. `LL_GPIO_Init()` —— 在 `.c` 里定义，本来就链接得到。
///
/// 两步全程不碰寄存器，写的都是官方函数名。
fn pc3_output_init() {
    // 1. 使能 GPIOC 时钟
    unsafe { ll::LL_AHB2_GRP1_EnableClock(ll::LL_AHB2_GRP1_PERIPH_GPIOC) };

    // 2. 走官方 C LL 填结构体：先取官方默认值（`LL_GPIO_StructInit` 在 `.c` 里，可 FFI），
    //    再覆盖本引脚需要的字段。
    let mut cfg = ll::LlGpioInit::zeroed();
    unsafe { ll::LL_GPIO_StructInit(&mut cfg) };
    cfg.Pin = ll::LL_GPIO_PIN_3; // 1 << 3
    cfg.Mode = ll::LL_GPIO_MODE_OUTPUT; // 通用推挽输出
    cfg.Speed = ll::LL_GPIO_SPEED_FREQ_LOW; // 1 ms 级翻转，低速档足够且 EMI 更小
    cfg.OutputType = ll::LL_GPIO_OUTPUT_PUSHPULL;
    cfg.Pull = ll::LL_GPIO_PULL_NO; // 推挽输出不需要上下拉

    unsafe {
        ll::LL_GPIO_Init(GPIOC, &mut cfg);
    }
}

/// 设置 PC3 电平：`high = true` 拉高、`false` 拉低。
///
/// 用官方的 `LL_GPIO_SetOutputPin` / `LL_GPIO_ResetOutputPin`（原本是头文件里的
/// `__STATIC_INLINE`，现由 cbind 的 `wrap_static_fns` 外部化）。它们内部是往 `BSRR`
/// 低 16 位 / `BRR` 低 16 位写 1，属**单次原子写**，不存在读-改-写竞态 ——
/// 若将来在中断里也翻转同一引脚，这点很关键。（因此不用「读 `ODR` 再改写」那种写法。）
#[inline]
fn pc3_set(high: bool) {
    unsafe {
        if high {
            ll::LL_GPIO_SetOutputPin(GPIOC, ll::LL_GPIO_PIN_3);
        } else {
            ll::LL_GPIO_ResetOutputPin(GPIOC, ll::LL_GPIO_PIN_3);
        }
    }
}

// ===========================================================================
// 四、UART2：引脚、参数
// ===========================================================================

/// UART2 外设指针（`#define UART2_BASE (0x40005000UL)`）。
///
/// ⚠️ PY32 同时有 **UART1/2/3** 与 **USART1/2/3**，是两套独立外设、寄存器布局也不同：
/// `UART_TypeDef` 的 `DR` 在**偏移 0x00**，`USART_TypeDef` 的 `DR` 在 0x04。
/// 本示例用 UART2，对应 `LL_UART_*` 函数族（不是 `LL_USART_*`）。
const UART2: *mut ll::UartType = ll::UART2_BASE as *mut ll::UartType;

/// UART2 数据寄存器 `DR` 的地址 —— DMA 的固定一端必须指向它。
///
/// `LL_DMA_SetDstAddress` / `SetSrcAddress` 只吃 `u32` 地址，而 `UartType` 在 Rust 侧是
/// opaque（拿不到字段偏移），所以这里按设备头的结构体布局写死偏移：
/// `UART_TypeDef { DR@0x00, BRR@0x04, RESERVED1[2], SR@0x10, CR1@0x14, … }`。
/// 换成 USART 时要改成 `+ 0x04`。
const UART2_DR_ADDR: u32 = ll::UART2_BASE;

/// UART2 波特率。`LL_UART_Init` 内部会用 `LL_RCC_GetSystemClocksFreq()` 读回
/// **真实 PCLK1** 再算 `BRR`，所以主频改了这里不用跟着改。
const UART_BAUD: u32 = 115_200;

/// UART2_TX 引脚所在的端口（PC12）。
const UART2_TX_PORT: *mut ll::GpioType = ll::GPIOC_BASE as *mut ll::GpioType;
/// UART2_TX 引脚号。
const UART2_TX_PIN: u32 = ll::LL_GPIO_PIN_12;
/// UART2_RX 引脚所在的端口（PD2）。
const UART2_RX_PORT: *mut ll::GpioType = ll::GPIOD_BASE as *mut ll::GpioType;
/// UART2_RX 引脚号。
const UART2_RX_PIN: u32 = ll::LL_GPIO_PIN_2;
/// UART2 的复用功能编号（官方例程用的是 AF5，TX/RX 同号）。
const UART2_AF: u32 = ll::LL_GPIO_AF_5;

/// 初始化 UART2：引脚复用 + 引脚参数 + 串口参数。
///
/// 注意 PY32 的 UART **没有「使能位」**（LL 里也没有 `LL_UART_Enable`）：时钟一开、
/// `LL_UART_Init` 配完就直接开始工作 —— 官方 LL 例程同样没有额外的使能步骤。
fn uart2_init() {
    unsafe {
        // 端口时钟（GPIO 挂 AHB2）+ 外设时钟（UART2 挂 APB1）
        ll::LL_AHB2_GRP1_EnableClock(ll::LL_AHB2_GRP1_PERIPH_GPIOC);
        ll::LL_AHB2_GRP1_EnableClock(ll::LL_AHB2_GRP1_PERIPH_GPIOD);
        ll::LL_APB1_GRP1_EnableClock(ll::LL_APB1_GRP1_PERIPH_UART2);
    }

    // TX / RX 引脚：复用推挽 + 上拉（与官方 `APP_ConfigUart()` 同款配置）
    let mut gpio = ll::LlGpioInit::zeroed();
    unsafe { ll::LL_GPIO_StructInit(&mut gpio) };
    gpio.Mode = ll::LL_GPIO_MODE_ALTERNATE;
    gpio.Speed = ll::LL_GPIO_SPEED_FREQ_VERY_HIGH;
    gpio.OutputType = ll::LL_GPIO_OUTPUT_PUSHPULL;
    gpio.Pull = ll::LL_GPIO_PULL_UP;
    gpio.Alternate = UART2_AF;

    gpio.Pin = UART2_TX_PIN;
    unsafe { ll::LL_GPIO_Init(UART2_TX_PORT, &mut gpio) };
    gpio.Pin = UART2_RX_PIN;
    unsafe { ll::LL_GPIO_Init(UART2_RX_PORT, &mut gpio) };

    // 串口参数：115200 8N1，LSB first
    let mut cfg = ll::LlUartInit::zeroed();
    unsafe { ll::LL_UART_StructInit(&mut cfg) };
    cfg.BaudRate = UART_BAUD;
    cfg.DataWidth = ll::LL_UART_DATAWIDTH_8B;
    cfg.StopBits = ll::LL_UART_STOPBITS_1;
    cfg.Parity = ll::LL_UART_PARITY_NONE;
    cfg.BitOrder = ll::LL_UART_BITORDER_LSBFIRST;

    // `ErrorStatus`：SUCCESS = 0（与本芯片的 STM32 血统相反，别写反）
    let status = unsafe { ll::LL_UART_Init(UART2, &mut cfg) };
    if status != 0 {
        defmt::error!(
            "UART2 init FAILED (ErrorStatus = {}，检查 UART2 是否处于 BUSY)",
            status
        );
        spin();
    }
}

// ===========================================================================
// 五、DMA：把 UART2 挂上 DMA1 CH1(TX) / CH2(RX)
// ===========================================================================

/// DMA1 外设指针（`#define DMA1_BASE (AHB1PERIPH_BASE + 0x00000000UL)`）。
const DMA1: *mut ll::DmaType = ll::DMA1_BASE as *mut ll::DmaType;

/// 发送通道：官方例程里 UART2_TX 用 `DMA1_Channel1`。
const DMA_CH_TX: u32 = ll::LL_DMA_CHANNEL_1;
/// 接收通道：官方例程里 UART2_RX 用 `DMA1_Channel2`。
const DMA_CH_RX: u32 = ll::LL_DMA_CHANNEL_2;

/// 接收块长度（字节）：**收满这么多才触发一次处理**。
///
/// 这是 DMA 的「定长块」模型，与官方 `UART_HyperTerminal_DMA` 例程一致 ——
/// 用法是「上位机一次发 16 个字符，板子原样回 16 个」。
///
/// 想改成「来一个字节就能读到」，要么把这里改成 1（每字节重挂一次，会有丢字节窗口），
/// 要么改用循环模式（`LL_DMA_MODE_CIRCULAR`）并读 `CH_CTRLH` 的剩余长度 ——
/// 后者依赖「传输中 `CTRLH` 是否递减」这一手册没写死的行为，请先实测再采用。
const RX_BLOCK: usize = 16;

/// 初始化两条 DMA 通道，并把请求号映射到 UART2。**必须在 [`uart2_init`] 之后调用**
/// （UART 未初始化时它的 TXE/RXNE 请求没有意义）。
fn uart2_dma_init() {
    unsafe {
        // DMA1 挂 AHB1
        ll::LL_AHB1_GRP1_EnableClock(ll::LL_AHB1_GRP1_PERIPH_DMA1);
        // ⚠️ `LL_SYSCFG_SetDMARemap` 写的是 `SYSCFG->CFGR`：不开这个时钟，remap 静默失效
        ll::LL_APB2_GRP1_EnableClock(ll::LL_APB2_GRP1_PERIPH_SYSCFG);
    }

    // ---- 通道 1：内存 → UART2.DR（发送）----
    //
    // 用官方的结构体入口 `LL_DMA_Init`（在 `.c` 里定义，是真实可链接的函数），
    // 字段含义与官方例程里那串 `LL_DMA_ConfigTransfer(... | ...)` 位或写法一一对应：
    // `Direction` ↔ `TT`、`SrcIncMode`/`DstIncMode` ↔ `SINC`/`DINC`、
    // `SrcWidth`/`DstWidth` ↔ `*_TR_WIDTH`、`Priority` ↔ `CH_PRIOR`、
    // `*BurstLen` ↔ `*_MSIZE`、`*HandshakeType` ↔ `HS_SEL_*`。
    let mut tx = ll::LlDmaInit::zeroed();
    unsafe { ll::LL_DMA_StructInit(&mut tx) };
    tx.Direction = ll::LL_DMA_DIRECTION_MEMORY_TO_PERIPH;
    tx.SrcIncMode = ll::LL_DMA_SRC_ADDR_INC; // 内存侧地址递增
    tx.DstIncMode = ll::LL_DMA_DST_ADDR_FIX; // 外设侧地址固定（始终写 DR）
    tx.SrcWidth = ll::LL_DMA_SRC_WIDTH_BYTE; // 两边都是 8 位，正好一个 UART 数据帧
    tx.DstWidth = ll::LL_DMA_DST_WIDTH_BYTE;
    tx.SrcBurstLen = ll::LL_DMA_SRC_BURST_LEN_1; // 官方例程就是 1
    tx.DstBurstLen = ll::LL_DMA_DST_BURST_LEN_1;
    tx.SrcHandshakeType = ll::LL_DMA_SRC_HANDSHAKE_TYPE_SOFT; // 内存侧无需握手
    tx.DstHandshakeType = ll::LL_DMA_DST_HANDSHAKE_TYPE_HARD; // 外设侧用硬件请求（TXE）
    tx.Priority = ll::LL_DMA_PRIORITY_1;
    unsafe {
        ll::LL_DMA_Init(DMA1, DMA_CH_TX, &mut tx);
        // 成对配置：本通道用「请求号 1」，SYSCFG 把「请求号 1」接到 UART2_TX
        ll::LL_DMA_SetDstPeriphMap(DMA1, DMA_CH_TX, ll::LL_DMA_DST_PERIPH_MAP_1);
        ll::LL_SYSCFG_SetDMARemap(
            DMA1,
            ll::LL_SYSCFG_DMA_PERIPH_MAP_1,
            ll::LL_SYSCFG_DMA_MAP_UART2_TX,
        );
    }

    // ---- 通道 2：UART2.DR → 内存（接收）----
    let mut rx = ll::LlDmaInit::zeroed();
    unsafe { ll::LL_DMA_StructInit(&mut rx) };
    rx.Direction = ll::LL_DMA_DIRECTION_PERIPH_TO_MEMORY;
    rx.SrcIncMode = ll::LL_DMA_SRC_ADDR_FIX; // 外设侧地址固定
    rx.DstIncMode = ll::LL_DMA_DST_ADDR_INC; // 内存侧地址递增
    rx.SrcWidth = ll::LL_DMA_SRC_WIDTH_BYTE;
    rx.DstWidth = ll::LL_DMA_DST_WIDTH_BYTE;
    rx.SrcBurstLen = ll::LL_DMA_SRC_BURST_LEN_1;
    rx.DstBurstLen = ll::LL_DMA_DST_BURST_LEN_1;
    rx.SrcHandshakeType = ll::LL_DMA_SRC_HANDSHAKE_TYPE_HARD; // 外设侧用硬件请求（RXNE）
    rx.DstHandshakeType = ll::LL_DMA_DST_HANDSHAKE_TYPE_SOFT;
    rx.Priority = ll::LL_DMA_PRIORITY_2; // 比 TX 低一档：先保证发得出去
    unsafe {
        ll::LL_DMA_Init(DMA1, DMA_CH_RX, &mut rx);
        ll::LL_DMA_SetSrcPeriphMap(DMA1, DMA_CH_RX, ll::LL_DMA_SRC_PERIPH_MAP_2);
        ll::LL_SYSCFG_SetDMARemap(
            DMA1,
            ll::LL_SYSCFG_DMA_PERIPH_MAP_2,
            ll::LL_SYSCFG_DMA_MAP_UART2_RX,
        );
    }

    // 打开两条通道的「传输完成」中断屏蔽位，但**不动 NVIC**：
    // 本示例全程轮询，这里只是保证 `STATUSTFR` 的完成标志会可靠置位。
    unsafe {
        ll::LL_DMA_EnableIT_TFR(DMA1, DMA_CH_TX);
        ll::LL_DMA_EnableIT_TFR(DMA1, DMA_CH_RX);
    }
}

/// 启动一笔 DMA 发送。**非阻塞**，调用后立刻返回。
///
/// # 调用方约定
/// `buf` 必须在整笔传输期间**保持有效且内容不变** —— DMA 读的是它的物理地址，
/// 指针交给硬件后就"逃逸"了，Rust 的生命周期在这里帮不上忙。
/// 本示例只传两类缓冲：`'static` 的字节串字面量，和 `main` 栈帧里的局部数组
/// （`main` 是 `-> !`，栈帧永不返回，正合适）。
fn uart2_tx_start(buf: &[u8]) {
    unsafe {
        // 1. 改地址/长度必须在通道停止时做
        ll::LL_DMA_DisableChannel(DMA1, DMA_CH_TX);
        // 2. 清上次的完成标志：它是"置位保持"的，不清会误判为已完成
        ll::LL_DMA_ClearFlag_TFR1(DMA1);
        // 3. 配置这一笔：源 = 内存、目的 = UART2.DR、长度
        ll::LL_DMA_SetSrcAddress(DMA1, DMA_CH_TX, buf.as_ptr() as u32);
        ll::LL_DMA_SetDstAddress(DMA1, DMA_CH_TX, UART2_DR_ADDR);
        ll::LL_DMA_SetBlockLength(DMA1, DMA_CH_TX, buf.len() as u32);
        // 4. 开总开关 + 通道，再让 UART 开始发 TX 请求（官方例程同样每笔都开一次总开关）
        ll::LL_DMA_Enable(DMA1);
        ll::LL_DMA_EnableChannel(DMA1, DMA_CH_TX);
        ll::LL_UART_EnableDMAReq_TX(UART2);
    }
}

/// 「这一笔发送已把 `buf` 全部交给 UART」是否成立（DMA 传输完成标志）。
///
/// ⚠️ 这表示 DMA 搬完了，**不等于最后一个字节已经离开 TX 引脚**。
/// 要等最后一个字节真的发完，得再看 `LL_UART_IsActiveFlag_BUSY(UART2) == 0`
/// （本芯片的 UART 只有 TDRE/TXE，没有经典的 TC 位）。本示例不需要那么精确 ——
/// 收尾时直接断掉 DMA 请求即可，UART 会自己把移位寄存器里剩下的数据发完。
fn uart2_tx_done() -> bool {
    unsafe { ll::LL_DMA_IsActiveFlag_TFR1(DMA1) != 0 }
}

/// 一笔发送结束后的收尾（**正常完成与超时中止都走这里**）：
/// 断掉 UART 的 TX DMA 请求、关通道、清标志。
///
/// 断请求这步不能省：请求是硬件握手的，通道停下来后若请求还挂着，
/// 下一次重新使能通道时可能凭空多搬几次。
fn uart2_tx_finish() {
    unsafe {
        ll::LL_UART_DisableDMAReq_TX(UART2);
        ll::LL_DMA_DisableChannel(DMA1, DMA_CH_TX);
        ll::LL_DMA_ClearFlag_TFR1(DMA1);
    }
}

/// 武装一次 DMA 接收：收满 `buf.len()` 个字节即停（正常模式，与官方例程同一模型）。
///
/// 每次调用前都要重新设置长度 —— DMA 的块长度寄存器在传输结束后会归零。
fn uart2_rx_start(buf: &mut [u8]) {
    unsafe {
        ll::LL_DMA_DisableChannel(DMA1, DMA_CH_RX);
        ll::LL_DMA_ClearFlag_TFR2(DMA1);
        // 源 = UART2.DR（固定）、目的 = 内存（递增）
        ll::LL_DMA_SetSrcAddress(DMA1, DMA_CH_RX, UART2_DR_ADDR);
        ll::LL_DMA_SetDstAddress(DMA1, DMA_CH_RX, buf.as_mut_ptr() as u32);
        ll::LL_DMA_SetBlockLength(DMA1, DMA_CH_RX, buf.len() as u32);
        ll::LL_DMA_Enable(DMA1);
        ll::LL_DMA_EnableChannel(DMA1, DMA_CH_RX);
        // 清一次溢出标志：上一帧没及时取走数据留下的 ORE 会挡住后续接收
        ll::LL_UART_ClearFlag_ORE(UART2);
        ll::LL_UART_EnableDMAReq_RX(UART2);
    }
}

/// 「这一块接收是否已收满」。
fn uart2_rx_done() -> bool {
    unsafe { ll::LL_DMA_IsActiveFlag_TFR2(DMA1) != 0 }
}

/// 一块接收结束后的收尾（与 [`uart2_tx_finish`] 对称）。
fn uart2_rx_finish() {
    unsafe {
        ll::LL_UART_DisableDMAReq_RX(UART2);
        ll::LL_DMA_DisableChannel(DMA1, DMA_CH_RX);
        ll::LL_DMA_ClearFlag_TFR2(DMA1);
    }
}

/// DMA **发送通道的软件记账**。
///
/// # 为什么自己记账，而不是问硬件
///
/// 两个硬件细节官方手册没写死，不敢依赖：
/// * `CH_EN` 通道使能位在传输结束后**是否自动清零**（官方例程每笔都先 Disable 再 Enable，
///   从用法上看不出结论）；
/// * `STATUSTFR` 完成标志在中断被屏蔽时**是否仍会置位**。
///
/// 所以只把硬件当成一个"完成信号"来用，`busy` 由软件维护，并加**超时兜底** ——
/// 硬件一旦不响（请求映射写错、时钟没开…），发送路径也不会永久卡死，
/// 而是打一条日志后恢复，现场能直接看出问题在哪。
struct TxState {
    /// 是否有一笔发送正在进行。
    busy: bool,
    /// 已忙碌的毫秒数（由 [`TxState::poll`] 每毫秒累加）。
    busy_ms: u32,
}

impl TxState {
    /// 单笔发送的最长容忍时间（ms）。16 字节 @115200 ≈ 1.4 ms，
    /// 给到 100 ms 已极宽；超了说明配置有问题，直接放弃这一笔。
    const TIMEOUT_MS: u32 = 100;

    /// 初始状态：空闲。
    const fn new() -> Self {
        Self {
            busy: false,
            busy_ms: 0,
        }
    }

    /// 启动一笔发送。返回 `false` 表示上一笔还没结束（本次数据被丢弃）。
    ///
    /// 传 `buf` 的约定与 [`uart2_tx_start`] 相同：整笔传输期间必须保持有效且不变。
    fn start(&mut self, buf: &[u8]) -> bool {
        if self.busy {
            return false;
        }
        uart2_tx_start(buf);
        self.busy = true;
        self.busy_ms = 0;
        true
    }

    /// 推进记账。**每 1 ms 调用一次**（正好跟主循环节拍同步）。
    fn poll(&mut self) {
        if !self.busy {
            return;
        }
        if uart2_tx_done() {
            uart2_tx_finish();
            self.busy = false;
            return;
        }
        self.busy_ms += 1;
        if self.busy_ms >= Self::TIMEOUT_MS {
            defmt::warn!(
                "UART2 TX DMA 超时 {} ms，强制中止（检查 DMA 请求号↔SYSCFG remap 是否配对、SYSCFG 时钟是否已开）",
                Self::TIMEOUT_MS
            );
            uart2_tx_finish();
            self.busy = false;
        }
    }
}

// ===========================================================================
// 六、主程序
// ===========================================================================

/// 每秒发一条的心跳内容（`'static`，DMA 期间天然有效）。
const HEARTBEAT: &[u8] = b"cbind-demo alive\r\n";

/// 主程序：按正常启动顺序初始化，然后 1 kHz 节拍里翻转 PC3，同时用 DMA 收发 UART2。
#[entry]
fn main() -> ! {
    // ---- 0. 复位阶段（由 cortex-m-rt 接管，无需自己写启动汇编）----
    // 官方 C 的 `Reset_Handler` + `SystemInit()` 里与本芯片通用相关的事，cortex-m-rt 已做：
    //   * 从向量表取初始 SP、把表基址写进 `SCB.VTOR`；
    //   * `.data` 从 Flash 拷到 RAM、`.bss` 清零；
    //   * `thumbv7em-none-eabihf` 目标下打开 FPU（CPACR 的 CP10/CP11 全访问）。
    //
    // 官方 `SystemInit()` 里还有一段 `GPIOF` + `FLASH_OPTR2.NRST_MODE` 的握手等待。
    // 本示例**有意省略**：它只在 NRST 被选项字节配成 GPIO 的组合下才会真的等待，
    // 同工作区的 `demo-py32`（embassy-py32）同样没有这段且实机验证可用。

    // ---- 1. 电压档：跑高频的前提（必须在切到 PLL 之前）----
    pwr_high_voltage();

    // ---- 2. 系统时钟：HSE 8 MHz → PLL ×21 → 168 MHz，PCLK1/2 = 84 MHz ----
    configure_system_clock();

    // ---- 3. SysTick 1 ms 时基：LL_mDelay 的前提 ----
    setup_1ms_tick();

    // ---- 4. GPIO：PC3 推挽输出 ----
    pc3_output_init();

    // ---- 5. UART2 + DMA ----
    // 顺序不能反：DMA 的硬件请求来自 UART，UART 没配好之前它不会发请求。
    uart2_init();
    uart2_dma_init();

    // ---- 6. DMA 用的缓冲区：放在 `main` 的栈上 ----
    // `main` 是 `-> !`，栈帧永不返回，因此这两块内存在整个程序生命周期内地址稳定 ——
    // 正是 DMA 需要的。不需要 `static mut`，也就避开了别名与竞态问题。
    let mut rx = [0u8; RX_BLOCK];
    // 回显缓冲单独一块：正在被 DMA 读的缓冲**绝不能**被改写（见主循环里的说明）。
    let mut echo = [0u8; RX_BLOCK];

    uart2_rx_start(&mut rx);

    defmt::info!(
        "boot ok — UART2 {} 8N1 via DMA1 CH1(TX)/CH2(RX); PC3 toggling every 1 ms (500 Hz)",
        UART_BAUD
    );

    let mut tx = TxState::new();
    let mut high = false;
    let mut tick: u32 = 0;

    // ---- 7. 主循环：1 kHz 节拍 ----
    loop {
        // 7.1 翻转 PC3（高/低各 1 ms → 500 Hz 方波）
        pc3_set(high);
        high = !high;
        unsafe { ll::LL_mDelay(1) };
        tick = tick.wrapping_add(1);

        // 7.2 发送侧：记账 + 完成/超时收尾（非阻塞，不会打断上面的节拍）
        tx.poll();

        // 7.3 接收侧：收满一块就回显
        if uart2_rx_done() {
            uart2_rx_finish();

            // 回显缓冲只有在发送通道空闲时才能改写：`echo` 一旦交给 DMA，
            // 硬件就会一直按地址读它，此时 `copy_from_slice` 会让发出去的内容前后矛盾。
            if !tx.busy {
                echo.copy_from_slice(&rx);
                if tx.start(&echo) {
                    defmt::info!("RX {} B -> echo: {:a}", RX_BLOCK, &echo[..]);
                }
            } else {
                defmt::warn!("RX {} B 到达时 TX 仍忙，本块丢弃", RX_BLOCK);
            }

            // 无论有没有回显，都要立刻重新武装，避免后面的字节丢掉
            uart2_rx_start(&mut rx);
        }

        // 7.4 每秒一条心跳（复用同一条发送通道，TX 忙时自动跳过不阻塞）
        if tick % 1000 == 0 {
            tx.start(HEARTBEAT);
        }
    }
}

/// 死循环自旋（错误分支用）。`spin_loop` 只是给 CPU 一个「在忙等」的提示。
fn spin() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

/// 极简 panic 处理：先用 RTT 打一条带位置的日志（否则只能看到程序「静默卡死」，
/// 分不清是没跑起来还是 panic 了），再挂死原地等调试器接管。
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", defmt::Display2Format(info));
    spin()
}
