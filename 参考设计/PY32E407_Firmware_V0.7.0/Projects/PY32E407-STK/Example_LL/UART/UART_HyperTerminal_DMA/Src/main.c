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
uint8_t aTxStartMessage[] = "\n\r UART Hyperterminal communication based on DMA\n\r Enter 12 characters using keyboard :\n\r";
uint8_t aTxEndMessage[] = "\n\r Example Finished\n\r";
__IO ITStatus UartReady = RESET;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_ConfigUart(void);
static void APP_ConfigDma(void);
static void APP_UartTransmit_DMA(UART_TypeDef *UARTx, uint8_t *pData, uint16_t Size);
static void APP_UartReceive_DMA(UART_TypeDef *UARTx, uint8_t *pData, uint16_t Size);
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

  /* UART configuration */
  APP_ConfigUart();

 /* Start the transmission process */
  APP_UartTransmit_DMA(UART2, (uint8_t*)aTxStartMessage, TXSTARTMESSAGESIZE);
  APP_WaitToReady();
  
  /* Put UART peripheral in reception process */
  APP_UartReceive_DMA(UART2, (uint8_t *)aRxBuffer, 12);
  APP_WaitToReady();

  /* Send the received Buffer */
  APP_UartTransmit_DMA(UART2, (uint8_t*)aRxBuffer, 12);
  APP_WaitToReady();
  
  /* Send the End Message */
  APP_UartTransmit_DMA(UART2, (uint8_t*)aTxEndMessage, TXENDMESSAGESIZE);
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
  while (UartReady != SET);
  
  UartReady = RESET;
}

/**
  * @brief  UART2 configuration function
  * @param  None
  * @retval None
  */
static void APP_ConfigUart(void)
{
  LL_GPIO_InitTypeDef GPIO_InitStruct = {0};
  LL_UART_InitTypeDef UART_InitStruct = {0};

  /* Enable GPIOC clock */
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOC);
  /* Enable GPIOD clock */
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOD);
  /* Enable UART2 clock */
  LL_APB1_GRP1_EnableClock(LL_APB1_GRP1_PERIPH_UART2);

  /**UART2 GPIO Configuration
  PC12    ------> UART2_TX
  PD2     ------> UART2_RX
  */
  /* Config Tx Pin */
  GPIO_InitStruct.Pin        = LL_GPIO_PIN_12;
  /* Select alternate function mode */
  GPIO_InitStruct.Mode       = LL_GPIO_MODE_ALTERNATE;
  /* Set output speed */
  GPIO_InitStruct.Speed      = LL_GPIO_SPEED_FREQ_VERY_HIGH;
  /* Set output type to push pull */
  GPIO_InitStruct.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  /* Enable pull up */
  GPIO_InitStruct.Pull       = LL_GPIO_PULL_UP;
  /* Set alternate function to UART2 function  */
  GPIO_InitStruct.Alternate  = LL_GPIO_AF_5;
  /* Initialize GPIOC */
  LL_GPIO_Init(GPIOC, &GPIO_InitStruct);
  
  /* Config Rx Pin */
  GPIO_InitStruct.Pin        = LL_GPIO_PIN_2;
  /* Initialize GPIOD */
  LL_GPIO_Init(GPIOD, &GPIO_InitStruct);

  /* Set UART2 interrupt priority  */
  NVIC_SetPriority(UART2_IRQn, 0);
  /* Enable UART2 interrupt request */
  NVIC_EnableIRQ(UART2_IRQn);

  /* Configure DMA */
  APP_ConfigDma();

  /* Set UART feature */
  /* Set baud rate */
  UART_InitStruct.BaudRate  = 115200;
  /* set word length to 8 bits: Start bit, 8 data bits, n stop bits */
  UART_InitStruct.DataWidth = LL_UART_DATAWIDTH_8B;
  /* 1 stop bit */
  UART_InitStruct.StopBits  = LL_UART_STOPBITS_1;
  /* Parity control disabled  */
  UART_InitStruct.Parity    = LL_UART_PARITY_NONE;
  /* MSB first disable */
  UART_InitStruct.BitOrder  = LL_UART_BITORDER_LSBFIRST;

  /* Initialize UART */
  LL_UART_Init(UART2, &UART_InitStruct);
}

/**
  * @brief  DMA configuration function
  * @param  None
  * @retval None
  */
static void APP_ConfigDma(void)
{
  /* Enable DMA、syscfg clock */
  LL_AHB1_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_DMA1);

  /* Configure DMA channel LL_DMA_CHANNEL_1 for transmission */
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

  /* Map the DMA request to UART2 */
  /* UART2_TX Corresponding channel LL_DMA_CHANNEL_1 */
  LL_DMA_SetDstPeriphMap(DMA1, LL_DMA_CHANNEL_1, LL_DMA_DST_PERIPH_MAP_1);
  LL_SYSCFG_SetDMARemap(DMA1, LL_SYSCFG_DMA_PERIPH_MAP_1, LL_SYSCFG_DMA_MAP_UART2_TX);

  /* UART2_RX Corresponding channel LL_DMA_CHANNEL_2 */
  LL_DMA_SetSrcPeriphMap(DMA1, LL_DMA_CHANNEL_2, LL_DMA_SRC_PERIPH_MAP_2);
  LL_SYSCFG_SetDMARemap(DMA1, LL_SYSCFG_DMA_PERIPH_MAP_2, LL_SYSCFG_DMA_MAP_UART2_RX);
  
  /* Set interrupt priority */
  NVIC_SetPriority(DMA1_Channel1_IRQn, 1);
   /* Enable interrupt */
  NVIC_EnableIRQ(DMA1_Channel1_IRQn);
  
  /* Set interrupt priority */
  NVIC_SetPriority(DMA1_Channel2_IRQn, 1);
  /* Enable interrupt */
  NVIC_EnableIRQ(DMA1_Channel2_IRQn);
}

/**
  * @brief  UART transmit function
  * @param  UARTx：UART module
  * @param  pData：transmit buffer
  * @param  Size：Size of the transmit buffer
  * @retval None
  */
static void APP_UartTransmit_DMA(UART_TypeDef *UARTx, uint8_t *pData, uint16_t Size)
{  
  /* Configure DMA channel 1 */
  LL_DMA_DisableChannel(DMA1, LL_DMA_CHANNEL_1);
  
  LL_DMA_ClearFlag_TFR1(DMA1);
  uint32_t *temp = (uint32_t *)&pData;
  LL_DMA_SetSrcAddress(DMA1, LL_DMA_CHANNEL_1, *(uint32_t *)temp);
  LL_DMA_SetDstAddress(DMA1, LL_DMA_CHANNEL_1, (uint32_t)&(UARTx->DR));
  LL_DMA_SetBlockLength(DMA1, LL_DMA_CHANNEL_1, Size);
  
  LL_DMA_EnableIT_TFR(DMA1, LL_DMA_CHANNEL_1);

  LL_DMA_Enable(DMA1);
  LL_DMA_EnableChannel(DMA1, LL_DMA_CHANNEL_1);

  /* Enable UART DMA channel for transmission */
  LL_UART_EnableDMAReq_TX(UARTx);
}

/**
  * @brief  UART receive function
  * @param  UARTx：UART module
  * @param  pData：receive buffer
  * @param  Size：Size of the receive buffer
  * @retval None
  */
static void APP_UartReceive_DMA(UART_TypeDef *UARTx, uint8_t *pData, uint16_t Size)
{
  /*Configure DMA channel 2*/
  LL_DMA_DisableChannel(DMA1, LL_DMA_CHANNEL_2);
  
  LL_DMA_ClearFlag_TFR2(DMA1);
  uint32_t *temp = (uint32_t *)&pData;
  LL_DMA_SetDstAddress(DMA1, LL_DMA_CHANNEL_2, *(uint32_t *)temp);
  LL_DMA_SetSrcAddress(DMA1, LL_DMA_CHANNEL_2, (uint32_t)&UARTx->DR);
  LL_DMA_SetBlockLength(DMA1, LL_DMA_CHANNEL_2, Size);

  LL_DMA_EnableIT_TFR(DMA1, LL_DMA_CHANNEL_2);

  LL_DMA_Enable(DMA1);
  LL_DMA_EnableChannel(DMA1, LL_DMA_CHANNEL_2);
  
  LL_UART_ClearFlag_ORE(UARTx);
  
  /* Enable UART DMA channel for transmission */
  LL_UART_EnableDMAReq_RX(UARTx);
}

/**
  * @brief  UART interrupt callback function
  * @param  UARTx：UART module
  * @retval None
  */
void APP_UartIRQCallback(UART_TypeDef *UARTx)
{
  /* Transmission complete */
  if ((LL_UART_IsActiveFlag_TDRE(UARTx) != RESET) && (LL_UART_IsEnabledIT_TDRE(UARTx) != RESET))
  {
    LL_UART_DisableIT_TDRE(UARTx);
    UartReady = SET;
  
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
    LL_UART_DisableDMAReq_TX(UART2);
    LL_UART_EnableIT_TDRE(UART2);
  }
}

/**
  * @brief  DMA channel 2 interrupt interrupt callback function for reception
  * @param  None
  * @retval None
  */
void APP_DmaChannel2_IRQCallback(void)
{
  if ((LL_DMA_IsActiveFlag_TFR2(DMA1) == 1) && (LL_DMA_IsEnabledIT_TFR(DMA1, LL_DMA_CHANNEL_2) == 1))
  {
    /* Disable transfer complete interrupt */
    LL_DMA_DisableIT_TFR(DMA1, LL_DMA_CHANNEL_2);
    LL_UART_DisableDMAReq_RX(UART2);
    UartReady = SET;
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
