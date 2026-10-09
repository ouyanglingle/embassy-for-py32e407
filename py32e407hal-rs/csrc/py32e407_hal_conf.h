/**
 * @file    py32e407_hal_conf.h
 * @brief   HAL configuration file for py32e407hal-rs.
 *
 * **不是官方原始文件**——是本 crate 选定的精简配置。官方每个例程各自带一份，
 * 字段几乎一样；这里挑出 cbind-demo + hal-demo 共同需要的：
 *
 * | 宏 | 含义 |
 * |---|---|
 * | `HAL_*_MODULE_ENABLED` | HAL 哪些外设要被编译（其余模块的 `HAL_*_Init` 等不会被编进来） |
 * | `HSE_VALUE` | 外部晶振频率，HAL 的 `HAL_RCC_GetSysClockFreq` 用它算 PLL 参数 |
 * | `HSE_STARTUP_TIMEOUT` | HAL 等 HSE 起振的毫秒数 |
 * | `HSI_VALUE` / `HSI48_VALUE` / `LSI_VALUE` / `LSE_VALUE` | 各内置振荡器 |
 * | `TICK_INT_PRIORITY` | SysTick 中断优先级 |
 * | `USE_RTOS` | 本 crate 不带 RTOS，必须是 0 |
 * | `USE_*_REGISTER_CALLBACKS` | 每个外设的可选 MSP 回调（0 = 关闭，省 .text） |
 *
 * **改动这里改 enable 列表**，不要动文件结构 —— HAL 头会按这个列表 #include 相应子头。
 */

#ifndef __PY32E407_HAL_CONF_H
#define __PY32E407_HAL_CONF_H

#ifdef __cplusplus
 extern "C" {
#endif

/* =================== Module Selection ===================
 *
 * **只 enable 真正需要的**——HAL 的每个模块都有自己的 `_Init`/`_DeInit`/状态机，
 * 多 enable 一个就让 .text +2~10 KB。本 demo 只用 UART+DMA，配套开
 * GPIO / PWR / RCC / CORTEX / FLASH。其它（USART / SPI / I2C / TIM 等）按需补。
 */
#define HAL_MODULE_ENABLED
#define HAL_CORTEX_MODULE_ENABLED
#define HAL_FLASH_MODULE_ENABLED
#define HAL_PWR_MODULE_ENABLED
#define HAL_RCC_MODULE_ENABLED
#define HAL_GPIO_MODULE_ENABLED
#define HAL_DMA_MODULE_ENABLED
#define HAL_UART_MODULE_ENABLED
/* 其余模块默认关；以后要开（USART/SPI/I2C/TIM …）取消对应行的注释即可。 */
/* #define HAL_EXTI_MODULE_ENABLED */
/* #define HAL_USART_MODULE_ENABLED */
/* #define HAL_SPI_MODULE_ENABLED */
/* #define HAL_I2C_MODULE_ENABLED */
/* #define HAL_TIM_MODULE_ENABLED */
/* #define HAL_ADC_MODULE_ENABLED */
/* #define HAL_RTC_MODULE_ENABLED */

/* =================== Register Callbacks ===================
 *
 * 每个外设可选择是否暴露 MSP 回调指针（节省几百字节 .text / 避免 1 KB+ 的
 * 函数指针表）。本 demo 关掉所有 —— 0 = 关闭。
 */
#define USE_HAL_UART_REGISTER_CALLBACKS        0U
#define USE_HAL_USART_REGISTER_CALLBACKS       0U
#define USE_HAL_GPIO_REGISTER_CALLBACKS        0U
#define USE_HAL_DMA_REGISTER_CALLBACKS         0U
#define USE_HAL_RCC_REGISTER_CALLBACKS         0U
#define USE_HAL_PWR_REGISTER_CALLBACKS         0U
#define USE_HAL_FLASH_REGISTER_CALLBACKS       0U
#define USE_HAL_CORTEX_REGISTER_CALLBACKS      0U

/* =================== Oscillator Values ===================
 *
 * ⚠️ **本板 HSE 是 8 MHz**，不是 STK 模板里的 24 MHz。
 * HAL 的 `HAL_RCC_GetSysClockFreq` 会读这些宏算 PLL 输出频率，**用错会得到错的波特率**。
 */
#define HSE_VALUE                          8000000U
#define HSE_STARTUP_TIMEOUT                  100U
#define HSI_VALUE                           16000000U
#define HSI48_VALUE                         48000000U
#define LSI_VALUE                              40000U
#define LSE_VALUE                              32768U
#define LSE_STARTUP_TIMEOUT                    5000U

/* =================== System Configuration =================== */
#define VDD_VALUE                            3300U
#define TICK_INT_PRIORITY                    0x07U
#define USE_RTOS                                0U

/* =================== Assert ===================
 *
 * 关闭断言 → release 构建省 .text；release 出问题靠日志/行为而不是 panic。
 * 调试时改成 1U 可以让 `IS_*_INSTANCE()` 等检查在越界时直接死。
 *
 * ⚠️ **`assert_param` 宏本体必须显式定义**：HAL 各模块 `.c` 在函数体里调它，
 * 编译时若没宏就是「implicit declaration of function」。USE_FULL_ASSERT=0 时走
 * `((void)0U)`，不开销也不报错。
 */
#define USE_FULL_ASSERT                        0U

#if USE_FULL_ASSERT
#define assert_param(expr) ((expr) ? (void)0U : assert_failed((uint8_t *)__FILE__, __LINE__))
void assert_failed(uint8_t *file, uint32_t line);
#else
#define assert_param(expr) ((void)0U)
#endif

/* =================== SPI 的 CRC 特性 =================== */
#define USE_SPI_CRC                            0U

/* **不要在这里手动 include HAL 主头** —— 会与 HAL 自身对 conf 的 include 互锁。
 * HAL 头由 bin 工程的 main.c / bindgen 的 wrapper.h 在合适时机 include。 */

/* **不要在这里手动 include HAL 主头** —— 会与 HAL 自身对 conf 的 include 互锁。
 * HAL 头由 bin 工程的 main.c / bindgen 的 wrapper.h 在合适时机 include。 */

/* `py32e4xx_hal.h` 引用了 GPIO_TypeDef / uint32_t / DMA_HandleTypeDef 等设备/HAL 类型，但**自己不**
 * include 它们 —— 它假定 conf.h 已经拉过。所以这里按 enable 列表 include 对应的模块头，
 * 让模块头把 `py32e407_hal_def.h`（进而把 `py32e4xx.h`）拉进来。
 *
 * **每个模块单独 include**：HAL 各 `.c` 文件只 include `py32e4xx_hal.h`，不显式 include
 * 自己模块的 HAL 头 —— 与官方设计一致。官方示例里也是这样布局。
 */
#ifdef HAL_CORTEX_MODULE_ENABLED
#include "py32e407_hal_cortex.h"
#endif
#ifdef HAL_FLASH_MODULE_ENABLED
#include "py32e407_hal_flash.h"
#endif
#ifdef HAL_PWR_MODULE_ENABLED
#include "py32e407_hal_pwr.h"
#endif
#ifdef HAL_RCC_MODULE_ENABLED
#include "py32e407_hal_rcc.h"
#endif
#ifdef HAL_GPIO_MODULE_ENABLED
#include "py32e407_hal_gpio.h"
#endif
#ifdef HAL_DMA_MODULE_ENABLED
#include "py32e407_hal_dma.h"
#endif
#ifdef HAL_UART_MODULE_ENABLED
#include "py32e407_hal_uart.h"
#endif
/* 其它模块默认关；要开取消对应 #if 块并加 enable。 */

#ifdef __cplusplus
}
#endif

#endif /* __PY32E407_HAL_CONF_H */
