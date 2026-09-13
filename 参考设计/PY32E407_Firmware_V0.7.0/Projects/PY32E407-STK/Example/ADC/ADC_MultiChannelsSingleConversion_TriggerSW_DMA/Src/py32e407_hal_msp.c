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
DMA_HandleTypeDef hDMAChannel;

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
  * @brief Initialize ADC MSP.
  */
void HAL_ADC_MspInit(ADC_HandleTypeDef *hadc)
{
  RCC_PeriphCLKInitTypeDef RCC_PeriphCLKInit = {0};
  GPIO_InitTypeDef         GPIO_InitStruct   = {0};
  
  __HAL_RCC_ADC1_CLK_ENABLE();
  __HAL_RCC_GPIOC_CLK_ENABLE();
  __HAL_RCC_SYSCFG_CLK_ENABLE();
  
  RCC_PeriphCLKInit.PeriphClockSelection= RCC_PERIPHCLK_ADC;
  RCC_PeriphCLKInit.AdcClockSelection   = RCC_ADCPCLK2_DIV8;
  HAL_RCCEx_PeriphCLKConfig(&RCC_PeriphCLKInit);
  
  GPIO_InitStruct.Pin = GPIO_PIN_0 | GPIO_PIN_1 | GPIO_PIN_2 | GPIO_PIN_3;
  GPIO_InitStruct.Mode = GPIO_MODE_ANALOG;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  HAL_GPIO_Init(GPIOC, &GPIO_InitStruct);
  HAL_SYSCFG_EnableGPIOAnalogMode(GPIOC, GPIO_PIN_0);
  HAL_SYSCFG_EnableGPIOAnalogMode(GPIOC, GPIO_PIN_1);
  HAL_SYSCFG_EnableGPIOAnalogMode(GPIOC, GPIO_PIN_2);
  HAL_SYSCFG_EnableGPIOAnalogMode(GPIOC, GPIO_PIN_3);
  
  /* Enable clock */
  __HAL_RCC_DMA1_CLK_ENABLE();
  
  /* Configure the DMA */
  hDMAChannel.Instance = DMA1_Channel1;
  hDMAChannel.Init.Direction = DMA_PERIPH_TO_MEMORY;
  hDMAChannel.Init.SrcWidth = DMA_SRC_WIDTH_HALFWORD;
  hDMAChannel.Init.DstWidth = DMA_DST_WIDTH_HALFWORD;
  hDMAChannel.Init.SrcBurstLen = DMA_SRC_BURST_LEN_1;
  /* hDMAChannel.Init.DstBurstLen = DMA_DST_BURST_LEN_1; */
  hDMAChannel.Init.SrcInc = DMA_SRC_ADDR_FIX;
  hDMAChannel.Init.DstInc = DMA_DST_ADDR_INC;
  hDMAChannel.Init.Mode = DMA_NORMAL;
  hDMAChannel.Init.SrcReload = DMA_SRC_RELOAD_DISABLE;
  hDMAChannel.Init.DstReload = DMA_DST_RELOAD_DISABLE;
  hDMAChannel.Init.SrcHandshakeType = DMA_SRC_HS_TYPE_HARD;
  /* hDMAChannel.Init.DstHandshakeType = DMA_DST_HS_TYPE_HARD; */
  hDMAChannel.Init.Priority = DMA_PRIORITY_1;
  hDMAChannel.Init.BlockMaxLen = DMA_BLOCK_DEFAULT_MAX_LEN;
  HAL_DMA_Init(&hDMAChannel);
  
  /* Set DMA channel map */
  HAL_DMA_ChannelReq(&hDMAChannel, DMA_REQ_TYPE_SRC, DMA_REQ_NUM_2, DMA_REQ_FUNC_ADC1);
  
  /* NVIC enable */
  HAL_NVIC_SetPriority(DMA1_Channel1_IRQn, 2, 0);
  HAL_NVIC_EnableIRQ(DMA1_Channel1_IRQn);

  /* DMA handles are associated to SD handles */
  __HAL_LINKDMA(hadc, DMA_Handle, hDMAChannel);
}

/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
