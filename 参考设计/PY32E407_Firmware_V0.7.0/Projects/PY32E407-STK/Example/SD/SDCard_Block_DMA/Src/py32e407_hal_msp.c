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
  * @brief Initialize block read as DMA
  * @param hdma_sd: Pointer to DMA handle
  * @retval None
  */
void HAL_SD_DMA_ReadBlockInit(DMA_HandleTypeDef  *hdma_sd)
{
  /* enable clock */
  __HAL_RCC_DMA1_CLK_ENABLE();
  __HAL_RCC_SYSCFG_CLK_ENABLE();
  
  /*##-1- Configure the DMA ##########################################*/
  hdma_sd->Instance = DMA1_Channel1;
  hdma_sd->Init.Direction = DMA_PERIPH_TO_MEMORY;
  hdma_sd->Init.SrcWidth = DMA_SRC_WIDTH_WORD;
  hdma_sd->Init.DstWidth = DMA_DST_WIDTH_WORD;
  hdma_sd->Init.SrcBurstLen = DMA_SRC_BURST_LEN_1;
  hdma_sd->Init.SrcInc = DMA_SRC_ADDR_FIX;
  hdma_sd->Init.DstInc = DMA_DST_ADDR_INC;
  hdma_sd->Init.Mode = DMA_NORMAL;
  hdma_sd->Init.SrcHandshakeType = DMA_SRC_HS_TYPE_HARD;
  hdma_sd->Init.Priority = DMA_PRIORITY_1;
  hdma_sd->Init.BlockMaxLen = DMA_BLOCK_DEFAULT_MAX_LEN;
  HAL_DMA_Init(hdma_sd);
  
  /* Set DMA channel map */
  HAL_DMA_ChannelReq(hdma_sd, DMA_REQ_TYPE_SRC, DMA_REQ_NUM_2, DMA_REQ_FUNC_SDIO);
  
  /* NVIC enable */
  HAL_NVIC_SetPriority(DMA1_Channel1_IRQn, 2, 0);
  HAL_NVIC_EnableIRQ(DMA1_Channel1_IRQn);
  
}
/**
  * @brief Initialize block write as DMA
  * @param hdma_sd: Pointer to DMA handle
  * @retval None
  */
void HAL_SD_DMA_WriteBlockInit(DMA_HandleTypeDef  *hdma_sd)
{
  /* enable clock */
  __HAL_RCC_DMA1_CLK_ENABLE();
  __HAL_RCC_SYSCFG_CLK_ENABLE();
  
  /*##-1- Configure the DMA ##########################################*/
  hdma_sd->Instance = DMA1_Channel2;
  hdma_sd->Init.Direction = DMA_MEMORY_TO_PERIPH;
  hdma_sd->Init.SrcWidth = DMA_SRC_WIDTH_WORD;
  hdma_sd->Init.DstWidth = DMA_DST_WIDTH_WORD;
  hdma_sd->Init.DstBurstLen = DMA_DST_BURST_LEN_1;
  hdma_sd->Init.SrcInc = DMA_SRC_ADDR_INC;
  hdma_sd->Init.DstInc = DMA_DST_ADDR_FIX;
  hdma_sd->Init.Mode = DMA_NORMAL;
  hdma_sd->Init.DstHandshakeType = DMA_DST_HS_TYPE_HARD;
  hdma_sd->Init.Priority = DMA_PRIORITY_1;
  hdma_sd->Init.BlockMaxLen = DMA_BLOCK_DEFAULT_MAX_LEN;
  HAL_DMA_Init(hdma_sd);
  
  /* Set DMA channel map. */
  HAL_DMA_ChannelReq(hdma_sd, DMA_REQ_TYPE_DST, DMA_REQ_NUM_5, DMA_REQ_FUNC_SDIO);
  
  /* NVIC enable */
  HAL_NVIC_SetPriority(DMA1_Channel2_IRQn, 2, 0);
  HAL_NVIC_EnableIRQ(DMA1_Channel2_IRQn);
  
}

void HAL_SD_MspInit(SD_HandleTypeDef *hsd)
{
  GPIO_InitTypeDef SDIOInit = {0};
  
  /* enable clock */
  __HAL_RCC_SDIO_CLK_ENABLE();
  __HAL_RCC_GPIOC_CLK_ENABLE();
  __HAL_RCC_GPIOD_CLK_ENABLE();
  
  /* configuration pins
  PD2:  CMD
  PC8:  D0
  PC9:  D1
  PC10: D2
  PC11: D3
  PC12: SCK
  */

  SDIOInit.Mode       = GPIO_MODE_AF_PP;
  SDIOInit.Pull       = GPIO_PULLUP;
  SDIOInit.Speed      = GPIO_SPEED_FREQ_VERY_HIGH;
  SDIOInit.Alternate  = GPIO_AF11_SDIO;
  
  SDIOInit.Pin    = GPIO_PIN_2;
  HAL_GPIO_Init(GPIOD,&SDIOInit);
  
  SDIOInit.Pin    = GPIO_PIN_8;
  HAL_GPIO_Init(GPIOC,&SDIOInit);
  
  SDIOInit.Pin    = GPIO_PIN_9;
  HAL_GPIO_Init(GPIOC,&SDIOInit);
  
  SDIOInit.Pin    = GPIO_PIN_10;
  HAL_GPIO_Init(GPIOC,&SDIOInit);
  
  SDIOInit.Pin    = GPIO_PIN_11;
  HAL_GPIO_Init(GPIOC,&SDIOInit);
  
  SDIOInit.Pin    = GPIO_PIN_12;
  HAL_GPIO_Init(GPIOC,&SDIOInit);
  
  /*  Enable SDIO Global IT  */
  __HAL_SD_ENABLE_GLOBAL_IT(&SDHandle);
  
  /* Enable NVIC */
  HAL_NVIC_SetPriority(SDIO_IRQn, 2, 0);
  HAL_NVIC_EnableIRQ(SDIO_IRQn);
  
  /* configure the DMA */
  HAL_SD_DMA_ReadBlockInit(&SD_DMA_ReadBlockHandle);
  HAL_SD_DMA_WriteBlockInit(&SD_DMA_WriteBlockHandle);
  
  /* DMA handles are associated to SD handles */
  __HAL_LINKDMA(&SDHandle, hdmarx, SD_DMA_ReadBlockHandle);
  __HAL_LINKDMA(&SDHandle, hdmatx, SD_DMA_WriteBlockHandle);
}
/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
