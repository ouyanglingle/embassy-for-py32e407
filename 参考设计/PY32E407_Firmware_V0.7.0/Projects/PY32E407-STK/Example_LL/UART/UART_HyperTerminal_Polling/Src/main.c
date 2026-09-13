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
uint8_t aTxStartMessage[] = "\n\r UART Hyperterminal communication based on polling\n\r Enter 12 characters using keyboard :\n\r";
uint8_t aTxEndMessage[] = "\n\r Example Finished\n\r";

uint8_t *TxBuff = NULL;
__IO uint16_t TxCount = 0;

uint8_t *RxBuff = NULL;
__IO uint16_t RxCount = 0;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_ConfigUart(void);
static void APP_UartTransmit(UART_TypeDef *UARTx, uint8_t *pData, uint16_t Size);
static void APP_UartReceive(UART_TypeDef *UARTx, uint8_t *pData, uint16_t Size);

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
  APP_UartTransmit(UART2, (uint8_t*)aTxStartMessage, TXSTARTMESSAGESIZE);
  
  /* Put UART peripheral in reception process */
  APP_UartReceive(UART2, (uint8_t *)aRxBuffer, 12);

  /* Send the received Buffer */
  APP_UartTransmit(UART2, (uint8_t*)aRxBuffer, 12);
  
  /* Send the End Message */
  APP_UartTransmit(UART2, (uint8_t*)aTxEndMessage, TXENDMESSAGESIZE);

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
  * @brief  UART transmit function
  * @param  UARTx：UART module
  * @param  pData：transmit buffer
  * @param  Size：Size of the transmit buffer
  * @retval None
  */
static void APP_UartTransmit(UART_TypeDef *UARTx, uint8_t *pData, uint16_t Size)
{  
  TxBuff = pData;
  TxCount = Size;

  /* transmit data */
  while (TxCount > 0)
  {
    /* Wait for TDRE bit to be set */
    while(LL_UART_IsActiveFlag_TDRE(UARTx) != 1);
    /* transmit data */
    LL_UART_TransmitData(UARTx, *TxBuff);
    TxBuff++;
    TxCount--;
  }

  /* Wait for transmission complete */
  while(LL_UART_IsActiveFlag_TXE(UARTx) != 1);
}

/**
  * @brief  UART receive function
  * @param  UARTx：UART module, can be UART1 or UART2
  * @param  pData：receive buffer
  * @param  Size：Size of the receive buffer
  * @retval None
  */
static void APP_UartReceive(UART_TypeDef *UARTx, uint8_t *pData, uint16_t Size)
{
  RxBuff = pData;
  RxCount = Size;

  /* Receive data */
  while (RxCount > 0)
  {
    /* Wait for RxNE bit to be set */
    while(LL_UART_IsActiveFlag_RXNE(UARTx) != 1);
    /* Receive data */
    *RxBuff = LL_UART_ReceiveData(UARTx);
    RxBuff++;
    RxCount--;
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
