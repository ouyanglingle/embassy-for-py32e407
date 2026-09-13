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
#ifdef TRANSMITTER_BOARD
/**
* @brief USART MSP Initialization
* This function configures the hardware resources used in this example
* @param husart: USART handle pointer
* @retval None
*/
void HAL_USART_MspInit(USART_HandleTypeDef* husart)
{
  GPIO_InitTypeDef GPIO_InitStruct = {0};
  if(husart->Instance == USART1)
  {
    /* Peripheral clock enable */
    __HAL_RCC_USART1_CLK_ENABLE();
    __HAL_RCC_GPIOA_CLK_ENABLE();
    __HAL_RCC_DMA1_CLK_ENABLE();
    
    /**USART1 GPIO Configuration
    PA8     ------> USART1_CK
    PA9     ------> USART1_TX
    PA10    ------> USART1_RX
    */
    GPIO_InitStruct.Pin       = GPIO_PIN_8|GPIO_PIN_9|GPIO_PIN_10;
    GPIO_InitStruct.Mode      = GPIO_MODE_AF_PP;
    GPIO_InitStruct.Pull      = GPIO_PULLDOWN;
    GPIO_InitStruct.Speed     = GPIO_SPEED_FREQ_VERY_HIGH;
    GPIO_InitStruct.Alternate = GPIO_AF7_USART1;
    HAL_GPIO_Init(GPIOA, &GPIO_InitStruct);

    /* USART1 DMA Init */
    /* USART1_RX Init */
    hdma_usart_rx.Instance              = DMA1_Channel2;
    hdma_usart_rx.Init.Direction        = DMA_PERIPH_TO_MEMORY;
    hdma_usart_rx.Init.DstInc           = DMA_DST_ADDR_INC;
    hdma_usart_rx.Init.SrcInc           = DMA_SRC_ADDR_FIX;
    hdma_usart_rx.Init.DstWidth         = DMA_DST_WIDTH_BYTE;
    hdma_usart_rx.Init.SrcWidth         = DMA_SRC_WIDTH_BYTE;
    hdma_usart_rx.Init.SrcBurstLen      = DMA_SRC_BURST_LEN_1;
    hdma_usart_rx.Init.SrcHandshakeType = DMA_SRC_HS_TYPE_HARD;
    hdma_usart_rx.Init.BlockMaxLen      = DMA_BLOCK_DEFAULT_MAX_LEN;

    hdma_usart_rx.Init.Mode             = DMA_NORMAL;
    hdma_usart_rx.Init.Priority         = DMA_PRIORITY_1;
    
    if (HAL_DMA_Init(&hdma_usart_rx) != HAL_OK)
    {
      APP_ErrorHandler();
    }
    HAL_DMA_ChannelReq(&hdma_usart_rx, DMA_REQ_TYPE_SRC, DMA_REQ_NUM_1, DMA_REQ_FUNC_USART1_RX);

    __HAL_LINKDMA(husart, hdmarx, hdma_usart_rx);

    /* USART1_TX Init */
    hdma_usart_tx.Instance              = DMA1_Channel1;
    hdma_usart_tx.Init.Direction        = DMA_MEMORY_TO_PERIPH;
    hdma_usart_tx.Init.DstInc           = DMA_DST_ADDR_FIX;
    hdma_usart_tx.Init.SrcInc           = DMA_SRC_ADDR_INC;
    hdma_usart_tx.Init.DstWidth         = DMA_DST_WIDTH_BYTE;
    hdma_usart_tx.Init.SrcWidth         = DMA_SRC_WIDTH_BYTE;
    hdma_usart_tx.Init.DstBurstLen      = DMA_DST_BURST_LEN_1;
    hdma_usart_tx.Init.DstHandshakeType = DMA_DST_HS_TYPE_HARD;
    hdma_usart_tx.Init.BlockMaxLen      = DMA_BLOCK_DEFAULT_MAX_LEN;

    hdma_usart_tx.Init.Mode             = DMA_NORMAL;
    hdma_usart_tx.Init.Priority         = DMA_PRIORITY_0;

    if (HAL_DMA_Init(&hdma_usart_tx) != HAL_OK)
    {
      APP_ErrorHandler();
    }
    HAL_DMA_ChannelReq(&hdma_usart_tx, DMA_REQ_TYPE_DST, DMA_REQ_NUM_0, DMA_REQ_FUNC_USART1_TX);

    __HAL_LINKDMA(husart,hdmatx,hdma_usart_tx);

    /* NVIC enable */
    HAL_NVIC_SetPriority(DMA1_Channel1_IRQn, 2, 0);
    HAL_NVIC_EnableIRQ(DMA1_Channel1_IRQn);
    HAL_NVIC_SetPriority(DMA1_Channel2_IRQn, 2, 0);
    HAL_NVIC_EnableIRQ(DMA1_Channel2_IRQn);
    /* USART1 interrupt Init */
    HAL_NVIC_SetPriority(USART1_IRQn, 1, 0);
    HAL_NVIC_EnableIRQ(USART1_IRQn);
  }
}
#else
/**
  * @brief Initialize MSP for SPI
  */
void HAL_SPI_MspInit(SPI_HandleTypeDef *Spi1Handle)
{
  GPIO_InitTypeDef GPIO_InitStruct = {0};

  __HAL_RCC_GPIOA_CLK_ENABLE();                   /* GPIOA clock enable */
  __HAL_RCC_SPI1_CLK_ENABLE();                    /* SPI1 clock enable */
  __HAL_RCC_DMA1_CLK_ENABLE();
  
  /*
    PA5   ------> SPI1_SCK
    PA6   ------> SPI1_MISO
    PA7   ------> SPI1_MOSI
  */
  /* SPI SCK GPIO pin configuration  */
  GPIO_InitStruct.Pin       = GPIO_PIN_5;
  if (Spi1Handle->Init.CLKPolarity == SPI_POLARITY_LOW)
  {
    GPIO_InitStruct.Pull = GPIO_PULLDOWN;
  }
  else
  {
    GPIO_InitStruct.Pull = GPIO_PULLUP;
  }
  GPIO_InitStruct.Mode      = GPIO_MODE_AF_PP;
  GPIO_InitStruct.Speed     = GPIO_SPEED_FREQ_VERY_HIGH;
  GPIO_InitStruct.Alternate = GPIO_AF5_SPI1;
  HAL_GPIO_Init(GPIOA, &GPIO_InitStruct);

  /* Configure GPIO as SPI: MISO/MOSI*/
  GPIO_InitStruct.Pin       = GPIO_PIN_6 | GPIO_PIN_7;
  GPIO_InitStruct.Mode      = GPIO_MODE_AF_PP;
  GPIO_InitStruct.Pull      = GPIO_PULLDOWN;
  GPIO_InitStruct.Speed     = GPIO_SPEED_FREQ_VERY_HIGH;
  GPIO_InitStruct.Alternate = GPIO_AF5_SPI1;
  HAL_GPIO_Init(GPIOA, &GPIO_InitStruct);
      
  /* SPI1 DMA Init */
  /* DMA_CH1 configuration */
  hdma_spi_tx.Instance                 = DMA1_Channel1;
  hdma_spi_tx.Init.Direction           = DMA_MEMORY_TO_PERIPH;
  hdma_spi_tx.Init.DstInc              = DMA_DST_ADDR_FIX;
  hdma_spi_tx.Init.SrcInc              = DMA_SRC_ADDR_INC;
  if (Spi1Handle->Init.DataSize <= SPI_DATASIZE_8BIT)
  {
    hdma_spi_tx.Init.DstWidth = DMA_DST_WIDTH_BYTE;
    hdma_spi_tx.Init.SrcWidth = DMA_SRC_WIDTH_BYTE;
  }
  else
  {
    hdma_spi_tx.Init.DstWidth = DMA_DST_WIDTH_HALFWORD;
    hdma_spi_tx.Init.SrcWidth = DMA_SRC_WIDTH_HALFWORD;
  }
  hdma_spi_tx.Init.DstBurstLen      = DMA_DST_BURST_LEN_1;
  hdma_spi_tx.Init.DstHandshakeType = DMA_DST_HS_TYPE_HARD;
  hdma_spi_tx.Init.BlockMaxLen      = DMA_BLOCK_DEFAULT_MAX_LEN;

  hdma_spi_tx.Init.Mode                = DMA_NORMAL;
  hdma_spi_tx.Init.Priority            = DMA_PRIORITY_1;
  /* Initialize DMA */
  HAL_DMA_Init(&hdma_spi_tx);
  
  /* Set DMA channel map. */
  HAL_DMA_ChannelReq(&hdma_spi_tx, DMA_REQ_TYPE_DST, DMA_REQ_NUM_0, DMA_REQ_FUNC_SPI1_TX);
  
  /* DMA handle is associated with SPI handle */
  __HAL_LINKDMA(Spi1Handle, hdmatx, hdma_spi_tx);

  /* DMA_CH2 configuration */
  hdma_spi_rx.Instance                 = DMA1_Channel2;
  hdma_spi_rx.Init.Direction           = DMA_PERIPH_TO_MEMORY;
  hdma_spi_rx.Init.DstInc              = DMA_DST_ADDR_INC;
  hdma_spi_rx.Init.SrcInc              = DMA_SRC_ADDR_FIX;
  if (Spi1Handle->Init.DataSize <= SPI_DATASIZE_8BIT)
  {
    hdma_spi_rx.Init.DstWidth = DMA_DST_WIDTH_BYTE;
    hdma_spi_rx.Init.SrcWidth = DMA_SRC_WIDTH_BYTE;
  }
  else
  {
    hdma_spi_rx.Init.DstWidth = DMA_DST_WIDTH_HALFWORD;
    hdma_spi_rx.Init.SrcWidth = DMA_SRC_WIDTH_HALFWORD;
  }
  hdma_spi_rx.Init.DstBurstLen      = DMA_DST_BURST_LEN_1;
  hdma_spi_rx.Init.DstHandshakeType = DMA_DST_HS_TYPE_HARD;
  hdma_spi_rx.Init.BlockMaxLen      = DMA_BLOCK_DEFAULT_MAX_LEN;

  hdma_spi_rx.Init.Mode                = DMA_NORMAL;
  hdma_spi_rx.Init.Priority            = DMA_PRIORITY_0;
  
  /* Initialize DMA*/
  HAL_DMA_Init(&hdma_spi_rx);
  
  /* Set DMA channel map. */
  HAL_DMA_ChannelReq(&hdma_spi_rx, DMA_REQ_TYPE_SRC, DMA_REQ_NUM_1, DMA_REQ_FUNC_SPI1_RX);
  
  /* DMA handle is associated with SPI handle*/
  __HAL_LINKDMA(Spi1Handle, hdmarx, hdma_spi_rx);
  
  /* DMA interrupt configuration*/
  HAL_NVIC_SetPriority(DMA1_Channel1_IRQn, 1, 0);
  HAL_NVIC_EnableIRQ(DMA1_Channel1_IRQn);
  HAL_NVIC_SetPriority(DMA1_Channel2_IRQn, 1, 0);
  HAL_NVIC_EnableIRQ(DMA1_Channel2_IRQn);
  /* Interrupt configuration */
  HAL_NVIC_SetPriority(SPI1_IRQn, 1, 0);
  HAL_NVIC_EnableIRQ(SPI1_IRQn);
}
#endif

/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
