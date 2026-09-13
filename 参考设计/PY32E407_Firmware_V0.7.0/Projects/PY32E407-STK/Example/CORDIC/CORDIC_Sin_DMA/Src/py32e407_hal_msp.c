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
DMA_HandleTypeDef         hDMAW;
DMA_HandleTypeDef         hDMAR;

/* Private function prototypes -----------------------------------------------*/
/* External functions --------------------------------------------------------*/
extern CORDIC_HandleTypeDef      hCORDIC;

/**
  * @brief Initialize global MSP
  */
void HAL_MspInit(void)
{
  __HAL_RCC_SYSCFG_CLK_ENABLE();
  __HAL_RCC_PWR_CLK_ENABLE();
}

/**
  * @brief  Initialize the CORDIC MSP.
  * @param  hcordic CORDIC handle
  * @retval None
  */
void HAL_CORDIC_MspInit(CORDIC_HandleTypeDef *hcordic)
{
  __HAL_RCC_CORDIC_CLK_ENABLE();
  __HAL_RCC_DMA1_CLK_ENABLE();
  __HAL_RCC_SYSCFG_CLK_ENABLE();
  
  /* Cordic write request DMA init */
  hDMAW.Instance = DMA1_Channel2;
  hDMAW.Init.Direction = DMA_MEMORY_TO_PERIPH;
  hDMAW.Init.SrcWidth = DMA_SRC_WIDTH_WORD;
  hDMAW.Init.DstWidth = DMA_DST_WIDTH_WORD;
  /* hDMAW.Init.SrcBurstLen = DMA_SRC_BURST_LEN_1; */
  hDMAW.Init.DstBurstLen = DMA_DST_BURST_LEN_1;
  hDMAW.Init.SrcInc = DMA_SRC_ADDR_INC;
  hDMAW.Init.DstInc = DMA_DST_ADDR_FIX;
  hDMAW.Init.Mode = DMA_NORMAL;
  /* hDMAW.Init.SrcReload = ; */
  /* hDMAW.Init.DstReload = ; */
  /* hDMAW.Init.SrcHandshakeType = DMA_SRC_HS_TYPE_HARD; */
  hDMAW.Init.DstHandshakeType = DMA_DST_HS_TYPE_HARD;
  hDMAW.Init.Priority = DMA_PRIORITY_1;
  hDMAW.Init.BlockMaxLen = DMA_BLOCK_DEFAULT_MAX_LEN;
  HAL_DMA_Init(&hDMAW);
  
  /* Set DMA channel map. */
  HAL_DMA_ChannelReq(&hDMAW, DMA_REQ_TYPE_DST, DMA_REQ_NUM_1, DMA_REQ_FUNC_CORDIC_WR);
  
  /* NVIC enable */
  HAL_NVIC_SetPriority(DMA1_Channel2_IRQn, 2, 0);
  HAL_NVIC_EnableIRQ(DMA1_Channel2_IRQn);
  
  /* DMA handles are associated to cordic handles */
  __HAL_LINKDMA(&hCORDIC, hdmaIn, hDMAW);
  
  /* Cordic read request DMA init */
  hDMAR.Instance = DMA1_Channel1;
  hDMAR.Init.Direction = DMA_PERIPH_TO_MEMORY;
  hDMAR.Init.SrcWidth = DMA_SRC_WIDTH_WORD;
  hDMAR.Init.DstWidth = DMA_DST_WIDTH_WORD;
  hDMAR.Init.SrcBurstLen = DMA_SRC_BURST_LEN_1;
  /* hDMAR.Init.DstBurstLen = DMA_DST_BURST_LEN_1; */
  hDMAR.Init.SrcInc = DMA_SRC_ADDR_FIX;
  hDMAR.Init.DstInc = DMA_DST_ADDR_INC;
  hDMAR.Init.Mode = DMA_NORMAL;
  /* hDMAR.Init.SrcReload = ; */
  /* hDMAR.Init.DstReload = ; */
  hDMAR.Init.SrcHandshakeType = DMA_SRC_HS_TYPE_HARD;
  /*  hDMAR.Init.DstHandshakeType = DMA_DST_HS_TYPE_HARD; */
  hDMAR.Init.Priority = DMA_PRIORITY_1;
  hDMAR.Init.BlockMaxLen = DMA_BLOCK_DEFAULT_MAX_LEN;
  HAL_DMA_Init(&hDMAR);
  
  /* Set DMA channel map. */
  HAL_DMA_ChannelReq(&hDMAR, DMA_REQ_TYPE_SRC, DMA_REQ_NUM_0, DMA_REQ_FUNC_CORDIC_RD);
  
  /* NVIC enable */
  HAL_NVIC_SetPriority(DMA1_Channel1_IRQn, 2, 0);
  HAL_NVIC_EnableIRQ(DMA1_Channel1_IRQn);
  
  /* DMA handles are associated to cordic handles */
  __HAL_LINKDMA(&hCORDIC, hdmaOut, hDMAR);
  
}

/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
