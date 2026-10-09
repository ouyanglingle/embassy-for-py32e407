//! 外设单例（对齐 embassy-stm32 的 `peripherals` 模块形态）。
//!
//! 目前含 GPIO 引脚（PA0…PF15）、串口（UART1/2/3、USART1/2/3、LPUART1）、
//! SPI/I2C/TIM/CORDIC/ESMC，以及 EXTI 线通道（EXTI0…EXTI15，见 [`crate::exti`]）。
//! 每个类型都是零大小的标记
//! 类型；[`Peripherals`] 结构体的字段是 [`crate::Peri`] 句柄，由 [`crate::init`]
//! 一次性取出（对齐 embassy 的 `p.PA5` / `p.UART2` 访问方式）。
//!
//! 引脚的端口号/引脚号通过 [`crate::gpio::Pin`] 暴露（`pin_port` = 端口号 × 16 + 引脚号）。

// Peripherals 字段名对齐 embassy 的外设单例命名（PA0/USART1…），豁免 snake_case 检查
#![allow(non_snake_case)]

use core::sync::atomic::{AtomicBool, Ordering};

use crate::gpio::{Pin, SealedPin};
use crate::{Peri, PeripheralType};

/// 生成单个引脚标记类型：`PeripheralType` + `SealedPin` + `Pin` + `Into<AnyPin>`。
macro_rules! pin {
    ($name:ident, $port_num:literal, $pin_num:literal) => {
        #[doc = concat!(stringify!($name), " 引脚")]
        #[derive(Debug, Clone, Copy)]
        pub struct $name;

        impl PeripheralType for $name {}

        impl SealedPin for $name {
            #[inline]
            fn pin_port(&self) -> u8 {
                $port_num * 16 + $pin_num
            }
        }

        impl Pin for $name {}

        impl From<$name> for crate::gpio::AnyPin {
            fn from(val: $name) -> Self {
                Self {
                    pin_port: val.pin_port(),
                }
            }
        }
    };
}

// GPIOA（pin_port = 0x + n）
pin!(PA0, 0, 0);
pin!(PA1, 0, 1);
pin!(PA2, 0, 2);
pin!(PA3, 0, 3);
pin!(PA4, 0, 4);
pin!(PA5, 0, 5);
pin!(PA6, 0, 6);
pin!(PA7, 0, 7);
pin!(PA8, 0, 8);
pin!(PA9, 0, 9);
pin!(PA10, 0, 10);
pin!(PA11, 0, 11);
pin!(PA12, 0, 12);
pin!(PA13, 0, 13);
pin!(PA14, 0, 14);
pin!(PA15, 0, 15);

// GPIOB（pin_port = 0x10 + n）
pin!(PB0, 1, 0);
pin!(PB1, 1, 1);
pin!(PB2, 1, 2);
pin!(PB3, 1, 3);
pin!(PB4, 1, 4);
pin!(PB5, 1, 5);
pin!(PB6, 1, 6);
pin!(PB7, 1, 7);
pin!(PB8, 1, 8);
pin!(PB9, 1, 9);
pin!(PB10, 1, 10);
pin!(PB11, 1, 11);
pin!(PB12, 1, 12);
pin!(PB13, 1, 13);
pin!(PB14, 1, 14);
pin!(PB15, 1, 15);

// GPIOC（pin_port = 0x20 + n）
pin!(PC0, 2, 0);
pin!(PC1, 2, 1);
pin!(PC2, 2, 2);
pin!(PC3, 2, 3);
pin!(PC4, 2, 4);
pin!(PC5, 2, 5);
pin!(PC6, 2, 6);
pin!(PC7, 2, 7);
pin!(PC8, 2, 8);
pin!(PC9, 2, 9);
pin!(PC10, 2, 10);
pin!(PC11, 2, 11);
pin!(PC12, 2, 12);
pin!(PC13, 2, 13);
pin!(PC14, 2, 14);
pin!(PC15, 2, 15);

// GPIOD（pin_port = 0x30 + n）
pin!(PD0, 3, 0);
pin!(PD1, 3, 1);
pin!(PD2, 3, 2);
pin!(PD3, 3, 3);
pin!(PD4, 3, 4);
pin!(PD5, 3, 5);
pin!(PD6, 3, 6);
pin!(PD7, 3, 7);
pin!(PD8, 3, 8);
pin!(PD9, 3, 9);
pin!(PD10, 3, 10);
pin!(PD11, 3, 11);
pin!(PD12, 3, 12);
pin!(PD13, 3, 13);
pin!(PD14, 3, 14);
pin!(PD15, 3, 15);

// GPIOE（pin_port = 0x40 + n）
pin!(PE0, 4, 0);
pin!(PE1, 4, 1);
pin!(PE2, 4, 2);
pin!(PE3, 4, 3);
pin!(PE4, 4, 4);
pin!(PE5, 4, 5);
pin!(PE6, 4, 6);
pin!(PE7, 4, 7);
pin!(PE8, 4, 8);
pin!(PE9, 4, 9);
pin!(PE10, 4, 10);
pin!(PE11, 4, 11);
pin!(PE12, 4, 12);
pin!(PE13, 4, 13);
pin!(PE14, 4, 14);
pin!(PE15, 4, 15);

// GPIOF（pin_port = 0x50 + n）
pin!(PF0, 5, 0);
pin!(PF1, 5, 1);
pin!(PF2, 5, 2);
pin!(PF3, 5, 3);
pin!(PF4, 5, 4);
pin!(PF5, 5, 5);
pin!(PF6, 5, 6);
pin!(PF7, 5, 7);
pin!(PF8, 5, 8);
pin!(PF9, 5, 9);
pin!(PF10, 5, 10);
pin!(PF11, 5, 11);
pin!(PF12, 5, 12);
pin!(PF13, 5, 13);
pin!(PF14, 5, 14);
pin!(PF15, 5, 15);

/// 为每个引脚实现 [`crate::gpio::ExtiPin`]：**线号只由引脚号决定**，端口只影响
/// `SYSCFG_EXTICR` 里的选择值（RM 13.2.4）——所以 `PA5` / `PB5` / `PC5` … 都是 `EXTI5`。
///
/// 用独立宏块而不是给上面的 [`pin!`] 加参数，是为了不动既有引脚定义。
macro_rules! impl_exti_pin {
    ($($pin:ident => $ch:ident,)*) => {
        $(
            impl crate::gpio::ExtiPin for $pin {
                type ExtiChannel = $ch;
            }
        )*
    };
}

impl_exti_pin! {
    PA0 => EXTI0, PB0 => EXTI0, PC0 => EXTI0, PD0 => EXTI0, PE0 => EXTI0, PF0 => EXTI0,
    PA1 => EXTI1, PB1 => EXTI1, PC1 => EXTI1, PD1 => EXTI1, PE1 => EXTI1, PF1 => EXTI1,
    PA2 => EXTI2, PB2 => EXTI2, PC2 => EXTI2, PD2 => EXTI2, PE2 => EXTI2, PF2 => EXTI2,
    PA3 => EXTI3, PB3 => EXTI3, PC3 => EXTI3, PD3 => EXTI3, PE3 => EXTI3, PF3 => EXTI3,
    PA4 => EXTI4, PB4 => EXTI4, PC4 => EXTI4, PD4 => EXTI4, PE4 => EXTI4, PF4 => EXTI4,
    PA5 => EXTI5, PB5 => EXTI5, PC5 => EXTI5, PD5 => EXTI5, PE5 => EXTI5, PF5 => EXTI5,
    PA6 => EXTI6, PB6 => EXTI6, PC6 => EXTI6, PD6 => EXTI6, PE6 => EXTI6, PF6 => EXTI6,
    PA7 => EXTI7, PB7 => EXTI7, PC7 => EXTI7, PD7 => EXTI7, PE7 => EXTI7, PF7 => EXTI7,
    PA8 => EXTI8, PB8 => EXTI8, PC8 => EXTI8, PD8 => EXTI8, PE8 => EXTI8, PF8 => EXTI8,
    PA9 => EXTI9, PB9 => EXTI9, PC9 => EXTI9, PD9 => EXTI9, PE9 => EXTI9, PF9 => EXTI9,
    PA10 => EXTI10, PB10 => EXTI10, PC10 => EXTI10, PD10 => EXTI10, PE10 => EXTI10, PF10 => EXTI10,
    PA11 => EXTI11, PB11 => EXTI11, PC11 => EXTI11, PD11 => EXTI11, PE11 => EXTI11, PF11 => EXTI11,
    PA12 => EXTI12, PB12 => EXTI12, PC12 => EXTI12, PD12 => EXTI12, PE12 => EXTI12, PF12 => EXTI12,
    PA13 => EXTI13, PB13 => EXTI13, PC13 => EXTI13, PD13 => EXTI13, PE13 => EXTI13, PF13 => EXTI13,
    PA14 => EXTI14, PB14 => EXTI14, PC14 => EXTI14, PD14 => EXTI14, PE14 => EXTI14, PF14 => EXTI14,
    PA15 => EXTI15, PB15 => EXTI15, PC15 => EXTI15, PD15 => EXTI15, PE15 => EXTI15, PF15 => EXTI15,
}

/// 生成一个非 GPIO 的外设标记类型（`PeripheralType`）。
///
/// 具体功能（寄存器块、RCC 位、引脚 AF）由对应驱动模块的密封 trait 提供，
/// 例如 [`crate::uart::Instance`] 为 UART1/2/3 提供寄存器块与时钟位。
macro_rules! periph {
    ($name:ident) => {
        #[doc = concat!(stringify!($name), " 外设单例")]
        #[derive(Debug, Clone, Copy)]
        pub struct $name;

        impl PeripheralType for $name {}
    };
}

// UART（RM 第 38 章）：UART1 = 0x4000_4C00、UART2 = 0x4000_5000、UART3 = 0x4000_5C00，
// 时钟同在 RCC_APB1ENR1（bit19 / bit20 / bit23）。
periph!(UART1);
periph!(UART2);
periph!(UART3);

// LPUART（RM 第 39 章）：单实例，时钟在 RCC_APB1ENR2（bit1）。
periph!(LPUART1);

// USART（RM 第 37 章）：USART1 = 0x4001_3800（APB2ENR bit14）、
// USART2 = 0x4000_4400、USART3 = 0x4000_4800（同在 APB1ENR1，bit17 / bit18）。
// 与第 38 章 UART、第 39 章 LPUART 是三套寄存器映射互不兼容的独立外设。
periph!(USART1);
periph!(USART2);
periph!(USART3);

// SPI（RM 第 35 章）：SPI1 = 0x4001_3000（APB2ENR bit12）、
// SPI2 = 0x4000_3800（APB1ENR1 bit14）、SPI3 = 0x4000_3c00（APB1ENR1 bit15）。
// 三个实例共用同一寄存器块（PAC 中 spi2/spi3 是 spi1 的别名）。
periph!(SPI1);
periph!(SPI2);
periph!(SPI3);

// I2C（RM 第 36 章）：I2C1 = 0x4000_5400（APB1ENR1 bit21）、I2C2 = 0x4000_5800（bit22）、
// I2C3 = 0x4000_7800（bit30）、I2C4 = 0x4000_8400（APB1ENR2 bit2）。
// 四个实例共用同一寄存器块（PAC 中 i2c2/3/4 是 i2c1 的别名）。
periph!(I2C1);
periph!(I2C2);
periph!(I2C3);
periph!(I2C4);

// TIM 定时器（RM 第 19–24 章）：TIM1–TIM19。功能（寄存器块、RCC 位、位宽）
// 由 [`crate::timer::Instance`] 提供。注意 APB2ENR 上的定时器位号与 STM32 不同
// （TIM1=bit11、TIM8=bit13、TIM15/16/17=22/23/24、TIM19=25，见 SVD）。
periph!(TIM1);
periph!(TIM2);
periph!(TIM3);
periph!(TIM4);
periph!(TIM5);
periph!(TIM6);
periph!(TIM7);
periph!(TIM8);
periph!(TIM9);
periph!(TIM10);
periph!(TIM11);
periph!(TIM12);
periph!(TIM13);
periph!(TIM14);
periph!(TIM15);
periph!(TIM16);
periph!(TIM17);
periph!(TIM18);
periph!(TIM19);

// CORDIC 协处理器（RM 第 43 章）：单实例 0x4002_C000，
// 时钟/复位在 RCC_AHB1ENR / AHB1RSTR（bit5）。
periph!(CORDIC);

// 外部串行存储控制器 ESMC / QSPI（RM 第 30 章）：单实例 0xA000_1000（AHB3），
// 时钟/复位在 RCC_AHB1ENR / AHB1RSTR（bit12，ESMCEN/ESMCRST）。
periph!(ESMC);

// EXTI 通道（RM 第 13 章）：**不是独立外设**，而是"EXTI 线"的所有权句柄。
// 理由同 embassy-stm32：一条线可能被同端口号的多个引脚共用（PA5/PB5/PC5… → EXTI5），
// 借由单例保证同一时刻只有一个驱动能占用它。通道实现见 [`crate::exti::Channel`]。
periph!(EXTI0);
periph!(EXTI1);
periph!(EXTI2);
periph!(EXTI3);
periph!(EXTI4);
periph!(EXTI5);
periph!(EXTI6);
periph!(EXTI7);
periph!(EXTI8);
periph!(EXTI9);
periph!(EXTI10);
periph!(EXTI11);
periph!(EXTI12);
periph!(EXTI13);
periph!(EXTI14);
periph!(EXTI15);

/// 生成 [`Peripherals`] 结构体与 `take()`。
macro_rules! impl_peripherals {
    ($($name:ident,)*) => {
        /// 全部外设单例（对齐 embassy-stm32 的 `Peripherals`）。
        ///
        /// 字段是 [`Peri`] 句柄，直接传给各驱动构造器（如 `Output::new(p.PA5, ...)`）。
        /// 每个单例在整个程序中只能被取出一次。
        pub struct Peripherals {
            $(
                #[doc = concat!(stringify!($name), " 单例")]
                pub $name: Peri<'static, $name>,
            )*
        }

        impl Peripherals {
            /// 取出全部外设单例；重复调用 panic。
            pub fn take() -> Self {
                static TAKEN: AtomicBool = AtomicBool::new(false);
                if TAKEN.swap(true, Ordering::SeqCst) {
                    panic!("Peripherals::take() called twice");
                }
                unsafe { Self { $($name: Peri::new_unchecked($name),)* } }
            }
        }
    };
}

impl_peripherals!(
    PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10, PA11, PA12, PA13, PA14, PA15, PB0, PB1,
    PB2, PB3, PB4, PB5, PB6, PB7, PB8, PB9, PB10, PB11, PB12, PB13, PB14, PB15, PC0, PC1, PC2, PC3,
    PC4, PC5, PC6, PC7, PC8, PC9, PC10, PC11, PC12, PC13, PC14, PC15, PD0, PD1, PD2, PD3, PD4, PD5,
    PD6, PD7, PD8, PD9, PD10, PD11, PD12, PD13, PD14, PD15, PE0, PE1, PE2, PE3, PE4, PE5, PE6, PE7,
    PE8, PE9, PE10, PE11, PE12, PE13, PE14, PE15, PF0, PF1, PF2, PF3, PF4, PF5, PF6, PF7, PF8, PF9,
    PF10, PF11, PF12, PF13, PF14, PF15, UART1, UART2, UART3, LPUART1, USART1, USART2, USART3, SPI1,
    SPI2, SPI3, I2C1, I2C2, I2C3, I2C4, TIM1, TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM8, TIM9, TIM10,
    TIM11, TIM12, TIM13, TIM14, TIM15, TIM16, TIM17, TIM18, TIM19, CORDIC, ESMC,
    EXTI0, EXTI1, EXTI2, EXTI3, EXTI4, EXTI5, EXTI6, EXTI7, EXTI8, EXTI9, EXTI10, EXTI11, EXTI12,
    EXTI13, EXTI14, EXTI15,
);
