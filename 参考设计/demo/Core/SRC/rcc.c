/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * @file    rcc.c
  * @brief   This file provides code for the configuration
  *          of all used RCC.
  ******************************************************************************
  * @attention
  *
  * <h2><center>&copy; Copyright (c) 2026 Puya Semiconductor Co.
  * All rights reserved.</center></h2>
  *
  * This software component is licensed by Puya under BSD 3-Clause license,
  * the License ; You may not use this file except in compliance with the
  * License.You may obtain a copy of the License at:
  *                        opensource.org/licenses/BSD-3-Clause
  *
  ******************************************************************************
  * @attention
  *
  * <h2><center>&copy; Copyright (c) 2016 STMicroelectronics.
  * All rights reserved.</center></h2>
  *
  * This software component is licensed by ST under BSD 3-Clause license
  * the "License"; You may not use this file except in compliance with the
  * License. You may obtain a copy of the License at:
  *                        opensource.org/licenses/BSD-3-Clause
  *
  ******************************************************************************
  */
/* USER CODE END Header */

/* Includes ------------------------------------------------------------------*/
#include "rcc.h"
/* USER CODE BEGIN Includes */

/* USER CODE END Includes */

/* Private typedef -----------------------------------------------------------*/
/* USER CODE BEGIN TD */

/* USER CODE END TD */

/* Private define ------------------------------------------------------------*/
/* USER CODE BEGIN Define */

/* USER CODE END Define */

/* Private macro -------------------------------------------------------------*/
/* USER CODE BEGIN Macro */

/* USER CODE END Macro */

/* Public variables ---------------------------------------------------------*/
/* USER CODE BEGIN PV */
/* USER CODE END PV */

/* Private variables ---------------------------------------------------------*/
/* USER CODE BEGIN Private */

/* USER CODE END Private */

/* Private function prototypes -----------------------------------------------*/
/* USER CODE BEGIN PFP */

/* USER CODE END PFP */

/* External variables --------------------------------------------------------*/
/* USER CODE BEGIN EV */

/* USER CODE END EV */

/***************************************
 * @brief RCC Init
 **************************************/
void Studio_RCC_Init(void)
{
  /* USER CODE BEGIN Studio_RCC_Init 0 */

  /* USER CODE END Studio_RCC_Init 0 */

  RCC_OscInitTypeDef RCC_OscInitStruct = {0};
  RCC_ClkInitTypeDef RCC_ClkInitStruct = {0};
  RCC_PeriphCLKInitTypeDef RCC_PeriphClkInitStruct = {0};

  RCC_OscInitStruct.OscillatorType = RCC_OSCILLATORTYPE_HSE | RCC_OSCILLATORTYPE_HSI48M;
  RCC_OscInitStruct.HSI48MState = RCC_HSI48M_ON;
  RCC_OscInitStruct.HSEState = RCC_HSE_ON;
  RCC_OscInitStruct.HSEFreq = RCC_HSE_4_8MHz;
  RCC_OscInitStruct.PLL.PLLState = RCC_PLL_ON;
  RCC_OscInitStruct.PLL.PLLSource = RCC_PLLSOURCE_HSE;
  RCC_OscInitStruct.PLL.PLLMUL = 168;
  RCC_OscInitStruct.PLL.PLLPrediv = RCC_PLL_PREDIV_DIV8;
  RCC_OscInitStruct.PLL.PLLPostdiv = RCC_PLL_POSTDIV_DIV1;

  /* Initialize RCC oscillator */
  if (HAL_RCC_OscConfig(&RCC_OscInitStruct) != HAL_OK)
  {
    Error_Handler();
  }

  /* Initialize CPU, AHB, and APB bus clocks */
  RCC_ClkInitStruct.ClockType = RCC_CLOCKTYPE_SYSCLK | RCC_CLOCKTYPE_HCLK | RCC_CLOCKTYPE_PCLK1 | RCC_CLOCKTYPE_PCLK2;
  RCC_ClkInitStruct.SYSCLKSource = RCC_SYSCLKSOURCE_PLLCLK;
  RCC_ClkInitStruct.AHBCLKDivider = RCC_SYSCLK_DIV1;
  RCC_ClkInitStruct.APB1CLKDivider = RCC_HCLK_DIV4;
  RCC_ClkInitStruct.APB2CLKDivider = RCC_HCLK_DIV4;
  if (HAL_RCC_ClockConfig(&RCC_ClkInitStruct, FLASH_LATENCY_7) != HAL_OK)
  {
    Error_Handler();
  }

  __HAL_RCC_CAN_CONFIG(RCC_CANCLKSOURCE_HSE);

  RCC_PeriphClkInitStruct.PeriphClockSelection = RCC_PERIPHCLK_USB;
  RCC_PeriphClkInitStruct.UsbClockSelection = RCC_USBCLKSOURCE_HSI48M;
  if (HAL_RCCEx_PeriphCLKConfig(&RCC_PeriphClkInitStruct) != HAL_OK)
  {
    Error_Handler();
  }

  RCC_PeriphClkInitStruct.PeriphClockSelection = RCC_PERIPHCLK_USART1;
  RCC_PeriphClkInitStruct.Usart1ClockSelection = RCC_USART1CLKSOURCE_PCLK2;
  if (HAL_RCCEx_PeriphCLKConfig(&RCC_PeriphClkInitStruct) != HAL_OK)
  {
    Error_Handler();
  }

  RCC_PeriphClkInitStruct.PeriphClockSelection = RCC_PERIPHCLK_USART2;
  RCC_PeriphClkInitStruct.Usart2ClockSelection = RCC_USART2CLKSOURCE_PCLK1;
  if (HAL_RCCEx_PeriphCLKConfig(&RCC_PeriphClkInitStruct) != HAL_OK)
  {
    Error_Handler();
  }

  RCC_PeriphClkInitStruct.PeriphClockSelection = RCC_PERIPHCLK_USART3;
  RCC_PeriphClkInitStruct.Usart3ClockSelection = RCC_USART3CLKSOURCE_PCLK1;
  if (HAL_RCCEx_PeriphCLKConfig(&RCC_PeriphClkInitStruct) != HAL_OK)
  {
    Error_Handler();
  }

  RCC_PeriphClkInitStruct.PeriphClockSelection = RCC_PERIPHCLK_LPUART1;
  RCC_PeriphClkInitStruct.Lpuart1ClockSelection = RCC_LPUART1CLKSOURCE_PCLK1;
  if (HAL_RCCEx_PeriphCLKConfig(&RCC_PeriphClkInitStruct) != HAL_OK)
  {
    Error_Handler();
  }

  /* USER CODE BEGIN Studio_RCC_Init 1 */

  /* USER CODE END Studio_RCC_Init 1 */
}

/* USER CODE BEGIN ExternalFunctions */

/* USER CODE END ExternalFunctions */

/* USER CODE BEGIN 0 */

/* USER CODE END 0 */

/* USER CODE BEGIN 1 */

/* USER CODE END 1 */
