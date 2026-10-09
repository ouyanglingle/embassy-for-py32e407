/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * @file    lpuart.c
  * @brief   This file provides code for the configuration
  *          of all used LPUART.
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
#include "lpuart.h"
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
LPUART_HandleTypeDef hlpuart;
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
 * @brief LPUART Init
 **************************************/
void Studio_LPUART_Init(void)
{
  /* USER CODE BEGIN Studio_LPUART_Init 0 */

  /* USER CODE END Studio_LPUART_Init 0 */

  hlpuart.Instance = LPUART;
  hlpuart.Init.BaudRate = 115200;
  hlpuart.Init.WordLength = LPUART_WORDLENGTH_8B;
  hlpuart.Init.StopBits = LPUART_STOPBITS_1;
  hlpuart.Init.Parity = LPUART_PARITY_NONE;
  hlpuart.Init.Mode = LPUART_MODE_TX_RX;
  hlpuart.Init.HwFlowCtl = LPUART_HWCONTROL_NONE;
  hlpuart.Init.ClockPrescaler = LPUART_PRESCALER_DIV1;
  hlpuart.AdvancedInit.AdvFeatureInit = LPUART_ADVFEATURE_NO_INIT;
  if (HAL_LPUART_Init(&hlpuart) != HAL_OK)
  {
    Error_Handler();
  }

  if (HAL_LPUARTEx_SetTxFifoThreshold(&hlpuart, LPUART_TXFIFO_THRESHOLD_1_8) != HAL_OK)
  {
    Error_Handler();
  }
  if (HAL_LPUARTEx_SetRxFifoThreshold(&hlpuart, LPUART_RXFIFO_THRESHOLD_1_8) != HAL_OK)
  {
    Error_Handler();
  }
  if (HAL_LPUARTEx_DisableFifoMode(&hlpuart) != HAL_OK)
  {
    Error_Handler();
  }

  /* USER CODE BEGIN Studio_LPUART_Init 1 */

  /* USER CODE END Studio_LPUART_Init 1 */
}

void HAL_LPUART_MspInit(LPUART_HandleTypeDef *hlpuart)
{
  GPIO_InitTypeDef GPIO_InitStruct = {0};

  if (hlpuart->Instance == LPUART)
  {
  /* USER CODE BEGIN LPUART_MspInit 0 */

  /* USER CODE END LPUART_MspInit 0 */

    __HAL_RCC_LPUART1_CLK_ENABLE();

    __HAL_RCC_GPIOC_CLK_ENABLE();
    /**LPUART GPIO Configuration
    *PC0     ------> LPUART_RX
    *PC1     ------> LPUART_TX
    */

    GPIO_InitStruct.Pin = GPIO_PIN_0|GPIO_PIN_1;
    GPIO_InitStruct.Mode = GPIO_MODE_AF_PP;
    GPIO_InitStruct.Pull = GPIO_PULLUP;
    GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_HIGH;
    GPIO_InitStruct.Alternate = GPIO_AF8_LPUART1;
    HAL_GPIO_Init(GPIOC, &GPIO_InitStruct);

  /* USER CODE BEGIN LPUART_MspInit 1 */

  /* USER CODE END LPUART_MspInit 1 */
  }
}

void HAL_LPUART_MspDeInit(LPUART_HandleTypeDef *hlpuart)
{
  if (hlpuart->Instance == LPUART)
  {
  /* USER CODE BEGIN LPUART_MspDeInit 0 */

  /* USER CODE END LPUART_MspDeInit 0 */

    __HAL_RCC_LPUART1_CLK_DISABLE();

    HAL_GPIO_DeInit(GPIOC, GPIO_PIN_0|GPIO_PIN_1);
  /* USER CODE BEGIN LPUART_MspDeInit 1 */

  /* USER CODE END LPUART_MspDeInit 1 */
  }
}

/* USER CODE BEGIN ExternalFunctions */

/* USER CODE END ExternalFunctions */

/* USER CODE BEGIN 0 */

/* USER CODE END 0 */

/* USER CODE BEGIN 1 */

/* USER CODE END 1 */
