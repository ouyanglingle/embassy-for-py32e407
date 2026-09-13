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
#define DATA_LENGTH       15

/* Private variables ---------------------------------------------------------*/
#ifdef TRANSMITTER_BOARD
USART_HandleTypeDef UsartHandle;
#else
SPI_HandleTypeDef   Spi1Handle;
#endif

/* Buffer used for reception */
uint8_t TxBuff[15] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15};
uint8_t RxBuff[15] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
#ifdef TRANSMITTER_BOARD
static void APP_USART_Init(void);
#else
static void APP_SPI_Init(void);
#endif
static void APP_WaitAndCheckEndOfTransfer(void);
static uint8_t APP_Buffercmp8(uint8_t* pBuffer1, uint8_t* pBuffer2, uint8_t BufferLength);
static void APP_LedBlinking(void);

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
#ifdef TRANSMITTER_BOARD
  /* Initialize USART */
  APP_USART_Init();
  
  /* Initialize button */
  BSP_PB_Init(BUTTON_KEY,BUTTON_MODE_GPIO);
  /* Wait for the button to be pressed */
  while (BSP_PB_GetState(BUTTON_KEY) == 0)
  {
  }
  
  /* USART transfer */
  if (HAL_USART_TransmitReceive(&UsartHandle, TxBuff, RxBuff, DATA_LENGTH, 5000) != HAL_OK)
  {
    APP_ErrorHandler();
  }
#else
  /* Initialize SPI */
  APP_SPI_Init();

  /*SPI receive*/
  if (HAL_SPI_TransmitReceive(&Spi1Handle, (uint8_t *)TxBuff, (uint8_t *)RxBuff, DATA_LENGTH, 5000) != HAL_OK)
  {
    APP_ErrorHandler();
  }
#endif
  /* Wait for the transfer to end and check the received data */
  APP_WaitAndCheckEndOfTransfer();

  /* Infinite loop */
  while (1)
  {

  }
}

/**
  * @brief  Wait for the transfer be completed and check the received data
  * @param  None
  * @retval None
  */
static void APP_WaitAndCheckEndOfTransfer(void)
{
#ifdef TRANSMITTER_BOARD
  /* Wait for transfer to complete */
  while (UsartHandle.State != HAL_USART_STATE_READY)
  {}
#else  /* Wait for transfer to complete */
  while (Spi1Handle.State != HAL_SPI_STATE_READY)
  {}
#endif
  /* Compare sent and received data */
  if(APP_Buffercmp8((uint8_t*)TxBuff, (uint8_t*)RxBuff, DATA_LENGTH))
  {
    /* error handling */
    APP_LedBlinking();
  }
  else
  {
    /* If data is received, the LED is turned on */
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
  * @brief  LED blink
  * @param  None
  * @retval None
  */
static void APP_LedBlinking(void)
{
  while (1)
  {
    BSP_LED_Toggle(LED_GREEN); 
    HAL_Delay(500);
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
/* OscInitstruct.PLL.PLLPrediv   = RCC_PLL_PREDIV_DIV1;    */               /* PLL clock Prediv factor */
/* OscInitstruct.PLL.PLLMUL      = 12;                     */               /* PLL clock source 12-fold frequency */
/* OscInitstruct.PLL.PLLPostdiv  = RCC_PLL_POSTDIV_DIV1;   */               /* PLL clock Postdiv factor */
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

#ifdef TRANSMITTER_BOARD
/**
  * @brief USART Initialization Function
  * @param None
  * @retval None
  */
static void APP_USART_Init(void)
{
  UsartHandle.Instance                    = USART1;
  UsartHandle.Init.BaudRate               = 115200;
  UsartHandle.Init.WordLength             = USART_WORDLENGTH_8B;
  UsartHandle.Init.StopBits               = USART_STOPBITS_1;
  UsartHandle.Init.Parity                 = USART_PARITY_NONE;
  UsartHandle.Init.Mode                   = USART_MODE_TX_RX;
  UsartHandle.Init.CLKPolarity            = USART_POLARITY_LOW;
  UsartHandle.Init.CLKPhase               = USART_PHASE_2EDGE;
  UsartHandle.Init.CLKLastBit             = USART_LASTBIT_ENABLE;
  if (HAL_USART_Init(&UsartHandle) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  if (HAL_USARTEx_SetTxFifoThreshold(&UsartHandle, USART_TXFIFO_THRESHOLD_1_8) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  if (HAL_USARTEx_SetRxFifoThreshold(&UsartHandle, USART_RXFIFO_THRESHOLD_1_8) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  if (HAL_USARTEx_DisableFifoMode(&UsartHandle) != HAL_OK)
  {
    APP_ErrorHandler();
  }
}
#else
/**
  * @brief USART Initialization Function
  * @param None
  * @retval None
  */
static void APP_SPI_Init(void)
{
  /*SPI initialization */
  Spi1Handle.Instance               = SPI1;                        /* SPI1 */
  Spi1Handle.Init.BaudRatePrescaler = SPI_BAUDRATEPRESCALER_256;   /* Prescaler: 256 */
  Spi1Handle.Init.Direction         = SPI_DIRECTION_2LINES;        /* Full-duplex */
  Spi1Handle.Init.CLKPolarity       = SPI_POLARITY_LOW;            /* Clock polarity: Low */
  Spi1Handle.Init.CLKPhase          = SPI_PHASE_2EDGE ;            /* Data sampling on the second clock edge */
  Spi1Handle.Init.DataSize          = SPI_DATASIZE_8BIT;           /* SPI data size: 8-bit */
  Spi1Handle.Init.FirstBit          = SPI_FIRSTBIT_LSB;            /* LSB transmitted first */
  Spi1Handle.Init.NSS               = SPI_NSS_SOFT;                /* NSS software mode */
  Spi1Handle.Init.Mode              = SPI_MODE_SLAVE;              /* Configured as slave */
  Spi1Handle.Init.TIMode            = SPI_TIMODE_DISABLE;          /* TI Mode disable */
  if (HAL_SPI_Init(&Spi1Handle) != HAL_OK)
  {
    APP_ErrorHandler();
  }
}
#endif

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
