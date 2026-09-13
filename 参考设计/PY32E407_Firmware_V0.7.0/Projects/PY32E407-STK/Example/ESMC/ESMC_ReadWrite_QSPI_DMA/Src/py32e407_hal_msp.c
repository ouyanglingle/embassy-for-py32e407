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
extern ESMC_HandleTypeDef hesmc;

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
void HAL_ESMC_DMA_ReadBlockInit(DMA_HandleTypeDef  *hdma_esmc)
{
  /* enable clock */
  __HAL_RCC_DMA1_CLK_ENABLE();
  __HAL_RCC_SYSCFG_CLK_ENABLE();
  
  /*##-1- Configure the DMA ##########################################*/
  hdma_esmc->Instance = DMA1_Channel1;
  hdma_esmc->Init.Direction = DMA_PERIPH_TO_MEMORY;
  hdma_esmc->Init.SrcWidth = DMA_SRC_WIDTH_WORD;
  hdma_esmc->Init.DstWidth = DMA_DST_WIDTH_WORD;
  hdma_esmc->Init.SrcBurstLen = DMA_SRC_BURST_LEN_1;
  hdma_esmc->Init.DstBurstLen = DMA_DST_BURST_LEN_1;
  hdma_esmc->Init.SrcInc = DMA_SRC_ADDR_FIX;
  hdma_esmc->Init.Mode = DMA_NORMAL;
  hdma_esmc->Init.SrcReload = DMA_SRC_RELOAD_DISABLE;
  hdma_esmc->Init.DstReload = DMA_DST_RELOAD_DISABLE;
  hdma_esmc->Init.SrcHandshakeType = DMA_SRC_HS_TYPE_HARD;
  hdma_esmc->Init.DstHandshakeType = DMA_DST_HS_TYPE_HARD;
  hdma_esmc->Init.Priority = DMA_PRIORITY_1;
  hdma_esmc->Init.BlockMaxLen = 4095;
  HAL_DMA_Init(hdma_esmc);
  
  /* Set DMA channel map */
  HAL_DMA_ChannelReq(hdma_esmc, DMA_REQ_TYPE_SRC, DMA_REQ_NUM_0, DMA_REQ_FUNC_ESMC_RX);
  
  /* NVIC enable */
  HAL_NVIC_SetPriority(DMA1_Channel1_IRQn, 2, 0);
  HAL_NVIC_EnableIRQ(DMA1_Channel1_IRQn);
}

/**
  * @brief Initialize block write as DMA
  * @param hdma_sd: Pointer to DMA handle
  * @retval None
  */
void HAL_ESMC_DMA_WriteBlockInit(DMA_HandleTypeDef  *hdma_esmc)
{
  /* enable clock */
  __HAL_RCC_DMA1_CLK_ENABLE();
  __HAL_RCC_SYSCFG_CLK_ENABLE();
  
  /*##-1- Configure the DMA ##########################################*/
  hdma_esmc->Instance = DMA1_Channel2;
  hdma_esmc->Init.Direction = DMA_MEMORY_TO_PERIPH;
  hdma_esmc->Init.SrcWidth = DMA_SRC_WIDTH_WORD;
  hdma_esmc->Init.DstWidth = DMA_DST_WIDTH_WORD;
  hdma_esmc->Init.SrcBurstLen = DMA_SRC_BURST_LEN_1;
  hdma_esmc->Init.DstBurstLen = DMA_DST_BURST_LEN_1;
  hdma_esmc->Init.SrcInc = DMA_SRC_ADDR_INC;
  hdma_esmc->Init.DstInc = DMA_DST_ADDR_FIX;
  hdma_esmc->Init.Mode = DMA_NORMAL;
  hdma_esmc->Init.SrcReload = DMA_SRC_RELOAD_DISABLE;
  hdma_esmc->Init.DstReload = DMA_DST_RELOAD_DISABLE;
  hdma_esmc->Init.SrcHandshakeType = DMA_SRC_HS_TYPE_HARD;
  hdma_esmc->Init.DstHandshakeType = DMA_DST_HS_TYPE_HARD;
  hdma_esmc->Init.Priority = DMA_PRIORITY_1;
  hdma_esmc->Init.BlockMaxLen = 4095;
  HAL_DMA_Init(hdma_esmc);
  
  /* Set DMA channel map. */
  HAL_DMA_ChannelReq(hdma_esmc, DMA_REQ_TYPE_DST, DMA_REQ_NUM_1, DMA_REQ_FUNC_ESMC_TX);
  
  /* NVIC enable */
  HAL_NVIC_SetPriority(DMA1_Channel2_IRQn, 2, 0);
  HAL_NVIC_EnableIRQ(DMA1_Channel2_IRQn);
}

/**
  * @brief Initialize ESMC MSP.
  */
void HAL_ESMC_MspInit(ESMC_HandleTypeDef *hesmc)
{
  GPIO_InitTypeDef GPIO_InitStructure = {0};

  /* Enable ESMC Clock */
  __HAL_RCC_ESMC_CLK_ENABLE();

  /* Enable GPIO Clock */
  __HAL_RCC_GPIOA_CLK_ENABLE();
  __HAL_RCC_GPIOB_CLK_ENABLE();
  __HAL_RCC_GPIOD_CLK_ENABLE();
  
  /* ESMC GPIO pin initialization */
  GPIO_InitStructure.Mode = GPIO_MODE_AF_PP;
  GPIO_InitStructure.Alternate = GPIO_AF10_ESMC;
  GPIO_InitStructure.Speed = GPIO_SPEED_FREQ_VERY_HIGH;

  /* CLK */
  GPIO_InitStructure.Pin = GPIO_PIN_3;
  GPIO_InitStructure.Pull =GPIO_NOPULL;
  HAL_GPIO_Init(GPIOA, &GPIO_InitStructure);
 
  /* SS5 */
  GPIO_InitStructure.Pin = GPIO_PIN_2;
  GPIO_InitStructure.Pull = GPIO_PULLUP;
  HAL_GPIO_Init(GPIOD, &GPIO_InitStructure);

  /* IO0 */
  GPIO_InitStructure.Pin = GPIO_PIN_1;
  GPIO_InitStructure.Pull =GPIO_NOPULL;
  HAL_GPIO_Init(GPIOB, &GPIO_InitStructure);

  /* IO1 */
  GPIO_InitStructure.Pin = GPIO_PIN_0;
  GPIO_InitStructure.Pull =GPIO_NOPULL;
  HAL_GPIO_Init(GPIOB, &GPIO_InitStructure); 
   
  /* IO2 */
  GPIO_InitStructure.Pin = GPIO_PIN_7;
  GPIO_InitStructure.Pull = GPIO_PULLUP;
  HAL_GPIO_Init(GPIOA, &GPIO_InitStructure);

  /* IO3 */
  GPIO_InitStructure.Pin = GPIO_PIN_6;
  GPIO_InitStructure.Pull = GPIO_PULLUP;
  HAL_GPIO_Init(GPIOA, &GPIO_InitStructure);

  /* configure the DMA */
  HAL_ESMC_DMA_ReadBlockInit(&ESMC_DMA_ReadBlockHandle);
  HAL_ESMC_DMA_WriteBlockInit(&ESMC_DMA_WriteBlockHandle);
  
  /* DMA handles are associated to ESMC handles */
  __HAL_LINKDMA(hesmc, hdmarx, ESMC_DMA_ReadBlockHandle);
  __HAL_LINKDMA(hesmc, hdmatx, ESMC_DMA_WriteBlockHandle);

  HAL_NVIC_SetPriority(ESMC_IRQn, 0, 0);
  HAL_NVIC_EnableIRQ(ESMC_IRQn);
}

/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
