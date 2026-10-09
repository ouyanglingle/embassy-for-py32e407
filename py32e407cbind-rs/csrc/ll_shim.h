/**
  ******************************************************************************
  * @file    ll_shim.h
  * @brief   把 LL 头文件里 **__STATIC_INLINE** 的函数"外部化"，供 Rust FFI 调用。
  ******************************************************************************
  *
  * ## 为什么需要这个文件
  *
  * `py32e407_ll_*.h` 里约 2000 个函数写成 `__STATIC_INLINE`（展开为 `static inline`）。
  * `static` 意为**内部链接**：它们会被内联进每个用到它们的 `.c`，最多留下**局部符号**
  * （`arm-none-eabi-nm` 里是小写 `t`），**永远不会产生全局符号**。因此：
  *
  *   - bindgen 不会为它们生成绑定（实测 bindings.rs 里 `LL_GPIO_TogglePin` 零匹配）；
  *   - 即使手工声明成 `extern "C"`，链接期也必然 `undefined symbol`。
  *
  * 本文件为需要的那批函数**各生成一个外部可见的包装**，命名规则：
  *
  *       <官方名>__ext        例如  LL_GPIO_TogglePin__ext
  *
  * 后缀 `__ext`（externalized）与 bindgen 自己 `wrap_static_fns` 的默认后缀 `__extern`
  * 同源，见到即知"这是被外部化的官方函数"。Rust 侧 `src/lib.rs` 再用 `pub use`
  * 把它们别名回官方名字，调用方仍然写 `LL_GPIO_TogglePin(...)`。
  *
  * ## 为什么用 X 宏表而不是手写两遍
  *
  * 原型（给 bindgen 看）与定义（给链接器看）必须**逐字一致**。本文件用同一张
  * `SHIM_*` 表，靠 `PY32_LL_SHIM_IMPL` 开关决定展开成原型还是定义 —— 改一处签名
  * 不会出现"声明和定义漂移"这种最难查的 bug：
  *
  *   - `ll_shim.c` 先 `#define PY32_LL_SHIM_IMPL` 再 include 本文件 → 展开出**函数定义**；
  *   - bindgen 的 wrapper.h 直接 include 本文件 → 展开出**函数原型**。
  *
 * ## 覆盖范围（2026-09-21 定的"精选"范围）
 *
 * 目前三组，共 52 个：
 *
 *   - **GPIO（23 个）**：引脚级配置与读写 —— 也就是原先不得不用 PAC 点寄存器的唯一原因；
 *   - **bus（25 个）**：5 组（AHB1/AHB2/APB1_GRP1/APB1_GRP2/APB2_GRP1）× 5 个操作
 *     （Enable/IsEnabled/Disable/ForceReset/ReleaseReset）—— 使能外设时钟的前提；
 *   - **PWR（4 个）**：`VOS` / `SRAM_RETV` 的读写 —— 官方 `SystemInit()` 靠裸寄存器写
 *     完成这两步复位，LL 里有对应的 `LL_PWR_SetRegulVoltageScaling` /
 *     `LL_PWR_SetSramVoltageInStandby`，但同样是 `static inline`。
 *     故这里用**官方 LL 函数**外部化，而不是自造 API
 *     （`LL_PWR_REGU_VOLTAGE_HIGH` 与 `LL_PWR_SRAM2_VOLTAGE_HIGH` 都是 0，
 *      等价于官方那句 `CLEAR_BIT(PWR->CR1, PWR_CR1_VOS/SRAM_RETV)`）。
 *
 * **时钟树本身不在其中**，因为 `LL_PLL_ConfigSystemClock_HSE/_HSI` 已在 `.c` 里定义、
 * 本来就是可链接的，一条调用等于官方整套时钟配置。
 *
 * 以后要用别的外设的 inline 函数（USART 收发、TIM 启停、RCC 选时钟源…），
 * 在这里的表里加一行即可；若覆盖面继续膨胀，再考虑改用 bindgen 的
 * `wrap_static_fns` 全量自动包装。
 */
#ifndef PY32_LL_SHIM_H
#define PY32_LL_SHIM_H

#include "py32e407_ll_gpio.h"
#include "py32e407_ll_bus.h"
#include "py32e407_ll_pwr.h"

#ifdef PY32_LL_SHIM_IMPL
/* ── 定义模式（ll_shim.c 用）：生成外部可见的函数体 ── */
#define SHIM_VOID(name, params, args) void name##__ext params { name args; }
#define SHIM_U32(name, params, args) uint32_t name##__ext params { return name args; }
#else
/* ── 原型模式（bindgen 用）：只生成声明 ── */
#define SHIM_VOID(name, params, args) void name##__ext params;
#define SHIM_U32(name, params, args) uint32_t name##__ext params;
#endif

/* ==========================================================================
 * GPIO —— 引脚级配置与读写（共 23 个，覆盖 py32e407_ll_gpio.h 的全部 inline）
 *
 * 这一组是原先必须用 PAC 点寄存器的原因：
 *   - LL_GPIO_SetOutputPin / ResetOutputPin / TogglePin 操作 BSRR/BRR；
 *   - LL_GPIO_SetPinMode / Speed / OutputType / Pull / SetAFPin_* 是 LL_GPIO_Init
 *     内部调用的那些局部函数（gpio.c 里是小写 t），单独调用同样需要外部化。
 * ========================================================================== */
SHIM_VOID(LL_GPIO_SetPinMode,        (GPIO_TypeDef *GPIOx, uint32_t Pin, uint32_t Mode),           (GPIOx, Pin, Mode))
SHIM_U32 (LL_GPIO_GetPinMode,        (GPIO_TypeDef *GPIOx, uint32_t Pin),                           (GPIOx, Pin))
SHIM_VOID(LL_GPIO_SetPinOutputType,  (GPIO_TypeDef *GPIOx, uint32_t PinMask, uint32_t OutputType),  (GPIOx, PinMask, OutputType))
SHIM_U32 (LL_GPIO_GetPinOutputType,  (GPIO_TypeDef *GPIOx, uint32_t Pin),                           (GPIOx, Pin))
SHIM_VOID(LL_GPIO_SetPinSpeed,       (GPIO_TypeDef *GPIOx, uint32_t Pin, uint32_t Speed),           (GPIOx, Pin, Speed))
SHIM_U32 (LL_GPIO_GetPinSpeed,       (GPIO_TypeDef *GPIOx, uint32_t Pin),                           (GPIOx, Pin))
SHIM_VOID(LL_GPIO_SetPinPull,        (GPIO_TypeDef *GPIOx, uint32_t Pin, uint32_t Pull),            (GPIOx, Pin, Pull))
SHIM_U32 (LL_GPIO_GetPinPull,        (GPIO_TypeDef *GPIOx, uint32_t Pin),                           (GPIOx, Pin))
SHIM_VOID(LL_GPIO_SetAFPin_0_7,      (GPIO_TypeDef *GPIOx, uint32_t Pin, uint32_t Alternate),       (GPIOx, Pin, Alternate))
SHIM_U32 (LL_GPIO_GetAFPin_0_7,      (GPIO_TypeDef *GPIOx, uint32_t Pin),                           (GPIOx, Pin))
SHIM_VOID(LL_GPIO_SetAFPin_8_15,     (GPIO_TypeDef *GPIOx, uint32_t Pin, uint32_t Alternate),       (GPIOx, Pin, Alternate))
SHIM_U32 (LL_GPIO_GetAFPin_8_15,     (GPIO_TypeDef *GPIOx, uint32_t Pin),                           (GPIOx, Pin))
SHIM_VOID(LL_GPIO_LockPin,           (GPIO_TypeDef *GPIOx, uint32_t PinMask),                       (GPIOx, PinMask))
SHIM_U32 (LL_GPIO_IsPinLocked,       (GPIO_TypeDef *GPIOx, uint32_t PinMask),                       (GPIOx, PinMask))
SHIM_U32 (LL_GPIO_IsAnyPinLocked,    (GPIO_TypeDef *GPIOx),                                         (GPIOx))
SHIM_U32 (LL_GPIO_ReadInputPort,     (GPIO_TypeDef *GPIOx),                                         (GPIOx))
SHIM_U32 (LL_GPIO_IsInputPinSet,     (GPIO_TypeDef *GPIOx, uint32_t PinMask),                       (GPIOx, PinMask))
SHIM_VOID(LL_GPIO_WriteOutputPort,   (GPIO_TypeDef *GPIOx, uint32_t PortValue),                     (GPIOx, PortValue))
SHIM_U32 (LL_GPIO_ReadOutputPort,    (GPIO_TypeDef *GPIOx),                                         (GPIOx))
SHIM_U32 (LL_GPIO_IsOutputPinSet,    (GPIO_TypeDef *GPIOx, uint32_t PinMask),                       (GPIOx, PinMask))
SHIM_VOID(LL_GPIO_SetOutputPin,      (GPIO_TypeDef *GPIOx, uint32_t PinMask),                       (GPIOx, PinMask))
SHIM_VOID(LL_GPIO_ResetOutputPin,    (GPIO_TypeDef *GPIOx, uint32_t PinMask),                       (GPIOx, PinMask))
SHIM_VOID(LL_GPIO_TogglePin,         (GPIO_TypeDef *GPIOx, uint32_t PinMask),                       (GPIOx, PinMask))

/* ==========================================================================
 * 总线时钟门控 / 复位（共 25 个 = 5 组 × 5 个操作）
 *
 * 这是"打开外设时钟"的唯一途径 —— `LL_<BUS>_GRP<n>_EnableClock` 等全在
 * py32e407_ll_bus.h 里是 static inline。注意 PY32 各组的**位序与 STM32 不同**
 * （例如 AHB2 的 GPIO 从 bit2 起：IOPAEN=bit2 … IOPFEN=bit7），
 * 具体位号用 `LL_<BUS>_GRP<n>_PERIPH_<X>` 常量传，别手写掩码。
 * ========================================================================== */
SHIM_VOID(LL_AHB1_GRP1_EnableClock,     (uint32_t Periphs), (Periphs))
SHIM_U32 (LL_AHB1_GRP1_IsEnabledClock,  (uint32_t Periphs), (Periphs))
SHIM_VOID(LL_AHB1_GRP1_DisableClock,    (uint32_t Periphs), (Periphs))
SHIM_VOID(LL_AHB1_GRP1_ForceReset,      (uint32_t Periphs), (Periphs))
SHIM_VOID(LL_AHB1_GRP1_ReleaseReset,    (uint32_t Periphs), (Periphs))

SHIM_VOID(LL_AHB2_GRP1_EnableClock,     (uint32_t Periphs), (Periphs))
SHIM_U32 (LL_AHB2_GRP1_IsEnabledClock,  (uint32_t Periphs), (Periphs))
SHIM_VOID(LL_AHB2_GRP1_DisableClock,    (uint32_t Periphs), (Periphs))
SHIM_VOID(LL_AHB2_GRP1_ForceReset,      (uint32_t Periphs), (Periphs))
SHIM_VOID(LL_AHB2_GRP1_ReleaseReset,    (uint32_t Periphs), (Periphs))

SHIM_VOID(LL_APB1_GRP1_EnableClock,     (uint32_t Periphs), (Periphs))
SHIM_U32 (LL_APB1_GRP1_IsEnabledClock,  (uint32_t Periphs), (Periphs))
SHIM_VOID(LL_APB1_GRP1_DisableClock,    (uint32_t Periphs), (Periphs))
SHIM_VOID(LL_APB1_GRP1_ForceReset,      (uint32_t Periphs), (Periphs))
SHIM_VOID(LL_APB1_GRP1_ReleaseReset,    (uint32_t Periphs), (Periphs))

SHIM_VOID(LL_APB1_GRP2_EnableClock,     (uint32_t Periphs), (Periphs))
SHIM_U32 (LL_APB1_GRP2_IsEnabledClock,  (uint32_t Periphs), (Periphs))
SHIM_VOID(LL_APB1_GRP2_DisableClock,    (uint32_t Periphs), (Periphs))
SHIM_VOID(LL_APB1_GRP2_ForceReset,      (uint32_t Periphs), (Periphs))
SHIM_VOID(LL_APB1_GRP2_ReleaseReset,    (uint32_t Periphs), (Periphs))

SHIM_VOID(LL_APB2_GRP1_EnableClock,     (uint32_t Periphs), (Periphs))
SHIM_U32 (LL_APB2_GRP1_IsEnabledClock,  (uint32_t Periphs), (Periphs))
SHIM_VOID(LL_APB2_GRP1_DisableClock,    (uint32_t Periphs), (Periphs))
SHIM_VOID(LL_APB2_GRP1_ForceReset,      (uint32_t Periphs), (Periphs))
SHIM_VOID(LL_APB2_GRP1_ReleaseReset,    (uint32_t Periphs), (Periphs))

/* ==========================================================================
 * PWR —— 电压档（VOS）与 SRAM 保持电压（SRAM_RETV）（共 4 个）
 *
 * 官方 `SystemInit()` 里这两步是**裸寄存器写**：
 *     SET_BIT(RCC->APB1ENR1, PWREN);
 *     CLEAR_BIT(PWR->CR1, PWR_CR1_VOS);
 *     CLEAR_BIT(PWR->CR1, PWR_CR1_SRAM_RETV);
 *     CLEAR_BIT(RCC->APB1ENR1, PWREN);
 *
 * LL 里其实有对应函数，只是同样写成 static inline，所以在这里外部化：
 *   - 清 `VOS`        ≡ `LL_PWR_SetRegulVoltageScaling(LL_PWR_REGU_VOLTAGE_HIGH)`
 *                        （`LL_PWR_REGU_VOLTAGE_HIGH` == SCALE1 == 0x0）
 *   - 清 `SRAM_RETV`  ≡ `LL_PWR_SetSramVoltageInStandby(LL_PWR_SRAM2_VOLTAGE_HIGH)`
 *                        （`LL_PWR_SRAM2_VOLTAGE_HIGH` == 0x0）
 *
 * ⚠️ `PWR_CR1` 属**备份域，系统复位不会清零**：复位后可能停在低压档，此时 HCLK 上不去、
 *    高频下 PLL 根本不锁定。所以必须在切 PLL 之前做。
 * ⚠️ PWR 挂在 **APB1**（不是 STM32F4 的 APB1ENR）：开时钟用
 *    `LL_APB1_GRP1_EnableClock(LL_APB1_GRP1_PERIPH_PWR)`（见上面的 bus 组）。
 * ========================================================================== */
SHIM_VOID(LL_PWR_SetRegulVoltageScaling,    (uint32_t VoltageLevel), (VoltageLevel))
SHIM_U32 (LL_PWR_GetRegulVoltageScaling,    (void),                  ())
SHIM_VOID(LL_PWR_SetSramVoltageInStandby,   (uint32_t VoltageLevel), (VoltageLevel))
SHIM_U32 (LL_PWR_GetSramVoltageInStandby,   (void),                  ())

#undef SHIM_VOID
#undef SHIM_U32

#endif /* PY32_LL_SHIM_H */
