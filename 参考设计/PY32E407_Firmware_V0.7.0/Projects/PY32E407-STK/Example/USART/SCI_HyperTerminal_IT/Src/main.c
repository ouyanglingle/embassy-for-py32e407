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

/* Private define ------------------------------------------------------------*/
/* Private variables ---------------------------------------------------------*/
SCI_HandleTypeDef hsci;

/* Buffer used for transmission */
uint8_t aTxStartMessage[] = "\n\r ****SCI-Hyperterminal communication based on IT ****\n\r Enter 12 characters using keyboard :\n\r";
uint8_t aTxEndMessage[]   = "\n\r Example Finished\n\r";

/* Buffer used for reception */
uint8_t aRxBuffer[RXBUFFERSIZE];

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_SCI_Init(void);

/**
  * @brief  Main program.
  * @retval int
  */
int main(void)
{
  /* Reset of all peripherals, Initializes the Systick. */ 
  HAL_Init();
  
  /* System clock configuration */
  APP_SystemClockConfig(); 

  /* Initialize GPIO */
  BSP_LED_Init(LED2);

  /* Initialize SCI */
  APP_SCI_Init();
  
  /* Start the transmission process */
  if(HAL_SCI_Transmit_IT(&hsci, (uint8_t*)aTxStartMessage, TXSTARTMESSAGESIZE)!= HAL_OK)
  {
    /* Transfer error in transmission process */
    APP_ErrorHandler();
  }

  /* Put SCI peripheral in reception process */
  if(HAL_SCI_Receive_IT(&hsci, (uint8_t *)aRxBuffer, RXBUFFERSIZE) != HAL_OK)
  {
    /* Transfer error in reception process */
    APP_ErrorHandler();
  }

  /* Wait for the end of the transfer */
  while (HAL_SCI_GetState(&hsci) != HAL_SCI_STATE_READY)
  {
  }

  /* Send the received Buffer */
  if(HAL_SCI_Transmit_IT(&hsci, (uint8_t*)aRxBuffer, RXBUFFERSIZE)!= HAL_OK)
  {
    /* Transfer error in transmission process */
    APP_ErrorHandler();
  }

  /* Wait for the end of the transfer */
  while (HAL_SCI_GetState(&hsci) != HAL_SCI_STATE_READY)
  {
  }

  /* Send the End Message */
  if(HAL_SCI_Transmit_IT(&hsci, (uint8_t*)aTxEndMessage, TXENDMESSAGESIZE)!= HAL_OK)
  {
    /* Transfer error in transmission process */
    APP_ErrorHandler();
  }

  /* Wait for the end of the transfer */
  while (HAL_SCI_GetState(&hsci) != HAL_SCI_STATE_READY)
  {
  }

  /* Turn on LED2 if test passes then enter infinite loop */
  BSP_LED_On(LED2);

  /* Infinite loop */
  while (1)
  {
  }  
}

/**
  * @brief  System clock configuration function.
  * @param  None
  * @retval None
  */
static void APP_SystemClockConfig(void)
{
  RCC_OscInitTypeDef  OscInitstruct = {0};
  RCC_ClkInitTypeDef  ClkInitstruct = {0};
  
  OscInitstruct.OscillatorType  = RCC_OSCILLATORTYPE_HSE | RCC_OSCILLATORTYPE_HSI | RCC_OSCILLATORTYPE_LSE | 
                                  RCC_OSCILLATORTYPE_LSI | RCC_OSCILLATORTYPE_HSI48M;
  OscInitstruct.HSEState        = RCC_HSE_OFF;                              /* Close HSE */
/* OscInitstruct.HSEFreq         = RCC_HSE_16_32MHz; */                       /* Choose HSE frequency of 16-32MHz */
  OscInitstruct.HSI48MState     = RCC_HSI48M_OFF;                           /* Close HSI48M */
  OscInitstruct.HSIState        = RCC_HSI_ON;                               /* Enable HSI */
  OscInitstruct.LSEState        = RCC_LSE_OFF;                              /* Close LSE */
/* OscInitstruct.LSEDriver       = RCC_LSEDRIVE_HIGH; */                    /* Drive capability level: high */
  OscInitstruct.LSIState        = RCC_LSI_OFF;                              /* Close LSI */
  OscInitstruct.PLL.PLLState    = RCC_PLL_OFF;                              /* Close PLL */
/* OscInitstruct.PLL.PLLSource   = RCC_PLLSOURCE_HSI_DIV2; */               /* PLL clock source selection HSI/2 */
/* OscInitstruct.PLL.PLLPrediv   = RCC_PLL_PREDIV_DIV1; */                  /* PLL clock Prediv factor */
/* OscInitstruct.PLL.PLLMUL      = 12; */                                   /* PLL clock source 12-fold frequency */
/* OscInitstruct.PLL.PLLPostdiv  = RCC_PLL_POSTDIV_DIV1; */                 /* PLL clock Postdiv factor */
  /* Configure oscillator */
  if(HAL_RCC_OscConfig(&OscInitstruct) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  
  ClkInitstruct.ClockType       = RCC_CLOCKTYPE_SYSCLK | RCC_CLOCKTYPE_HCLK | RCC_CLOCKTYPE_PCLK1 | RCC_CLOCKTYPE_PCLK2;
  ClkInitstruct.SYSCLKSource    = RCC_SYSCLKSOURCE_HSI;                 /* System clock selection HSI */
  ClkInitstruct.AHBCLKDivider   = RCC_SYSCLK_DIV1;                      /* AHB clock 1 division */
  ClkInitstruct.APB1CLKDivider  = RCC_HCLK_DIV1;                        /* APB1 clock 1 division */
  ClkInitstruct.APB2CLKDivider  = RCC_HCLK_DIV2;                        /* APB2 clock 2 division */
  /* Configure Clock */
  if(HAL_RCC_ClockConfig(&ClkInitstruct, FLASH_LATENCY_0) != HAL_OK)
  {
    APP_ErrorHandler();
  }
}

/**
  * @brief SCI Initialization Function
  * @param None
  * @retval None
  */
static void APP_SCI_Init(void)
{
  hsci.Instance                    = USART1;
  hsci.Init.BaudRate               = 115200;
  hsci.Init.WordLength             = SCI_WORDLENGTH_8B;
  hsci.Init.StopBits               = SCI_STOPBITS_1;
  hsci.Init.Parity                 = SCI_PARITY_NONE;
  hsci.Init.Mode                   = SCI_MODE_TX_RX;
  hsci.Init.HwFlowCtl              = SCI_HWCONTROL_NONE;
  hsci.Init.OverSampling           = SCI_OVERSAMPLING_16;
  hsci.AdvancedInit.AdvFeatureInit = SCI_ADVFEATURE_NO_INIT;
  if (HAL_SCI_Init(&hsci) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  if (HAL_SCIEx_SetTxFifoThreshold(&hsci, SCI_TXFIFO_THRESHOLD_1_8) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  if (HAL_SCIEx_SetRxFifoThreshold(&hsci, SCI_RXFIFO_THRESHOLD_1_8) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  if (HAL_SCIEx_DisableFifoMode(&hsci) != HAL_OK)
  {
    APP_ErrorHandler();
  }
}

/**
  * @brief  This function is executed in case of error occurrence.
  * @param  None
  * @retval None
  */
void APP_ErrorHandler(void)
{
  while (1)
  {
  }
}

#ifdef  USE_FULL_ASSERT
/**
  * @brief  Reports the name of the source file and the source line number
  *         where the assert_param error has occurred.
  * @param  file: pointer to the source file name
  * @param  line: assert_param error line source number
  * @retval None
  */
void assert_failed(uint8_t *file, uint32_t line)
{
  /* User can add his own implementation to report the file name and line number,
     ex: printf("Wrong parameters value: file %s on line %d\r\n", file, line) */

  /* Infinite loop */
  while (1)
  {
  }
}
#endif /* USE_FULL_ASSERT */

/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
