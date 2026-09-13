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
ADC_HandleTypeDef        hADC2;
uint32_t                 uhADCxConvertedData;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_AdcConfig(void);

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

  /* Initialize UART */
  BSP_UART_Config();

  /* Configure ADC */
  APP_AdcConfig();

  while(1)
  {
    /* ADC Start */
    HAL_ADC_Start(&hADC2);
    HAL_ADC_Start(&hADC1);
    
    /* Poll For ADC Conversion */
    if(HAL_ADC_PollForConversion(&hADC1, 2000) == HAL_OK)
    {
      /* Get ADC Value */
      uhADCxConvertedData = HAL_ADC_GetValue(&hADC1);
      /* Printf VCC Data */
      printf("ADC1 DR: %u\r\n",(unsigned int)(uhADCxConvertedData & 0xFFFF));
      printf("ADC2 DR: %u\r\n",(unsigned int)(uhADCxConvertedData >> 16));
    }
    HAL_Delay(1000);
  } 
}

/**
  * @brief  ADC Configuration
  * @param  None
  * @retval None
  */
static void APP_AdcConfig(void)
{
  ADC_ChannelConfTypeDef     sConfig = {0};
  ADC_MultiModeTypeDef MultiModeInit = {0};
  
  hADC1.Instance = ADC1;
  
  hADC1.Init.Resolution            = ADC_RESOLUTION_12B;               /* 12-bit resolution for converted data  */
  hADC1.Init.DataAlign             = ADC_DATAALIGN_RIGHT;              /* Right-alignment for converted data */
  hADC1.Init.GainCompensation      = 0;                                /* Gain compensation disabled */
  hADC1.Init.ScanConvMode          = ADC_SCAN_DISABLE;                 /* Scan Mode Disable */
  hADC1.Init.EOCSelection          = ADC_EOC_SINGLE_CONV;              /* End of unitary conversion flag */ 
  hADC1.Init.LowPowerAutoWait      = DISABLE;                          /* Low power mode disable */
  hADC1.Init.ContinuousConvMode    = DISABLE;                          /* Single Mode */
  hADC1.Init.NbrOfConversion       = 1;                                /* Conversion Number 1 */
  hADC1.Init.DiscontinuousConvMode = DISABLE;                          /* Discontinuous Mode Disable */
  /* hADC1.Init.NbrOfDiscConversion   = 1; */                          /* Discontinuous Conversion Number 1 */
  hADC1.Init.ExternalTrigConv      = ADC_SOFTWARE_START;               /* SW Trigger */
  /* hADC1.Init.ExternalTrigConvEdge  = ADC_EXTERNALTRIGCONVEDGE_RISING; */  /* Rising edge trigger */
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
  sConfig.SamplingTime = ADC_SAMPLETIME_24CYCLES_5;
  sConfig.SingleDiff   = ADC_SINGLE_ENDED;
  sConfig.OffsetNumber = ADC_OFFSET_NONE;
  /* sConfig.Offset       = 0;                         */
  /* sConfig.OffsetSign   = ADC_OFFSET_SIGN_POSITIVE;  */
  /* sConfig.OffsetSaturation = DISABLE;               */
  if (HAL_ADC_ConfigChannel(&hADC1, &sConfig) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  
  MultiModeInit.Mode=ADC_DUALMODE_REGSIMULT;
  HAL_ADCEx_MultiModeConfigChannel(&hADC1,&MultiModeInit);
  
  hADC2.Instance = ADC2;
  
  hADC2.Init.Resolution            = ADC_RESOLUTION_12B;               /* 12-bit resolution for converted data  */
  hADC2.Init.DataAlign             = ADC_DATAALIGN_RIGHT;              /* Right-alignment for converted data */
  hADC2.Init.GainCompensation      = 0;                                /* Gain compensation disabled */
  hADC2.Init.ScanConvMode          = ADC_SCAN_DISABLE;                 /* Scan Mode Disable */
  hADC2.Init.EOCSelection          = ADC_EOC_SINGLE_CONV;              /* End of unitary conversion flag */ 
  hADC2.Init.LowPowerAutoWait      = DISABLE;                          /* Low power mode disable */
  hADC2.Init.ContinuousConvMode    = DISABLE;                          /* Single Mode */
  hADC2.Init.NbrOfConversion       = 1;                                /* Conversion Number 1 */
  hADC2.Init.DiscontinuousConvMode = DISABLE;                          /* Discontinuous Mode Disable */
  /* hADC2.Init.NbrOfDiscConversion   = 1; */                          /* Discontinuous Conversion Number 1 */
  hADC2.Init.ExternalTrigConv      = ADC_SOFTWARE_START;               /* SW Trigger */
  /* hADC2.Init.ExternalTrigConvEdge  = ADC_EXTERNALTRIGCONVEDGE_RISING; */  /* Rising edge trigger */
  hADC2.Init.OversamplingMode      = DISABLE;                          /* Oversampling disable */
  /* hADC2.Init.Oversampling.Ratio    = ADC_OVERSAMPLING_RATIO_16;                     */
  /* hADC2.Init.Oversampling.RightBitShift = ADC_RIGHTBITSHIFT_4;                      */
  /* hADC2.Init.Oversampling.TriggeredMode = ADC_TRIGGEREDMODE_MULTI_TRIGGER;          */
  /* hADC2.Init.Oversampling.OversamplingStopReset = ADC_REGOVERSAMPLING_RESUMED_MODE; */
  
  if (HAL_ADC_Init(&hADC2) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  
  sConfig.Channel      = ADC_CHANNEL_8;
  sConfig.Rank         = ADC_REGULAR_RANK_1;
  sConfig.SamplingTime = ADC_SAMPLETIME_24CYCLES_5;
  sConfig.SingleDiff   = ADC_SINGLE_ENDED;
  sConfig.OffsetNumber = ADC_OFFSET_NONE;
  /* sConfig.Offset       = 0;                         */
  /* sConfig.OffsetSign   = ADC_OFFSET_SIGN_POSITIVE;  */
  /* sConfig.OffsetSaturation = DISABLE;               */
  if (HAL_ADC_ConfigChannel(&hADC2, &sConfig) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  
  /* ADC Calibrate */   
  if (HAL_ADCEx_Calibration_Start(&hADC1, ADC_CALIBRATION_NUMBER_1) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  
  /* ADC Calibrate */   
  if (HAL_ADCEx_Calibration_Start(&hADC2, ADC_CALIBRATION_NUMBER_1) != HAL_OK)
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
