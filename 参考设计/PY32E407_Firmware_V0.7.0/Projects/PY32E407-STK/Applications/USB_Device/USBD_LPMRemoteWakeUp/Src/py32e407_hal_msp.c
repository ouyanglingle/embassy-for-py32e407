/**
  ******************************************************************************
  * @file    py32e407_hal_msp.c
  * @author  MCU Application Team
  * @brief   This file provides code for the MSP Initialization
  *          and de-Initialization codes.
  ******************************************************************************
  * @attention
  *
  * <h2><center>&copy; Copyright (c) 2023 Puya Semiconductor Co.
  * All rights reserved.</center></h2>
  *
  * This software component is licensed by Puya under BSD 3-Clause license,
  * the "License"; You may not use this file except in compliance with the
  * License. You may obtain a copy of the License at:
  *                        opensource.org/licenses/BSD-3-Clause
  *
  ******************************************************************************
  * @attention
  *
  * <h2><center>&copy; Copyright (c) 2016 STMicroelectronics.
  * All rights reserved.</center></h2>
  *
  * This software component is licensed by ST under BSD 3-Clause license,
  * the "License"; You may not use this file except in compliance with the
  * License. You may obtain a copy of the License at:
  *                        opensource.org/licenses/BSD-3-Clause
  *
  ******************************************************************************
  */

/* Includes ------------------------------------------------------------------*/
#include "main.h"

/* Private typedef -----------------------------------------------------------*/
/* Private define ------------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private variables ---------------------------------------------------------*/
EXTI_HandleTypeDef        ExtiHandle;

/* Private function prototypes -----------------------------------------------*/
/* External functions --------------------------------------------------------*/

/**
  * @brief Initialize global MSP
  */
void HAL_MspInit(void)
{
  __HAL_RCC_SYSCFG_CLK_ENABLE();
  __HAL_RCC_PWR_CLK_ENABLE();
}

/**
  * @brief  Initializes the USBD MSP.
  * @param  husbd: USBDEV handle
  * @retval None
  */
void HAL_USBDEV_MspInit(USBDEV_HandleTypeDef * husbd)
{
  RCC_PeriphCLKInitTypeDef rccperiphclkinit = {0};
  EXTI_ConfigTypeDef        ExtiCfg = {0};
  
  /* USB clock selection */
  rccperiphclkinit.PeriphClockSelection = RCC_PERIPHCLK_USB;
  rccperiphclkinit.UsbClockSelection = RCC_USBCLKSOURCE_HSI48M;
  HAL_RCCEx_PeriphCLKConfig(&rccperiphclkinit);
   
  /* Enable USB OTG FS Clock */
  __HAL_RCC_USB1_CLK_ENABLE();

  /* Set USBFS Interrupt priority */
  HAL_NVIC_SetPriority(OTG1_FS_IRQn, 6, 0);

  /* Enable USBFS Interrupt */
  HAL_NVIC_EnableIRQ(OTG1_FS_IRQn);
  
  /* Configure Interrupt Wakeup */
  ExtiCfg.Line = EXTI_LINE_33;
  ExtiCfg.Mode = EXTI_MODE_INTERRUPT;
  HAL_EXTI_SetConfigLine(&ExtiHandle, &ExtiCfg);
  
  /* Set USBFS wakeup Interrupt priority */
  HAL_NVIC_SetPriority(OTG1_FS_WKUP_IRQn, 5, 0);

  /* Enable USBFS wakeup Interrupt */
  HAL_NVIC_EnableIRQ(OTG1_FS_WKUP_IRQn);
  
}

/**
  * @brief  De-Initializes the USBDEV MSP.
  * @param  hpcd: USBDEV handle
  * @retval None
  */
void HAL_USBDEV_MspDeInit(USBDEV_HandleTypeDef * hpcd)
{
  /* Disable USB OTG FS Clock */
}


/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
