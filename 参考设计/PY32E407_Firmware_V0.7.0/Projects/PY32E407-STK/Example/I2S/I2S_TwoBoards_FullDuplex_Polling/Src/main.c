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
#define BOARD_IN_MASTER_MODE
#define DATA_LENGTH       16

/* Private variables ---------------------------------------------------------*/
I2S_HandleTypeDef I2sHandle;
uint16_t TxBuff[16] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16};
uint16_t RxBuff[16] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_WaitAndCheckEndOfTransfer(void);
static uint8_t APP_Buffercmp8(uint8_t* pBuffer1, uint8_t* pBuffer2, uint8_t BufferLength);
static void APP_LedBlinking(void);

/**
  * @brief  Main program
  * @retval int
  */
int main(void)
{
  /* Reset of all peripherals, Initializes the Systick */
  HAL_Init();
  
  /* Configure Systemclock */
  APP_SystemClockConfig(); 
  
  /* Initialize LED */
  BSP_LED_Init(LED_GREEN);

  /* Initialize BUTTON */
  BSP_PB_Init(BUTTON_KEY,BUTTON_MODE_GPIO);

  /* Wait for the button to be pressed */
  while(BSP_PB_GetState(BUTTON_KEY) == 0)
  {
  }
  
  /* Initialize I2S */
  I2sHandle.Instance          = SPI1;
#ifdef BOARD_IN_MASTER_MODE
  I2sHandle.Init.Mode         = I2S_MODE_MASTER_TX;  /* Master transmit */
#else
  I2sHandle.Init.Mode         = I2S_MODE_SLAVE_RX;   /* Slave receive */
#endif
  I2sHandle.Init.Standard     = I2S_STANDARD_PHILIPS;
  I2sHandle.Init.DataFormat   = I2S_DATAFORMAT_16B;
  I2sHandle.Init.MCLKOutput   = I2S_MCLKOUTPUT_DISABLE;
  I2sHandle.Init.AudioFreq    = I2S_AUDIOFREQ_8K;
  I2sHandle.Init.CPOL         = I2S_CPOL_HIGH;
  if (HAL_I2S_Init(&I2sHandle) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  
#ifdef BOARD_IN_MASTER_MODE
  /* Transmit and Receive an amount of data in non-blocking mode with polling */
  if (HAL_I2S_Transmit(&I2sHandle, (uint16_t *)TxBuff, DATA_LENGTH, 5000) != HAL_OK)
  {
    APP_ErrorHandler();
  }
#else
  /* Receive an amount of data in non-blocking mode with polling */
  if (HAL_I2S_Receive(&I2sHandle, (uint16_t *)RxBuff, DATA_LENGTH, 5000) != HAL_OK)
  {
    APP_ErrorHandler();
  }
#endif
  /* Wait for transmit completion */
  while (HAL_I2S_GetState(&I2sHandle) != HAL_I2S_STATE_READY)
  {
  }
  
  HAL_Delay(1); /* Wait for the slave to be ready for data transmission */
  
  /* Initialize I2S */
  I2sHandle.Instance          = SPI1;
#ifdef BOARD_IN_MASTER_MODE
  I2sHandle.Init.Mode         = I2S_MODE_MASTER_RX;  /* Master receive */
#else
  I2sHandle.Init.Mode         = I2S_MODE_SLAVE_TX;   /* Slave transmit */
#endif
  I2sHandle.Init.Standard     = I2S_STANDARD_PHILIPS;
  I2sHandle.Init.DataFormat   = I2S_DATAFORMAT_16B;
  I2sHandle.Init.MCLKOutput   = I2S_MCLKOUTPUT_DISABLE;
  I2sHandle.Init.AudioFreq    = I2S_AUDIOFREQ_8K;
  I2sHandle.Init.CPOL         = I2S_CPOL_HIGH;  
  if (HAL_I2S_Init(&I2sHandle) != HAL_OK)
  {
    APP_ErrorHandler();
  }

#ifdef BOARD_IN_MASTER_MODE
  /* Receive an amount of data in non-blocking mode with polling */
  if (HAL_I2S_Receive(&I2sHandle, (uint16_t *)RxBuff, DATA_LENGTH, 5000) != HAL_OK)
  {
    APP_ErrorHandler();
  }
#else
  /* Transmit data as slave */
  if (HAL_I2S_Transmit(&I2sHandle, (uint16_t *)TxBuff, DATA_LENGTH, 5000) != HAL_OK)
  {
    APP_ErrorHandler();
  }
#endif
  /* Wait for receive completion  */
  while (HAL_I2S_GetState(&I2sHandle) != HAL_I2S_STATE_READY)
  {
  }
  
  /* Wait for the transfer be completed and check the received data */
  APP_WaitAndCheckEndOfTransfer();
  
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
  /* Wait for the transfer be completed */
  while (I2sHandle.State != HAL_I2S_STATE_READY)
  {
  }

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
  * @param  file：Pointer to the source file name
  * @param  line：assert_param error line source number
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
