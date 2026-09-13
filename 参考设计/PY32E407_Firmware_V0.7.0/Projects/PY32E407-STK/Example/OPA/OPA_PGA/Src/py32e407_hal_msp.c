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
  * @brief Initialize OPA MSP
  */
void HAL_OPA_MspInit(OPA_HandleTypeDef *hopa)
{
  GPIO_InitTypeDef  GPIO_InitStruct = {0};
  
  __HAL_RCC_OPA_CLK_ENABLE();                    /* Enable the OPA clock*/
  __HAL_RCC_GPIOA_CLK_ENABLE();                  /* Enable the GPIO clock*/
  
  GPIO_InitStruct.Mode  = GPIO_MODE_ANALOG;      /* GPIO mode is Analog */
  GPIO_InitStruct.Pull  = GPIO_NOPULL;           /* NO PULL */
  GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_HIGH;  /* The speed is high */
  GPIO_InitStruct.Pin = GPIO_PIN_7 | GPIO_PIN_2; /* PA7/PA2 Analog */
  HAL_GPIO_Init(GPIOA, &GPIO_InitStruct);   
}

/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
