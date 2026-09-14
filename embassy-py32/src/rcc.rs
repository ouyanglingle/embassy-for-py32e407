//! Reset and Clock Control (RCC)（同步 Blocking 驱动）
//!
//! 适配 PY32E407 系列（Cortex-M4F，Puya）。寄存器流程依据：
//! - HAL C: `py32e407_hal_rcc.c`（`HAL_RCC_OscConfig` / `HAL_RCC_ClockConfig`）
//! - LL:    `py32e407_ll_rcc.h` / `py32e407_ll_bus.h`
//! - RM:    PY32E407 参考手册 V0.6 第 6 章（RCC，6.10 寄存器）、第 3 章（Flash 等待周期切换步骤）
//! - DS:    PY32E407 数据手册 V0.13（5.3.4 节 fHCLK-Flash 等待周期表；最高 170 MHz）
//!
//! 公开 API 形态对齐 embassy-stm32 的 `rcc` 模块（`Config` 字段名与 STM32F412 例程
//! 一致：`hse` / `pll_src` / `pll` / `sys` / `ahb_pre` / `apb1_pre` / `apb2_pre`），
//! 由 [`crate::init`] 统一调用；时钟结果经 [`clocks()`] 全局可查。
//!
//! 与 STM32 的关键差异（已对照 RM + SVD + PAC 交叉验证）：
//! - **SW/SWS 编码**：00=HSI16、01=HSE、10=PLL；
//! - **PLL 参数在 `CFGR3`**：`PLLSRC`(bit0，0=HSI/2、1=HSE)、`PLLXTPRE`(bit1)、
//!   `PLL_PREDIV[3:2]`(/1/2/4/8)、`PLL_POSTDIV[5:4]`(/1/2/4/8)、`PLL_FBDIV[15:8]`
//!   （倍频 = FBDIV + 2，范围 ×2..×257）；
//! - `HSION` 复位值为 1（CR 复位值 0x0080_2083），复位后 HSI16 已使能且就绪；
//! - Flash 等待周期表无 2 档：0-25:0 / 25-50:1 / 50-75:3 / 75-100:4 / 100-125:5 /
//!   125-150:6 / >150:7（DS 5.3.4）；
//! - GPIO 等时钟使能位在 `RCC_AHB2ENR`，GPIOA 从 bit2 起（与 ST 的 bit0 起始不同）；
//!   LL 使能宏在置位后强制读回一次 RCC 寄存器（"Delay after an RCC peripheral clock
//!   enabling"），驱动已复刻该动作。
//!
//! 仅实现同步（轮询 + 超时）驱动；不含中断（CIR 相关）与 LSI/LSE（RTC 备份域，
//! 需 PWR.DBP，后续按需扩展）。

use core::sync::atomic::{AtomicU32, Ordering};

use crate::pac;
pub use crate::Hertz;

/// HSI16 频率（`py32e407_ll_rcc.h` 的 `HSI_VALUE`，复位默认系统时钟源）。
pub const HSI_HZ: u32 = 16_000_000;
/// HCLK 最大频率（DS："最高频率 170 MHz"）。
pub const MAX_HCLK_HZ: u32 = 170_000_000;
/// APB（PCLK1/PCLK2）最大频率（DS 时钟树："APB 最高时钟为 85 MHz"）。
pub const MAX_PCLK_HZ: u32 = 85_000_000;

/// HSE 就绪等待上限（循环数）。对应 `HSE_STARTUP_TIMEOUT` = 100 ms
/// （`py32e407_hal_conf.h`）；16 MHz 下 2_000_000 次循环 ≈ 0.5~1 s，留足裕量。
const TIMEOUT_HSE_LOOPS: u32 = 2_000_000;
/// PLL 锁定等待上限（循环数）。对应 `PLL_TIMEOUT_VALUE` = 3 ms，已放大裕量。
const TIMEOUT_PLL_LOOPS: u32 = 500_000;
/// SYSCLK 切换（SWS 置位）等待上限（循环数）。对应 `CLOCKSWITCH_TIMEOUT_VALUE` = 5 s。
const TIMEOUT_SWITCH_LOOPS: u32 = 20_000_000;

/// RCC 错误类型。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// 等待就绪标志/时钟切换超时（见各 `TIMEOUT_*_LOOPS` 常量的量级依据）。
    Timeout,
    /// 配置非法：频率超上限（HCLK > 170 MHz / PCLK > 85 MHz）、PLL 倍频不在
    /// ×2..×257、选择了 HSE/PLL 但缺少对应配置等。
    InvalidConfig,
    /// Flash 等待周期写入后读回校验失败（RM 第 3 章要求读回确认）。
    FlashLatencyVerify,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            Self::Timeout => "Operation Timed Out",
            Self::InvalidConfig => "Invalid Configuration",
            Self::FlashLatencyVerify => "Flash Latency Verify Failed",
        };
        f.write_str(msg)
    }
}

impl core::error::Error for Error {}

/// HSE 晶振驱动能力（`py32e407_hal_rcc.h` 的 `RCC_HSE_*MHz`，CFGR2.HSEDRV[1:0]）。
///
/// 注意：V0.7.0 HAL 的 `IS_RCC_HSE_FREQ` 仅放行 4-8 MHz 档（其余两档被注释），
/// 使用其他档位前请结合数据手册确认晶振频段。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HseDrive {
    /// 4~8 MHz（00）。
    Low,
    /// 8~16 MHz（01）。
    Medium,
    /// 16~32 MHz（10）。
    High,
}

/// HSE 配置（对齐 embassy-stm32 的 `Hse`，`drive` 为 PY32 特有字段）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Hse {
    /// HSE 晶振/输入频率。板级请对照实际晶振填写。
    pub freq: Hertz,
    /// 工作模式。
    pub mode: HseMode,
    /// 晶振驱动能力（PY32 特有，CFGR2.HSEDRV），按晶振频段选择。
    pub drive: HseDrive,
}

/// HSE 工作模式（对齐 embassy-stm32 的 `HseMode`）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HseMode {
    /// 片内振荡器 + 外部晶振。
    Oscillator,
    /// 旁路模式（外部有源时钟输入，屏蔽晶振，CR.HSEBYP）。
    Bypass,
}

/// PLL 输入时钟源（对齐 embassy-stm32 的 `PllSource`；CFGR3.PLLSRC）。
///
/// 注意 PY32 的 `HSI` 档是 **HSI16 二分频（8 MHz）** 后作为 PLL 输入
/// （RM 6.10.19 PLLSRC=0），与 STM32 的 HSI 直入不同。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PllSource {
    /// HSI16 二分频（8 MHz）作为 PLL 输入。
    HSI,
    /// HSE 作为 PLL 输入（需 `Config::hse`）。
    HSE,
}

/// PLL 前分频比（CFGR3.PLL_PREDIV[3:2]）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PllPreDiv {
    /// 1 分频。
    DIV1,
    /// 2 分频。
    DIV2,
    /// 4 分频。
    DIV4,
    /// 8 分频。
    DIV8,
}

/// PLL 后分频比（CFGR3.PLL_POSTDIV[5:4]）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PllPostDiv {
    /// 1 分频。
    DIV1,
    /// 2 分频。
    DIV2,
    /// 4 分频。
    DIV4,
    /// 8 分频。
    DIV8,
}

/// PLL 配置（对齐 embassy-stm32 的 `Pll`）。
///
/// 倍频 `mul` 范围 ×2..×257（`IS_RCC_PLL_MUL`，写寄存器时按 FBDIV = mul − 2 编码；
/// PY32 倍频范围大，故用整数而非 STM32 的 `PllMul::MULn` 枚举）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pll {
    /// 倍频因子 ×2..×257。
    pub mul: u16,
    /// 前分频。
    pub prediv: PllPreDiv,
    /// 后分频。
    pub postdiv: PllPostDiv,
}

/// 系统时钟（SYSCLK）源选择（对齐 embassy-stm32 的 `Sysclk`）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Sysclk {
    /// HSI16 内部 RC（复位默认）。
    HSI,
    /// HSE 外部时钟（需 `Config::hse`）。
    HSE,
    /// PLL 输出（需 `Config::pll`）。
    PLL,
}

/// AHB 预分频（对齐 embassy-stm32 的 `AHBPrescaler`；CFGR.HPRE[3:0]，
/// RM：0xxx=/1、1000=/2、1001=/4、1010=/8、1011=/16、1100=/64、1101=/128、
/// 1110=/256、1111=/512）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AHBPrescaler {
    /// 1 分频。
    DIV1,
    /// 2 分频。
    DIV2,
    /// 4 分频。
    DIV4,
    /// 8 分频。
    DIV8,
    /// 16 分频。
    DIV16,
    /// 64 分频。
    DIV64,
    /// 128 分频。
    DIV128,
    /// 256 分频。
    DIV256,
    /// 512 分频。
    DIV512,
}

impl AHBPrescaler {
    /// 映射为 HPRE 的 4-bit 编码（RM 6.10.2；HAL `RCC_CFGR_HPRE_DIV1` = 0x00）。
    #[inline]
    fn to_bits(self) -> u8 {
        match self {
            Self::DIV1 => 0b0000,
            Self::DIV2 => 0b1000,
            Self::DIV4 => 0b1001,
            Self::DIV8 => 0b1010,
            Self::DIV16 => 0b1011,
            Self::DIV64 => 0b1100,
            Self::DIV128 => 0b1101,
            Self::DIV256 => 0b1110,
            Self::DIV512 => 0b1111,
        }
    }

    /// 分频值。
    #[inline]
    fn div(self) -> u32 {
        match self {
            Self::DIV1 => 1,
            Self::DIV2 => 2,
            Self::DIV4 => 4,
            Self::DIV8 => 8,
            Self::DIV16 => 16,
            Self::DIV64 => 64,
            Self::DIV128 => 128,
            Self::DIV256 => 256,
            Self::DIV512 => 512,
        }
    }
}

/// APB 预分频（对齐 embassy-stm32 的 `APBPrescaler`；CFGR.PPRE1/PPRE2[2:0]，
/// RM：0xx=/1、100=/2、101=/4、110=/8、111=/16）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum APBPrescaler {
    /// 1 分频。
    DIV1,
    /// 2 分频。
    DIV2,
    /// 4 分频。
    DIV4,
    /// 8 分频。
    DIV8,
    /// 16 分频。
    DIV16,
}

impl APBPrescaler {
    /// 映射为 PPRE 的 3-bit 编码（RM 6.10.2；HAL `RCC_CFGR_PPRE1_DIV1` = 0x00）。
    #[inline]
    fn to_bits(self) -> u8 {
        match self {
            Self::DIV1 => 0b000,
            Self::DIV2 => 0b100,
            Self::DIV4 => 0b101,
            Self::DIV8 => 0b110,
            Self::DIV16 => 0b111,
        }
    }

    /// 分频值。
    #[inline]
    fn div(self) -> u32 {
        match self {
            Self::DIV1 => 1,
            Self::DIV2 => 2,
            Self::DIV4 => 4,
            Self::DIV8 => 8,
            Self::DIV16 => 16,
        }
    }
}

/// RCC 时钟树配置（对齐 embassy-stm32 的 `rcc::Config` 字段命名）。
///
/// `#[non_exhaustive]`，外部构造请用 [`Config::default()`] 后逐字段赋值
/// （参见工程 demo 的 `enable80mhz()`）。
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub struct Config {
    /// HSE 配置（None = 不使用 HSE）。
    pub hse: Option<Hse>,
    /// PLL 输入源（`sys = PLL` 或 `pll = Some` 时生效）。
    pub pll_src: PllSource,
    /// PLL 配置（None = 不使用 PLL）。
    pub pll: Option<Pll>,
    /// SYSCLK 源。
    pub sys: Sysclk,
    /// AHB 预分频（HCLK = SYSCLK / ahb_pre）。
    pub ahb_pre: AHBPrescaler,
    /// APB1 预分频（PCLK1 = HCLK / apb1_pre，≤ 85 MHz）。
    pub apb1_pre: APBPrescaler,
    /// APB2 预分频（PCLK2 = HCLK / apb2_pre，≤ 85 MHz）。
    pub apb2_pre: APBPrescaler,
}

impl Default for Config {
    /// 复位默认时钟树：HSI16，全部分频器 1 分频（不动任何寄存器即可直接用）。
    fn default() -> Self {
        Self {
            hse: None,
            pll_src: PllSource::HSI,
            pll: None,
            sys: Sysclk::HSI,
            ahb_pre: AHBPrescaler::DIV1,
            apb1_pre: APBPrescaler::DIV1,
            apb2_pre: APBPrescaler::DIV1,
        }
    }
}

/// 配置完成后的实际时钟频率（Hz）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Clocks {
    /// SYSCLK 频率。
    pub sys: Hertz,
    /// AHB 总线时钟（HCLK，Cortex-M SysTick 也以此为源）。
    pub hclk: Hertz,
    /// APB1 外设时钟（PCLK1）。
    pub apb1: Hertz,
    /// APB2 外设时钟（PCLK2）。
    pub apb2: Hertz,
}

// 时钟结果的进程内全局副本（对齐 embassy 的 rcc::clocks()；单核同步驱动用原子量即可）
static SYS_HZ: AtomicU32 = AtomicU32::new(HSI_HZ);
static HCLK_HZ: AtomicU32 = AtomicU32::new(HSI_HZ);
static APB1_HZ: AtomicU32 = AtomicU32::new(HSI_HZ);
static APB2_HZ: AtomicU32 = AtomicU32::new(HSI_HZ);

/// 查询当前时钟频率（对齐 `embassy_stm32::rcc::clocks()`）。
///
/// [`init`](crate::init) 之前返回复位默认值（HSI16，全 1 分频）。
pub fn clocks() -> Clocks {
    Clocks {
        sys: Hertz(SYS_HZ.load(Ordering::Relaxed)),
        hclk: Hertz(HCLK_HZ.load(Ordering::Relaxed)),
        apb1: Hertz(APB1_HZ.load(Ordering::Relaxed)),
        apb2: Hertz(APB2_HZ.load(Ordering::Relaxed)),
    }
}

/// PllPreDiv 的分频值。
#[inline]
fn prediv_div(d: PllPreDiv) -> u32 {
    match d {
        PllPreDiv::DIV1 => 1,
        PllPreDiv::DIV2 => 2,
        PllPreDiv::DIV4 => 4,
        PllPreDiv::DIV8 => 8,
    }
}

/// PllPostDiv 的分频值。
#[inline]
fn postdiv_div(d: PllPostDiv) -> u32 {
    match d {
        PllPostDiv::DIV1 => 1,
        PllPostDiv::DIV2 => 2,
        PllPostDiv::DIV4 => 4,
        PllPostDiv::DIV8 => 8,
    }
}

/// SW/SWS 编码（RM 6.10.2：00=HSI16、01=HSE、10=PLL）。
#[inline]
fn sw_bits(sys: Sysclk) -> u8 {
    match sys {
        Sysclk::HSI => 0b00,
        Sysclk::HSE => 0b01,
        Sysclk::PLL => 0b10,
    }
}

/// Flash 等待周期（DS 5.3.4 注释中的 fHCLK 分档表；注意无 2 档）。
#[inline]
fn flash_wait_states(hclk_hz: u32) -> u8 {
    match hclk_hz {
        0..=25_000_000 => 0,
        25_000_001..=50_000_000 => 1,
        50_000_001..=75_000_000 => 3,
        75_000_001..=100_000_000 => 4,
        100_000_001..=125_000_000 => 5,
        125_000_001..=150_000_000 => 6,
        _ => 7,
    }
}

/// 读取 Flash ACR 当前等待周期。
#[inline]
fn get_flash_latency() -> u8 {
    let flash = unsafe { &*pac::Flash::PTR };
    (flash.acr().read().bits() & 0xF) as u8
}

/// 设置 Flash 等待周期并读回校验。
///
/// 注意 RM 3.x："在配置 latency 位时，该寄存器其他位不能写入"——此处按 HAL 的
/// `MODIFY_REG(FLASH->ACR, FLASH_ACR_LATENCY, v)` 方式仅改 LATENCY 字段、
/// 其余位保持原值写回（与官方 C 驱动一致）。
fn set_flash_latency(ws: u8) -> Result<(), Error> {
    let flash = unsafe { &*pac::Flash::PTR };
    flash
        .acr()
        .modify(|r, w| unsafe { w.bits((r.bits() & !0xF) | ws as u32) });
    // 读回确认（HAL_RCC_ClockConfig + RM 第 3 章软件步骤 2）
    if get_flash_latency() != ws {
        return Err(Error::FlashLatencyVerify);
    }
    Ok(())
}

/// 使能 HSE 并等待就绪。
///
/// 序列对照 `HAL_RCC_OscConfig` HSE 分支：先关 HSE 并等 HSERDY 清零 →
/// 写 HSEBYP（RM：CR.HSEBYP 只能在 HSEON=0 时写）→ 设 CFGR2.HSEDRV →
/// 开 HSE → 等 HSERDY 置位（超时 100 ms，见 `TIMEOUT_HSE_LOOPS`）。
fn enable_hse(rcc: &pac::rcc::RegisterBlock, hse: &Hse) -> Result<(), Error> {
    // HSE 正被用作系统时钟（直接或经 PLL）时不允许关闭重配（HAL_OscConfig 前置检查）
    let sws = rcc.cfgr().read().sws().bits();
    let pll_from_hse = rcc.cfgr3().read().pllsrc().bit_is_set();
    if sws == 0b01 || (sws == 0b10 && pll_from_hse) {
        return Err(Error::InvalidConfig);
    }

    // 1. 关 HSE，等待 HSERDY 清零
    rcc.cr().modify(|_, w| w.hseon().clear_bit());
    for _ in 0..TIMEOUT_HSE_LOOPS {
        if !rcc.cr().read().hserdy().bit_is_set() {
            break;
        }
    }
    if rcc.cr().read().hserdy().bit_is_set() {
        return Err(Error::Timeout);
    }

    // 2. 旁路选择（只能在 HSEON=0 时写）
    let bypass = hse.mode == HseMode::Bypass;
    rcc.cr().modify(move |_, w| {
        if bypass {
            w.hsebyp().set_bit()
        } else {
            w.hsebyp().clear_bit()
        }
    });

    // 3. 晶振驱动能力（CFGR2.HSEDRV）
    let drive = hse.drive;
    rcc.cfgr2()
        .modify(|_, w| unsafe { w.hsedrv().bits(drive as u8) });

    // 4. 开 HSE，等待 HSERDY 置位
    rcc.cr().modify(|_, w| w.hseon().set_bit());
    for _ in 0..TIMEOUT_HSE_LOOPS {
        if rcc.cr().read().hserdy().bit_is_set() {
            return Ok(());
        }
    }
    Err(Error::Timeout)
}

/// 配置并使能 PLL，等待锁定。
///
/// 序列对照 `HAL_RCC_OscConfig` PLL 分支：关 PLL 等 PLLRDY 清零 → 写 CFGR3
/// （RM：PLLSRC/PLLXTPRE 只能在 PLL 关闭时写）→ 开 PLL → 等 PLLRDY 置位。
fn setup_pll(rcc: &pac::rcc::RegisterBlock, pll: &Pll, src_is_hse: bool) -> Result<(), Error> {
    if !(2..=257).contains(&pll.mul) {
        return Err(Error::InvalidConfig);
    }

    // 关 PLL，等待 PLLRDY 清零
    rcc.cr().modify(|_, w| w.pllon().clear_bit());
    for _ in 0..TIMEOUT_PLL_LOOPS {
        if !rcc.cr().read().pllrdy().bit_is_set() {
            break;
        }
    }
    if rcc.cr().read().pllrdy().bit_is_set() {
        return Err(Error::Timeout);
    }

    // 写 PLL 参数（CFGR3）：PLLSRC（HSI/2 或 HSE；HSE/2 档用 prediv 等效实现）、
    // PLL_PREDIV、PLL_POSTDIV、PLL_FBDIV（倍频 = FBDIV + 2，HAL_RCC_OscConfig / RM 6.10.19）
    let src_bit = src_is_hse;
    let fbdiv = (pll.mul - 2) as u8;
    let pre = pll.prediv;
    let post = pll.postdiv;
    rcc.cfgr3().modify(move |_, w| {
        if src_bit {
            w.pllsrc().set_bit();
        } else {
            w.pllsrc().clear_bit();
        }
        unsafe {
            w.pllprediv().bits(pre as u8);
            w.pllpostdiv().bits(post as u8);
            w.pllfbdiv().bits(fbdiv);
        }
        w
    });

    // 开 PLL，等待 PLLRDY 置位
    rcc.cr().modify(|_, w| w.pllon().set_bit());
    for _ in 0..TIMEOUT_PLL_LOOPS {
        if rcc.cr().read().pllrdy().bit_is_set() {
            return Ok(());
        }
    }
    Err(Error::Timeout)
}

/// 配置时钟树（阻塞，含全部就绪等待与超时）。
///
/// 由 [`crate::init`] 调用；执行顺序对照 `HAL_RCC_OscConfig` + `HAL_RCC_ClockConfig`：
/// 振荡器/PLL 准备 → 提高 Flash 等待周期 →（必要时 HPRE 先降到 /2 过渡）→
/// 写 CFGR.SW 并轮询 SWS → 恢复 HPRE → 降低 Flash 等待周期 → 设 PPRE1/PPRE2。
/// 成功后结果同时写入全局（供 [`clocks()`] 查询）。
pub fn init(config: Config) -> Result<Clocks, Error> {
    let rcc = unsafe { &*pac::Rcc::PTR };

    // ---- 1. 频率计算与校验（DS：HCLK ≤ 170 MHz，PCLK ≤ 85 MHz）----
    let sysclk_hz;
    let pll_hz;
    match config.sys {
        Sysclk::HSI => {
            sysclk_hz = HSI_HZ;
            pll_hz = None;
        }
        Sysclk::HSE => {
            let hse = config.hse.as_ref().ok_or(Error::InvalidConfig)?;
            sysclk_hz = hse.freq.0;
            pll_hz = None;
        }
        Sysclk::PLL => {
            let pll = config.pll.as_ref().ok_or(Error::InvalidConfig)?;
            // 频率公式对照 HAL_RCC_GetSysClockFreq：
            // HSE 源：HSE / PREDIV × MUL / POSTDIV；HSI 源：(HSI/2) / PREDIV × MUL / POSTDIV
            let src_hz = match config.pll_src {
                PllSource::HSI => HSI_HZ / 2,
                PllSource::HSE => config.hse.as_ref().ok_or(Error::InvalidConfig)?.freq.0,
            };
            let f = src_hz / prediv_div(pll.prediv) * (pll.mul as u32) / postdiv_div(pll.postdiv);
            sysclk_hz = f;
            pll_hz = Some(f);
        }
    }
    let hclk_hz = sysclk_hz / config.ahb_pre.div();
    let pclk1_hz = hclk_hz / config.apb1_pre.div();
    let pclk2_hz = hclk_hz / config.apb2_pre.div();
    if hclk_hz > MAX_HCLK_HZ || pclk1_hz > MAX_PCLK_HZ || pclk2_hz > MAX_PCLK_HZ {
        return Err(Error::InvalidConfig);
    }

    // ---- 2. 振荡器/PLL 准备（HSI 复位默认已开且就绪，CR 复位值 0x0080_2083）----
    if let Some(hse) = &config.hse {
        enable_hse(rcc, hse)?;
    }
    if let Some(pll) = &config.pll {
        setup_pll(rcc, pll, config.pll_src == PllSource::HSE)?;
    }

    // ---- 3. Flash 等待周期：升频先加等待（HAL_RCC_ClockConfig + RM 第 3 章步骤）----
    let target_ws = flash_wait_states(hclk_hz);
    if target_ws > get_flash_latency() {
        set_flash_latency(target_ws)?;
    }

    // ---- 4. HPRE 中间档管理（HAL_RCC_ClockConfig：80 MHz 上/下穿越须先经 /2）----
    // 目标源为 PLL 时按目标 PLL 频率判断，否则按当前 SYSCLK 频率判断
    let freq_for_overshoot = pll_hz.unwrap_or_else(|| {
        // 当前 SWS（RM 6.10.2：00=HSI、01=HSE、10=PLL）
        match rcc.cfgr().read().sws().bits() {
            0b01 => config.hse.as_ref().map(|h| h.freq.0).unwrap_or(HSI_HZ),
            _ => HSI_HZ,
        }
    });
    let mut hpre_intermediate = false;
    let current_hpre = rcc.cfgr().read().hpre().bits();
    if freq_for_overshoot > 80_000_000 && current_hpre & 0b1000 == 0 {
        // 当前 HPRE 为 1 分频（0xxx），先切到 /2 过渡
        rcc.cfgr().modify(|_, w| unsafe { w.hpre().bits(0b1000) });
        hpre_intermediate = true;
    }

    // ---- 5. 就绪检查 + 写 SW 并轮询 SWS（RM：目标源未就绪时硬件不会切换）----
    // HAL_RCC_ClockConfig 对三种源均检查 ready 标志后才写 SW
    let ready = match config.sys {
        Sysclk::HSI => rcc.cr().read().hsirdy().bit_is_set(),
        Sysclk::HSE => rcc.cr().read().hserdy().bit_is_set(),
        Sysclk::PLL => rcc.cr().read().pllrdy().bit_is_set(),
    };
    if !ready {
        return Err(Error::Timeout);
    }
    let target = sw_bits(config.sys);
    rcc.cfgr().modify(|_, w| unsafe { w.sw().bits(target) });
    let mut switched = false;
    for _ in 0..TIMEOUT_SWITCH_LOOPS {
        if rcc.cfgr().read().sws().bits() == target {
            switched = true;
            break;
        }
    }
    if !switched {
        return Err(Error::Timeout);
    }

    // ---- 6. 恢复 HPRE / 降低 Flash 等待 / 设 APB 分频 ----
    if hpre_intermediate {
        let ahb_pre = config.ahb_pre;
        rcc.cfgr()
            .modify(move |_, w| unsafe { w.hpre().bits(ahb_pre.to_bits()) });
    }
    if target_ws < get_flash_latency() {
        set_flash_latency(target_ws)?;
    }
    let apb1_pre = config.apb1_pre;
    let apb2_pre = config.apb2_pre;
    rcc.cfgr().modify(move |_, w| unsafe {
        w.ppre1().bits(apb1_pre.to_bits());
        w.ppre2().bits(apb2_pre.to_bits());
        w
    });

    // ---- 7. 结果写入全局，供 clocks() 查询 ----
    SYS_HZ.store(sysclk_hz, Ordering::Relaxed);
    HCLK_HZ.store(hclk_hz, Ordering::Relaxed);
    APB1_HZ.store(pclk1_hz, Ordering::Relaxed);
    APB2_HZ.store(pclk2_hz, Ordering::Relaxed);

    Ok(Clocks {
        sys: Hertz(sysclk_hz),
        hclk: Hertz(hclk_hz),
        apb1: Hertz(pclk1_hz),
        apb2: Hertz(pclk2_hz),
    })
}

/// 使能 AHB1 总线上的外设时钟（`RCC_AHB1ENR`，位名查 `py32e407_ll_bus.h`）。
///
/// 对应 `LL_AHB1_GRP1_EnableClock`：置位后读回一次使能寄存器（"Delay after an
/// RCC peripheral clock enabling"），确保时钟真正生效。`mask` 为目标外设使能位的
/// 按位或（如 DMA1 位）。
pub fn enable_ahb1(mask: u32) {
    let rcc = unsafe { &*pac::Rcc::PTR };
    rcc.ahb1enr()
        .modify(|r, w| unsafe { w.bits(r.bits() | mask) });
    let _ = rcc.ahb1enr().read();
}

/// 使能 AHB2 总线上的外设时钟（`RCC_AHB2ENR`，含 GPIOA–F）。
///
/// 语义同 [`enable_ahb1`]（读回等待时钟生效）；GPIO 驱动内部已自带调用。
pub fn enable_ahb2(mask: u32) {
    let rcc = unsafe { &*pac::Rcc::PTR };
    rcc.ahb2enr()
        .modify(|r, w| unsafe { w.bits(r.bits() | mask) });
    let _ = rcc.ahb2enr().read();
}

/// 使能 APB1 总线上的外设时钟（`RCC_APB1ENR1`/`RCC_APB1ENR2`）。
///
/// * `enr1` — `RCC_APB1ENR1` 的置位掩码；
/// * `enr2` — `RCC_APB1ENR2` 的置位掩码（可传 0）。
pub fn enable_apb1(enr1: u32, enr2: u32) {
    let rcc = unsafe { &*pac::Rcc::PTR };
    if enr1 != 0 {
        rcc.apb1enr1()
            .modify(|r, w| unsafe { w.bits(r.bits() | enr1) });
        let _ = rcc.apb1enr1().read();
    }
    if enr2 != 0 {
        rcc.apb1enr2()
            .modify(|r, w| unsafe { w.bits(r.bits() | enr2) });
        let _ = rcc.apb1enr2().read();
    }
}

/// 使能 APB2 总线上的外设时钟（`RCC_APB2ENR`）。语义同 [`enable_ahb1`]。
pub fn enable_apb2(mask: u32) {
    let rcc = unsafe { &*pac::Rcc::PTR };
    rcc.apb2enr()
        .modify(|r, w| unsafe { w.bits(r.bits() | mask) });
    let _ = rcc.apb2enr().read();
}

/// 复位 AHB1 总线上的外设（`RCC_AHB1RSTR`，对应 LL 的 Force + Release：置位后清零）。
///
/// # Safety
///
/// 复位会立即停止该外设并恢复其寄存器复位值；使用前必须确认目标外设
/// 当前未被使用。
pub unsafe fn reset_ahb1(mask: u32) {
    let rcc = unsafe { &*pac::Rcc::PTR };
    rcc.ahb1rstr()
        .modify(|r, w| unsafe { w.bits(r.bits() | mask) });
    rcc.ahb1rstr()
        .modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
}

/// 复位 AHB2 总线上的外设（`RCC_AHB2RSTR`）。语义同 [`reset_ahb1`]。
///
/// 注意：不要对 GPIO 端口位调用此函数——复位整个端口会清掉同端口其他
/// 已配置引脚（GPIO 驱动中亦不做端口复位）。
pub unsafe fn reset_ahb2(mask: u32) {
    let rcc = unsafe { &*pac::Rcc::PTR };
    rcc.ahb2rstr()
        .modify(|r, w| unsafe { w.bits(r.bits() | mask) });
    rcc.ahb2rstr()
        .modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
}

/// 复位 APB1 总线上的外设（`RCC_APB1RSTR1`/`RCC_APB1RSTR2`）。语义同 [`reset_ahb1`]。
pub unsafe fn reset_apb1(rstr1: u32, rstr2: u32) {
    let rcc = unsafe { &*pac::Rcc::PTR };
    if rstr1 != 0 {
        rcc.apb1rstr1()
            .modify(|r, w| unsafe { w.bits(r.bits() | rstr1) });
        rcc.apb1rstr1()
            .modify(|r, w| unsafe { w.bits(r.bits() & !rstr1) });
    }
    if rstr2 != 0 {
        rcc.apb1rstr2()
            .modify(|r, w| unsafe { w.bits(r.bits() | rstr2) });
        rcc.apb1rstr2()
            .modify(|r, w| unsafe { w.bits(r.bits() & !rstr2) });
    }
}

/// 复位 APB2 总线上的外设（`RCC_APB2RSTR`）。语义同 [`reset_ahb1`]。
pub unsafe fn reset_apb2(mask: u32) {
    let rcc = unsafe { &*pac::Rcc::PTR };
    rcc.apb2rstr()
        .modify(|r, w| unsafe { w.bits(r.bits() | mask) });
    rcc.apb2rstr()
        .modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
}
