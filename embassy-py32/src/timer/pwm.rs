//! TIM 定时器 PWM 输出（同步 Blocking 驱动）
//!
//! 在 [`super::Timer`] 时基之上实现边沿对齐 PWM 输出。寄存器流程依据：
//! - HAL C: `py32e407_hal_tim.c` 的 `HAL_TIM_PWM_Init`（→ `TIM_Base_SetConfig`）、
//!   `HAL_TIM_PWM_ConfigChannel`（→ `TIM_OCx_SetConfig` + 无条件置 `OCxPE`）、
//!   `HAL_TIM_PWM_Start`（`TIM_CCxChannelCmd` → `__HAL_TIM_MOE_ENABLE` → `__HAL_TIM_ENABLE`）
//! - LL:    `py32e407_ll_tim.h`（`LL_TIM_OC_SetMode`、`LL_TIM_OC_EnablePreload`、
//!   `LL_TIM_OC_SetPolarity`、`LL_TIM_CC_EnableChannel`、`LL_TIM_EnableAllOutputs`）
//! - RM:    PY32E407 参考手册 V0.6 第 19–24 章（各定时器「PWM 模式」节，如 20.2.10）
//! - DS:    PY32E407 数据手册 V0.13 端口复用功能映射表（PWM 引脚 AF 号）
//!
//! # 关键实现事实（已用 RM + SVD + PAC + HAL 交叉验证）
//!
//! - **PWM 模式 1 的 `OCxM = 0b110`、模式 2 为 `0b111`**（RM 20.2.10）。本驱动固定使用
//!   模式 1，极性由 `CCER.CCxP` 决定（`ActiveHigh` → `CCxP = 0`）。
//! - **必须置 `OCxPE`（CCR 预装载）并推荐 `ARPE`（ARR 预装载）**：影子寄存器仅在更新
//!   事件装入（RM 同节），`HAL_TIM_PWM_ConfigChannel` 亦无条件置 `OCxPE`。
//! - **高级定时器需要 `BDTR.MOE = 1`** 才能输出：HAL `IS_TIM_BREAK_INSTANCE` 指明
//!   **TIM1/TIM8/TIM15/TIM16/TIM17**（RM 各章 BDTR 寄存器）。其余定时器无 BDTR。
//! - **通道数**（HAL `IS_TIM_CCx_INSTANCE` / RM 各章）：
//!   TIM1/8/2/3/4/5/18 = 4；TIM9/12/15 = 2；TIM10/11/13/14/16/17/19 = 1；
//!   **TIM6/7 无输出通道**，不实现 [`PwmInstance`]，故无法构造 PWM。
//! - **PAC 字段名分歧**（逐文件确认，勿按 ST 习惯猜）：`CCMRx` 经 `ccmr1_output()` /
//!   `ccmr2_output()` 访问；`OCxM` 在 TIM1/2/15/16 上拆成 `ocXm_l` + `ocx_m_h`，
//!   在 TIM9/10 上是单一 `ocXm`；CCR 用整寄存器 `bits(val as _)` 写（各块位宽不一）。
//!
//! # 使用示例
//!
//! ```ignore
//! use embassy_py32::{init, Hertz};
//! use embassy_py32::timer::pwm::{Channel, Pwm, PwmPin};
//!
//! let p = init(Default::default());
//! // TIM1_CH1 = PA8（AF6，与官方例程 Example_LL/TIM/TIM1_PWM_Init 一致）
//! let ch1 = PwmPin::new_ch1(p.PA8);
//! let mut pwm = Pwm::new(p.TIM1, Some(ch1), None, None, None,
//!     embassy_py32::timer::pwm::Config { frequency: Hertz(1_000), ..Default::default() }).unwrap();
//! pwm.enable(Channel::Ch1).unwrap();
//! pwm.set_duty_cycle_percent(Channel::Ch1, 50).unwrap();
//! ```
//!
//! 仅实现同步配置：边沿对齐向上计数、单端（主）输出，不含互补输出/死区/中心对齐、
//! 输入捕获与 DMA/中断；后续将按需扩展 Async。

use core::marker::PhantomData;

use crate::gpio::{Flex, OutputType, Pin, Pull, Speed};
use crate::{Hertz, Peri};

use super::Error;

/// PWM 输出极性（`CCER.CCxP`，RM 各章 CCER 位表）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Polarity {
    /// 高电平有效（`CCxP = 0`，复位默认）：`CNT < CCRx` 时输出高。
    ActiveHigh,
    /// 低电平有效（`CCxP = 1`）：`CNT < CCRx` 时输出低。
    ActiveLow,
}

/// PWM 通道号。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Channel {
    /// 通道 1。
    Ch1,
    /// 通道 2。
    Ch2,
    /// 通道 3。
    Ch3,
    /// 通道 4。
    Ch4,
}

impl Channel {
    /// 通道下标（0..=3），对应 `CCMRx`/`CCER`/`CCRx` 的通道序号。
    #[inline]
    pub const fn index(self) -> usize {
        match self {
            Self::Ch1 => 0,
            Self::Ch2 => 1,
            Self::Ch3 => 2,
            Self::Ch4 => 3,
        }
    }
}

/// 通道标记类型（用作 [`PwmPin`] 的类型参数）。
pub enum Ch1 {}
/// 通道标记类型（见 [`Ch1`]）。
pub enum Ch2 {}
/// 通道标记类型（见 [`Ch1`]）。
pub enum Ch3 {}
/// 通道标记类型（见 [`Ch1`]）。
pub enum Ch4 {}

mod sealed_channel {
    /// 密封的通道标记（防止外部新增通道）。
    pub trait SealedTimerChannel {}
}

/// 定时器通道标记 trait（[`Ch1`]……[`Ch4`]）。
#[allow(private_bounds)]
pub trait TimerChannel: sealed_channel::SealedTimerChannel {
    /// 对应的 [`Channel`] 值。
    const CHANNEL: Channel;
}

impl sealed_channel::SealedTimerChannel for Ch1 {}
impl TimerChannel for Ch1 {
    const CHANNEL: Channel = Channel::Ch1;
}
impl sealed_channel::SealedTimerChannel for Ch2 {}
impl TimerChannel for Ch2 {
    const CHANNEL: Channel = Channel::Ch2;
}
impl sealed_channel::SealedTimerChannel for Ch3 {}
impl TimerChannel for Ch3 {
    const CHANNEL: Channel = Channel::Ch3;
}
impl sealed_channel::SealedTimerChannel for Ch4 {}
impl TimerChannel for Ch4 {
    const CHANNEL: Channel = Channel::Ch4;
}

/// PWM 配置。
///
/// `#[non_exhaustive]`，外部构造请用 [`Config::default()`] 后逐字段赋值。
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// PWM 频率（周期 = `1 / frequency`）。实际频率取定时器分频可达的最近档。
    pub frequency: Hertz,
    /// 输出极性。
    pub polarity: Polarity,
    /// 复用引脚的输出速度（`OSPEEDR`）。
    pub gpio_speed: Speed,
}

impl Default for Config {
    /// 默认：1 kHz、高电平有效、GPIO 超高速档。
    fn default() -> Self {
        Self {
            frequency: Hertz(1_000),
            polarity: Polarity::ActiveHigh,
            gpio_speed: Speed::VeryHigh,
        }
    }
}

// ---------------------------------------------------------------------------
// 引脚复用映射
// ---------------------------------------------------------------------------

/// 密封的通道引脚查询（防止外部为任意类型实现 [`ChannelPin`]）。
pub(crate) mod sealed_pin {
    /// 某引脚在某实例下用作某通道时的复用功能编号。
    pub trait SealedChannelPin<T: super::PwmInstance, C: super::TimerChannel> {
        /// 该引脚在此实例/通道下的复用功能编号（查 DS 端口复用功能映射表）。
        fn af_num(&self) -> u8;
    }
}

/// 可作为某实例某通道 PWM 输出的引脚。
#[allow(private_bounds)]
pub trait ChannelPin<T: PwmInstance, C: TimerChannel>:
    sealed_pin::SealedChannelPin<T, C> + Pin
{
}

/// 为引脚生成某实例/通道的 [`ChannelPin`] 实现（含 AF 号）。
macro_rules! impl_channel_pin {
    ($inst:ident, $ch:ident, $pin:ident, $af:literal) => {
        impl sealed_pin::SealedChannelPin<crate::peripherals::$inst, $ch>
            for crate::peripherals::$pin
        {
            #[inline]
            fn af_num(&self) -> u8 {
                $af
            }
        }
        impl ChannelPin<crate::peripherals::$inst, $ch> for crate::peripherals::$pin {}
    };
}

// 引脚与 AF 号来自 DS V0.13 端口复用功能映射表（表 3-3 端口 A … 表 3-8 端口 F）。
// TIM1 的 PA8/PA9/PA10 与官方例程 Example_LL/TIM/TIM1_PWM_Init（AF6）一致；
// 其余映射为 DS 表抽取结果（该表为 17 列干净表，逐单元格取值）。
// Note: PB3/PB4/PA13/PA14/PA15 属 SWJ 调试脚（JTDO/NJTRST/SWDIO/SWCLK/JTDI），
// 用作 PWM 需在板级确认调试口取舍，驱动只做 GPIO 层面的 AF 配置。

// TIM1（每通道含 N 输出，本驱动只用主输出）
impl_channel_pin!(TIM1, Ch1, PA8, 6);
impl_channel_pin!(TIM1, Ch1, PE9, 2);
impl_channel_pin!(TIM1, Ch1, PC0, 2);
impl_channel_pin!(TIM1, Ch2, PA9, 6);
impl_channel_pin!(TIM1, Ch2, PE11, 2);
impl_channel_pin!(TIM1, Ch2, PC1, 2);
impl_channel_pin!(TIM1, Ch3, PA10, 6);
impl_channel_pin!(TIM1, Ch3, PE13, 2);
impl_channel_pin!(TIM1, Ch3, PC2, 2);
impl_channel_pin!(TIM1, Ch4, PA11, 12);
impl_channel_pin!(TIM1, Ch4, PE14, 2);
impl_channel_pin!(TIM1, Ch4, PC3, 2);

// TIM2
impl_channel_pin!(TIM2, Ch1, PA0, 1);
impl_channel_pin!(TIM2, Ch1, PA5, 1);
impl_channel_pin!(TIM2, Ch1, PA15, 1);
impl_channel_pin!(TIM2, Ch1, PD3, 2);
impl_channel_pin!(TIM2, Ch2, PA1, 1);
impl_channel_pin!(TIM2, Ch2, PB3, 1);
impl_channel_pin!(TIM2, Ch2, PD4, 2);
impl_channel_pin!(TIM2, Ch3, PA2, 1);
impl_channel_pin!(TIM2, Ch3, PA9, 10);
impl_channel_pin!(TIM2, Ch3, PB10, 1);
impl_channel_pin!(TIM2, Ch3, PD7, 2);
impl_channel_pin!(TIM2, Ch4, PA3, 1);
impl_channel_pin!(TIM2, Ch4, PA10, 10);
impl_channel_pin!(TIM2, Ch4, PB11, 1);
impl_channel_pin!(TIM2, Ch4, PD6, 2);

// TIM3
impl_channel_pin!(TIM3, Ch1, PA6, 2);
impl_channel_pin!(TIM3, Ch1, PB4, 2);
impl_channel_pin!(TIM3, Ch1, PC6, 2);
impl_channel_pin!(TIM3, Ch1, PE2, 2);
impl_channel_pin!(TIM3, Ch2, PA4, 2);
impl_channel_pin!(TIM3, Ch2, PA7, 2);
impl_channel_pin!(TIM3, Ch2, PB5, 2);
impl_channel_pin!(TIM3, Ch2, PC7, 2);
impl_channel_pin!(TIM3, Ch2, PE3, 2);
impl_channel_pin!(TIM3, Ch3, PB0, 2);
impl_channel_pin!(TIM3, Ch3, PC8, 2);
impl_channel_pin!(TIM3, Ch3, PE4, 2);
impl_channel_pin!(TIM3, Ch4, PB1, 2);
impl_channel_pin!(TIM3, Ch4, PB7, 10);
impl_channel_pin!(TIM3, Ch4, PC9, 2);
impl_channel_pin!(TIM3, Ch4, PE5, 2);

// TIM4
impl_channel_pin!(TIM4, Ch1, PB6, 2);
impl_channel_pin!(TIM4, Ch1, PD12, 2);
impl_channel_pin!(TIM4, Ch1, PA11, 10);
impl_channel_pin!(TIM4, Ch2, PB7, 2);
impl_channel_pin!(TIM4, Ch2, PD13, 2);
impl_channel_pin!(TIM4, Ch2, PA12, 10);
impl_channel_pin!(TIM4, Ch3, PB8, 2);
impl_channel_pin!(TIM4, Ch3, PD14, 2);
impl_channel_pin!(TIM4, Ch3, PA13, 10);
impl_channel_pin!(TIM4, Ch4, PB9, 2);
impl_channel_pin!(TIM4, Ch4, PD15, 2);

// TIM5
impl_channel_pin!(TIM5, Ch1, PA0, 2);
impl_channel_pin!(TIM5, Ch1, PB2, 2);
impl_channel_pin!(TIM5, Ch2, PA1, 2);
impl_channel_pin!(TIM5, Ch2, PC12, 1);
impl_channel_pin!(TIM5, Ch3, PA2, 2);
impl_channel_pin!(TIM5, Ch3, PE8, 1);
impl_channel_pin!(TIM5, Ch4, PA3, 2);
impl_channel_pin!(TIM5, Ch4, PE9, 1);
impl_channel_pin!(TIM5, Ch4, PF3, 6);

// TIM8
impl_channel_pin!(TIM8, Ch1, PA15, 2);
impl_channel_pin!(TIM8, Ch1, PB6, 5);
impl_channel_pin!(TIM8, Ch1, PC6, 4);
impl_channel_pin!(TIM8, Ch2, PA14, 5);
impl_channel_pin!(TIM8, Ch2, PB8, 10);
impl_channel_pin!(TIM8, Ch2, PC7, 4);
impl_channel_pin!(TIM8, Ch3, PB9, 10);
impl_channel_pin!(TIM8, Ch3, PC8, 4);
impl_channel_pin!(TIM8, Ch4, PA12, 4);
impl_channel_pin!(TIM8, Ch4, PC9, 4);
impl_channel_pin!(TIM8, Ch4, PD1, 4);

// TIM9
impl_channel_pin!(TIM9, Ch1, PA0, 3);
impl_channel_pin!(TIM9, Ch1, PB1, 3);
impl_channel_pin!(TIM9, Ch1, PE4, 3);
impl_channel_pin!(TIM9, Ch2, PA1, 3);
impl_channel_pin!(TIM9, Ch2, PC12, 2);
impl_channel_pin!(TIM9, Ch2, PE5, 3);

// TIM10
impl_channel_pin!(TIM10, Ch1, PA4, 3);
impl_channel_pin!(TIM10, Ch1, PA7, 3);
impl_channel_pin!(TIM10, Ch1, PC7, 3);
impl_channel_pin!(TIM10, Ch1, PE3, 3);

// TIM11
impl_channel_pin!(TIM11, Ch1, PA0, 12);
impl_channel_pin!(TIM11, Ch1, PA5, 3);
impl_channel_pin!(TIM11, Ch1, PA15, 12);

// TIM12
impl_channel_pin!(TIM12, Ch1, PA2, 3);
impl_channel_pin!(TIM12, Ch1, PE8, 3);
impl_channel_pin!(TIM12, Ch2, PA3, 3);
impl_channel_pin!(TIM12, Ch2, PE9, 3);
impl_channel_pin!(TIM12, Ch2, PF3, 7);

// TIM13
impl_channel_pin!(TIM13, Ch1, PA6, 3);
impl_channel_pin!(TIM13, Ch1, PB4, 3);
impl_channel_pin!(TIM13, Ch1, PC6, 3);

// TIM14
impl_channel_pin!(TIM14, Ch1, PA8, 1);
impl_channel_pin!(TIM14, Ch1, PB9, 3);
impl_channel_pin!(TIM14, Ch1, PE1, 3);

// TIM15
impl_channel_pin!(TIM15, Ch1, PA2, 9);
impl_channel_pin!(TIM15, Ch1, PB14, 1);
impl_channel_pin!(TIM15, Ch1, PF3, 3);
impl_channel_pin!(TIM15, Ch2, PA3, 9);
impl_channel_pin!(TIM15, Ch2, PB15, 1);
impl_channel_pin!(TIM15, Ch2, PF4, 3);

// TIM16
impl_channel_pin!(TIM16, Ch1, PA6, 1);
impl_channel_pin!(TIM16, Ch1, PA12, 1);
impl_channel_pin!(TIM16, Ch1, PB4, 1);
impl_channel_pin!(TIM16, Ch1, PB8, 1);
impl_channel_pin!(TIM16, Ch1, PE0, 4);

// TIM17
impl_channel_pin!(TIM17, Ch1, PA7, 1);
impl_channel_pin!(TIM17, Ch1, PB5, 10);
impl_channel_pin!(TIM17, Ch1, PB9, 1);
impl_channel_pin!(TIM17, Ch1, PE1, 4);

// TIM18
impl_channel_pin!(TIM18, Ch1, PA8, 3);
impl_channel_pin!(TIM18, Ch1, PB2, 3);
impl_channel_pin!(TIM18, Ch1, PC8, 3);
impl_channel_pin!(TIM18, Ch2, PA14, 8);
impl_channel_pin!(TIM18, Ch2, PC12, 3);
impl_channel_pin!(TIM18, Ch2, PD13, 8);
impl_channel_pin!(TIM18, Ch3, PA2, 4);
impl_channel_pin!(TIM18, Ch3, PB3, 3);
impl_channel_pin!(TIM18, Ch3, PE7, 3);
impl_channel_pin!(TIM18, Ch4, PA3, 4);
impl_channel_pin!(TIM18, Ch4, PE6, 3);
impl_channel_pin!(TIM18, Ch4, PE10, 3);

// TIM19
impl_channel_pin!(TIM19, Ch1, PA0, 11);
impl_channel_pin!(TIM19, Ch1, PA5, 4);
impl_channel_pin!(TIM19, Ch1, PA15, 13);

// ---------------------------------------------------------------------------
// PWM 通道寄存器操作（按 PAC 寄存器块分类）
// ---------------------------------------------------------------------------

mod sealed_block {
    /// 各类 TIM 寄存器块统一的 PWM 通道操作。
    ///
    /// 六类块（tim1/tim2/tim9/tim10/tim15/tim16）在 svd2rust 下类型不同，且
    /// `OCxM` 字段命名分两派（拆分 `ocXm_l`/`ocx_m_h` vs 单一 `ocXm`），故用宏
    /// 分别实现；全部经 `py32e407-pac` 类型化 API 访问。
    pub trait SealedPwmBlock {
        /// 配置通道为 PWM 模式 1：`CCxS = 0`（输出）、`OCxM = 0b110`、
        /// `OCxPE = 1`（CCR 预装载），并按 `active_low` 设 `CCER.CCxP`。
        fn configure_channel(&self, ch: usize, active_low: bool);

        /// 置/清 `CCER.CCxE`（输出使能）。
        fn set_channel_enable(&self, ch: usize, on: bool);

        /// 读 `CCER.CCxE`。
        fn channel_enabled(&self, ch: usize) -> bool;

        /// 写 `CCRx`（比较值）。
        fn write_ccr(&self, ch: usize, val: u32);

        /// 读 `CCRx`。
        fn read_ccr(&self, ch: usize) -> u32;

        /// 置/清 `BDTR.MOE`（主输出使能）。无 BDTR 的块默认空操作。
        fn set_moe(&self, _on: bool) {}
    }
}

/// 拆分式 `OCxM`（`ocXm_l` + `ocx_m_h`）的单通道配置。
macro_rules! pwm_cfg_one_split {
    ($regs:expr, $ccmr:ident, $ccs:ident, $pe:ident, $ml:ident, $mh:ident, $ccp:ident, $low:expr) => {{
        $regs.$ccmr().modify(|_, w| unsafe {
            w.$ccs().bits(0);
            w.$pe().set_bit();
            w.$ml().bits(0b110);
            w.$mh().bit(false)
        });
        $regs.ccer().modify(|_, w| w.$ccp().bit($low));
    }};
}

/// 单一 `OCxM` 的单通道配置。
macro_rules! pwm_cfg_one_single {
    ($regs:expr, $ccmr:ident, $ccs:ident, $pe:ident, $m:ident, $ccp:ident, $low:expr) => {{
        $regs.$ccmr().modify(|_, w| unsafe {
            w.$ccs().bits(0);
            w.$pe().set_bit();
            w.$m().bits(0b110)
        });
        $regs.ccer().modify(|_, w| w.$ccp().bit($low));
    }};
}

/// 4 通道、拆分式 `OCxM`、**带 BDTR/MOE** 的块（tim1 = TIM1/TIM8）。
macro_rules! impl_pwm_block_4ch_split_moe {
    ($pacmod:ident) => {
        impl sealed_block::SealedPwmBlock for crate::pac::$pacmod::RegisterBlock {
            #[inline]
            fn configure_channel(&self, ch: usize, active_low: bool) {
                match ch {
                    0 => pwm_cfg_one_split!(
                        self,
                        ccmr1_output,
                        cc1s,
                        oc1pe,
                        oc1m_l,
                        oc1m_h,
                        cc1p,
                        active_low
                    ),
                    1 => pwm_cfg_one_split!(
                        self,
                        ccmr1_output,
                        cc2s,
                        oc2pe,
                        oc2m_l,
                        oc2m_h,
                        cc2p,
                        active_low
                    ),
                    2 => pwm_cfg_one_split!(
                        self,
                        ccmr2_output,
                        cc3s,
                        oc3pe,
                        oc3m_l,
                        oc3m_h,
                        cc3p,
                        active_low
                    ),
                    _ => pwm_cfg_one_split!(
                        self,
                        ccmr2_output,
                        cc4s,
                        oc4pe,
                        oc4m_l,
                        oc4m_h,
                        cc4p,
                        active_low
                    ),
                }
            }
            #[inline]
            fn set_channel_enable(&self, ch: usize, on: bool) {
                match ch {
                    0 => self.ccer().modify(|_, w| w.cc1e().bit(on)),
                    1 => self.ccer().modify(|_, w| w.cc2e().bit(on)),
                    2 => self.ccer().modify(|_, w| w.cc3e().bit(on)),
                    _ => self.ccer().modify(|_, w| w.cc4e().bit(on)),
                };
            }
            #[inline]
            fn channel_enabled(&self, ch: usize) -> bool {
                let ccer = self.ccer().read();
                match ch {
                    0 => ccer.cc1e().bit_is_set(),
                    1 => ccer.cc2e().bit_is_set(),
                    2 => ccer.cc3e().bit_is_set(),
                    _ => ccer.cc4e().bit_is_set(),
                }
            }
            #[inline]
            fn write_ccr(&self, ch: usize, val: u32) {
                match ch {
                    0 => self.ccr1().write(|w| unsafe { w.bits(val as _) }),
                    1 => self.ccr2().write(|w| unsafe { w.bits(val as _) }),
                    2 => self.ccr3().write(|w| unsafe { w.bits(val as _) }),
                    _ => self.ccr4().write(|w| unsafe { w.bits(val as _) }),
                };
            }
            #[inline]
            fn read_ccr(&self, ch: usize) -> u32 {
                match ch {
                    0 => self.ccr1().read().bits() as u32,
                    1 => self.ccr2().read().bits() as u32,
                    2 => self.ccr3().read().bits() as u32,
                    _ => self.ccr4().read().bits() as u32,
                }
            }
            #[inline]
            fn set_moe(&self, on: bool) {
                self.bdtr().modify(|_, w| w.moe().bit(on));
            }
        }
    };
}

/// 4 通道、拆分式 `OCxM`、无 BDTR 的块（tim2 = TIM2–5/18）。
macro_rules! impl_pwm_block_4ch_split {
    ($pacmod:ident) => {
        impl sealed_block::SealedPwmBlock for crate::pac::$pacmod::RegisterBlock {
            #[inline]
            fn configure_channel(&self, ch: usize, active_low: bool) {
                match ch {
                    0 => pwm_cfg_one_split!(
                        self,
                        ccmr1_output,
                        cc1s,
                        oc1pe,
                        oc1m_l,
                        oc1m_h,
                        cc1p,
                        active_low
                    ),
                    1 => pwm_cfg_one_split!(
                        self,
                        ccmr1_output,
                        cc2s,
                        oc2pe,
                        oc2m_l,
                        oc2m_h,
                        cc2p,
                        active_low
                    ),
                    2 => pwm_cfg_one_split!(
                        self,
                        ccmr2_output,
                        cc3s,
                        oc3pe,
                        oc3m_l,
                        oc3m_h,
                        cc3p,
                        active_low
                    ),
                    _ => pwm_cfg_one_split!(
                        self,
                        ccmr2_output,
                        cc4s,
                        oc4pe,
                        oc4m_l,
                        oc4m_h,
                        cc4p,
                        active_low
                    ),
                }
            }
            #[inline]
            fn set_channel_enable(&self, ch: usize, on: bool) {
                match ch {
                    0 => self.ccer().modify(|_, w| w.cc1e().bit(on)),
                    1 => self.ccer().modify(|_, w| w.cc2e().bit(on)),
                    2 => self.ccer().modify(|_, w| w.cc3e().bit(on)),
                    _ => self.ccer().modify(|_, w| w.cc4e().bit(on)),
                };
            }
            #[inline]
            fn channel_enabled(&self, ch: usize) -> bool {
                let ccer = self.ccer().read();
                match ch {
                    0 => ccer.cc1e().bit_is_set(),
                    1 => ccer.cc2e().bit_is_set(),
                    2 => ccer.cc3e().bit_is_set(),
                    _ => ccer.cc4e().bit_is_set(),
                }
            }
            #[inline]
            fn write_ccr(&self, ch: usize, val: u32) {
                match ch {
                    0 => self.ccr1().write(|w| unsafe { w.bits(val as _) }),
                    1 => self.ccr2().write(|w| unsafe { w.bits(val as _) }),
                    2 => self.ccr3().write(|w| unsafe { w.bits(val as _) }),
                    _ => self.ccr4().write(|w| unsafe { w.bits(val as _) }),
                };
            }
            #[inline]
            fn read_ccr(&self, ch: usize) -> u32 {
                match ch {
                    0 => self.ccr1().read().bits() as u32,
                    1 => self.ccr2().read().bits() as u32,
                    2 => self.ccr3().read().bits() as u32,
                    _ => self.ccr4().read().bits() as u32,
                }
            }
        }
    };
}

/// 2 通道、拆分式 `OCxM` 的块（tim15 = TIM15，带 BDTR/MOE）。
macro_rules! impl_pwm_block_2ch_split {
    ($pacmod:ident) => {
        impl sealed_block::SealedPwmBlock for crate::pac::$pacmod::RegisterBlock {
            #[inline]
            fn configure_channel(&self, ch: usize, active_low: bool) {
                match ch {
                    0 => pwm_cfg_one_split!(
                        self,
                        ccmr1_output,
                        cc1s,
                        oc1pe,
                        oc1m_l,
                        oc1m_h,
                        cc1p,
                        active_low
                    ),
                    _ => pwm_cfg_one_split!(
                        self,
                        ccmr1_output,
                        cc2s,
                        oc2pe,
                        oc2m_l,
                        oc2m_h,
                        cc2p,
                        active_low
                    ),
                }
            }
            #[inline]
            fn set_channel_enable(&self, ch: usize, on: bool) {
                match ch {
                    0 => self.ccer().modify(|_, w| w.cc1e().bit(on)),
                    _ => self.ccer().modify(|_, w| w.cc2e().bit(on)),
                };
            }
            #[inline]
            fn channel_enabled(&self, ch: usize) -> bool {
                let ccer = self.ccer().read();
                match ch {
                    0 => ccer.cc1e().bit_is_set(),
                    _ => ccer.cc2e().bit_is_set(),
                }
            }
            #[inline]
            fn write_ccr(&self, ch: usize, val: u32) {
                match ch {
                    0 => self.ccr1().write(|w| unsafe { w.bits(val as _) }),
                    _ => self.ccr2().write(|w| unsafe { w.bits(val as _) }),
                };
            }
            #[inline]
            fn read_ccr(&self, ch: usize) -> u32 {
                match ch {
                    0 => self.ccr1().read().bits() as u32,
                    _ => self.ccr2().read().bits() as u32,
                }
            }
            #[inline]
            fn set_moe(&self, on: bool) {
                self.bdtr().modify(|_, w| w.moe().bit(on));
            }
        }
    };
}

/// 1 通道、拆分式 `OCxM` 的块（tim16 = TIM16/TIM17，带 BDTR/MOE）。
macro_rules! impl_pwm_block_1ch_split {
    ($pacmod:ident) => {
        impl sealed_block::SealedPwmBlock for crate::pac::$pacmod::RegisterBlock {
            #[inline]
            fn configure_channel(&self, _ch: usize, active_low: bool) {
                pwm_cfg_one_split!(
                    self,
                    ccmr1_output,
                    cc1s,
                    oc1pe,
                    oc1m_l,
                    oc1m_h,
                    cc1p,
                    active_low
                )
            }
            #[inline]
            fn set_channel_enable(&self, _ch: usize, on: bool) {
                self.ccer().modify(|_, w| w.cc1e().bit(on));
            }
            #[inline]
            fn channel_enabled(&self, _ch: usize) -> bool {
                self.ccer().read().cc1e().bit_is_set()
            }
            #[inline]
            fn write_ccr(&self, _ch: usize, val: u32) {
                self.ccr1().write(|w| unsafe { w.bits(val as _) });
            }
            #[inline]
            fn read_ccr(&self, _ch: usize) -> u32 {
                self.ccr1().read().bits() as u32
            }
            #[inline]
            fn set_moe(&self, on: bool) {
                self.bdtr().modify(|_, w| w.moe().bit(on));
            }
        }
    };
}

/// 2 通道、单一 `OCxM` 的块（tim9 = TIM9/TIM12，无 BDTR）。
macro_rules! impl_pwm_block_2ch_single {
    ($pacmod:ident) => {
        impl sealed_block::SealedPwmBlock for crate::pac::$pacmod::RegisterBlock {
            #[inline]
            fn configure_channel(&self, ch: usize, active_low: bool) {
                match ch {
                    0 => {
                        pwm_cfg_one_single!(self, ccmr1_output, cc1s, oc1pe, oc1m, cc1p, active_low)
                    }
                    _ => {
                        pwm_cfg_one_single!(self, ccmr1_output, cc2s, oc2pe, oc2m, cc2p, active_low)
                    }
                }
            }
            #[inline]
            fn set_channel_enable(&self, ch: usize, on: bool) {
                match ch {
                    0 => self.ccer().modify(|_, w| w.cc1e().bit(on)),
                    _ => self.ccer().modify(|_, w| w.cc2e().bit(on)),
                };
            }
            #[inline]
            fn channel_enabled(&self, ch: usize) -> bool {
                let ccer = self.ccer().read();
                match ch {
                    0 => ccer.cc1e().bit_is_set(),
                    _ => ccer.cc2e().bit_is_set(),
                }
            }
            #[inline]
            fn write_ccr(&self, ch: usize, val: u32) {
                match ch {
                    0 => self.ccr1().write(|w| unsafe { w.bits(val as _) }),
                    _ => self.ccr2().write(|w| unsafe { w.bits(val as _) }),
                };
            }
            #[inline]
            fn read_ccr(&self, ch: usize) -> u32 {
                match ch {
                    0 => self.ccr1().read().bits() as u32,
                    _ => self.ccr2().read().bits() as u32,
                }
            }
        }
    };
}

/// 1 通道、单一 `OCxM` 的块（tim10 = TIM10/11/13/14/19，无 BDTR）。
macro_rules! impl_pwm_block_1ch_single {
    ($pacmod:ident) => {
        impl sealed_block::SealedPwmBlock for crate::pac::$pacmod::RegisterBlock {
            #[inline]
            fn configure_channel(&self, _ch: usize, active_low: bool) {
                pwm_cfg_one_single!(self, ccmr1_output, cc1s, oc1pe, oc1m, cc1p, active_low)
            }
            #[inline]
            fn set_channel_enable(&self, _ch: usize, on: bool) {
                self.ccer().modify(|_, w| w.cc1e().bit(on));
            }
            #[inline]
            fn channel_enabled(&self, _ch: usize) -> bool {
                self.ccer().read().cc1e().bit_is_set()
            }
            #[inline]
            fn write_ccr(&self, _ch: usize, val: u32) {
                self.ccr1().write(|w| unsafe { w.bits(val as _) });
            }
            #[inline]
            fn read_ccr(&self, _ch: usize) -> u32 {
                self.ccr1().read().bits() as u32
            }
        }
    };
}

impl_pwm_block_4ch_split_moe!(tim1); // TIM1/TIM8，高级：含 BDTR.MOE
impl_pwm_block_4ch_split!(tim2); // TIM2–5/18，通用：无 MOE
impl_pwm_block_2ch_split!(tim15); // TIM15，高级
impl_pwm_block_1ch_split!(tim16); // TIM16/TIM17，高级
impl_pwm_block_2ch_single!(tim9); // TIM9/TIM12，无 MOE
impl_pwm_block_1ch_single!(tim10); // TIM10/11/13/14/19，无 MOE

// ---------------------------------------------------------------------------
// PWM 可用实例
// ---------------------------------------------------------------------------

mod sealed_instance {
    use super::sealed_block::SealedPwmBlock;

    /// 密封的 PWM 实例信息（防止外部为任意类型实现 [`super::PwmInstance`]）。
    pub trait SealedPwmInstance {
        /// 该实例的 PWM 寄存器块。
        fn pwm_regs() -> &'static dyn SealedPwmBlock;

        /// 该实例的 PWM 通道数（1..=4）。
        fn channels() -> u8;
    }
}

/// 支持 PWM 输出的 TIM 实例。
///
/// 因 [`super::super::timer::Instance`] 覆盖全部 TIM1–TIM19，而 TIM6/7 无输出通道，
/// 故单独用本 trait 区分；未实现本 trait 的实例无法构造 [`Pwm`]（编译期即拒绝）。
#[allow(private_bounds)]
pub trait PwmInstance: super::Instance + sealed_instance::SealedPwmInstance {}

/// 为 TIM 实例实现 [`sealed_instance::SealedPwmInstance`] 与 [`PwmInstance`]。
///
/// `$channels`：该实例的 PWM 通道数（HAL `IS_TIM_CCx_INSTANCE` / RM 各章）。
macro_rules! impl_pwm_instance {
    ($inst:ident, $pac:ident, $channels:literal) => {
        impl sealed_instance::SealedPwmInstance for crate::peripherals::$inst {
            #[inline]
            fn pwm_regs() -> &'static dyn sealed_block::SealedPwmBlock {
                // 基址与类型均来自 PAC；同块别名实例（如 TIM8 复用 tim1）指向各自基址。
                unsafe { &*crate::pac::$pac::PTR }
            }
            #[inline]
            fn channels() -> u8 {
                $channels
            }
        }
        impl PwmInstance for crate::peripherals::$inst {}
    };
}

// 4 通道：TIM1/8（BDTR）、TIM2/3/4/5/18（无 BDTR）
impl_pwm_instance!(TIM1, Tim1, 4);
impl_pwm_instance!(TIM8, Tim8, 4);
impl_pwm_instance!(TIM2, Tim2, 4);
impl_pwm_instance!(TIM3, Tim3, 4);
impl_pwm_instance!(TIM4, Tim4, 4);
impl_pwm_instance!(TIM5, Tim5, 4);
impl_pwm_instance!(TIM18, Tim18, 4);
// 2 通道：TIM9/12（无 BDTR）、TIM15（BDTR）
impl_pwm_instance!(TIM9, Tim9, 2);
impl_pwm_instance!(TIM12, Tim12, 2);
impl_pwm_instance!(TIM15, Tim15, 2);
// 1 通道：TIM10/11/13/14/19（无 BDTR）、TIM16/17（BDTR）
impl_pwm_instance!(TIM10, Tim10, 1);
impl_pwm_instance!(TIM11, Tim11, 1);
impl_pwm_instance!(TIM13, Tim13, 1);
impl_pwm_instance!(TIM14, Tim14, 1);
impl_pwm_instance!(TIM16, Tim16, 1);
impl_pwm_instance!(TIM17, Tim17, 1);
impl_pwm_instance!(TIM19, Tim19, 1);

// ---------------------------------------------------------------------------
// 驱动
// ---------------------------------------------------------------------------

/// PWM 通道引脚包装：持有引脚的 [`Flex`] 与 AF 号，构造 [`Pwm`] 时移交所有权。
pub struct PwmPin<'d, T: PwmInstance, C: TimerChannel> {
    flex: Flex<'d>,
    af: u8,
    _phantom: PhantomData<(T, C)>,
}

impl<'d, T: PwmInstance, C: TimerChannel> PwmPin<'d, T, C> {
    #[inline]
    fn new_inner(pin: Peri<'d, impl ChannelPin<T, C>>) -> Self {
        // AF 号在引脚被 Flex 接管前取出
        let af = pin.af_num();
        Self {
            flex: Flex::new(pin),
            af,
            _phantom: PhantomData,
        }
    }
}

impl<'d, T: PwmInstance> PwmPin<'d, T, Ch1> {
    /// 创建通道 1 的 PWM 引脚。
    #[inline]
    pub fn new_ch1(pin: Peri<'d, impl ChannelPin<T, Ch1>>) -> Self {
        Self::new_inner(pin)
    }
}

impl<'d, T: PwmInstance> PwmPin<'d, T, Ch2> {
    /// 创建通道 2 的 PWM 引脚。
    #[inline]
    pub fn new_ch2(pin: Peri<'d, impl ChannelPin<T, Ch2>>) -> Self {
        Self::new_inner(pin)
    }
}

impl<'d, T: PwmInstance> PwmPin<'d, T, Ch3> {
    /// 创建通道 3 的 PWM 引脚。
    #[inline]
    pub fn new_ch3(pin: Peri<'d, impl ChannelPin<T, Ch3>>) -> Self {
        Self::new_inner(pin)
    }
}

impl<'d, T: PwmInstance> PwmPin<'d, T, Ch4> {
    /// 创建通道 4 的 PWM 引脚。
    #[inline]
    pub fn new_ch4(pin: Peri<'d, impl ChannelPin<T, Ch4>>) -> Self {
        Self::new_inner(pin)
    }
}

/// 同步 PWM 输出驱动（边沿对齐、单端输出）。
///
/// 内部持有一个 [`super::Timer`] 时基与已占用的通道引脚。计数器在构造后即运行
/// （`CEN = 1`），但各通道输出默认关闭（`CCxE = 0`），需调用 [`Pwm::enable`] 打开。
pub struct Pwm<'d, T: PwmInstance> {
    timer: super::Timer<'d, T>,
    _ch1: Option<Flex<'d>>,
    _ch2: Option<Flex<'d>>,
    _ch3: Option<Flex<'d>>,
    _ch4: Option<Flex<'d>>,
    psc: u16,
    arr: u32,
}

impl<'d, T: PwmInstance> Pwm<'d, T> {
    /// 创建 PWM 驱动并完成初始化（阻塞）。
    ///
    /// 流程对照 `HAL_TIM_PWM_Init` + `HAL_TIM_PWM_ConfigChannel` + `HAL_TIM_PWM_Start`：
    /// 1. 时基初始化（[`super::Timer::new_blocking`]：时钟/复位 → `CR1`（含 `ARPE=1`）
    ///    → `ARR` → `PSC` → `EGR.UG` → 清 `UIF`）；
    /// 2. 逐通道配置为 PWM 模式 1（`CCxS=0`、`OCxM=0b110`、`OCxPE=1`）并设极性；
    /// 3. 高级定时器（TIM1/8/15/16/17）置 `BDTR.MOE=1`；
    /// 4. `CR1.CEN=1` 启动计数器（通道输出仍关闭，待 [`Pwm::enable`]）。
    ///
    /// `ch1`…`ch4` 以 `Option` 表达实际占用的通道，通道数须不超过该实例的通道数。
    ///
    /// # Errors
    ///
    /// - 频率不可达：返回 [`Error::InvalidConfig`]；
    /// - 传入的通道号超出该实例通道数：返回 [`Error::InvalidChannel`]。
    pub fn new(
        tim: Peri<'d, T>,
        ch1: Option<PwmPin<'d, T, Ch1>>,
        ch2: Option<PwmPin<'d, T, Ch2>>,
        ch3: Option<PwmPin<'d, T, Ch3>>,
        ch4: Option<PwmPin<'d, T, Ch4>>,
        config: Config,
    ) -> Result<Self, Error> {
        // 通道数校验（编译期已知通道号，运行期对比实例通道数）
        let channels = T::channels();
        check_channel::<Ch1>(ch1.is_some(), channels)?;
        check_channel::<Ch2>(ch2.is_some(), channels)?;
        check_channel::<Ch3>(ch3.is_some(), channels)?;
        check_channel::<Ch4>(ch4.is_some(), channels)?;

        // 时基配置：PWM 需 ARR 预装载（RM「PWM 模式」节），并整数频率换算 PSC/ARR
        let mut tcfg = super::Config::from_frequency(Hertz(tim.timer_clk()), config.frequency)?;
        tcfg.auto_reload_preload = true;

        let timer = super::Timer::new_blocking(tim, tcfg)?;
        let psc = tcfg.prescaler;
        let arr = tcfg.period;

        // 引脚切复用（推挽输出，无上下拉），并逐通道配置
        let active_low = config.polarity == Polarity::ActiveLow;
        let regs = T::pwm_regs();
        let speed = config.gpio_speed;

        #[inline]
        fn take_pin<'d, T: PwmInstance, C: TimerChannel>(
            pin: Option<PwmPin<'d, T, C>>,
            speed: Speed,
        ) -> Option<Flex<'d>> {
            pin.map(|p| {
                let PwmPin { mut flex, af, .. } = p;
                flex.set_as_alternate(af, speed, Pull::None, OutputType::PushPull);
                flex
            })
        }
        let ch1 = take_pin(ch1, speed);
        let ch2 = take_pin(ch2, speed);
        let ch3 = take_pin(ch3, speed);
        let ch4 = take_pin(ch4, speed);

        for ch in 0..channels as usize {
            regs.configure_channel(ch, active_low);
        }
        // 高级定时器主输出使能（无 BDTR 的实例为空操作）
        regs.set_moe(true);
        // 启动计数器（通道输出仍关闭，待 enable）
        timer.start();

        Ok(Self {
            timer,
            _ch1: ch1,
            _ch2: ch2,
            _ch3: ch3,
            _ch4: ch4,
            psc,
            arr,
        })
    }

    /// 该实例的 PWM 通道数（1..=4）。
    #[inline]
    pub fn channels(&self) -> u8 {
        T::channels()
    }

    /// 使能某通道输出（`CCER.CCxE = 1`，对照 `HAL_TIM_PWM_Start` 的 `TIM_CCxChannelCmd`）。
    ///
    /// # Errors
    ///
    /// 通道号超出该实例通道数时返回 [`Error::InvalidChannel`]。
    #[inline]
    pub fn enable(&mut self, ch: Channel) -> Result<(), Error> {
        self.check(ch)?;
        T::pwm_regs().set_channel_enable(ch.index(), true);
        Ok(())
    }

    /// 关闭某通道输出（`CCER.CCxE = 0`）。
    ///
    /// # Errors
    ///
    /// 通道号超出该实例通道数时返回 [`Error::InvalidChannel`]。
    #[inline]
    pub fn disable(&mut self, ch: Channel) -> Result<(), Error> {
        self.check(ch)?;
        T::pwm_regs().set_channel_enable(ch.index(), false);
        Ok(())
    }

    /// 查询某通道输出是否使能（`CCER.CCxE`）。
    ///
    /// # Errors
    ///
    /// 通道号超出该实例通道数时返回 [`Error::InvalidChannel`]。
    #[inline]
    pub fn is_enabled(&self, ch: Channel) -> Result<bool, Error> {
        self.check(ch)?;
        Ok(T::pwm_regs().channel_enabled(ch.index()))
    }

    /// 当前周期对应的最大占空比（= `ARR + 1`）。
    ///
    /// `set_duty` 的有效范围为 `0..=max_duty_cycle()`；因 `CCRx` 计数精度到 `ARR`，
    /// 满占空比实际写入 `ARR`（见 [`Pwm::set_duty`]）。
    #[inline]
    pub fn max_duty_cycle(&self) -> u32 {
        self.arr + 1
    }

    /// 设置某通道的占空比（比较值，`CCRx`）。
    ///
    /// `duty = 0` 为常低（模式 1、高有效），`duty >= max_duty_cycle()` 视为满占空比。
    /// 受 `CCRx ≤ ARR` 限制，满占空比写入 `ARR`（略小于 100%，差一个计数周期）。
    ///
    /// # Errors
    ///
    /// 通道号超出该实例通道数时返回 [`Error::InvalidChannel`]。
    #[inline]
    pub fn set_duty(&mut self, ch: Channel, duty: u32) -> Result<(), Error> {
        self.check(ch)?;
        T::pwm_regs().write_ccr(ch.index(), duty.min(self.arr));
        Ok(())
    }

    /// 读取某通道的占空比（比较值，`CCRx`）。
    ///
    /// # Errors
    ///
    /// 通道号超出该实例通道数时返回 [`Error::InvalidChannel`]。
    #[inline]
    pub fn get_duty(&self, ch: Channel) -> Result<u32, Error> {
        self.check(ch)?;
        Ok(T::pwm_regs().read_ccr(ch.index()))
    }

    /// 以百分比设置某通道占空比（`0..=100`）。
    ///
    /// # Errors
    ///
    /// 通道号超出该实例通道数时返回 [`Error::InvalidChannel`]。
    #[inline]
    pub fn set_duty_cycle_percent(&mut self, ch: Channel, percent: u8) -> Result<(), Error> {
        let duty = self.max_duty_cycle() * percent as u32 / 100;
        self.set_duty(ch, duty)
    }

    /// 修改 PWM 频率（重算 `PSC`/`ARR` 并产生更新事件立即装载）。
    ///
    /// # Errors
    ///
    /// 目标频率不可达时返回 [`Error::InvalidConfig`]。
    pub fn set_frequency(&mut self, freq: Hertz) -> Result<(), Error> {
        let cfg = super::Config::from_frequency(self.timer.timer_clk(), freq)?;
        self.timer.set_prescaler(cfg.prescaler);
        self.timer.set_reload(cfg.period)?;
        self.timer.generate_update();
        self.psc = cfg.prescaler;
        self.arr = cfg.period;
        Ok(())
    }

    /// 当前 PWM 频率（`CK_INT / (PSC+1) / (ARR+1)`）。
    #[inline]
    pub fn frequency(&self) -> Hertz {
        Hertz(self.timer.timer_clk().0 / (self.psc as u32 + 1) / (self.arr + 1))
    }

    /// 启动计数器（`CR1.CEN = 1`）。
    #[inline]
    pub fn start(&mut self) {
        self.timer.start();
    }

    /// 停止计数器（`CR1.CEN = 0`）。
    #[inline]
    pub fn stop(&mut self) {
        self.timer.stop();
    }

    /// 校验通道号是否在该实例范围内。
    #[inline]
    fn check(&self, ch: Channel) -> Result<(), Error> {
        if (ch.index() as u8) < T::channels() {
            Ok(())
        } else {
            Err(Error::InvalidChannel)
        }
    }
}

/// 编译期/运行期校验：`present` 为真时，`C` 的通道号须小于实例通道数。
#[inline]
fn check_channel<C: TimerChannel>(present: bool, channels: u8) -> Result<(), Error> {
    if present && (C::CHANNEL.index() as u8) >= channels {
        Err(Error::InvalidChannel)
    } else {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// embedded-hal 1.0 兼容实现（由 crate feature `embedded-hal-1` 开启）
// ---------------------------------------------------------------------------
//
// impl 写在类型所属 crate 内部（孤儿规则，理由见 `crate::gpio` 的对应小节）。
//
// [`Pwm`] 的模型是"一个定时器实例 + 最多 4 个通道"，而 `embedded-hal` 的
// `pwm::SetDutyCycle` 是"一个对象 = 一个通道"，两边对不齐，故用 [`PwmChannel`]
// 这个借用式单通道视图来桥接：第三方驱动拿到 `PwmChannel` 就能直接调占空比。
// （多通道共用时逐个借用即可，借用期间整个 `Pwm` 被独占。）

/// 单通道占空比视图：把 [`Pwm`] 固定到某一通道，实现 `embedded-hal` 的
/// `pwm::SetDutyCycle`。
#[cfg(feature = "embedded-hal-1")]
pub struct PwmChannel<'a, 'd, T: PwmInstance> {
    pwm: &'a mut Pwm<'d, T>,
    channel: Channel,
}

#[cfg(feature = "embedded-hal-1")]
impl<'a, 'd, T: PwmInstance> PwmChannel<'a, 'd, T> {
    /// 把 `pwm` 的 `channel` 通道借出为一个单通道句柄。
    #[inline]
    pub fn new(pwm: &'a mut Pwm<'d, T>, channel: Channel) -> Self {
        Self { pwm, channel }
    }
}

#[cfg(feature = "embedded-hal-1")]
mod embedded_hal_impls {
    use super::{Error, PwmChannel, PwmInstance};
    use embedded_hal_1::pwm::{ErrorKind, ErrorType, SetDutyCycle};

    /// `embedded-hal` 的 PWM 错误分类映射。
    ///
    /// `pwm::ErrorKind` 只有一个 `Other` 变体，故全部归入它。
    impl embedded_hal_1::pwm::Error for Error {
        fn kind(&self) -> ErrorKind {
            ErrorKind::Other
        }
    }

    impl<'a, 'd, T: PwmInstance> ErrorType for PwmChannel<'a, 'd, T> {
        type Error = Error;
    }

    impl<'a, 'd, T: PwmInstance> SetDutyCycle for PwmChannel<'a, 'd, T> {
        /// [`Pwm::max_duty_cycle`] 是 `ARR + 1`（u32）。[`super::super::Config::from_frequency`]
        /// 保证 `ARR ≤ 0xFFFF`，故只有 `ARR == 0xFFFF` 这一种配置会算出 0x10000，
        /// 此时钳到 `u16::MAX`——`set_duty` 本就把超出值钳到 `ARR`，满占空比语义不变。
        #[inline]
        fn max_duty_cycle(&self) -> u16 {
            self.pwm.max_duty_cycle().min(u16::MAX as u32) as u16
        }

        #[inline]
        fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
            self.pwm.set_duty(self.channel, u32::from(duty))
        }
    }
}
