/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * @file    esmc.c
  * @brief   This file provides code for the configuration
  *          of all used ESMC.
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
#include "esmc.h"
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
ESMC_HandleTypeDef hesmc;
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
 * @brief ESMC Init
 **************************************/
void Studio_ESMC_Init(void)
{
  /* USER CODE BEGIN Studio_ESMC_Init 0 */

  /* USER CODE END Studio_ESMC_Init 0 */

  hesmc.Instance = ESMC;
  hesmc.Init.ClockPrescaler = 2;
  hesmc.Init.ClockMode = ESMC_CLOCK_MODE_0;
  hesmc.Init.DualFlash = ESMC_DUALFLASH_DISABLE;
  hesmc.Init.ChipSelectHighTime = ESMC_CS_HIGH_TIME_1_CYCLE;
  hesmc.Init.ChipSelectHoldTime = ESMC_CS_HOLD_TIME_2_CYCLE;
  if (HAL_ESMC_Init(&hesmc) != HAL_OK)
  {
    Error_Handler();
  }

  /* USER CODE BEGIN Studio_ESMC_Init 1 */

  /* USER CODE END Studio_ESMC_Init 1 */
}

void HAL_ESMC_MspInit(ESMC_HandleTypeDef* hesmc)
{
  GPIO_InitTypeDef GPIO_InitStruct = {0};
  if (hesmc->Instance == ESMC)
  {
  /* USER CODE BEGIN ESMC_MspInit 0 */

  /* USER CODE END ESMC_MspInit 0 */
    __HAL_RCC_ESMC_CLK_ENABLE();

    __HAL_RCC_GPIOB_CLK_ENABLE();
    __HAL_RCC_GPIOE_CLK_ENABLE();
    /**ESMC GPIO Configuration
    *PB10     ------> ESMC_CLK
    *PE11     ------> ESMC_CS0_NCS
    *PE12     ------> ESMC_BK1_IO0
    *PE13     ------> ESMC_BK1_IO1
    *PE14     ------> ESMC_BK1_IO2
    *PE15     ------> ESMC_BK1_IO3
    */

    GPIO_InitStruct.Pin = GPIO_PIN_10;
    GPIO_InitStruct.Mode = GPIO_MODE_AF_PP;
    GPIO_InitStruct.Pull = GPIO_NOPULL;
    GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_VERY_HIGH;
    GPIO_InitStruct.Alternate = GPIO_AF10_ESMC;
    HAL_GPIO_Init(GPIOB, &GPIO_InitStruct);

    GPIO_InitStruct.Pin = GPIO_PIN_11|GPIO_PIN_12|GPIO_PIN_13|GPIO_PIN_14|GPIO_PIN_15;
    GPIO_InitStruct.Mode = GPIO_MODE_AF_PP;
    GPIO_InitStruct.Pull = GPIO_NOPULL;
    GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_VERY_HIGH;
    GPIO_InitStruct.Alternate = GPIO_AF10_ESMC;
    HAL_GPIO_Init(GPIOE, &GPIO_InitStruct);

  /* USER CODE BEGIN ESMC_MspInit 1 */

  /* USER CODE END ESMC_MspInit 1 */
  }
}

void HAL_ESMC_MspDeInit(ESMC_HandleTypeDef* hesmc)
{
  if (hesmc->Instance == ESMC)
  {
  /* USER CODE BEGIN ESMC_MspDeInit 0 */

  /* USER CODE END ESMC_MspDeInit 0 */
    __HAL_RCC_ESMC_CLK_DISABLE();

    HAL_GPIO_DeInit(GPIOB, GPIO_PIN_10);
    HAL_GPIO_DeInit(GPIOE, GPIO_PIN_11|GPIO_PIN_12|GPIO_PIN_13|GPIO_PIN_14|GPIO_PIN_15);

  /* USER CODE BEGIN ESMC_MspDeInit 1 */

  /* USER CODE END ESMC_MspDeInit 1 */
  }
}

/* USER CODE BEGIN ExternalFunctions */

/* USER CODE END ExternalFunctions */

/* USER CODE BEGIN 0 */

/* USER CODE END 0 */

/* USER CODE BEGIN 1 */

/* USER CODE END 1 */
