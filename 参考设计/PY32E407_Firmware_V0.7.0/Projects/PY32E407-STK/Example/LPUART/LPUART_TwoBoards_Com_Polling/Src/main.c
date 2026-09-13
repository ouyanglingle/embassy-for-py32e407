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
#define TRANSMITTER_BOARD

/* Private variables ---------------------------------------------------------*/
LPUART_HandleTypeDef hlpuart;

/* Buffer used for transmission */
uint8_t aTxBuffer[] = " ****LPUART_TwoBoards_ComIT**** ";
/* Buffer used for reception */
uint8_t aRxBuffer[RXBUFFERSIZE];

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_LPUART_Init(void);
static uint16_t Buffercmp(uint8_t *pBuffer1, uint8_t *pBuffer2, uint16_t BufferLength);

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

  /* Initialize LPUART */
  APP_LPUART_Init();
  
  /* Initialize GPIO */
  BSP_LED_Init(LED2);

#ifdef TRANSMITTER_BOARD
  /* Configure User push-button */
  BSP_PB_Init(BUTTON_USER, BUTTON_MODE_GPIO);

  /* Wait for User push-button press before starting the Communication */
  while (BSP_PB_GetState(BUTTON_USER) == 0)
  {
  }

  /* Flush the data registers from unexpected data */
  __HAL_LPUART_FLUSH_DRREGISTER(&hlpuart);

  /* The board sends the message and expects to receive it back */

  /* Start the transmission process */
  if(HAL_LPUART_Transmit(&hlpuart, (uint8_t*)aTxBuffer, TXBUFFERSIZE, 5000)!= HAL_OK)
  {
    APP_ErrorHandler();
  }

  /* Wait for the end of the transfer */
  while(HAL_LPUART_GetState(&hlpuart) != HAL_LPUART_STATE_READY)
  {
  }

  /* Put LPUART peripheral in reception process */
  if(HAL_LPUART_Receive(&hlpuart, (uint8_t *)aRxBuffer, RXBUFFERSIZE, 5000) != HAL_OK)
  {
    APP_ErrorHandler();
  }
#else
  /* The board receives the message and sends it back */
  /* Put LPUART peripheral in reception process */
  if(HAL_LPUART_Receive(&hlpuart, (uint8_t *)aRxBuffer, RXBUFFERSIZE, 5000) != HAL_OK)
  {
    APP_ErrorHandler();
  }

  /* Wait for the end of the transfer */
  while(HAL_LPUART_GetState(&hlpuart) != HAL_LPUART_STATE_READY)
  {
  }

  /* Wait some ms for synchronization */
  HAL_Delay(100);

  /* Start the transmission process */
  if(HAL_LPUART_Transmit(&hlpuart, (uint8_t*)aTxBuffer, TXBUFFERSIZE, 5000)!= HAL_OK)
  {
    APP_ErrorHandler();
  }

#endif /* TRANSMITTER_BOARD */

  /* Wait for the end of the transfer */
  while(HAL_LPUART_GetState(&hlpuart) != HAL_LPUART_STATE_READY)
  {
  }

  /* Compare the sent and received buffers */
  if(Buffercmp((uint8_t*)aTxBuffer,(uint8_t*)aRxBuffer,RXBUFFERSIZE))
  {
    APP_ErrorHandler();
  }

  /* Turn LED2 on: Transfer process is correct */
  BSP_LED_On(LED2);

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
  * @brief LPUART1 Initialization Function
  * @param None
  * @retval None
  */
static void APP_LPUART_Init(void)
{
  hlpuart.Instance = LPUART;
  hlpuart.Init.BaudRate = 115200;
  hlpuart.Init.WordLength = LPUART_WORDLENGTH_8B;
  hlpuart.Init.StopBits = LPUART_STOPBITS_1;
  hlpuart.Init.Parity = LPUART_PARITY_NONE;
  hlpuart.Init.Mode = LPUART_MODE_TX_RX;
  hlpuart.Init.HwFlowCtl = LPUART_HWCONTROL_NONE;
  hlpuart.AdvancedInit.AdvFeatureInit = LPUART_ADVFEATURE_NO_INIT;
  if (HAL_LPUART_Init(&hlpuart) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  if (HAL_LPUARTEx_SetTxFifoThreshold(&hlpuart, LPUART_TXFIFO_THRESHOLD_1_8) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  if (HAL_LPUARTEx_SetRxFifoThreshold(&hlpuart, LPUART_RXFIFO_THRESHOLD_1_8) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  if (HAL_LPUARTEx_DisableFifoMode(&hlpuart) != HAL_OK)
  {
    APP_ErrorHandler();
  }
}

/**
  * @brief  Compares two buffers.
  * @param  pBuffer1, pBuffer2: buffers to be compared.
  * @param  BufferLength: buffer's length
  * @retval 0  : pBuffer1 identical to pBuffer2
  *         >0 : pBuffer1 differs from pBuffer2
  */
static uint16_t Buffercmp(uint8_t *pBuffer1, uint8_t *pBuffer2, uint16_t BufferLength)
{
  while (BufferLength--)
  {
    if ((*pBuffer1) != *pBuffer2)
    {
      return BufferLength;
    }
    pBuffer1++;
    pBuffer2++;
  }

  return 0;
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
