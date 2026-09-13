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
/* Length of array of 32-bit long random numbers to be filled up */
#define N  8

/* Private variables ---------------------------------------------------------*/
uint32_t     aRandom32bit[N] = {0};
__IO uint8_t Index           = 0;
__IO uint8_t ubIsNbGenerated = 0;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_RandomNumbersGenerationIT(void);

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

  /* Initialize UART (for printf use) */
  BSP_UART_Config();

  /* Generate Random Numbers series */
  APP_RandomNumbersGenerationIT();

  /* Infinite loop */
  while (1)
  {
  }
}

/**
  * @brief  This function performs several random numbers generation.
  * @note   Generated random numbers are stored in global variable array, so that
  *         generated values could be observed by user by watching variable content
  *         in specific debugger window
  * @param  None
  * @retval None
  */
static void APP_RandomNumbersGenerationIT(void)
{
  uint8_t index = 0;

  /* Enable peripheral clock for RNG */
  LL_APB2_GRP1_EnableClock(LL_APB2_GRP1_PERIPH_RNG);

  /* Enable RNG NVIC */
  NVIC_SetPriority(RNG_IRQn, 0);  
  NVIC_EnableIRQ(RNG_IRQn);
  
  /* Initialize random numbers generation */
  LL_RNG_EnableBlockGeneration0(RNG);
  LL_RNG_EnableBlockGeneration1(RNG);

  /* Generate Random 32bit Numbers */
  for(Index = 0; Index < N; Index++)
  {
    ubIsNbGenerated = 0;

    /* Enable RNG generation interrupt */
    LL_RNG_EnableIT(RNG);

    /* Wait for Random Number generation completion */
    while (ubIsNbGenerated == 0)
    {
    }
  }

  /* Stop random numbers generation */
  LL_RNG_DisableBlockGeneration0(RNG);
  LL_RNG_DisableBlockGeneration1(RNG);

  /* print random numbers */
  printf("random numbers:\r\n");
  for (index = 0; index < N; index++)
  {
    printf("%u ", (unsigned int)aRandom32bit[index]);
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
  * @brief  Function called when RNG IT is triggered with DRDY flag set
  * @param  None
  * @retval None
  */
void APP_RNG_DataReady_Callback(void)
{
  /* Disable RNG IT generation */
  LL_RNG_DisableIT(RNG);

  /* Value of generated random number could be retrieved and stored in dedicated array */
  aRandom32bit[Index] = LL_RNG_ReadRandData32(RNG);

  /* Set Flag indicated random Number generation is completed */
  ubIsNbGenerated = 1;
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
