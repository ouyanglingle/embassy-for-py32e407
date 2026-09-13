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
static DMA_HandleTypeDef HdmaCh1;
static DMA_HandleTypeDef HdmaCh2;

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
  * @brief Initialize I2S related MSP
  */
void HAL_I2S_MspInit(I2S_HandleTypeDef *hi2s)
{
  GPIO_InitTypeDef  GPIO_InitStruct = {0};
  /* Initialize SPI1 */
  if (hi2s->Instance == SPI1)
  {
    __HAL_RCC_GPIOA_CLK_ENABLE();                   /* Enable GPIOA clock */
    __HAL_RCC_SYSCFG_CLK_ENABLE();                  /* Enable SYSCFG clock */
    __HAL_RCC_SPI1_CLK_ENABLE();                    /* Enable SPI1 clock */
    __HAL_RCC_DMA1_CLK_ENABLE();                    /* Enable DMA clock */
    
    /*
      PA5 ------> I2S1_CK
      PA7 ------> I2S1_SD
      PA4 ------> I2S1_WS
    */
    /*WS/CK/SD*/
    GPIO_InitStruct.Pin       = (GPIO_PIN_4 | GPIO_PIN_5 | GPIO_PIN_7);
    GPIO_InitStruct.Pull      = GPIO_NOPULL;
    GPIO_InitStruct.Mode      = GPIO_MODE_AF_PP;
    GPIO_InitStruct.Speed     = GPIO_SPEED_FREQ_HIGH;
    GPIO_InitStruct.Alternate = GPIO_AF5_I2S1;
    HAL_GPIO_Init(GPIOA, &GPIO_InitStruct);

    /* Interrupt configuration */
    HAL_NVIC_SetPriority(SPI1_IRQn, 1, 0);
    HAL_NVIC_EnableIRQ(SPI1_IRQn);
    
    /* I2S DMA Init */
    /* DMA_CH1 configuration */
    HdmaCh1.Instance                 = DMA1_Channel1;
    HdmaCh1.Init.Direction           = DMA_MEMORY_TO_PERIPH;
    HdmaCh1.Init.DstInc              = DMA_DST_ADDR_FIX;
    HdmaCh1.Init.SrcInc              = DMA_SRC_ADDR_INC;
    HdmaCh1.Init.DstWidth            = DMA_DST_WIDTH_HALFWORD;
    HdmaCh1.Init.SrcWidth            = DMA_SRC_WIDTH_HALFWORD;
    HdmaCh1.Init.DstBurstLen         = DMA_DST_BURST_LEN_1;
    HdmaCh1.Init.DstHandshakeType    = DMA_DST_HS_TYPE_HARD;
    HdmaCh1.Init.BlockMaxLen         = DMA_BLOCK_DEFAULT_MAX_LEN;

    HdmaCh1.Init.Mode                = DMA_NORMAL;
    HdmaCh1.Init.Priority            = DMA_PRIORITY_0;
    /* Initialize DMA */
    HAL_DMA_Init(&HdmaCh1);
    
    /* Set DMA channel map. */
    HAL_DMA_ChannelReq(&HdmaCh1, DMA_REQ_TYPE_DST, DMA_REQ_NUM_0, DMA_REQ_FUNC_SPI1_TX); /* SPI1_TX DMA1_CH1 */
    
    /* DMA handle is associated with SPI handle */
    __HAL_LINKDMA(hi2s, hdmatx, HdmaCh1);

    /* DMA_CH2 configuration */
    HdmaCh2.Instance                 = DMA1_Channel2;
    HdmaCh2.Init.Direction           = DMA_PERIPH_TO_MEMORY;
    HdmaCh2.Init.DstInc              = DMA_DST_ADDR_INC;
    HdmaCh2.Init.SrcInc              = DMA_SRC_ADDR_FIX;
    HdmaCh2.Init.DstWidth            = DMA_DST_WIDTH_HALFWORD;
    HdmaCh2.Init.SrcWidth            = DMA_SRC_WIDTH_HALFWORD;
    HdmaCh2.Init.SrcBurstLen         = DMA_SRC_BURST_LEN_1;
    HdmaCh2.Init.SrcHandshakeType    = DMA_SRC_HS_TYPE_HARD;
    HdmaCh2.Init.BlockMaxLen         = DMA_BLOCK_DEFAULT_MAX_LEN;

    HdmaCh2.Init.Mode                = DMA_NORMAL;
    HdmaCh2.Init.Priority            = DMA_PRIORITY_1;
    
    /* Initialize DMA*/
    HAL_DMA_Init(&HdmaCh2);
    
    /* Set DMA channel map. */
    HAL_DMA_ChannelReq(&HdmaCh2, DMA_REQ_TYPE_SRC, DMA_REQ_NUM_1, DMA_REQ_FUNC_SPI1_RX); /* SPI1_RX DMA1_CH2 */
    
    /* DMA handle is associated with SPI handle*/
    __HAL_LINKDMA(hi2s, hdmarx, HdmaCh2);

    /* DMA interrupt configuration*/
    HAL_NVIC_SetPriority(DMA1_Channel1_IRQn, 1, 0);
    HAL_NVIC_EnableIRQ(DMA1_Channel1_IRQn);
    HAL_NVIC_SetPriority(DMA1_Channel2_IRQn, 1, 0);
    HAL_NVIC_EnableIRQ(DMA1_Channel2_IRQn);
  }
}

/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
