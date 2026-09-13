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
uint32_t LED_BlinkingTime = 0;
LL_GPIO_InitTypeDef GPIO_InitStruct = {0};

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_EnterStandby(void);

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

  /* Turn on the LED */
  BSP_LED_On(LED_GREEN);
  
  if (LL_PWR_IsActiveFlag_SB() != 0)
  {
    /* Clear the STANDBY flag */
    LL_PWR_ClearFlag_SB();

    /* Clear the wake-up flag */
    LL_PWR_ClearFlag_WU();
    
    LED_BlinkingTime = 0;
    
    while(LED_BlinkingTime < 10)
    {
      LL_mDelay(500);
      BSP_LED_Toggle(LED_GREEN);
      LED_BlinkingTime++;
    }    
  }
  
  /* The Following Wakeup sequence is highly recommended prior to each Standby mode entry
     mainly when using more than one wakeup source this is to not miss any wakeup event.
     - Configure GPIO,
     - Disable all used wakeup sources,
     - Clear all related wakeup flags,
     - Re-enable all used wakeup sources,
     - Enter the Standby mode.
  */
  /* Configure PA2 as analog mode */
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOA);
  GPIO_InitStruct.Mode   = LL_GPIO_MODE_ANALOG;
  GPIO_InitStruct.Pull   = LL_GPIO_PULL_DOWN;
  GPIO_InitStruct.Speed  = LL_GPIO_SPEED_FREQ_VERY_HIGH;
  GPIO_InitStruct.Pin    = LL_GPIO_PIN_2;
  LL_GPIO_Init(GPIOA, &GPIO_InitStruct);
  
  /* Disable all used wakeup sources: PWR_WAKEUP_PIN4 */
  LL_PWR_DisableWakeUpPin(LL_PWR_WAKEUP_PIN4);

  /* Set the Wake-Up pin polarity high for the event detection */
  LL_PWR_SetWakeUpPinPolarityHigh(LL_PWR_WAKEUP_PIN4);
  
  /* Enable WakeUp Pin PWR_WAKEUP_PIN4 connected to PA2 */
  LL_PWR_EnableWakeUpPin(LL_PWR_WAKEUP_PIN4);

  /* Clear the wake-up flag */
  LL_PWR_ClearFlag_WU();
  
  /* Wait for the button to be pressed */
  while(BSP_PB_GetState(BUTTON_KEY) == 0);

  /* Turn off the LED */
  BSP_LED_Off(LED_GREEN);

  /* Enter STANDBY mode */
  APP_EnterStandby();

  /* Infinite loop */
  while (1)
  {
    /* LED toggle */
    BSP_LED_Toggle(LED_GREEN);
    LL_mDelay(500);
  }
}

/**
  * @brief Enter Standby mode.
  * @param  None
  * @retval None
  */
static void APP_EnterStandby(void)
{
  /* Clear the wake-up flag */
  LL_PWR_ClearFlag_WU();
  
  /* Set the VR output voltage level */
  LL_PWR_SetRegulVoltageScaling(LL_PWR_REGU_VOLTAGE_MEDIUM_LOW);
  
  /* Set the voltage of SRAM2 in standby mode */
  LL_PWR_SetSramVoltageInStandby(LL_PWR_SRAM2_VOLTAGE_MEDIUM_LOW);
  
  /* Set Low-Power mode */
  LL_PWR_SetPowerMode(LL_PWR_MODE_STANDBY);
  
  /* Set SLEEPDEEP bit of Cortex System Control Register */
  LL_LPM_EnableDeepSleep();
  
  /* This option is used to ensure that store operations are completed */
#if defined ( __CC_ARM)
  __force_stores();
#endif
  /* Request Wait For Interrupt */
  __WFI();
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
