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
uint8_t  aRxBuffer[RX_MAX_LEN] = {0};
uint32_t cRxIndex = 0;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_ConfigSci(void);

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

  /* Enable RX Not Empty Interrupt */
  LL_USART_EnableIT_RXNE(USART1);

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
  NVIC_SetPriority(USART1_IRQn, 0);
  /* Enable USART1 interrupt request */
  NVIC_EnableIRQ(USART1_IRQn);

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
  * @brief  SCI Error interrupt handler function
  * @param  None
  * @retval None
  */
void APP_SciErrorCallback(void)
{
  /* Turn on the LED */
  BSP_LED_On(LED_GREEN);
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
