//! TIM 正交编码器接口（QEI，同步 Blocking 驱动）
//!
//! 在 [`super::Timer`] 之上实现硬件正交编码器解码。寄存器流程依据：
//! - HAL C: `py32e407_hal_tim.c` 的 `HAL_TIM_Encoder_Init`（清 `SMCR.SMS|ECE` →
//!   写 `CCMR1` → 写 `CCER` → 写 `SMCR.SMS`；ARR 由 `TIM_Base_SetConfig` 设置）与
//!   `HAL_TIM_Encoder_Start`（`CC1E/CC2E` → `CEN`，**不写 `BDTR.MOE`**），
//!   配置结构体 `TIM_Encoder_InitTypeDef`（`py32e407_hal_tim.h`）
//! - LL:    `py32e407_ll_tim.h`（`LL_TIM_SetEncoderMode`、`LL_TIM_IC_SetActiveInput`、
//!   `LL_TIM_IC_SetPolarity`、`LL_TIM_IC_SetFilter`、`LL_TIM_IC_SetPrescaler`、
//!   `LL_TIM_GetCounter`、`LL_TIM_SetCounter`、`LL_TIM_GetDirection`）
//! - RM:    PY32E407 参考手册 V0.6 第 19 章（TIM1/8）与第 20 章（TIM2–5/18）的
//!   「编码器接口模式」节（19.2.23 / 20.2.17）与 19.3.3/20.3.3（`SMCR`）
//! - DS:    PY32E407 数据手册 V0.13 端口复用功能映射表（编码器 A/B 输入引脚 AF 号）
//!
//! # 关键实现事实（已用 RM + CMSIS + PAC + HAL 交叉验证）
//!
//! - **只有 7 个实例支持硬件编码器接口**：TIM1、TIM2、TIM3、TIM4、TIM5、TIM8、TIM18
//!   （CMSIS `IS_TIM_ENCODER_INTERFACE_INSTANCE` + RM 19/20 章）。TIM9/12/15/16/17
//!   虽有 CH1/CH2，但其 `SMCR.SMS` 的 001/010/011 为**保留**，无法做 QEI——故本模块
//!   只为上述 7 个实例实现 [`EncoderInstance`]。
//! - **`SMCR.SMS` 语义**（RM 20.3.3）：`001` = 编码器模式 1（在 TI1 边沿计数，x2）、
//!   `010` = 模式 2（TI2，x2）、`011` = 模式 3（TI1+TI2 双边沿，x4）。PY32 把 SMS 拆成
//!   `SMS[2:0]` + `SMS[3]`（bit16），编码器三值只用低 3 位，`SMS[3]=0`。
//! - **`CCMR1` 输入视图**：`CC1S=CC2S=01`（IC1 映射到 TI1、IC2 映射到 TI2）；`IC1F/IC2F`
//!   为 4 位输入滤波，`IC1PSC/IC2PSC` 为 2 位输入预分频（RM 19.3.7）。
//! - **`CCER`**：`CC1E/CC2E` 使能两路输入；`CC1P/CC2P` 设极性（`0` 上升/不反相、
//!   `1` 下降/反相；RM 明确编码器模式下**不能**用双边沿 `CCxP=CCxNP=1`）。
//! - **`CR1.DIR`（bit4）在编码器模式下为只读**，由硬件按旋转方向设置（RM 19.3.1）。
//! - **位置即 `CNT`、回绕边界即 `ARR`**（RM 19.2.23.1：「计数器的内容始终指示着编码器的
//!   位置」；开始计数前必须配置 `ARR`）。TIM2 为 31/32 位计数器，其余为 16 位。
//!
//! # 引脚复用
//!
//! 编码器 A/B 输入即该定时器的 **CH1/CH2 引脚**（编码器模式下它们作 TI1/TI2 输入）。
//! 本模块**直接复用 [`super::pwm`] 的通道引脚表**（`ChannelPin<T, Ch1>` / `ChannelPin<T, Ch2>`），
//! 不重复定义引脚。TIM1 的 PA8/PA9 = AF6 有官方例程
//! `Example/TIM/TIM1_EncoderTI2AndTI1` 佐证；其余为 DS 表抽取。
//!
//! # 使用示例
//!
//! ```ignore
//! use embassy_py32::timer::qei::{Config, Qei};
//! use embassy_py32::init;
//!
//! let p = init(Default::default());
//! // TIM2：PA0 = TI1（CH1）、PA1 = TI2（CH2），AF1；模式 3（x4）
//! let mut qei = Qei::new(p.TIM2, p.PA0, p.PA1, Config::default()).unwrap();
//! loop {
//!     let pos = qei.count();      // 当前编码器位置
//!     let dir = qei.read_direction();
//! }
//! ```
//!
//! 仅实现同步配置与读取：不含 DMA/中断/Async，也不含指数（index）输入与从模式触发。

use crate::gpio::{Flex, OutputType, Pull, Speed};
use crate::Peri;

use super::Error;
use super::pwm::{Ch1, Ch2, ChannelPin};

/// 编码器计数模式（`SMCR.SMS`，RM 20.3.3）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QeiMode {
    /// 编码器模式 1（`SMS = 001`）：在 TI1 边沿计数、方向由 TI2 电平决定（x2 分辨率）。
    Mode1,
    /// 编码器模式 2（`SMS = 010`）：在 TI2 边沿计数、方向由 TI1 电平决定（x2 分辨率）。
    Mode2,
    /// 编码器模式 3（`SMS = 011`）：在 TI1 与 TI2 双边沿计数（x4 分辨率，复位默认）。
    Mode3,
}

impl QeiMode {
    /// 映射为 `SMCR.SMS[2:0]`（见模块 doc）。
    #[inline]
    fn sms(self) -> u8 {
        match self {
            Self::Mode1 => 0b001,
            Self::Mode2 => 0b010,
            Self::Mode3 => 0b011,
        }
    }
}

/// 编码器输入极性（`CCER.CCxP`，RM 19.3.9）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Polarity {
    /// 上升沿计数 / 不反相（`CCxP = 0`）。
    Rising,
    /// 下降沿计数 / 反相（`CCxP = 1`）。交换 A/B 相即可反转计数方向。
    Falling,
}

impl Polarity {
    /// 映射为 `CCxP`。
    #[inline]
    fn bit(self) -> bool {
        matches!(self, Self::Falling)
    }
}

/// 编码器输入预分频（`CCMR1.ICxPSC`，RM 19.3.7）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Prescaler {
    /// 每 1 个有效边沿计数（`ICxPSC = 00`，复位默认）。
    Div1,
    /// 每 2 个有效边沿计数（`ICxPSC = 01`）。
    Div2,
    /// 每 4 个有效边沿计数（`ICxPSC = 10`）。
    Div4,
    /// 每 8 个有效边沿计数（`ICxPSC = 11`）。
    Div8,
}

impl Prescaler {
    /// 映射为 `ICxPSC[1:0]`。
    #[inline]
    fn bits(self) -> u8 {
        match self {
            Self::Div1 => 0b00,
            Self::Div2 => 0b01,
            Self::Div4 => 0b10,
            Self::Div8 => 0b11,
        }
    }
}

/// 计数方向（`CR1.DIR`，编码器模式下只读，RM 19.3.1）。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    /// 向上计数（`DIR = 0`）。
    Up,
    /// 向下计数（`DIR = 1`）。
    Down,
}

/// QEI 配置。
///
/// `#[non_exhaustive]`，外部构造请用 [`Config::default()`] 后逐字段赋值。
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// 编码器计数模式。
    pub mode: QeiMode,
    /// 通道 1（TI1）输入极性。
    pub ch1_polarity: Polarity,
    /// 通道 2（TI2）输入极性。
    pub ch2_polarity: Polarity,
    /// 通道 1 输入滤波（`IC1F`，0..=15；0 = 不滤波）。数值越大滤波越强，见 RM 19.3.7。
    pub ch1_filter: u8,
    /// 通道 2 输入滤波（`IC2F`，0..=15）。
    pub ch2_filter: u8,
    /// 通道 1 输入预分频。
    pub ch1_prescaler: Prescaler,
    /// 通道 2 输入预分频。
    pub ch2_prescaler: Prescaler,
    /// 自动重装载值（`ARR`）：编码器计数的回绕边界，`CNT` 在 `0..=auto_reload` 间循环。
    /// 16 位实例须 ≤ 0xFFFF，TIM2 可到 0x7FFF_FFFF。
    pub auto_reload: u32,
    /// 通道 1 引脚的内部上/下拉。
    pub ch1_pull: Pull,
    /// 通道 2 引脚的内部上/下拉。
    pub ch2_pull: Pull,
    /// 复用引脚的输出速度（`OSPEEDR`；编码器为输入，速度无实际影响）。
    pub gpio_speed: Speed,
}

impl Default for Config {
    /// 默认值对照 `LL_TIM_ENCODER_StructInit`：模式 3（x4）、上升沿、不滤波、不分频、
    /// ARR = 0xFFFF、无上下拉、GPIO 超高速档。
    fn default() -> Self {
        Self {
            mode: QeiMode::Mode3,
            ch1_polarity: Polarity::Rising,
            ch2_polarity: Polarity::Rising,
            ch1_filter: 0,
            ch2_filter: 0,
            ch1_prescaler: Prescaler::Div1,
            ch2_prescaler: Prescaler::Div1,
            auto_reload: 0xFFFF,
            ch1_pull: Pull::None,
            ch2_pull: Pull::None,
            gpio_speed: Speed::VeryHigh,
        }
    }
}

// ---------------------------------------------------------------------------
// 寄存器块与实例
// ---------------------------------------------------------------------------

/// 编码器配置的寄存器写入参数（由 [`Config`] 归一化而来）。
mod sealed {
    /// 编码器配置的寄存器写入参数（由 [`super::Config`] 归一化而来）。
    #[derive(Debug, Clone, Copy)]
    pub struct EncCfg {
        /// `SMCR.SMS[2:0]`。
        pub sms: u8,
        /// 通道 1 极性（`CC1P`）。
        pub pol1: bool,
        /// 通道 2 极性（`CC2P`）。
        pub pol2: bool,
        /// `CCMR1.IC1F`。
        pub f1: u8,
        /// `CCMR1.IC2F`。
        pub f2: u8,
        /// `CCMR1.IC1PSC`。
        pub psc1: u8,
        /// `CCMR1.IC2PSC`。
        pub psc2: u8,
    }

    /// 各 TIM 寄存器块统一的编码器寄存器操作。
    ///
    /// 编码器实例只落在两类 PAC 寄存器块上：`tim1`（TIM1/TIM8）与 `tim2`
    /// （TIM2/3/4/5/18）；两块字段名一致，故用同一宏实现。CNT/ARR 位宽随块不同
    /// （TIM1 16 位 / TIM2 31 位），用 `as _` 让编译器推断。
    pub trait SealedEncoderBlock {
        /// 配置编码器模式：清 `SMCR.SMS|ECE` → 写 `CCMR1`（输入视图）→ 写 `CCER`
        /// → 写 `SMCR.SMS`（对照 `HAL_TIM_Encoder_Init`）。
        fn configure(&self, cfg: EncCfg);

        /// 置/清 `CR1.CEN`（`__HAL_TIM_ENABLE` / `DISABLE`）。
        fn set_cen(&self, on: bool);

        /// 读 `CR1.DIR`（`true` = 向下计数）。
        fn dir(&self) -> bool;

        /// 读 `CNT`（编码器位置）。
        fn count(&self) -> u32;

        /// 写 `CNT`。
        fn set_count(&self, val: u32);

        /// 写 `ARR`（回绕边界）。
        fn set_arr(&self, val: u32);

        /// 读 `ARR`。
        fn arr(&self) -> u32;
    }

    /// 密封的编码器实例信息（防止外部为任意类型实现 [`super::EncoderInstance`]）。
    pub trait SealedEncoderInstance {
        /// 该实例的编码器寄存器块。
        fn encoder_block(&self) -> &'static dyn SealedEncoderBlock;

        /// 该实例计数器的最大回绕值（16 位实例 0xFFFF，TIM2 为 0x7FFF_FFFF）。
        fn max_count(&self) -> u32;
    }
}

/// 为一个 PAC 寄存器块实现 [`sealed::SealedEncoderBlock`]。
macro_rules! impl_encoder_block {
    ($pacmod:ident) => {
        impl sealed::SealedEncoderBlock for crate::pac::$pacmod::RegisterBlock {
            #[inline]
            fn configure(&self, cfg: sealed::EncCfg) {
                // HAL_TIM_Encoder_Init：先清 SMCR.SMS|ECE（仅清这两项，TS/ETR 不动）
                self.smcr().modify(|_, w| unsafe {
                    w.sms_l().bits(0);
                    w.sms_h().bit(false);
                    w.ece().clear_bit()
                });
                // CCMR1 输入视图：CC1S=CC2S=01（ICx 映射到 TIx）、ICxF、ICxPSC（RM 19.3.7）
                self.ccmr1_input().modify(|_, w| unsafe {
                    w.cc1s().bits(0b01);
                    w.cc2s().bits(0b01);
                    w.ic1f().bits(cfg.f1);
                    w.ic2f().bits(cfg.f2);
                    w.ic1psc().bits(cfg.psc1);
                    w.ic2psc().bits(cfg.psc2)
                });
                // CCER：使能两路输入（CC1E/CC2E），设极性（CC1P/CC2P）（RM 19.3.9）
                self.ccer().modify(|_, w| {
                    w.cc1e().set_bit();
                    w.cc2e().set_bit();
                    w.cc1p().bit(cfg.pol1);
                    w.cc2p().bit(cfg.pol2);
                    w
                });
                // SMCR.SMS：最后写编码器模式（SMS[3]=0）（RM 19.3.3）
                self.smcr().modify(|_, w| unsafe {
                    w.sms_l().bits(cfg.sms);
                    w.sms_h().bit(false)
                });
            }

            #[inline]
            fn set_cen(&self, on: bool) {
                self.cr1().modify(|_, w| w.cen().bit(on));
            }

            #[inline]
            fn dir(&self) -> bool {
                self.cr1().read().dir().bit_is_set()
            }

            #[inline]
            fn count(&self) -> u32 {
                self.cnt().read().cnt().bits() as u32
            }

            #[inline]
            fn set_count(&self, val: u32) {
                self.cnt().write(|w| unsafe { w.bits(val as _) });
            }

            #[inline]
            fn set_arr(&self, val: u32) {
                self.arr().write(|w| unsafe { w.bits(val as _) });
            }

            #[inline]
            fn arr(&self) -> u32 {
                self.arr().read().arr().bits()
            }
        }
    };
}

impl_encoder_block!(tim1); // TIM1 / TIM8
impl_encoder_block!(tim2); // TIM2 / TIM3 / TIM4 / TIM5 / TIM18

/// 支持硬件编码器接口的 TIM 实例。
///
/// 需同时满足：是普通定时器实例（[`super::Instance`]）且具备 CH1/CH2 通道
/// （[`super::pwm::PwmInstance`]）。只有 TIM1/2/3/4/5/8/18 实现本 trait
/// （CMSIS `IS_TIM_ENCODER_INTERFACE_INSTANCE`），其余实例无法构造 [`Qei`]。
#[allow(private_bounds)]
pub trait EncoderInstance: super::Instance + super::pwm::PwmInstance + sealed::SealedEncoderInstance {}

/// 为 TIM 实例实现 [`sealed::SealedEncoderInstance`] 与 [`EncoderInstance`]。
///
/// `$max`：该实例计数器的最大回绕值（`IS_TIM_32B_COUNTER_INSTANCE` 仅 TIM2）。
macro_rules! impl_encoder_instance {
    ($inst:ident, $pac:ident, $max:expr) => {
        impl sealed::SealedEncoderInstance for crate::peripherals::$inst {
            #[inline]
            fn encoder_block(&self) -> &'static dyn sealed::SealedEncoderBlock {
                // 基址与类型均来自 PAC；同块别名实例（如 TIM8 复用 tim1）指向各自基址。
                unsafe { &*crate::pac::$pac::PTR }
            }

            #[inline]
            fn max_count(&self) -> u32 {
                $max
            }
        }

        impl EncoderInstance for crate::peripherals::$inst {}
    };
}

impl_encoder_instance!(TIM1, Tim1, 0xFFFF);
impl_encoder_instance!(TIM8, Tim8, 0xFFFF);
impl_encoder_instance!(TIM2, Tim2, 0x7FFF_FFFF);
impl_encoder_instance!(TIM3, Tim3, 0xFFFF);
impl_encoder_instance!(TIM4, Tim4, 0xFFFF);
impl_encoder_instance!(TIM5, Tim5, 0xFFFF);
impl_encoder_instance!(TIM18, Tim18, 0xFFFF);

// ---------------------------------------------------------------------------
// 驱动
// ---------------------------------------------------------------------------

/// 正交编码器接口（QEI）同步驱动。
///
/// 持有定时器单例与 A/B 两路输入引脚。构造后计数器立即运行（`CEN = 1`），
/// 编码器位置由 [`Qei::count`] 读取，方向由 [`Qei::read_direction`] 读取。
pub struct Qei<'d, T: EncoderInstance> {
    _peri: Peri<'d, T>,
    _ch1: Flex<'d>,
    _ch2: Flex<'d>,
    max_count: u32,
}

impl<'d, T: EncoderInstance> Qei<'d, T> {
    /// 创建编码器驱动并完成初始化（阻塞）。
    ///
    /// 流程对照 `HAL_TIM_Encoder_Init` + `HAL_TIM_Encoder_Start`：
    /// 1. 使能该实例时钟并复位；
    /// 2. 把 `ch1`/`ch2`（TI1/TI2）配为 AF 输入（推挽、按配置上下拉）；
    /// 3. 清 `SMCR.SMS|ECE` → 写 `CCMR1`（`CCxS=01`、滤波、预分频）→ 写 `CCER`
    ///    （`CCxE=1`、极性）→ 写 `SMCR.SMS` → 写 `ARR` → `CR1.CEN = 1`。
    ///
    /// 编码器为输入功能，无需 `BDTR.MOE`（对照 `HAL_TIM_Encoder_Start` 不写 MOE）。
    ///
    /// # Errors
    ///
    /// 输入滤波 > 15 或 `auto_reload` 超出该实例计数器位宽时返回 [`Error::InvalidConfig`]。
    pub fn new(
        peri: Peri<'d, T>,
        ch1: Peri<'d, impl ChannelPin<T, Ch1>>,
        ch2: Peri<'d, impl ChannelPin<T, Ch2>>,
        config: Config,
    ) -> Result<Self, Error> {
        if config.ch1_filter > 15 || config.ch2_filter > 15 {
            return Err(Error::InvalidConfig);
        }
        let max_count = peri.max_count();
        if config.auto_reload > max_count {
            return Err(Error::InvalidConfig);
        }

        // 使能时钟 + 复位（复用 timer 模块的 RCC 逻辑）
        peri.enable_clock();
        // Safety: 持有该外设唯一句柄，复位不会影响其他使用者
        unsafe { peri.reset() };

        // AF 号要在引脚被 Flex 接管之前取出（编码器输入即 CH1/CH2 引脚）
        let af1 = ch1.af_num();
        let af2 = ch2.af_num();
        let mut c1 = Flex::new(ch1);
        let mut c2 = Flex::new(ch2);
        // 复用模式（推挽）；编码器为输入，上下拉按配置（编码器常用上拉抑制悬空）
        c1.set_as_alternate(af1, config.gpio_speed, config.ch1_pull, OutputType::PushPull);
        c2.set_as_alternate(af2, config.gpio_speed, config.ch2_pull, OutputType::PushPull);

        let block = peri.encoder_block();
        block.configure(sealed::EncCfg {
            sms: config.mode.sms(),
            pol1: config.ch1_polarity.bit(),
            pol2: config.ch2_polarity.bit(),
            f1: config.ch1_filter,
            f2: config.ch2_filter,
            psc1: config.ch1_prescaler.bits(),
            psc2: config.ch2_prescaler.bits(),
        });
        // ARR：编码器回绕边界（RM 19.2.23.1 要求计数前配置）
        block.set_arr(config.auto_reload);
        // 启动计数器（CEN=1；RM 19.3.1 注：编码器模式需 CEN=1 才工作）
        block.set_cen(true);

        Ok(Self {
            _peri: peri,
            _ch1: c1,
            _ch2: c2,
            max_count,
        })
    }

    /// 读当前编码器位置（`CNT`）。
    #[inline]
    pub fn count(&self) -> u32 {
        self._peri.encoder_block().count()
    }

    /// 写编码器位置（`CNT`）。
    ///
    /// # Errors
    ///
    /// `count` 超出该实例计数器位宽时返回 [`Error::InvalidConfig`]。
    pub fn set_count(&mut self, count: u32) -> Result<(), Error> {
        if count > self.max_count {
            return Err(Error::InvalidConfig);
        }
        self._peri.encoder_block().set_count(count);
        Ok(())
    }

    /// 位置清零（`CNT = 0`）。
    #[inline]
    pub fn reset(&mut self) {
        self._peri.encoder_block().set_count(0);
    }

    /// 该实例的最大回绕值（`ARR` 上限）。
    #[inline]
    pub fn max_count(&self) -> u32 {
        self.max_count
    }

    /// 读当前配置的 `ARR`（回绕边界）。
    #[inline]
    pub fn max_auto_reload(&self) -> u32 {
        self._peri.encoder_block().arr()
    }

    /// 读编码器旋转方向（`CR1.DIR`）。
    #[inline]
    pub fn read_direction(&self) -> Direction {
        if self._peri.encoder_block().dir() {
            Direction::Down
        } else {
            Direction::Up
        }
    }

    /// 启动计数器（`CR1.CEN = 1`）。
    #[inline]
    pub fn start(&mut self) {
        self._peri.encoder_block().set_cen(true);
    }

    /// 停止计数器（`CR1.CEN = 0`）；位置保持，`start` 后继续。
    #[inline]
    pub fn stop(&mut self) {
        self._peri.encoder_block().set_cen(false);
    }
}
