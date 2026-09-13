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
#define ADC_CALIBRATION_TIMEOUT_MS         5

/* Private variables ---------------------------------------------------------*/
/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_AdcConfig(void);
static void APP_AdcCalibrate(void);
static void APP_AdcEnable(void);
static void APP_TimerInit(void);

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

  /* LED Init */
  BSP_LED_Init(LED_GREEN);

  /* ADC configuration */
  APP_AdcConfig();

  /* ADC Calibrate */
  APP_AdcCalibrate();

  /* ADC Enable */ 
  APP_AdcEnable();

  LL_ADC_REG_StartConversion(ADC1);

  APP_TimerInit();

  while (1)
  {
  }
}

/**
  * @brief  Watchdog callback function
  * @param  None
  * @retval None
  */
void APP_AdcAnalogWatchdogCallback(void)
{  
  BSP_LED_On(LED_GREEN);
}

/**
  * @brief  TIM Configuration Function.
  * @param  None
  * @retval None
  */
static void APP_TimerInit()
{
  /* Enable TIM2 clock */
  LL_APB1_GRP1_EnableClock(LL_APB1_GRP1_PERIPH_TIM2);

  /* Set TIM2 prescaler */
  LL_TIM_SetPrescaler(TIM2,2000);

  /* Set TIM2 automatic reload value */
  LL_TIM_SetAutoReload(TIM2, 4000);

  /* Set TIM2 update trigger */
  LL_TIM_SetTriggerOutput(TIM2,LL_TIM_TRGO_UPDATE);

  /* Enable TIM2 */
  LL_TIM_EnableCounter(TIM2);
}

/**
  * @brief  ADC calibration function.
  * @param  None
  * @retval None
  */
static void APP_AdcCalibrate()
{
  #if (USE_TIMEOUT == 1)
  uint32_t Timeout = 0; /* Variable used for timeout management */
  #endif /* USE_TIMEOUT */
  
  if (LL_ADC_IsEnabled(ADC1) == 0)
  { 
    LL_ADC_StartCalibration(ADC1,LL_ADC_CALIBRATION_NUMBER_1);
    
    /* Poll for ADC effectively calibrated */
    #if (USE_TIMEOUT == 1)
    Timeout = ADC_CALIBRATION_TIMEOUT_MS;
    #endif /* USE_TIMEOUT */
    
    while (LL_ADC_IsCalibrationOnGoing(ADC1) != 0)
    {
    #if (USE_TIMEOUT == 1)
      /* Check Systick counter flag to decrement the time-out value */
      if (LL_SYSTICK_IsActiveCounterFlag())
      {
        if(Timeout-- == 0)
        {
          APP_ErrorHandler();
        }
      }
    #endif /* USE_TIMEOUT */
    }  

    LL_mDelay(1);
  }
}

/**
  * @brief  ADC Enable
  * @param  None
  * @retval None
  */
static void APP_AdcEnable(void)
{
  /* Enable ADC */
  LL_ADC_Enable(ADC1);
  
  LL_mDelay(1);
}

/**
  * @brief ADC1 Initialization Function
  * @param None
  * @retval None
  */
static void APP_AdcConfig(void)
{
  LL_APB2_GRP1_EnableClock(LL_APB2_GRP1_PERIPH_ADC1);
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOC);

  /* Configure pins PC0 as analog */
  LL_GPIO_SetPinMode(GPIOC, LL_GPIO_PIN_0, LL_GPIO_MODE_ANALOG);
  LL_SYSCFG_EnableGPIOCAnalog2(LL_SYSCFG_GPIOC_PIN_0);

  LL_RCC_SetADCClockSource(LL_RCC_ADC_CLKSOURCE_PCLK2_DIV4);                                /* Clock Sync Pclk div4 */

  LL_ADC_SetDataAlignment(ADC1,LL_ADC_DATA_ALIGN_RIGHT);                                    /* DataAlignment Right */
  LL_ADC_SetResolution(ADC1,LL_ADC_RESOLUTION_12B);                                         /* Resolution 12 */
  LL_ADC_SetLowPowerMode(ADC1,LL_ADC_LP_MODE_NONE);                                         /* LowPowerMode None */

  LL_ADC_REG_SetTriggerSource(ADC1,LL_ADC_REG_TRIG_EXT_TIM2_TRGO);                          /* Trigger TIM2 TRGO */
  LL_ADC_SetSequencersScanMode(ADC1,LL_ADC_SEQ_SCAN_DISABLE);                               /* Disable Scan */
  LL_ADC_REG_SetSequencerLength(ADC1,LL_ADC_REG_SEQ_SCAN_DISABLE);                          /* Disable Scan */
  LL_ADC_REG_SetDMATransfer(ADC1,LL_ADC_REG_DMA_TRANSFER_NONE);                             /* Disable DMA */ 
  LL_ADC_REG_SetContinuousMode(ADC1,LL_ADC_REG_CONV_SINGLE);                                /* Single Mode */

  LL_ADC_SetGainCompensation(ADC1, 0);
  LL_ADC_SetOverSamplingScope(ADC1, LL_ADC_OVS_DISABLE);

  LL_ADC_REG_SetSequencerRanks(ADC1, LL_ADC_REG_RANK_1, LL_ADC_CHANNEL_11);                 /* Channel Channel */

  LL_ADC_SetChannelSamplingTime(ADC1, LL_ADC_CHANNEL_11, LL_ADC_SAMPLINGTIME_247CYCLES_5);  /* Sampletime 247.5 */

  /* Set channel 11 as the watchdog channel */
  LL_ADC_SetAnalogWDMonitChannels(ADC1, LL_ADC_AWD1,LL_ADC_AWD_CHANNEL_11_REG);
  
  /* Set the upper limit of the watchdog to 0x800 */
  LL_ADC_SetAnalogWDThresholds(ADC1, LL_ADC_AWD1, LL_ADC_AWD_THRESHOLD_HIGH,0x800);
  
  /* Set the lower limit of the watchdog to 0x000 */
  LL_ADC_SetAnalogWDThresholds(ADC1,LL_ADC_AWD1, LL_ADC_AWD_THRESHOLD_LOW, 0x000);
  
  /* Enable watchdog interrupt */
  LL_ADC_EnableIT_AWD1(ADC1); 

  NVIC_SetPriority(ADC1_2_IRQn, 0);
  NVIC_EnableIRQ(ADC1_2_IRQn);
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
