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
/* Private variables ---------------------------------------------------------*/
ADC_HandleTypeDef        hADC1;
TIM_HandleTypeDef        hTIM1;
TIM_MasterConfigTypeDef  sMasterConfig;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_AdcConfig(void);
static void APP_TimConfig(void);

/**
  * @brief  Main program.
  * @retval int
  */
int main(void)
{
  /* Reset of all peripherals, Initializes the Systick. */
  HAL_Init();
  
  /* Configure system clock */
  APP_SystemClockConfig(); 

  /* Initialize LED */
  BSP_LED_Init(LED_GREEN);

  /* Configure ADC */
  APP_AdcConfig();
   
  /* ADC Calibrate */   
  if (HAL_ADCEx_Calibration_Start(&hADC1, ADC_CALIBRATION_NUMBER_1) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  
  /* ADC Start */
  HAL_ADC_Start(&hADC1);
  
  /* TIM Cofig */
  APP_TimConfig();
  
  /* Infinite loop */
  while (1)
  {
  }
}

/**
  * @brief  Adc WatchDog Callback
  * @param  hadc：ADC handle
  * @retval None
  */
void HAL_ADC_LevelOutOfWindowCallback(ADC_HandleTypeDef* hadc)
{
  BSP_LED_On(LED_GREEN);
}

/**
  * @brief  ADC Configuration
  * @param  None
  * @retval None
  */
static void APP_AdcConfig(void)
{
  ADC_ChannelConfTypeDef   sConfig = {0};
  ADC_AnalogWDGConfTypeDef sAWDGConfig = {0};
  
  hADC1.Instance = ADC1;
  
  hADC1.Init.Resolution            = ADC_RESOLUTION_12B;               /* 12-bit resolution for converted data  */
  hADC1.Init.DataAlign             = ADC_DATAALIGN_RIGHT;              /* Right-alignment for converted data */
  hADC1.Init.GainCompensation      = 0;                                /* Gain compensation disabled */
  hADC1.Init.ScanConvMode          = ADC_SCAN_DISABLE;                 /* Scan Mode Disable */
  hADC1.Init.EOCSelection          = ADC_EOC_SINGLE_CONV;              /* End of unitary conversion flag */ 
  hADC1.Init.LowPowerAutoWait      = DISABLE;                          /* Low power mode disable */
  hADC1.Init.ContinuousConvMode    = DISABLE;                          /* Single Conversion  */
  hADC1.Init.NbrOfConversion       = 1;                                /* Conversion Number 1 */
  hADC1.Init.DiscontinuousConvMode = DISABLE;                          /* Discontinuous Mode Disable */
  /* hADC1.Init.NbrOfDiscConversion   = 1; */                          /* Discontinuous Conversion Number 1 */
  hADC1.Init.ExternalTrigConv      = ADC_EXTERNALTRIG_T1_TRGO;         /* TIM1 TRGO Trigger */
  hADC1.Init.ExternalTrigConvEdge  = ADC_EXTERNALTRIGCONVEDGE_RISING;  /* Rising edge trigger */
  hADC1.Init.OversamplingMode      = DISABLE;                          /* Oversampling disable */
  /* hADC1.Init.Oversampling.Ratio    = ADC_OVERSAMPLING_RATIO_16;                     */
  /* hADC1.Init.Oversampling.RightBitShift = ADC_RIGHTBITSHIFT_4;                      */
  /* hADC1.Init.Oversampling.TriggeredMode = ADC_TRIGGEREDMODE_MULTI_TRIGGER;          */
  /* hADC1.Init.Oversampling.OversamplingStopReset = ADC_REGOVERSAMPLING_RESUMED_MODE; */
  
  if (HAL_ADC_Init(&hADC1) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  
  sConfig.Channel      = ADC_CHANNEL_7;             
  sConfig.Rank         = ADC_REGULAR_RANK_1;        
  sConfig.SamplingTime = ADC_SAMPLETIME_640CYCLES_5; 
  sConfig.SingleDiff   = ADC_SINGLE_ENDED;          
  sConfig.OffsetNumber = ADC_OFFSET_NONE;           
  /* sConfig.Offset       = 0;                         */
  /* sConfig.OffsetSign   = ADC_OFFSET_SIGN_POSITIVE;  */
  /* sConfig.OffsetSaturation = DISABLE;               */
  if (HAL_ADC_ConfigChannel(&hADC1, &sConfig) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  
  sAWDGConfig.WatchdogNumber   = ADC_ANALOGWATCHDOG_1;
  sAWDGConfig.WatchdogMode     = ADC_ANALOGWATCHDOG_SINGLE_REG;
  sAWDGConfig.Channel          = ADC_CHANNEL_7;
  sAWDGConfig.ITMode           = ENABLE;
  sAWDGConfig.HighThreshold    = 0x800;
  sAWDGConfig.LowThreshold     = 0x0;
  sAWDGConfig.FilteringConfig  = ADC_AWD_FILTERING_NONE;
  if (HAL_ADC_AnalogWDGConfig(&hADC1, &sAWDGConfig) != HAL_OK)
  {
    APP_ErrorHandler();
  }
}

/**
  * @brief  TIM Configure function
  * @param  None
  * @retval None
  */
static void APP_TimConfig(void)
{
  __HAL_RCC_TIM1_CLK_ENABLE();                                    /* Enable TIM1 Clock */
  hTIM1.Instance = TIM1;                                          /* TIM1 */
  hTIM1.Init.Period            = 16000 - 1;                       /* Period = 16000-1  */
  hTIM1.Init.Prescaler         = 1000 - 1;                        /* Prescaler = 1000-1 */
  hTIM1.Init.ClockDivision     = TIM_CLOCKDIVISION_DIV1;          /* ClockDivision = 0 */
  hTIM1.Init.CounterMode       = TIM_COUNTERMODE_UP;              /* Counter Direction = up */
  hTIM1.Init.RepetitionCounter = 0;                               /* Repetition = 0  */
  hTIM1.Init.AutoReloadPreload = TIM_AUTORELOAD_PRELOAD_DISABLE;  /* Auto-Reload register not buffered */
  if (HAL_TIM_Base_Init(&hTIM1) != HAL_OK)                        /* Initialize TIM1 */
  {
    APP_ErrorHandler();
  }

  sMasterConfig.MasterOutputTrigger = TIM_TRGO_UPDATE;            /* Select Update Event as Trigger Source */
  sMasterConfig.MasterOutputTrigger2 = TIM_TRGO2_RESET;           /* SIMx_EGR.UG bit as Trigger Source */
  sMasterConfig.MasterSlaveMode = TIM_MASTERSLAVEMODE_DISABLE;    /* Disable Master/Slave Mode  */
  HAL_TIMEx_MasterConfigSynchronization(&hTIM1, &sMasterConfig);  /* Configure TIM1 */
  
  if (HAL_TIM_Base_Start(&hTIM1) != HAL_OK)                       /* TIM1 Start */
  {
    APP_ErrorHandler();
  }
}

/**
  * @brief  System Clock Configuration
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

/**
  * @brief  This function is executed in case of error occurrence.
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
  * @param  file: pointer to the source file name
  * @param  line: assert_param error line source number
  * @retval None
  */
void assert_failed(uint8_t *file, uint32_t line)
{
  /* User can add his own implementation to report the file name and line number,
     for example: printf("Wrong parameters value: file %s on line %d\r\n", file, line)  */
  /* Infinite loop */
  while (1)
  {
  }
}
#endif /* USE_FULL_ASSERT */

/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
