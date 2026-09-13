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
  * @brief Initialize I2C MSP
  */
void HAL_I2C_MspInit(I2C_HandleTypeDef *hi2c)
{
  GPIO_InitTypeDef GPIO_InitStruct = {0};
  
  __HAL_RCC_GPIOC_CLK_ENABLE();                   /* Enable GPIOC clock */
  __HAL_RCC_SYSCFG_CLK_ENABLE();                  /* Enable SYSCFG clock */
  __HAL_RCC_I2C3_CLK_ENABLE();                    /* Enable I2C3 clock */
  __HAL_RCC_DMA1_CLK_ENABLE();                    /* Enable DMA clock */

  /**I2C GPIO Configuration
  PC9  -------> I2C3_SDA
  PC8  -------> I2C3_SCL
  */
  GPIO_InitStruct.Pin = GPIO_PIN_9 | GPIO_PIN_8;
  GPIO_InitStruct.Mode = GPIO_MODE_AF_OD;                     /* Open-drain mode */
  GPIO_InitStruct.Pull = GPIO_PULLUP;                         /* Pull-up */
  GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_VERY_HIGH;
  GPIO_InitStruct.Alternate = GPIO_AF8_I2C3;                  /* Alternate as I2C */
  HAL_GPIO_Init(GPIOC, &GPIO_InitStruct);                     /* Initialize GPIO */

  /* Enable NVIC interrupt for I2C */
  HAL_NVIC_SetPriority(I2C3_EV_IRQn, 0, 0);                   /* Set interrupt priority */
  HAL_NVIC_EnableIRQ(I2C3_EV_IRQn);                           /* Enable I2C interrupt */
  HAL_NVIC_SetPriority(I2C3_ER_IRQn, 0, 0);
  HAL_NVIC_EnableIRQ(I2C3_ER_IRQn);
    
  /* I2C3 DMA Init */
  /* I2C3_TX Init */
  HdmaCh1.Instance = DMA1_Channel1;
  HdmaCh1.Init.Direction = DMA_MEMORY_TO_PERIPH;
  HdmaCh1.Init.DstInc = DMA_DST_ADDR_FIX;
  HdmaCh1.Init.SrcInc = DMA_SRC_ADDR_INC;
  HdmaCh1.Init.DstWidth = DMA_DST_WIDTH_BYTE;
  HdmaCh1.Init.SrcWidth = DMA_SRC_WIDTH_BYTE;
  
  HdmaCh1.Init.DstBurstLen      = DMA_DST_BURST_LEN_1;
  HdmaCh1.Init.DstHandshakeType = DMA_DST_HS_TYPE_HARD;
  HdmaCh1.Init.BlockMaxLen      = DMA_BLOCK_DEFAULT_MAX_LEN;
  
  HdmaCh1.Init.Mode = DMA_NORMAL;
  HdmaCh1.Init.Priority = DMA_PRIORITY_1;
  if (HAL_DMA_Init(&HdmaCh1) != HAL_OK)
  {
    APP_ErrorHandler();
  }

  /* Set DMA channel map. */
  HAL_DMA_ChannelReq(&HdmaCh1, DMA_REQ_TYPE_DST, DMA_REQ_NUM_0, DMA_REQ_FUNC_I2C3_TX);
  
  /* DMA handle is associated with I2C handle */
  __HAL_LINKDMA(hi2c,hdmatx,HdmaCh1);

  /* I2C3_RX Init */
  HdmaCh2.Instance = DMA1_Channel2;
  HdmaCh2.Init.Direction = DMA_PERIPH_TO_MEMORY;
  HdmaCh2.Init.DstInc = DMA_DST_ADDR_INC;
  HdmaCh2.Init.SrcInc = DMA_SRC_ADDR_FIX;
  HdmaCh2.Init.DstWidth = DMA_DST_WIDTH_BYTE;
  HdmaCh2.Init.SrcWidth = DMA_SRC_WIDTH_BYTE;
  HdmaCh2.Init.SrcBurstLen      = DMA_SRC_BURST_LEN_1;
  HdmaCh2.Init.SrcHandshakeType = DMA_SRC_HS_TYPE_HARD;
  HdmaCh2.Init.BlockMaxLen      = DMA_BLOCK_DEFAULT_MAX_LEN;
  
  HdmaCh2.Init.Mode = DMA_NORMAL;
  HdmaCh2.Init.Priority = DMA_PRIORITY_0;
  if (HAL_DMA_Init(&HdmaCh2) != HAL_OK)
  {
    APP_ErrorHandler();
  }

  /* Set DMA channel map. */
  HAL_DMA_ChannelReq(&HdmaCh2, DMA_REQ_TYPE_SRC, DMA_REQ_NUM_1, DMA_REQ_FUNC_I2C3_RX);
  
  /* DMA handle is associated with I2C handle */
  __HAL_LINKDMA(hi2c,hdmarx,HdmaCh2);
    
  /* DMA interrupt configuration*/
  HAL_NVIC_SetPriority(DMA1_Channel1_IRQn, 0, 0);
  HAL_NVIC_EnableIRQ(DMA1_Channel1_IRQn);
  HAL_NVIC_SetPriority(DMA1_Channel2_IRQn, 0, 0);
  HAL_NVIC_EnableIRQ(DMA1_Channel2_IRQn);
}

/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
