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
DMA_HandleTypeDef  hdma_tim;

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
  * @brief Initialize TIM1 related MSP
  */
void HAL_TIM_Base_MspInit(TIM_HandleTypeDef *htim)
{
  GPIO_InitTypeDef   GPIO_InitStruct = {0};
  
  /* Enable DMA clock */
  __HAL_RCC_DMA1_CLK_ENABLE();
  /* Enable SYSCFG clock */
  __HAL_RCC_SYSCFG_CLK_ENABLE();
  /* Enable TIM1 clock */
  __HAL_RCC_TIM1_CLK_ENABLE();
  /* Enable GPIOA clock */
  __HAL_RCC_GPIOA_CLK_ENABLE();
  
  hdma_tim.Instance = DMA1_Channel2;                        /* Select DMA channel 2 */
  hdma_tim.Init.Direction = DMA_MEMORY_TO_PERIPH;           /* Memory to peripheral direction */
  hdma_tim.Init.SrcWidth = DMA_SRC_WIDTH_WORD;              /* Memory increment mode Enable */
  hdma_tim.Init.DstWidth = DMA_DST_WIDTH_WORD;              /* Peripheral data alignment : Word */
  hdma_tim.Init.SrcBurstLen = DMA_SRC_BURST_LEN_4;
  hdma_tim.Init.DstBurstLen = DMA_DST_BURST_LEN_4;
  hdma_tim.Init.SrcInc = DMA_SRC_ADDR_INC;                  /* Memory increment mode Enable */
  hdma_tim.Init.DstInc = DMA_DST_ADDR_FIX;                  /* Peripheral increment mode Disable */
  hdma_tim.Init.Mode = DMA_NORMAL;                          /* Normal Mode */
  hdma_tim.Init.SrcHandshakeType = DMA_SRC_HS_TYPE_HARD;    /* Src shake hand Type: DMA_SRC_HS_TYPE_HARD*/
  hdma_tim.Init.DstHandshakeType = DMA_DST_HS_TYPE_HARD;    /* DST shake hand Type: DMA_DST_HS_TYPE_HARD*/
  hdma_tim.Init.Priority = DMA_PRIORITY_1;                  /* Priority level : DMA_PRIORITY_1 */
  hdma_tim.Init.BlockMaxLen = 2;

  /* Set DMA channel map. */
  HAL_DMA_ChannelReq(&hdma_tim, DMA_REQ_TYPE_DST, DMA_REQ_NUM_1, DMA_REQ_FUNC_TIM1_UP);

  /* DMA1 associated TIM_UP event */
  __HAL_LINKDMA(htim, hdma[TIM_DMA_ID_UPDATE], hdma_tim);
  /* Initialize DMA */
  HAL_DMA_Init(htim->hdma[TIM_DMA_ID_UPDATE]);
  
  GPIO_InitStruct.Mode = GPIO_MODE_AF_PP;
  GPIO_InitStruct.Pull = GPIO_PULLUP;
  GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_HIGH;
  GPIO_InitStruct.Alternate = GPIO_AF6_TIM1;
  /* A8-TIM1_CH1 */
  GPIO_InitStruct.Pin = GPIO_PIN_8;
  HAL_GPIO_Init(GPIOA, &GPIO_InitStruct);
  /* A9-TIM1_CH2 */
  GPIO_InitStruct.Pin = GPIO_PIN_9;
  HAL_GPIO_Init(GPIOA, &GPIO_InitStruct);
  
  /* NVIC enable */
  HAL_NVIC_SetPriority(DMA1_Channel2_IRQn, 2, 0);
  HAL_NVIC_EnableIRQ(DMA1_Channel2_IRQn);

  /* NVIC enable */
  HAL_NVIC_SetPriority(TIM1_UP_TIM10_IRQn, 2, 0);
  HAL_NVIC_EnableIRQ(TIM1_UP_TIM10_IRQn);
}

/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
