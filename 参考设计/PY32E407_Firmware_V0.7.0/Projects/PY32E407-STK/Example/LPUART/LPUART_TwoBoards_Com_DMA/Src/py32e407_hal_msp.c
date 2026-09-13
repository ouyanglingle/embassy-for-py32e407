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
* @brief LPUART MSP Initialization
* This function configures the hardware resources used in this example
* @param hlpuart: LPUART handle pointer
* @retval None
*/
void HAL_LPUART_MspInit(LPUART_HandleTypeDef* hlpuart)
{
  GPIO_InitTypeDef GPIO_InitStruct = {0};
  if(hlpuart->Instance == LPUART)
  {
    /* Peripheral clock enable */
    __HAL_RCC_LPUART1_CLK_ENABLE();
    /* DMA controller clock enable */
    __HAL_RCC_DMA1_CLK_ENABLE();
    __HAL_RCC_GPIOC_CLK_ENABLE();
    
    /**LPUART1 GPIO Configuration
    PC0     ------> LPUART_RX
    PC1     ------> LPUART_TX
    */
    GPIO_InitStruct.Pin = GPIO_PIN_0|GPIO_PIN_1;
    GPIO_InitStruct.Mode = GPIO_MODE_AF_PP;
    GPIO_InitStruct.Pull = GPIO_PULLUP;
    GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_HIGH;
    GPIO_InitStruct.Alternate = GPIO_AF8_LPUART1;
    HAL_GPIO_Init(GPIOC, &GPIO_InitStruct);

    /* USART1 DMA Init */
    /* USART1_RX Init */
    hdma_lpuart_rx.Instance              = DMA1_Channel2;
    hdma_lpuart_rx.Init.Direction        = DMA_PERIPH_TO_MEMORY;
    hdma_lpuart_rx.Init.DstInc           = DMA_DST_ADDR_INC;
    hdma_lpuart_rx.Init.SrcInc           = DMA_SRC_ADDR_FIX;
    hdma_lpuart_rx.Init.DstWidth         = DMA_DST_WIDTH_BYTE;
    hdma_lpuart_rx.Init.SrcWidth         = DMA_SRC_WIDTH_BYTE;
    hdma_lpuart_rx.Init.SrcBurstLen      = DMA_SRC_BURST_LEN_1;
    hdma_lpuart_rx.Init.SrcHandshakeType = DMA_SRC_HS_TYPE_HARD;
    hdma_lpuart_rx.Init.BlockMaxLen      = DMA_BLOCK_DEFAULT_MAX_LEN;

    hdma_lpuart_rx.Init.Mode             = DMA_NORMAL;
    hdma_lpuart_rx.Init.Priority         = DMA_PRIORITY_1;
    
    if (HAL_DMA_Init(&hdma_lpuart_rx) != HAL_OK)
    {
      APP_ErrorHandler();
    }
    HAL_DMA_ChannelReq(&hdma_lpuart_rx, DMA_REQ_TYPE_SRC, DMA_REQ_NUM_1, DMA_REQ_FUNC_LPUART1_RX);

    __HAL_LINKDMA(hlpuart, hdmarx, hdma_lpuart_rx);

    /* USART1_TX Init */
    hdma_lpuart_tx.Instance              = DMA1_Channel1;
    hdma_lpuart_tx.Init.Direction        = DMA_MEMORY_TO_PERIPH;
    hdma_lpuart_tx.Init.DstInc           = DMA_DST_ADDR_FIX;
    hdma_lpuart_tx.Init.SrcInc           = DMA_SRC_ADDR_INC;
    hdma_lpuart_tx.Init.DstWidth         = DMA_DST_WIDTH_BYTE;
    hdma_lpuart_tx.Init.SrcWidth         = DMA_SRC_WIDTH_BYTE;
    hdma_lpuart_tx.Init.DstBurstLen      = DMA_DST_BURST_LEN_1;
    hdma_lpuart_tx.Init.DstHandshakeType = DMA_DST_HS_TYPE_HARD;
    hdma_lpuart_tx.Init.BlockMaxLen      = DMA_BLOCK_DEFAULT_MAX_LEN;

    hdma_lpuart_tx.Init.Mode             = DMA_NORMAL;
    hdma_lpuart_tx.Init.Priority         = DMA_PRIORITY_0;

    if (HAL_DMA_Init(&hdma_lpuart_tx) != HAL_OK)
    {
      APP_ErrorHandler();
    }
    HAL_DMA_ChannelReq(&hdma_lpuart_tx, DMA_REQ_TYPE_DST, DMA_REQ_NUM_0, DMA_REQ_FUNC_LPUART1_TX);

    __HAL_LINKDMA(hlpuart,hdmatx,hdma_lpuart_tx);

    /* NVIC enable */
    HAL_NVIC_SetPriority(DMA1_Channel1_IRQn, 2, 0);
    HAL_NVIC_EnableIRQ(DMA1_Channel1_IRQn);
    HAL_NVIC_SetPriority(DMA1_Channel2_IRQn, 2, 0);
    HAL_NVIC_EnableIRQ(DMA1_Channel2_IRQn);
    /* LPUART1 interrupt Init */
    HAL_NVIC_SetPriority(LPUART_IRQn, 1, 0);
    HAL_NVIC_EnableIRQ(LPUART_IRQn);
  }
}

/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
