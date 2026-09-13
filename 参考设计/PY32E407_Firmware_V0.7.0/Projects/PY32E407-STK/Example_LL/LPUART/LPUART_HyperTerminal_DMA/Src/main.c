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
#define COUNTOF(__BUFFER__)   (sizeof(__BUFFER__) / sizeof(*(__BUFFER__)))
#define TXSTARTMESSAGESIZE    (COUNTOF(aTxStartMessage) - 1)
#define TXENDMESSAGESIZE      (COUNTOF(aTxEndMessage) - 1)

/* Private variables ---------------------------------------------------------*/
uint8_t aRxBuffer[12] = {0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff};
uint8_t aTxStartMessage[] = "\n\r LPUART Hyperterminal communication based on DMA\n\r Enter 12 characters using keyboard :\n\r";
uint8_t aTxEndMessage[] = "\n\r Example Finished\n\r";
__IO ITStatus LpuartReady = RESET;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_ConfigLpuart(void);
static void APP_ConfigDma(void);
static void APP_LpuartTransmit_DMA(LPUART_TypeDef *LPLPUARTx, uint8_t *pData, uint16_t Size);
static void APP_LpuartReceive_DMA(LPUART_TypeDef *LPLPUARTx, uint8_t *pData, uint16_t Size);
static void APP_WaitToReady(void);

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

  /* LPUART configuration */
  APP_ConfigLpuart();

 /* Start the transmission process */
  APP_LpuartTransmit_DMA(LPUART, (uint8_t*)aTxStartMessage, TXSTARTMESSAGESIZE);
  APP_WaitToReady();
  
  /* Put LPUART peripheral in reception process */
  APP_LpuartReceive_DMA(LPUART, (uint8_t *)aRxBuffer, 12);
  APP_WaitToReady();

  /* Send the received Buffer */
  APP_LpuartTransmit_DMA(LPUART, (uint8_t*)aRxBuffer, 12);
  APP_WaitToReady();
  
  /* Send the End Message */
  APP_LpuartTransmit_DMA(LPUART, (uint8_t*)aTxEndMessage, TXENDMESSAGESIZE);
  APP_WaitToReady();

  /* Turn on LED if test passes then enter infinite loop */
  BSP_LED_On(LED_GREEN);

  /* Infinite loop */
  while (1)
  {
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
  * @brief  Wait transfer complete
  * @param  None
  * @retval None
  */
static void APP_WaitToReady(void)
{
  while (LpuartReady != SET);
  
  LpuartReady = RESET;
}

/**
  * @brief  LPUART1 configuration function
  * @param  None
  * @retval None
  */
static void APP_ConfigLpuart(void)
{
  LL_GPIO_InitTypeDef GPIO_InitStruct = {0};
  LL_LPUART_InitTypeDef LPUART_InitStruct = {0};

  /* Enable GPIOC clock */
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOC);
  /* Enable LPUART1 clock */
  LL_APB1_GRP2_EnableClock(LL_APB1_GRP2_PERIPH_LPUART1);
  
  /**LPUART1 GPIO Configuration
  PC0     ------> LPUART_RX
  PC1     ------> LPUART_TX
  */
  /* Config Tx, Rx Pin */
  GPIO_InitStruct.Pin        = (LL_GPIO_PIN_0 | LL_GPIO_PIN_1);
  /* Select alternate function mode */
  GPIO_InitStruct.Mode       = LL_GPIO_MODE_ALTERNATE;
  /* Set output speed */
  GPIO_InitStruct.Speed      = LL_GPIO_SPEED_FREQ_VERY_HIGH;
  /* Set output type to push pull */
  GPIO_InitStruct.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  /* Enable pull up */
  GPIO_InitStruct.Pull       = LL_GPIO_PULL_UP;
  /* Set alternate function to LPUART1 function  */
  GPIO_InitStruct.Alternate  = LL_GPIO_AF_8;
  /* Initialize GPIOC */
  LL_GPIO_Init(GPIOC,&GPIO_InitStruct);

  /* Set LPUART1 interrupt priority  */
  NVIC_SetPriority(LPUART_IRQn, 1);
  /* Enable LPUART1 interrupt request */
  NVIC_EnableIRQ(LPUART_IRQn);

  /* Configure DMA */
  APP_ConfigDma();

  /* Set LPUART feature */
  LPUART_InitStruct.PrescalerValue      = LL_LPUART_PRESCALER_DIV1;
  /* Set baud rate */
  LPUART_InitStruct.BaudRate            = 9600;
  /* set word length to 8 bits: Start bit, 8 data bits, n stop bits */
  LPUART_InitStruct.DataWidth           = LL_LPUART_DATAWIDTH_8B;
  /* 1 stop bit */
  LPUART_InitStruct.StopBits            = LL_LPUART_STOPBITS_1;
  /* Parity control disabled  */
  LPUART_InitStruct.Parity              = LL_LPUART_PARITY_NONE;
  /* Set transmit and receive direction */
  LPUART_InitStruct.TransferDirection   = LL_LPUART_DIRECTION_TX_RX;
  LPUART_InitStruct.HardwareFlowControl = LL_LPUART_HWCONTROL_NONE;

  /* Initialize LPUART */
  LL_LPUART_Init(LPUART, &LPUART_InitStruct);
  LL_LPUART_Enable(LPUART);
}

/**
  * @brief  DMA configuration function
  * @param  None
  * @retval None
  */
static void APP_ConfigDma(void)
{
  /* Enable DMA clock */
  LL_AHB1_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_DMA1);

  /* Use DMA channel LL_DMA_CHANNEL_1 for transmission */
  LL_DMA_ConfigTransfer(DMA1, LL_DMA_CHANNEL_1, LL_DMA_DIRECTION_MEMORY_TO_PERIPH | \
                      LL_DMA_MODE_NORMAL        | \
                      LL_DMA_DST_ADDR_FIX       | \
                      LL_DMA_SRC_ADDR_INC       | \
                      LL_DMA_DST_WIDTH_BYTE     | \
                      LL_DMA_SRC_WIDTH_BYTE);
  
  LL_DMA_SetDstBurstLen(DMA1, LL_DMA_CHANNEL_1, LL_DMA_DST_BURST_LEN_1);
  LL_DMA_SetDstHandshakeType(DMA1, LL_DMA_CHANNEL_1, LL_DMA_DST_HANDSHAKE_TYPE_HARD);
  LL_DMA_SetChannelPriorityLevel(DMA1, LL_DMA_CHANNEL_1, LL_DMA_PRIORITY_0);

  /* Configure DMA channel LL_DMA_CHANNEL_2 for reception */
  LL_DMA_ConfigTransfer(DMA1, LL_DMA_CHANNEL_2, LL_DMA_DIRECTION_PERIPH_TO_MEMORY | \
                      LL_DMA_MODE_NORMAL        | \
                      LL_DMA_DST_ADDR_INC       | \
                      LL_DMA_SRC_ADDR_FIX       | \
                      LL_DMA_DST_WIDTH_BYTE     | \
                      LL_DMA_SRC_WIDTH_BYTE);
  
  LL_DMA_SetSrcBurstLen(DMA1, LL_DMA_CHANNEL_2, LL_DMA_SRC_BURST_LEN_1);
  LL_DMA_SetSrcHandshakeType(DMA1, LL_DMA_CHANNEL_2, LL_DMA_SRC_HANDSHAKE_TYPE_HARD);
  LL_DMA_SetChannelPriorityLevel(DMA1, LL_DMA_CHANNEL_2, LL_DMA_PRIORITY_1);

  /* Map the DMA request to LPUART1 */
  /* LPUART1_TX Corresponding channel LL_DMA_CHANNEL_1 */
  LL_DMA_SetDstPeriphMap(DMA1, LL_DMA_CHANNEL_1, LL_DMA_DST_PERIPH_MAP_1);
  LL_SYSCFG_SetDMARemap(DMA1, LL_SYSCFG_DMA_PERIPH_MAP_1, LL_SYSCFG_DMA_MAP_LPUART1_TX);

  /* LPUART1_RX Corresponding channel LL_DMA_CHANNEL_2 */
  LL_DMA_SetSrcPeriphMap(DMA1, LL_DMA_CHANNEL_2, LL_DMA_SRC_PERIPH_MAP_2);
  LL_SYSCFG_SetDMARemap(DMA1, LL_SYSCFG_DMA_PERIPH_MAP_2, LL_SYSCFG_DMA_MAP_LPUART1_RX);
  
  /* Set interrupt priority */
  NVIC_SetPriority(DMA1_Channel1_IRQn, 2);
   /* Enable interrupt */
  NVIC_EnableIRQ(DMA1_Channel1_IRQn);
  
  /* Set interrupt priority */
  NVIC_SetPriority(DMA1_Channel2_IRQn, 2);
  /* Enable interrupt */
  NVIC_EnableIRQ(DMA1_Channel2_IRQn);
}

/**
  * @brief  LPUART transmit function
  * @param  LPUARTx：LPUART module
  * @param  pData：transmit buffer
  * @param  Size：Size of the transmit buffer
  * @retval None
  */
static void APP_LpuartTransmit_DMA(LPUART_TypeDef *LPUARTx, uint8_t *pData, uint16_t Size)
{
  /* Configure DMA channel 1 */
  LL_DMA_DisableChannel(DMA1, LL_DMA_CHANNEL_1);
  
  LL_DMA_ClearFlag_TFR1(DMA1);
  uint32_t *temp = (uint32_t *)&pData;
  LL_DMA_SetSrcAddress(DMA1, LL_DMA_CHANNEL_1, *(uint32_t *)temp);
  LL_DMA_SetDstAddress(DMA1, LL_DMA_CHANNEL_1, (uint32_t)&(LPUARTx->TDR));
  LL_DMA_SetBlockLength(DMA1, LL_DMA_CHANNEL_1, Size);
  
  LL_DMA_EnableIT_TFR(DMA1, LL_DMA_CHANNEL_1);

  LL_DMA_Enable(DMA1);
  LL_DMA_EnableChannel(DMA1, LL_DMA_CHANNEL_1);

  LL_LPUART_ClearFlag_TC(LPUARTx);
  /* Enable LPUART DMA channel for transmission */
  LL_LPUART_EnableDMAReq_TX(LPUARTx);
}

/**
  * @brief  LPUART receive function
  * @param  LPUARTx：LPUART module
  * @param  pData：receive buffer
  * @param  Size：Size of the receive buffer
  * @retval None
  */
static void APP_LpuartReceive_DMA(LPUART_TypeDef *LPUARTx, uint8_t *pData, uint16_t Size)
{
  /*Configure DMA channel 2*/
  LL_DMA_DisableChannel(DMA1, LL_DMA_CHANNEL_2);
  
  LL_DMA_ClearFlag_TFR2(DMA1);
  uint32_t *temp = (uint32_t *)&pData;
  LL_DMA_SetDstAddress(DMA1, LL_DMA_CHANNEL_2, *(uint32_t *)temp);
  LL_DMA_SetSrcAddress(DMA1, LL_DMA_CHANNEL_2, (uint32_t)&LPUARTx->RDR);
  LL_DMA_SetBlockLength(DMA1, LL_DMA_CHANNEL_2, Size);

  LL_DMA_EnableIT_TFR(DMA1, LL_DMA_CHANNEL_2);

  LL_DMA_Enable(DMA1);
  LL_DMA_EnableChannel(DMA1, LL_DMA_CHANNEL_2);
  
  LL_LPUART_ClearFlag_ORE(LPUARTx);
  
  /* Enable LPUART DMA channel for transmission */
  LL_LPUART_EnableDMAReq_RX(LPUARTx);
}

/**
  * @brief  LPUART interrupt callback function
  * @param  LPUARTx：LPUART module
  * @retval None
  */
void APP_LpuartIRQCallback(LPUART_TypeDef *LPUARTx)
{
  /* Transmission complete */
  if ((LL_LPUART_IsActiveFlag_TC(LPUARTx) != RESET) && (LL_LPUART_IsEnabledIT_TC(LPUARTx) != RESET))
  {
    LL_LPUART_DisableIT_TC(LPUARTx);
    LpuartReady = SET;
  
    return;
  }
}

/**
  * @brief  DMA channel 1 interrupt callback function for transmission
  * @param  None
  * @retval None
  */
void APP_DmaChannel1_IRQCallback(void)
{
  if ((LL_DMA_IsActiveFlag_TFR1(DMA1) == 1) && (LL_DMA_IsEnabledIT_TFR(DMA1, LL_DMA_CHANNEL_1) == 1))
  {
    /* Disable transfer complete interrupt */
    LL_DMA_DisableIT_TFR(DMA1, LL_DMA_CHANNEL_1);
    LL_LPUART_DisableDMAReq_TX(LPUART);
    LL_LPUART_EnableIT_TC(LPUART);
  }
}

/**
  * @brief  DMA channel 2 interrupt callback function for reception
  * @param  None
  * @retval None
  */
void APP_DmaChannel2_IRQCallback(void)
{
  if ((LL_DMA_IsActiveFlag_TFR2(DMA1) == 1) && (LL_DMA_IsEnabledIT_TFR(DMA1, LL_DMA_CHANNEL_2) == 1))
  {
    /* Disable transfer complete interrupt */
    LL_DMA_DisableIT_TFR(DMA1, LL_DMA_CHANNEL_2);
    LL_LPUART_DisableDMAReq_RX(LPUART);
    LpuartReady = SET;
  }
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
