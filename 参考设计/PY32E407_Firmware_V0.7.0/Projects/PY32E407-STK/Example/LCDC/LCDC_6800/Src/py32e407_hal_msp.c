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
  * @brief Initialize LCDC MSP
  */
void HAL_LCDC_MspInit(LCDC_HandleTypeDef *hlcdc)
{
  GPIO_InitTypeDef  GPIOConfig = {0};
  
  __HAL_RCC_LCDC_CLK_ENABLE();
  __HAL_RCC_GPIOD_CLK_ENABLE();
  __HAL_RCC_GPIOE_CLK_ENABLE();
  __HAL_RCC_GPIOF_CLK_ENABLE();
  
  /* Initlize LCDC RS pin*/
  GPIOConfig.Pin        = GPIO_PIN_4;
  GPIOConfig.Mode       = GPIO_MODE_AF_PP;
  GPIOConfig.Pull       = GPIO_NOPULL;
  GPIOConfig.Speed      = GPIO_SPEED_FREQ_VERY_HIGH;
  GPIOConfig.Alternate  = GPIO_AF12_LCDC;
  HAL_GPIO_Init(GPIOF, &GPIOConfig); 
  
  /* Initlize LCDC D2/D3/RW/EN/CS/D0/D1 pin */
  GPIOConfig.Pin        = GPIO_PIN_0 | GPIO_PIN_1 | GPIO_PIN_4 | GPIO_PIN_5 |\
                          GPIO_PIN_7 | GPIO_PIN_14 | GPIO_PIN_15;
  GPIOConfig.Mode       = GPIO_MODE_AF_PP;
  GPIOConfig.Pull       = GPIO_NOPULL;
  GPIOConfig.Speed      = GPIO_SPEED_FREQ_VERY_HIGH;
  GPIOConfig.Alternate  = GPIO_AF12_LCDC;
  HAL_GPIO_Init(GPIOD, &GPIOConfig);
  
  /* Initlize LCDC D4/D5/D6/D7 pin */
  GPIOConfig.Pin        = GPIO_PIN_7 | GPIO_PIN_8 | GPIO_PIN_9 | GPIO_PIN_10 ;
  GPIOConfig.Mode       = GPIO_MODE_AF_PP;
  GPIOConfig.Pull       = GPIO_NOPULL;
  GPIOConfig.Speed      = GPIO_SPEED_FREQ_VERY_HIGH;
  GPIOConfig.Alternate  = GPIO_AF12_LCDC;
  HAL_GPIO_Init(GPIOE, &GPIOConfig);
}

/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
