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
/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_CompConfig(void);
static void APP_EnterStop(void);
static void APP_CompRccConfig(void);

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
  BSP_LED_On(LED_GREEN);

  /* Initialize BUTTON */
  BSP_PB_Init(BUTTON_KEY,BUTTON_MODE_GPIO);

  /* Wait Button */
  while(BSP_PB_GetState(BUTTON_KEY) == 0)
  {
  }

  BSP_LED_Off(LED_GREEN);

  /* COMP Rcc Config */
  APP_CompRccConfig();

  /* Configure COMP */
  APP_CompConfig();

  /* Enter Stop Mode */
  APP_EnterStop();

  while (1)
  {
    BSP_LED_Toggle(LED_GREEN);

    LL_mDelay(200);
  }
}

/**
  * @brief  Comparator Configuration Function
  * @param  None
  * @retval None
  */
static void APP_CompConfig(void)
{
  /* Enable GPIOB Clock */
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOB);
  /* Enable Comp clock*/
  LL_APB2_GRP1_EnableClock(LL_APB2_GRP1_PERIPH_COMP);

  /* Configure PB1 pin in analog mode */
  LL_GPIO_SetPinMode(GPIOB, LL_GPIO_PIN_1, LL_GPIO_MODE_ANALOG);
  
  /* Enable PB1 Analog2 */
  LL_SYSCFG_EnableGPIOBAnalog2(LL_SYSCFG_GPIOB_PIN_1);

  /* Set InputPlus IO1 PB1 */
  LL_COMP_SetInputPlus(COMP1,LL_COMP_INPUT_PLUS_IO1);

  /* Set InputMius VREFCMP */
  LL_COMP_SetInputMinus(COMP1,LL_COMP_INPUT_MINUS_VREFCMP);

  /* Set Output NONINVERTED */
  LL_COMP_SetOutputPolarity(COMP1,LL_COMP_OUTPUTPOL_NONINVERTED);

  /* Set Hysteresis Disable */
  LL_COMP_SetInputHysteresis(COMP1,LL_COMP_HYSTERESIS_DISABLE);

  /* Set PowerMode MediumSpeed */
  LL_COMP_SetPowerMode(COMP1,LL_COMP_POWERMODE_MEDIUMSPEED);

  /* Set BlankingSource None */
  LL_COMP_SetOutputBlankingSource(COMP1,LL_COMP_BLANKINGSRC_NONE);

  /* Disable Digital Filter */
  LL_COMP_DisableDigitalFilter(COMP1);
 
  /* Set DigitalFilter 0 */
  LL_COMP_SetDigitalFilter(COMP1,0);

  /* Select Vrefcmp Source VCC */
  LL_COMP_SetVrefCmpSource(COMP1,LL_COMP_VREFCMP_SOURCE_VCC);

  /* Set Vrefcmp Divider 32/64 */
  LL_COMP_SetVrefCmpDivider(COMP1,LL_COMP_VREFCMP_DIV_32_64VREFCMP);

  /* Enable Vrefcmp Divider */
  LL_COMP_EnableVrefCmpDivider(COMP1);
 
  /* Window mode not enabled */
  LL_COMP_SetCommonWindowMode(__LL_COMP_COMMON_INSTANCE(COMP1), LL_COMP_WINDOWMODE_DISABLE);

  __IO uint32_t wait_loop_index = 0;
  wait_loop_index = (LL_COMP_DELAY_VOLTAGE_SCALER_STAB_US * (SystemCoreClock / (1000000 * 2)));
  while(wait_loop_index != 0)
  {
    wait_loop_index--;
  }

  /* Enable rising edge interrupt */
  LL_EXTI_EnableRisingTrig_0_31(LL_EXTI_LINE_18);

  /* Enable interrupt */
  LL_EXTI_EnableIT_0_31(LL_EXTI_LINE_18);
  
  NVIC_EnableIRQ(COMP1_IRQn);            /* Enable COMP interrupt */
  NVIC_SetPriority(COMP1_IRQn, 0);

  /* Enable Comparator 1 */
  LL_COMP_Enable(COMP1);
  wait_loop_index = ((LL_COMP_DELAY_STARTUP_US / 10UL) * (SystemCoreClock / (100000UL * 2UL)));
  while(wait_loop_index != 0UL)
  {
    wait_loop_index--;
  }
}

/**
  * @brief  Comp Clock Init
  * @param  None
  * @retval None
  */
static void APP_CompRccConfig()
{
  /* Enable LSI */
  LL_RCC_LSI_Enable();
 
  /* Wait LSI Ready */
  while(LL_RCC_LSI_IsReady() != 1)
  {
  }
 
  /* Set LSC Source LSI */
  LL_RCC_SetCOMP1ClockSource(LL_RCC_COMP1_CLKSOURCE_LSI);
}

/**
  * @brief  Enter Stop
  * @param  None
  * @retval None
  */
void APP_EnterStop(void)
{
  /* Enable PWR Clock */
  LL_APB1_GRP1_EnableClock(LL_APB1_GRP1_PERIPH_PWR);

  /* Set Regular Voltage Medium Low */
  LL_PWR_SetRegulVoltageScaling(LL_PWR_REGU_VOLTAGE_MEDIUM_LOW);

  /* Set Low-Power mode */
  LL_PWR_SetPowerMode(LL_PWR_MODE_STOP1);

  /* Enter DeepSleep mode */
  LL_LPM_EnableDeepSleep();

  /* Waiting for interrupt instructions */
  __WFI();

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
