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
#define VAR_CONVERTED_DATA_INIT_VALUE      (__LL_ADC_DIGITAL_SCALE(LL_ADC_RESOLUTION_12B) + 1)
#define VDDA_APPLI                         ((uint32_t)3300)

/* Private variables ---------------------------------------------------------*/
__IO uint32_t wait_loop_index = 0;
__IO uint32_t ADCxConvertedDatas[4];
__IO uint16_t uhADCxConvertedData_Voltage_mVolt = 0;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_DmaConfig(void);
static void APP_TimConfig(void);
static void APP_AdcConfig(void);
static void APP_AdcCalibrate(void);
static void APP_AdcEnable(void);

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

  /* ADC Config */
  APP_AdcConfig();

  /* DMA Config */
  APP_DmaConfig();

  /* ADC Calibrate */
  APP_AdcCalibrate();

  /* ADC Enable */ 
  APP_AdcEnable();

  /* ADC Start */
  LL_ADC_REG_StartConversion(ADC1);

  /* Tim Config */
  APP_TimConfig();

  while (1)
  {
  }
}

/**
  * @brief  ADC interrupt function.
  * @param  None
  * @retval None
  */
void APP_TransferCompleteCallback()
{
  /*Convert ADC sampling values to voltage values*/
  uhADCxConvertedData_Voltage_mVolt = __LL_ADC_CALC_DATA_TO_VOLTAGE(VDDA_APPLI, ADCxConvertedDatas[0], LL_ADC_RESOLUTION_12B);
  printf("%s%s%u mV\r\n","Channel11","Voltage:",uhADCxConvertedData_Voltage_mVolt);
  
  /*Convert ADC sampling values to voltage values*/
  uhADCxConvertedData_Voltage_mVolt = __LL_ADC_CALC_DATA_TO_VOLTAGE(VDDA_APPLI, ADCxConvertedDatas[1], LL_ADC_RESOLUTION_12B);
  printf("%s%s%u mV\r\n","Channel7","Voltage:",uhADCxConvertedData_Voltage_mVolt);
  
  /*Convert ADC sampling values to voltage values*/
  uhADCxConvertedData_Voltage_mVolt = __LL_ADC_CALC_DATA_TO_VOLTAGE(VDDA_APPLI, ADCxConvertedDatas[2], LL_ADC_RESOLUTION_12B);
  printf("%s%s%u mV\r\n","Channel8","Voltage:",uhADCxConvertedData_Voltage_mVolt);
  
  /*Convert ADC sampling values to voltage values*/
  uhADCxConvertedData_Voltage_mVolt = __LL_ADC_CALC_DATA_TO_VOLTAGE(VDDA_APPLI, ADCxConvertedDatas[3], LL_ADC_RESOLUTION_12B);
  printf("%s%s%u mV\r\n\r\n","Channel9","Voltage:",uhADCxConvertedData_Voltage_mVolt);
}

/**
  * @brief  TIM Config function
  * @param  None
  * @retval None
  */
static void APP_TimConfig(void)
{
  /* Enable TIM1 clock */
  LL_APB2_GRP1_EnableClock(LL_APB2_GRP1_PERIPH_TIM1);

  /* Set TIM1 prescaler */
  LL_TIM_SetPrescaler(TIM1,2000);

  /* Set TIM1 auto-reload value */
  LL_TIM_SetAutoReload(TIM1, 8000);

  /* Set TIM1 update trigger */
  LL_TIM_SetTriggerOutput(TIM1,LL_TIM_TRGO_UPDATE);

  /* Enable TIM1 */
  LL_TIM_EnableCounter(TIM1);
}

/**
  * @brief  Dma Config.
  * @param  None
  * @retval None
  */
static void APP_DmaConfig()
{
  LL_DMA_InitTypeDef DMA_Initstruct = {0};

  LL_AHB1_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_DMA1);

  /* Configure DMA function parameters */
  DMA_Initstruct.SrcAddress    = (uint32_t)&ADC1->DR;;               /* Source Address Settings */
  DMA_Initstruct.DstAddress    = (uint32_t)&ADCxConvertedDatas;      /* Dest Address Settings */
  DMA_Initstruct.Direction     = LL_DMA_DIRECTION_PERIPH_TO_MEMORY;  /* Periph to Memory mode */
  DMA_Initstruct.SrcIncMode    = LL_DMA_SRC_ADDR_FIX;                /* Disable Src increment mode */
  DMA_Initstruct.DstIncMode    = LL_DMA_DST_ADDR_INC;                /* Enable Dst increment mode */
  DMA_Initstruct.SrcWidth      = LL_DMA_SRC_WIDTH_WORD;              /* Source data width is 32 bits */
  DMA_Initstruct.DstWidth      = LL_DMA_DST_WIDTH_WORD;              /* Dest data width is 32 bits */
  DMA_Initstruct.Priority      = LL_DMA_PRIORITY_0;                  /* Channel priority is 0 */
  DMA_Initstruct.SrcBurstLen   = LL_DMA_SRC_BURST_LEN_1;  
/*  DMA_Initstruct.DstBurstLen   = LL_DMA_DST_BURST_LEN_1; */ 
  DMA_Initstruct.SrcHandshakeType = LL_DMA_SRC_HANDSHAKE_TYPE_HARD;  
/*  DMA_Initstruct.DstHandshakeType = LL_DMA_DST_HANDSHAKE_TYPE_HARD; */
 
  /* Initialize DMA */
  if (LL_DMA_Init(DMA1, LL_DMA_CHANNEL_1, &DMA_Initstruct) != SUCCESS)
  {
    APP_ErrorHandler();
  }
  /* Enable Dest Reload */
  LL_DMA_EnableDstReload(DMA1,LL_DMA_CHANNEL_1);

  LL_DMA_SetSrcPeriphMap(DMA1, LL_DMA_CHANNEL_1, LL_DMA_SRC_PERIPH_MAP_1);
  LL_SYSCFG_SetDMARemap(DMA1,LL_SYSCFG_DMA_PERIPH_MAP_1,LL_SYSCFG_DMA_MAP_ADC1);

  /* Enable Block Interrupt */
  LL_DMA_EnableIT_BLOCK(DMA1,LL_DMA_CHANNEL_1);

  /* Enable Interrupt */
  LL_DMA_EnableIT(DMA1,LL_DMA_CHANNEL_1);

  LL_DMA_ClearFlag_BLOCK1(DMA1);

  /* Set Block Length 4 */
  LL_DMA_SetBlockLength(DMA1,LL_DMA_CHANNEL_1,4);

  /* Enable DMA */
  LL_DMA_Enable(DMA1);

  /* Enable DMA Channel1*/
  LL_DMA_EnableChannel(DMA1, LL_DMA_CHANNEL_1);

  NVIC_SetPriority(DMA1_Channel1_IRQn, 0);
  NVIC_EnableIRQ(DMA1_Channel1_IRQn);
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

  /* Configure pins PC0/PC1/PC2/PC3 as analog */
  LL_GPIO_SetPinMode(GPIOC, LL_GPIO_PIN_0, LL_GPIO_MODE_ANALOG);
  LL_GPIO_SetPinMode(GPIOC, LL_GPIO_PIN_1, LL_GPIO_MODE_ANALOG);
  LL_GPIO_SetPinMode(GPIOC, LL_GPIO_PIN_2, LL_GPIO_MODE_ANALOG);
  LL_GPIO_SetPinMode(GPIOC, LL_GPIO_PIN_3, LL_GPIO_MODE_ANALOG);

  /* Enable PC0/PC1/PC2/PC3 analog2 */
  LL_SYSCFG_EnableGPIOCAnalog2(LL_SYSCFG_GPIOC_PIN_0);                                    
  LL_SYSCFG_EnableGPIOCAnalog2(LL_SYSCFG_GPIOC_PIN_1);
  LL_SYSCFG_EnableGPIOCAnalog2(LL_SYSCFG_GPIOC_PIN_2);
  LL_SYSCFG_EnableGPIOCAnalog2(LL_SYSCFG_GPIOC_PIN_3);

  /* Clock Sync Pclk div4 */
  LL_RCC_SetADCClockSource(LL_RCC_ADC_CLKSOURCE_PCLK2_DIV4);                                
  
  /* DataAlignment Right */
  LL_ADC_SetDataAlignment(ADC1,LL_ADC_DATA_ALIGN_RIGHT);  
  
  /* Resolution 12 */
  LL_ADC_SetResolution(ADC1,LL_ADC_RESOLUTION_12B);                                         
  
  /* LowPowerMode None */
  LL_ADC_SetLowPowerMode(ADC1,LL_ADC_LP_MODE_NONE);                                         

  /* Trigger TIM1_TRGO */
  LL_ADC_REG_SetTriggerSource(ADC1,LL_ADC_REG_TRIG_EXT_TIM1_TRGO);                               

  /* Enable Scan */
  LL_ADC_SetSequencersScanMode(ADC1,LL_ADC_SEQ_SCAN_ENABLE);

  /* Scan 4 Ranks */
  LL_ADC_REG_SetSequencerLength(ADC1,LL_ADC_REG_SEQ_SCAN_ENABLE_4RANKS);                  
  
  /* Enable DMA */
  LL_ADC_REG_SetDMATransfer(ADC1,LL_ADC_REG_DMA_TRANSFER);                                   
 
  /* Single Mode */
  LL_ADC_REG_SetContinuousMode(ADC1,LL_ADC_REG_CONV_SINGLE);                                

  LL_ADC_SetGainCompensation(ADC1, 0);
  LL_ADC_SetOverSamplingScope(ADC1, LL_ADC_OVS_DISABLE);

  /* Rank1 Channel11 */
  LL_ADC_REG_SetSequencerRanks(ADC1, LL_ADC_REG_RANK_1, LL_ADC_CHANNEL_11);       

  /* Rank2 Channel7 */
  LL_ADC_REG_SetSequencerRanks(ADC1, LL_ADC_REG_RANK_2, LL_ADC_CHANNEL_7);                  
  
  /* Rank3 Channel8 */
  LL_ADC_REG_SetSequencerRanks(ADC1, LL_ADC_REG_RANK_3, LL_ADC_CHANNEL_8);                  
  
  /* Rank4 Channel9 */
  LL_ADC_REG_SetSequencerRanks(ADC1, LL_ADC_REG_RANK_4, LL_ADC_CHANNEL_9);                 

  /* Sampletime 247.5 */
  LL_ADC_SetChannelSamplingTime(ADC1, LL_ADC_CHANNEL_11, LL_ADC_SAMPLINGTIME_247CYCLES_5);  

  /* Sampletime 247.5 */
  LL_ADC_SetChannelSamplingTime(ADC1, LL_ADC_CHANNEL_7, LL_ADC_SAMPLINGTIME_247CYCLES_5);  
  
  /* Sampletime 247.5 */
  LL_ADC_SetChannelSamplingTime(ADC1, LL_ADC_CHANNEL_8, LL_ADC_SAMPLINGTIME_247CYCLES_5);   
  
  /* Sampletime 247.5 */
  LL_ADC_SetChannelSamplingTime(ADC1, LL_ADC_CHANNEL_9, LL_ADC_SAMPLINGTIME_247CYCLES_5);   
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
