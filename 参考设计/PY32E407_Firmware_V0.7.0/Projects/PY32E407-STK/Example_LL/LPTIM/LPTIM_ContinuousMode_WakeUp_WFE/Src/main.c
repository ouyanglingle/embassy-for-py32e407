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
LL_LPTIM_InitTypeDef LPTIM_InitStruct = {0};

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_ConfigLptimClock(void);
static void APP_ConfigLptim(void);
static void APP_EnterStop1(void);

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

  /* Initialize BUTTON */
  BSP_PB_Init(BUTTON_KEY,BUTTON_MODE_GPIO);

  /* Set wake-up mode of the LPTIM1(EXTI Line30) to event request */
  LL_EXTI_DisableIT_0_31(LL_EXTI_LINE_30);   /* Disable interrupt request for EXTI Line30 */
  LL_EXTI_EnableEvent_0_31(LL_EXTI_LINE_30); /* Enable event request for EXTI Line30 */

  /* Enable LPTIM clock */
  LL_APB1_GRP2_EnableClock(LL_APB1_GRP2_PERIPH_LPTIM1);
  
  /* Initialize LPTIM */
  LPTIM_InitStruct.ClockSource = LL_LPTIM_CLK_SOURCE_INTERNAL;
  LPTIM_InitStruct.Polarity = LL_LPTIM_OUTPUT_POLARITY_REGULAR;
  LPTIM_InitStruct.Prescaler = LL_LPTIM_PRESCALER_DIV128;        /* prescaler: 128 */
  LPTIM_InitStruct.Waveform = LL_LPTIM_OUTPUT_WAVEFORM_PWM;
  if (LL_LPTIM_Init(LPTIM1, &LPTIM_InitStruct) != SUCCESS)
  {
    APP_ErrorHandler();
  }

  /* Set LSI as LPTIM clcok source */
  APP_ConfigLptimClock();
  
  /* LED ON */
  BSP_LED_On(LED_GREEN);

  /* Wait the button be pressed */
  while (BSP_PB_GetState(BUTTON_USER) == 0)
  {
  }

  /* LED off */
  BSP_LED_Off(LED_GREEN);
  
  /* Set LPTIM to continus mode Enable autoreload match */
  APP_ConfigLptim();

  while (1)
  {
    /* Enter STOP mode */
    APP_EnterStop1();
    
    /* The Autoreload match flag must be cleared before entering stop mode the next time */
    if(LL_LPTIM_IsActiveFlag_ARRM(LPTIM1) == 1)
    {   
      /* Clear autoreload match flag */
      LL_LPTIM_ClearFlag_ARRM(LPTIM1);

      /* LED Toggle */
      BSP_LED_Toggle(LED_GREEN);
    }   
  }
}
/**
  * @brief  Configure LPTIM clock
  * @param  None
  * @retval None
  */
static void APP_ConfigLptimClock(void)
{
  LL_RCC_LSI_Enable();
  while(LL_RCC_LSI_IsReady() != 1)
  {
  }

  /* Select LSI as LTPIM clock source */
  LL_RCC_SetLPTIM1ClockSource(LL_RCC_LPTIM1_CLKSOURCE_LSI);
}

/**
  * @brief  Configure LPTIM
  * @param  None
  * @retval None
  */
static void APP_ConfigLptim(void)
{
  /* Enable LPTIM autoreload match interrupt  */
  LL_LPTIM_EnableIT_ARRM(LPTIM1);

  /* Enable LPTIM */
  LL_LPTIM_Enable(LPTIM1);

  /* Set autoreload value */
  LL_LPTIM_SetAutoReload(LPTIM1, 51-1);

  /* LPTIM starts in continuous mode */
  LL_LPTIM_StartCounter(LPTIM1, LL_LPTIM_OPERATING_MODE_CONTINUOUS);
}

/**
  * @brief  Enable Stop mode
  * @param  None
  * @retval None
  */
static void APP_EnterStop1(void)
{
  /* Enable PWR clock */
  LL_APB1_GRP1_EnableClock(LL_APB1_GRP1_PERIPH_PWR);
 
  /* Set Regular Voltage Medium Low */
  LL_PWR_SetRegulVoltageScaling(LL_PWR_REGU_VOLTAGE_MEDIUM_LOW);

  /* Set Low-Power mode */
  LL_PWR_SetPowerMode(LL_PWR_MODE_STOP1);

  /* Set SLEEPDEEP bit of Cortex System Control Register */
  LL_LPM_EnableDeepSleep();

  /* Request Wait For Event */
  __SEV();
  __WFE();
  __WFE();

  LL_LPM_EnableSleep();
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
