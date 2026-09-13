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
/* Private variables ---------------------------------------------------------*/
#define COUNTOF(__BUFFER__)   (sizeof(__BUFFER__) / sizeof(*(__BUFFER__)))
#define TXSTARTMESSAGESIZE    (COUNTOF(aTxStartMessage) - 1)
#define TXENDMESSAGESIZE      (COUNTOF(aTxEndMessage) - 1)

/* Private user code ---------------------------------------------------------*/
uint8_t aRxBuffer[12] = {0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff};
uint8_t aTxStartMessage[] = "\n\r SCI Hyperterminal communication based on DMA\n\r Enter 12 characters using keyboard :\n\r";
uint8_t aTxEndMessage[] = "\n\r Example Finished\n\r";
__IO ITStatus SciReady = RESET;

/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_ConfigSci(void);
static void APP_ConfigDma(void);
static void APP_SciTransmit_DMA(USART_TypeDef *USARTx, uint8_t *pData, uint16_t Size);
static void APP_SciReceive_DMA(USART_TypeDef *USARTx, uint8_t *pData, uint16_t Size);
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

  /* USART1 configuration */
  APP_ConfigSci();

 /* Start the transmission process */
  APP_SciTransmit_DMA(USART1, (uint8_t*)aTxStartMessage, TXSTARTMESSAGESIZE);
  APP_WaitToReady();
  
  /* Put USART peripheral in reception process */
  APP_SciReceive_DMA(USART1, (uint8_t *)aRxBuffer, 12);
  APP_WaitToReady();

  /* Send the received Buffer */
  APP_SciTransmit_DMA(USART1, (uint8_t*)aRxBuffer, 12);
  APP_WaitToReady();
  
  /* Send the End Message */
  APP_SciTransmit_DMA(USART1, (uint8_t*)aTxEndMessage, TXENDMESSAGESIZE);
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
  LL_RCC_SetAPB2Prescaler(LL_RCC_APB2_DIV_1);
  
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
  while (SciReady != SET);
  
  SciReady = RESET;
}

/**
  * @brief  USART1 configuration function
  * @param  None
  * @retval None
  */
static void APP_ConfigSci(void)
{
  LL_GPIO_InitTypeDef GPIO_InitStruct = {0};
  LL_SCI_InitTypeDef SCI_InitStruct = {0};

  /* Enable GPIOA clock */
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOA);
  /* Enable USART1 clock */
  LL_APB2_GRP1_EnableClock(LL_APB2_GRP1_PERIPH_USART1);
      
  /**USART1 GPIO Configuration
  PA09    ------> USART1_TX
  PA10    ------> USART1_RX
  */
  /* Config Tx, Rx Pin */
  GPIO_InitStruct.Pin        = (LL_GPIO_PIN_9 | LL_GPIO_PIN_10);
  /* Select alternate function mode */
  GPIO_InitStruct.Mode       = LL_GPIO_MODE_ALTERNATE;
  /* Set output speed */
  GPIO_InitStruct.Speed      = LL_GPIO_SPEED_FREQ_VERY_HIGH;
  /* Set output type to push pull */
  GPIO_InitStruct.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  /* Enable pull up */
  GPIO_InitStruct.Pull       = LL_GPIO_PULL_UP;
  /* Set alternate function to USART1 function  */
  GPIO_InitStruct.Alternate  = LL_GPIO_AF_7;
  /* Initialize GPIOA */
  LL_GPIO_Init(GPIOA,&GPIO_InitStruct);

  /* Set USART1 interrupt priority  */
  NVIC_SetPriority(USART1_IRQn, 1);
  /* Enable USART1 interrupt request */
  NVIC_EnableIRQ(USART1_IRQn);

  /* Configure DMA */
  APP_ConfigDma();

  /* Set USART1 feature */
  /* Set baud rate */
  SCI_InitStruct.BaudRate            = 115200;
  /* set word length to 8 bits: Start bit, 8 data bits, n stop bits */
  SCI_InitStruct.DataWidth           = LL_USART_DATAWIDTH_8B;
  /* 1 stop bit */
  SCI_InitStruct.StopBits            = LL_USART_STOPBITS_1;
  /* Parity control disabled  */
  SCI_InitStruct.Parity              = LL_USART_PARITY_NONE;
  /* Set transmit and receive direction */
  SCI_InitStruct.TransferDirection   = LL_USART_DIRECTION_TX_RX;
  SCI_InitStruct.HardwareFlowControl = LL_USART_HWCONTROL_NONE;
  SCI_InitStruct.OverSampling        = LL_USART_OVERSAMPLING_16;

  /* Initialize SCI */
  LL_USART_Init(USART1, &SCI_InitStruct);
  /* Configure as full duplex asynchronous mode */
  LL_USART_ConfigAsyncMode(USART1);
  /* Enable USART */
  LL_USART_Enable(USART1);
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

  /* Map the DMA request to USART1 */
  /* USART1_TX Corresponding channel LL_DMA_CHANNEL_1 */
  LL_DMA_SetDstPeriphMap(DMA1, LL_DMA_CHANNEL_1, LL_DMA_DST_PERIPH_MAP_1);
  LL_SYSCFG_SetDMARemap(DMA1, LL_SYSCFG_DMA_PERIPH_MAP_1, LL_SYSCFG_DMA_MAP_USART1_TX);

  /* USART1_RX Corresponding channel LL_DMA_CHANNEL_2 */
  LL_DMA_SetSrcPeriphMap(DMA1, LL_DMA_CHANNEL_2, LL_DMA_SRC_PERIPH_MAP_2);
  LL_SYSCFG_SetDMARemap(DMA1, LL_SYSCFG_DMA_PERIPH_MAP_2, LL_SYSCFG_DMA_MAP_USART1_RX);
  
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
  * @brief  USART transmit function
  * @param  USARTx：USART module
  * @param  pData：transmit buffer
  * @param  Size：Size of the transmit buffer
  * @retval None
  */
static void APP_SciTransmit_DMA(USART_TypeDef *USARTx, uint8_t *pData, uint16_t Size)
{  
  /* Configure DMA channel 1 */
  LL_DMA_DisableChannel(DMA1, LL_DMA_CHANNEL_1);
  
  LL_DMA_ClearFlag_TFR1(DMA1);
  uint32_t *temp = (uint32_t *)&pData;
  LL_DMA_SetSrcAddress(DMA1, LL_DMA_CHANNEL_1, *(uint32_t *)temp);
  LL_DMA_SetDstAddress(DMA1, LL_DMA_CHANNEL_1, (uint32_t)&(USARTx->DR));
  LL_DMA_SetBlockLength(DMA1, LL_DMA_CHANNEL_1, Size);
  
  LL_DMA_EnableIT_TFR(DMA1, LL_DMA_CHANNEL_1);

  LL_DMA_Enable(DMA1);
  LL_DMA_EnableChannel(DMA1, LL_DMA_CHANNEL_1);

  /* Enable USART DMA channel for transmission */
  __disable_irq();
  LL_USART_DisableDirectionTx(USARTx);
  LL_USART_EnableDirectionTx(USARTx);
  LL_USART_EnableDMAReq_TX(USARTx);
  __enable_irq();
}

/**
  * @brief  USART receive function
  * @param  USARTx：USART module
  * @param  pData：receive buffer
  * @param  Size：Size of the receive buffer
  * @retval None
  */
static void APP_SciReceive_DMA(USART_TypeDef *USARTx, uint8_t *pData, uint16_t Size)
{
  /*Configure DMA channel 2*/
  LL_DMA_DisableChannel(DMA1, LL_DMA_CHANNEL_2);
  
  LL_DMA_ClearFlag_TFR2(DMA1);
  uint32_t *temp = (uint32_t *)&pData;
  LL_DMA_SetDstAddress(DMA1, LL_DMA_CHANNEL_2, *(uint32_t *)temp);
  LL_DMA_SetSrcAddress(DMA1, LL_DMA_CHANNEL_2, (uint32_t)&USARTx->DR);
  LL_DMA_SetBlockLength(DMA1, LL_DMA_CHANNEL_2, Size);

  LL_DMA_EnableIT_TFR(DMA1, LL_DMA_CHANNEL_2);

  LL_DMA_Enable(DMA1);
  LL_DMA_EnableChannel(DMA1, LL_DMA_CHANNEL_2);
  
  LL_USART_ClearFlag_ORE(USARTx);
  
  /* Enable USART DMA channel for transmission */
  LL_USART_EnableDMAReq_RX(USARTx);
}

/**
  * @brief  USART interrupt callback function
  * @param  USARTx：USART module
  * @retval None
  */
void APP_SciIRQCallback(USART_TypeDef *USARTx)
{
  /* Transmission complete */
  if ((LL_USART_IsActiveFlag_TC(USARTx) != RESET) && (LL_USART_IsEnabledIT_TC(USARTx) != RESET))
  {
    LL_USART_DisableIT_TC(USARTx);
    SciReady = SET;
  
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
    LL_USART_DisableDMAReq_TX(USART1);
    LL_USART_EnableIT_TC(USART1);
  }
}

/**
  * @brief  DMA interrupt callback function for reception
  * @param  None
  * @retval None
  */
void APP_DmaChannel2_IRQCallback(void)
{
  if ((LL_DMA_IsActiveFlag_TFR2(DMA1) == 1) && (LL_DMA_IsEnabledIT_TFR(DMA1, LL_DMA_CHANNEL_2) == 1))
  {
    /* Disable transfer complete interrupt */
    LL_DMA_DisableIT_TFR(DMA1, LL_DMA_CHANNEL_2);
    LL_USART_DisableDMAReq_RX(USART1);
    SciReady = SET;
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
