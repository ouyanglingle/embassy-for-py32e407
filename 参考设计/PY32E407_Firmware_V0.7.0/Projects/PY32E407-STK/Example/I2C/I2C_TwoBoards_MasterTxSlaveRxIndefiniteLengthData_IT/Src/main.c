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
#define I2C_ADDRESS      0xA0               /* Own address 0xA0 */
#define I2C_SPEEDCLOCK   100000             /* Communication speed : 100K */
#define I2C_DUTYCYCLE    I2C_DUTYCYCLE_16_9 /* Duty cycle */

/* Private variables ---------------------------------------------------------*/
I2C_HandleTypeDef I2cHandle;

#ifdef BOARD_IN_MASTER_MODE
uint8_t aTxBuffer[100] = {0};
#else
#define RX_MAX_LEN 200     /* Single frame data, maximum received data length */
uint32_t RxLen = 0;        /* Single frame data, actual received data length */
uint8_t RxBuffer[RX_MAX_LEN] = {0}; /* Receive buffer */
__IO uint8_t RevOkFlag = 0;     /* Single frame data received completion flag */
#endif

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
#ifndef BOARD_IN_MASTER_MODE
static void APP_SlaveReceive_IT(void);
#endif

/**
  * @brief  Main program.
  * @retval int
  */
int main(void)
{
  uint32_t i = 0;
  
  /* Reset of all peripherals, Initializes the Systick. */ 
  HAL_Init();
  
  /* System clock configuration */
  APP_SystemClockConfig(); 

  /* Initialize LED */
  BSP_LED_Init(LED_GREEN);

  /* I2C initialization */
  I2cHandle.Instance             = I2C3;                        /* I2C */
  I2cHandle.Init.ClockSpeed      = I2C_SPEEDCLOCK;              /* I2C communication speed */
  I2cHandle.Init.DutyCycle       = I2C_DUTYCYCLE;               /* I2C Duty cycle */
  I2cHandle.Init.DataHoldTimeSel = I2C_DATA_HOLD_TIME_HARDWARE; /* Data hold time selection */
  I2cHandle.Init.AddressingMode  = I2C_ADDRESSINGMODE_7BIT;     /* 7-bit addressing mode */
  I2cHandle.Init.DualAddressMode = I2C_DUALADDRESS_DISABLE;     /* Disable dual address */
  I2cHandle.Init.GeneralCallMode = I2C_GENERALCALL_DISABLE;     /* Disable general call */
  I2cHandle.Init.NoStretchMode   = I2C_NOSTRETCH_DISABLE;       /* Disable clock stretching */
  I2cHandle.Init.OwnAddress1     = I2C_ADDRESS;                 /* I2C address */
  I2cHandle.Init.OwnAddress2     = 0x0;                         /* Second address */
  if (HAL_I2C_Init(&I2cHandle) != HAL_OK)
  {
    APP_ErrorHandler();
  }

#ifdef BOARD_IN_MASTER_MODE
  /* Initialize button */
  BSP_PB_Init(BUTTON_KEY,BUTTON_MODE_GPIO);
  
  /* Wait for user button press to start the main program */
  while (BSP_PB_GetState(BUTTON_KEY) == 0)
  {
  }

  for(i = 0; i < 10; i++)
  {
    aTxBuffer[i] = i;
  }
  /* I2C master interrupt mode transmission */
  while (HAL_I2C_Master_Transmit_IT(&I2cHandle, I2C_ADDRESS, (uint8_t *)aTxBuffer, 10) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  /* Check the current I2C state */
  while (HAL_I2C_GetState(&I2cHandle) != HAL_I2C_STATE_READY)
  {
  }
  
  /* Delay purpose: Wait for the slave to receive and print the received data */
  HAL_Delay(200);
  for(i = 0; i < 100; i++)
  {
    aTxBuffer[i] = i + 1;
  }
  /* I2C master interrupt mode transmission */
  while (HAL_I2C_Master_Transmit_IT(&I2cHandle, I2C_ADDRESS, (uint8_t *)aTxBuffer, 100) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  /* Check the current I2C state */
  while (HAL_I2C_GetState(&I2cHandle) != HAL_I2C_STATE_READY)
  {
  }
  
  /* Delay purpose: Wait for the slave to receive and print the received data */
  HAL_Delay(200);
  for(i = 0; i < 10; i++)
  {
    aTxBuffer[i] = i;
  }
  /* I2C master interrupt mode transmission */
  while (HAL_I2C_Master_Transmit_IT(&I2cHandle, I2C_ADDRESS, (uint8_t *)aTxBuffer, 10) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  /* Check the current I2C state */
  while (HAL_I2C_GetState(&I2cHandle) != HAL_I2C_STATE_READY)
  {
  }
#else
  /* Initialize UART */
  BSP_UART_Config();
  /* Slave receives data */
  APP_SlaveReceive_IT();
#endif
  
  while (1)
  {
    #ifdef BOARD_IN_MASTER_MODE
    /* Do nothing */
    #else
    if (RevOkFlag == 1)
    {
      for (i = 0; i < RxLen; i++)
      {
        printf("%d ", RxBuffer[i]);
      }
      RxLen = 0;
      RevOkFlag = 0;
    }
    #endif
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

#ifndef BOARD_IN_MASTER_MODE
/**
  * @brief  I2C reception function
  * @param  pData：Pointer to data to be received
  * @param  Size：Size of data to be received
  * @retval None
  */
static void APP_SlaveReceive_IT(void)
{
  /* Clear POS bit */
  CLEAR_BIT(I2C3->CR1, I2C_CR1_POS);
  
  /* Enable acknowledge */
  SET_BIT(I2C3->CR1, I2C_CR1_ACK);
  
  /* Enable interrupt */
  __HAL_I2C_ENABLE_IT(&I2cHandle, I2C_IT_EVT | I2C_IT_BUF | I2C_IT_ERR);
}

/**
  * @brief  I2C interrupt callback function
  * @param  None
  * @retval None
  */
void APP_SlaveIRQCallback(void)
{
  /* Set ADDR flag */
  if ((__HAL_I2C_GET_FLAG(&I2cHandle, I2C_FLAG_ADDR) != RESET) && (__HAL_I2C_GET_IT_SOURCE(&I2cHandle, I2C_IT_EVT) != RESET))
  {
    __HAL_I2C_CLEAR_ADDRFLAG(&I2cHandle);
  }
  /* Set STOP flag */
  else if (__HAL_I2C_GET_FLAG(&I2cHandle, I2C_FLAG_STOPF) != RESET)
  {
    __HAL_I2C_CLEAR_STOPFLAG(&I2cHandle);
    
    RevOkFlag = 1;
  }
  /* Slave Receive */
  else
  {
    /* Set RXNE flag, BTF flag is not set */
    if ((__HAL_I2C_GET_FLAG(&I2cHandle, I2C_FLAG_RXNE) != RESET) && (__HAL_I2C_GET_IT_SOURCE(&I2cHandle, I2C_IT_BUF) != RESET) && \
         (__HAL_I2C_GET_FLAG(&I2cHandle, I2C_FLAG_BTF) == RESET))
    {
        RxBuffer[RxLen++] = (uint8_t)(I2C3->DR);
    }
    /* Set BTF flag */
    else if ((__HAL_I2C_GET_FLAG(&I2cHandle, I2C_FLAG_BTF) != RESET) && (__HAL_I2C_GET_IT_SOURCE(&I2cHandle, I2C_IT_EVT) != RESET))
    {
        RxBuffer[RxLen++] = (uint8_t)(I2C3->DR);
    }
  }
}

/**
  * @brief  After the I2C master receives the last byte, send NACK to the slave, slave NACK 
  * @param  None
  * @retval None
  */
void APP_SlaveIRQCallback_NACK(void)
{
  if ((__HAL_I2C_GET_FLAG(&I2cHandle, I2C_FLAG_AF) != RESET) && (__HAL_I2C_GET_IT_SOURCE(&I2cHandle, I2C_IT_ERR) != RESET))
  {
     __HAL_I2C_CLEAR_FLAG(&I2cHandle, I2C_FLAG_AF);
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
