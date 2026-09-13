/**
  ******************************************************************************
  * @file    main.c
  * @author  MCU Application Team
  * @brief   Main program body
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
#include "py32e407xx_ll_Start_Kit.h"

/* Private define ------------------------------------------------------------*/
#define BOARD_IN_MASTER_MODE

/* Private variables ---------------------------------------------------------*/
/* Send buffer definition */
uint8_t aTxBuffer[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15};
uint8_t ubNbDataToTransmit = sizeof(aTxBuffer);

/* Receive buffer definition */
uint8_t aRxBuffer[sizeof(aTxBuffer)] = {0};
uint8_t ubNbDataToReceive = sizeof(aTxBuffer);

/* A flag that data is received and sent completely */
uint8_t txRxDataComplteFlag = RESET;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_ConfigSPI(void);
static void APP_SpiTransmitReceive_DMA(uint8_t *pTxData, uint8_t *pRxData, uint16_t Size);
static void APP_WaitAndCheckEndOfTransfer(void);
static uint8_t APP_Buffercmp8(uint8_t* pBuffer1, uint8_t* pBuffer2, uint8_t BufferLength);
static void APP_LedBlinking(void);

/**
  * @brief  Main program.
  * @param  None
  * @retval int
  */
int main(void)
{
  /* Enable SYSCFG and PWR clock */
  LL_APB2_GRP1_EnableClock(LL_APB2_GRP1_PERIPH_SYSCFG);
  LL_APB1_GRP1_EnableClock(LL_APB1_GRP1_PERIPH_PWR);
  
  /* 3 bits for pre-emption priority, 0 bits for subpriority */
  NVIC_SetPriorityGrouping(NVIC_PRIORITYGROUP_3);

  /* Configure system clock */
  APP_SystemClockConfig();

  /* Initialize LED */
  BSP_LED_Init(LED_GREEN);

  /* Initialize SPI */
  APP_ConfigSPI();

#ifdef BOARD_IN_MASTER_MODE
  /* Initialize BUTTON */
  BSP_PB_Init(BUTTON_KEY,BUTTON_MODE_GPIO);

  /* Wait for the button to be pressed */
  while(BSP_PB_GetState(BUTTON_KEY) == 0);
#endif
  /* SPI Sending and Receiving Data */
  APP_SpiTransmitReceive_DMA(aTxBuffer, aRxBuffer, ubNbDataToTransmit);

  /* Wait transfer to be completed and check the received data */
  APP_WaitAndCheckEndOfTransfer();

  while (1)
  {
  }
}

/**
  * @brief  SPI1 configuration function
  * @param  None
  * @retval None
  */
static void APP_ConfigSPI(void)
{
  LL_SPI_InitTypeDef SPI_InitStruct = {0};

  LL_GPIO_InitTypeDef GPIO_InitStruct = {0};

  LL_DMA_InitTypeDef DMA_InitStruct = {0};

  /* Enable SPI1 clock */
  LL_APB2_GRP1_EnableClock(LL_APB2_GRP1_PERIPH_SPI1);
  /* Enable DMA clock */
  LL_AHB1_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_DMA1);
  /* Enable GPIOA clock */
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOA);
  
  /** SPI1 Pin config
  PA5 ------> SPI1_SCK
  PA6 ------> SPI1_MISO
  PA7 ------> SPI1_MOSI
  */
  /* SPI1 SCK configuration */
  GPIO_InitStruct.Pin = LL_GPIO_PIN_5;
  GPIO_InitStruct.Mode = LL_GPIO_MODE_ALTERNATE;
  GPIO_InitStruct.Speed = LL_GPIO_SPEED_FREQ_HIGH;
  GPIO_InitStruct.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  GPIO_InitStruct.Pull = LL_GPIO_PULL_DOWN;
  GPIO_InitStruct.Alternate = LL_GPIO_AF_5;
  LL_GPIO_Init(GPIOA, &GPIO_InitStruct);

  /* SPI1 MISO/MOSI configuration */
  GPIO_InitStruct.Pin = (LL_GPIO_PIN_6 | LL_GPIO_PIN_7);
  GPIO_InitStruct.Mode = LL_GPIO_MODE_ALTERNATE;
  GPIO_InitStruct.Speed = LL_GPIO_SPEED_FREQ_HIGH;
  GPIO_InitStruct.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  GPIO_InitStruct.Pull = LL_GPIO_PULL_NO;
  GPIO_InitStruct.Alternate = LL_GPIO_AF_5;
  LL_GPIO_Init(GPIOA, &GPIO_InitStruct);

  /* DMA configuration */
  /* Set the DMA Remap */
  LL_DMA_SetDstPeriphMap(DMA1, LL_DMA_CHANNEL_1, LL_DMA_DST_PERIPH_MAP_1);
  LL_SYSCFG_SetDMARemap(DMA1, LL_SYSCFG_DMA_PERIPH_MAP_1, LL_SYSCFG_DMA_MAP_SPI1_TX);

  LL_DMA_SetSrcPeriphMap(DMA1, LL_DMA_CHANNEL_2, LL_DMA_SRC_PERIPH_MAP_2);
  LL_SYSCFG_SetDMARemap(DMA1, LL_SYSCFG_DMA_PERIPH_MAP_2, LL_SYSCFG_DMA_MAP_SPI1_RX);
  
  /* Initialize DMA Channel 1 */
  DMA_InitStruct.Direction         = LL_DMA_DIRECTION_MEMORY_TO_PERIPH;
  DMA_InitStruct.DstIncMode        = LL_DMA_DST_ADDR_FIX;
  DMA_InitStruct.SrcIncMode        = LL_DMA_SRC_ADDR_INC;
  DMA_InitStruct.DstWidth          = LL_DMA_DST_WIDTH_BYTE;
  DMA_InitStruct.SrcWidth          = LL_DMA_SRC_WIDTH_BYTE;
  DMA_InitStruct.Priority          = LL_DMA_PRIORITY_0;
  DMA_InitStruct.DstBurstLen       = LL_DMA_DST_BURST_LEN_1;
  DMA_InitStruct.DstHandshakeType  = LL_DMA_DST_HANDSHAKE_TYPE_HARD;
  LL_DMA_Init(DMA1, LL_DMA_CHANNEL_1, &DMA_InitStruct);
  
  /* Initialize DMA Channel 2 */
  DMA_InitStruct.Direction         = LL_DMA_DIRECTION_PERIPH_TO_MEMORY;
  DMA_InitStruct.DstIncMode        = LL_DMA_DST_ADDR_INC;
  DMA_InitStruct.SrcIncMode        = LL_DMA_SRC_ADDR_FIX;
  DMA_InitStruct.DstWidth          = LL_DMA_DST_WIDTH_BYTE;
  DMA_InitStruct.SrcWidth          = LL_DMA_SRC_WIDTH_BYTE;
  DMA_InitStruct.Priority          = LL_DMA_PRIORITY_1;
  DMA_InitStruct.SrcBurstLen       = LL_DMA_SRC_BURST_LEN_1;
  DMA_InitStruct.SrcHandshakeType  = LL_DMA_SRC_HANDSHAKE_TYPE_HARD;
  LL_DMA_Init(DMA1, LL_DMA_CHANNEL_2, &DMA_InitStruct);
  
  /* Enable DMA interrupt in NVIC */
  NVIC_SetPriority(DMA1_Channel1_IRQn, 1);
  NVIC_EnableIRQ(DMA1_Channel1_IRQn);
  
  NVIC_SetPriority(DMA1_Channel2_IRQn, 1);
  NVIC_EnableIRQ(DMA1_Channel2_IRQn);

  /* SPI1 parameter configuration*/
  SPI_InitStruct.TransferDirection = LL_SPI_FULL_DUPLEX;
#ifdef BOARD_IN_MASTER_MODE
  SPI_InitStruct.Mode              = LL_SPI_MODE_MASTER;
#else
  SPI_InitStruct.Mode              = LL_SPI_MODE_SLAVE;
#endif
  SPI_InitStruct.DataWidth         = LL_SPI_DATAWIDTH_8BIT;
  SPI_InitStruct.ClockPolarity     = LL_SPI_POLARITY_LOW;
  SPI_InitStruct.ClockPhase        = LL_SPI_PHASE_1EDGE;
  SPI_InitStruct.NSS               = LL_SPI_NSS_SOFT;
  SPI_InitStruct.BaudRate          = LL_SPI_BAUDRATEPRESCALER_DIV256;
  SPI_InitStruct.BitOrder          = LL_SPI_MSB_FIRST;
  SPI_InitStruct.CRCCalculation    = LL_SPI_CRCCALCULATION_DISABLE;
  SPI_InitStruct.CRCPoly           = 0x0U;
  LL_SPI_Init(SPI1, &SPI_InitStruct);
}

/**
  * @brief  SPI1 transmit and receive function
  * @param  pTxData：Pointer to Send data buffer
  * @param  pRxData：Pointer to receive data buffer
  * @param  Size：Size of the sent and received data
  * @retval None
  */
static void APP_SpiTransmitReceive_DMA(uint8_t *pTxData, uint8_t *pRxData, uint16_t Size)
{
  /* Sending and receiving data is not completed */
  txRxDataComplteFlag = RESET;
  
  /* Disable SPI */
  LL_SPI_Disable(SPI1);
  
  /* Enable the DMA receive interrupt of SPI */
  LL_DMA_DisableChannel(DMA1, LL_DMA_CHANNEL_2);
  
  LL_DMA_ClearFlag_TFR2(DMA1);
  LL_DMA_SetBlockLength(DMA1, LL_DMA_CHANNEL_2, Size);
  uint32_t dataRegAddr = LL_SPI_DMA_GetRegAddr(SPI1);
  LL_DMA_ConfigAddresses(DMA1, LL_DMA_CHANNEL_2, dataRegAddr, (uint32_t)pRxData);
  
  LL_DMA_DisableIT_HALFBLOCK(DMA1, LL_DMA_CHANNEL_2);
  LL_DMA_EnableIT_TFR(DMA1, LL_DMA_CHANNEL_2);
  LL_DMA_EnableIT_ERR(DMA1, LL_DMA_CHANNEL_2);
  
  LL_DMA_Enable(DMA1);
  LL_DMA_EnableChannel(DMA1, LL_DMA_CHANNEL_2);
  
  /* Enable DMA Rx reuqest */
  LL_SPI_EnableDMAReq_RX(SPI1);
  
  /* Enable the DMA transmit interrupt of SPI */
  LL_DMA_DisableChannel(DMA1, LL_DMA_CHANNEL_1);
  
  LL_DMA_ClearFlag_TFR1(DMA1);
  LL_DMA_SetBlockLength(DMA1, LL_DMA_CHANNEL_1, Size);
  dataRegAddr = LL_SPI_DMA_GetRegAddr(SPI1);
  LL_DMA_ConfigAddresses(DMA1, LL_DMA_CHANNEL_1, (uint32_t)pTxData, dataRegAddr);
  
  LL_DMA_DisableIT_HALFBLOCK(DMA1, LL_DMA_CHANNEL_1);
  LL_DMA_EnableIT_TFR(DMA1, LL_DMA_CHANNEL_1);
  LL_DMA_EnableIT_ERR(DMA1, LL_DMA_CHANNEL_1);
  
  LL_DMA_Enable(DMA1);
  LL_DMA_EnableChannel(DMA1, LL_DMA_CHANNEL_1);
  
  /* Enable SPI */
  LL_SPI_Enable(SPI1);
  
  /* Enable DMA Tx request */
  LL_SPI_EnableDMAReq_TX(SPI1);
}

/**
  * @brief  SPI DMA Tx callback
  * @param  None
  * @retval None
  */
void APP_DmaSpiTxIRQCallback(void)
{
    if ((LL_DMA_IsActiveFlag_TFR1(DMA1) == 1) && (LL_DMA_IsEnabledIT_TFR(DMA1, LL_DMA_CHANNEL_1) == 1))
  {
    /* Disable transfer complete interrupt */
    LL_DMA_DisableIT_TFR(DMA1, LL_DMA_CHANNEL_1);
    LL_DMA_DisableIT_ERR(DMA1, LL_DMA_CHANNEL_1);

    /* clear transfer complete flag */
    LL_DMA_ClearFlag_TFR1(DMA1);
  }
}

/**
  * @brief  SPI DMA Rx callback
  * @param  None
  * @retval None
  */
void APP_DmaSpiRxIRQCallback(void)
{
    if ((LL_DMA_IsActiveFlag_TFR2(DMA1) == 1) && (LL_DMA_IsEnabledIT_TFR(DMA1, LL_DMA_CHANNEL_2) == 1))
  {
    /* Disable transfer complete interrupt */
    LL_DMA_DisableIT_TFR(DMA1, LL_DMA_CHANNEL_2);
    LL_DMA_DisableIT_ERR(DMA1, LL_DMA_CHANNEL_2);
    
    /* clear transfer complete flag */
    LL_DMA_ClearFlag_TFR2(DMA1);
    
    /* Wait for transfer to complete */
    while(LL_SPI_GetTxFIFOLevel(SPI1) != LL_SPI_TX_FIFO_EMPTY);
    while(LL_SPI_IsActiveFlag_BSY(SPI1) != 0);
    while(LL_SPI_GetRxFIFOLevel(SPI1) != LL_SPI_RX_FIFO_EMPTY);
    
    /* Disable DMA Tx and Rx request */
    LL_SPI_DisableDMAReq_TX(SPI1);
    LL_SPI_DisableDMAReq_RX(SPI1);
    
    /* Set the flag to indicate that tranfer and receive data is completed */
    txRxDataComplteFlag = SET;
  }
}

/**
  * @brief  Wait transfer to be completed and check the received data
  * @param  None
  * @retval None
  */
static void APP_WaitAndCheckEndOfTransfer(void)
{
  /* 1 - Wait transfer to be completed  */
  while (txRxDataComplteFlag != SET)
  {
  }

  /* 2 - Compare sent and received data */
  if(APP_Buffercmp8((uint8_t*)aTxBuffer, (uint8_t*)aRxBuffer, ubNbDataToTransmit))
  {
    /* error handling */
    APP_LedBlinking();
  }
  else
  {
    /* If the data is received correctly,then the LED will be on */
    BSP_LED_On(LED_GREEN);
  }
}

/**
  * @brief  Character comparison function
  * @param  pBuffer1：Pointer to buffer 1 that to be compared
  * @param  pBuffer2：Pointer to buffer 2 that to be compared
  * @param  BufferLength：The number of characters to be compared
  * @retval 0: The comparison value is the same; 1: The comparison value is different
  */
static uint8_t APP_Buffercmp8(uint8_t* pBuffer1, uint8_t* pBuffer2, uint8_t BufferLength)
{
  while (BufferLength--)
  {
    if (*pBuffer1 != *pBuffer2)
    {
      return 1;
    }
    pBuffer1++;
    pBuffer2++;
  }

  return 0;
}

/**
  * @brief  LED blinking
  * @param  None
  * @retval None
  */
static void APP_LedBlinking(void)
{
  while (1)
  {
    BSP_LED_Toggle(LED_GREEN); 
    LL_mDelay(200);
  }
}

/**
  * @brief  Configure system clock
  * @param  None
  * @retval None
  */
static void APP_SystemClockConfig(void)
{
  /* Enable HSI */
  LL_RCC_HSI_Enable();
  while(LL_RCC_HSI_IsReady() != 1)
  {
  }

  /* Set AHB prescaler: HCLK = SYSCLK */
  LL_RCC_SetAHBPrescaler(LL_RCC_SYSCLK_DIV_1);

  /* Select HSI as system clock source */
  LL_RCC_SetSysClkSource(LL_RCC_SYS_CLKSOURCE_HSI);
  while(LL_RCC_GetSysClkSource() != LL_RCC_SYS_CLKSOURCE_STATUS_HSI)
  {
  }

  /* Set APB1 & APB2 prescaler: PCLK1 = HCLK, PCLK2 = 1/2*HCLK */
  LL_RCC_SetAPB1Prescaler(LL_RCC_APB1_DIV_1);
  LL_RCC_SetAPB2Prescaler(LL_RCC_APB2_DIV_2);
  
  /* Set systick to 1ms in using frequency set to 16MHz */
  LL_Init1msTick(16000000);

  /* Update the SystemCoreClock global variable(which can be updated also through SystemCoreClockUpdate function) */
  LL_SetSystemCoreClock(16000000);
}

/**
  * @brief  Error handling function
  * @param  None
  * @retval None
  */
void APP_ErrorHandler(void)
{
  /* Infinite loop */
  while (1)
  {
  }
}

#ifdef  USE_FULL_ASSERT
/**
  * @brief  Reports the name of the source file and the source line number
  *         where the assert_param error has occurred.
* @param  file : Pointer to the source file name
* @param  line : assert_param error line source number
  * @retval None
  */
void assert_failed(uint8_t *file, uint32_t line)
{
  /* User can add His own implementation to report the file name and line number,
     ex: printf("Wrong parameters value: file %s on line %d\r\n", file, line) */
  /* Infinite loop */
  while (1)
  {
  }
}
#endif /* USE_FULL_ASSERT */

/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
