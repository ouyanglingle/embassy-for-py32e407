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
static void APP_SystemClockDecrease(void);
static void APP_EnterLowPowerRun(void);
static void APP_EnterSleep(void);
static void APP_ExtiConfig(void);

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

  /* Initialize LED */
  BSP_LED_Init(LED_GREEN);

  /* Initialize BUTTON */
  BSP_PB_Init(BUTTON_KEY,BUTTON_MODE_GPIO);

   /* Initial EXTI */
  APP_ExtiConfig();
  
  /* Turn on the LED */
  BSP_LED_On(LED_GREEN);
  
  /* Wait for the button to be pressed */
  while(BSP_PB_GetState(BUTTON_KEY) == 0);
  
  /* LED off */
  BSP_LED_Off(LED_GREEN);
  
  /* Enter LpRun mode */
  APP_EnterLowPowerRun();

  /* Enter SLEEP mode */
  APP_EnterSleep();
  
  /* Switch the regulator from low-power mode to main mode */
  LL_PWR_DisableLowPowerRunMode();
  
  /* Wait for both the flags MR_RDY to be set. */
  while(LL_PWR_IsActiveFlag_MR_RDY() != 1)
  {}

  /* Configure system clock */
  APP_SystemClockConfig();

  /* Infinite loop */
  while (1)
  {
    /* LED toggle */
    BSP_LED_Toggle(LED_GREEN);
    LL_mDelay(500);
  }
}

/**
  * @brief  Configure Exti
  * @param  None
  * @retval None
  */
static void APP_ExtiConfig(void)
{
   LL_GPIO_InitTypeDef GPIO_InitStruct = {0};
   LL_EXTI_InitTypeDef EXTI_InitStruct = {0};
   /* Enable GPIOA clock */
   LL_AHB2_GRP1_EnableClock (LL_AHB2_GRP1_PERIPH_GPIOA);

   /* Choose PA06 Pin */
   GPIO_InitStruct.Pin = LL_GPIO_PIN_6;
   /* Choose input mode */
   GPIO_InitStruct.Mode = LL_GPIO_MODE_INPUT;
   /* Choose pull up */
   GPIO_InitStruct.Pull = LL_GPIO_PULL_UP;
   /* Initialze GPIOA */
   LL_GPIO_Init(GPIOA, &GPIO_InitStruct);

   /* Configure PA6 as the EXTI6 interrupt input */
   LL_SYSCFG_SetEXTISource(LL_SYSCFG_EXTI_PORTA, LL_SYSCFG_EXTI_LINE6);

   /* Enable EXTI6 */
   EXTI_InitStruct.Line_0_31 = LL_EXTI_LINE_6;

   EXTI_InitStruct.LineCommand = ENABLE;
   /* Interrupt Mode */
   EXTI_InitStruct.Mode = LL_EXTI_MODE_IT;
   /* Trigger Falling Mode */
   EXTI_InitStruct.Trigger = LL_EXTI_TRIGGER_FALLING;
   /* Initialize the EXTI registers according to the specified parameters in EXTI_InitStruct */
   LL_EXTI_Init(&EXTI_InitStruct);

   /* Set Interrupt Priority  */
   NVIC_SetPriority(EXTI9_5_IRQn,1);
   /* Enable interrupt */
   NVIC_EnableIRQ(EXTI9_5_IRQn);
}

/**
  * @brief  Enter SLEEP mode
  * @param  None
  * @retval None
  */
static void APP_EnterSleep(void)
{
  /* Enable PWR clock */
  LL_APB1_GRP1_EnableClock(LL_APB1_GRP1_PERIPH_PWR);
  
  /* Enter SLEEP mode */
  LL_LPM_EnableSleep();
  
  /* Wait for interrupt instruction */
  __WFI();
}

static void APP_SystemClockDecrease(void)
{
  /* Enable HSI */
  LL_RCC_HSI_Enable();
  while(LL_RCC_HSI_IsReady() != 1)
  {
  }

  /* Set AHB prescaler: HCLK = SYSCLK/4 */
  LL_RCC_SetAHBPrescaler(LL_RCC_SYSCLK_DIV_4);

  /* Select HSI as system clock source */
  LL_RCC_SetSysClkSource(LL_RCC_SYS_CLKSOURCE_HSI);
  while(LL_RCC_GetSysClkSource() != LL_RCC_SYS_CLKSOURCE_STATUS_HSI)
  {
  }

  /* Set APB1 & APB2 prescaler: PCLK1 = HCLK, PCLK2 = HCLK */
  LL_RCC_SetAPB1Prescaler(LL_RCC_APB1_DIV_1);
  LL_RCC_SetAPB2Prescaler(LL_RCC_APB2_DIV_1);
  
  /* Set systick to 1ms in using frequency set to 2MHz */
  LL_Init1msTick(2000000);

  /* Update the SystemCoreClock global variable(which can be updated also through SystemCoreClockUpdate function) */
  LL_SetSystemCoreClock(2000000);
}

/**
  * @brief  Enter low-power run mode
  * @param  None
  * @retval None
  */
static void APP_EnterLowPowerRun(void)
{
  /* Enable PWR clock */
  LL_APB1_GRP1_EnableClock(LL_APB1_GRP1_PERIPH_PWR);
  
  /* Configure the System clock as 2M Hz */
  APP_SystemClockDecrease();

  /* Switch the regulator from main mode to low-power mode */
  LL_PWR_EnableLowPowerRunMode();
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
