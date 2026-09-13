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
uint32_t TIM1DataBuff[] = {2,200,3,300,4,400,5,500,6,600,7,700,8,800,9,900};
__IO int32_t datalength = 16;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_ConfigTIM1(void);
static void APP_ConfigPWMChannel(void);
static void APP_ConfigDMATIM1Reload(void);

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

  /* Configure and enable DMA */
  APP_ConfigDMATIM1Reload();

  /* Configure and enable TIM1 counter mode */
  APP_ConfigTIM1();

  APP_ConfigPWMChannel();

  while (1)
  {
  }
}

/**
  * @brief  DMA Block CallBack
  * @param  None
  * @retval None
  */
void APP_DMABlockCallback()
{
  datalength = datalength - 2 ;
  if(datalength > 0)
  {
    LL_DMA_EnableChannel(DMA1,LL_DMA_CHANNEL_1);
  }
}

/**
  * @brief  Configure TIM count mode
  * @param  None
  * @retval None
  */
static void APP_ConfigTIM1(void)
{
  /* Configure TIM1 */
  LL_TIM_InitTypeDef TIM1CountInit = {0};
  
  /* Enable TIM1 clock */
  LL_APB2_GRP1_EnableClock(LL_APB2_GRP1_PERIPH_TIM1);
  
  TIM1CountInit.ClockDivision       = LL_TIM_CLOCKDIVISION_DIV1;/* No clock division             */
  TIM1CountInit.CounterMode         = LL_TIM_COUNTERMODE_UP;    /* Up counting mode              */
  TIM1CountInit.Prescaler           = 16000-1;                  /* Prescaler：16000              */
  TIM1CountInit.Autoreload          = 1000-1;                   /* Autoreload value：1000        */
  TIM1CountInit.RepetitionCounter   = 1;                        /* RepetitionCounter value：1    */
  
  /* Initialize TIM1 */
  LL_TIM_Init(TIM1,&TIM1CountInit);

  /* Enable update event DMA request */
  LL_TIM_EnableDMAReq_UPDATE(TIM1);
  
  /* Configure DMA burst transfer */
  LL_TIM_ConfigDMABurst(TIM1,LL_TIM_DMABURST_BASEADDR_RCR,LL_TIM_DMABURST_LENGTH_2TRANSFERS);
  
  /* Enable output */
  LL_TIM_EnableAllOutputs(TIM1);

  /* Enable TIM1 */
  LL_TIM_EnableCounter(TIM1);
}

/**
  * @brief  Configure TIM1 PWM mode and related GPIO
  * @param  None
  * @retval None
  */
static void APP_ConfigPWMChannel(void)
{
  LL_GPIO_InitTypeDef TIM1CH1MapInit= {0};
  LL_TIM_OC_InitTypeDef TIM_OC_Initstruct ={0};

  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOA);

  /* Configure PA8 as TIM1_CH1 */
  TIM1CH1MapInit.Pin        = LL_GPIO_PIN_8;
  TIM1CH1MapInit.Mode       = LL_GPIO_MODE_ALTERNATE;
  TIM1CH1MapInit.Alternate  = LL_GPIO_AF_6; 
  TIM1CH1MapInit.Speed      = LL_GPIO_SPEED_FREQ_HIGH;
  TIM1CH1MapInit.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  TIM1CH1MapInit.Pull       = LL_GPIO_PULL_NO;
  LL_GPIO_Init(GPIOA,&TIM1CH1MapInit);

  TIM_OC_Initstruct.OCMode        = LL_TIM_OCMODE_PWM1;     /* mode：PWM1 */
  TIM_OC_Initstruct.OCState       = LL_TIM_OCSTATE_ENABLE;  /* Channel enable */
  TIM_OC_Initstruct.OCNState      = LL_TIM_OCSTATE_DISABLE; /* Complementary channel disable */
  TIM_OC_Initstruct.OCPolarity    = LL_TIM_OCPOLARITY_HIGH; /* Compare output polarity: HIGH */
  TIM_OC_Initstruct.OCNPolarity   = LL_TIM_OCPOLARITY_HIGH; /* Compare complementary output polarity: HIGH */
  TIM_OC_Initstruct.OCIdleState   = LL_TIM_OCIDLESTATE_LOW; /* Output Idle state: LOW */
  TIM_OC_Initstruct.OCNIdleState  = LL_TIM_OCIDLESTATE_LOW; /* Complementary output Idle state: LOW */
  /* Channel 1 comparison value:100 */
  TIM_OC_Initstruct.CompareValue  = 100;
  /* Configure channel 1 */
  LL_TIM_OC_Init(TIM1,LL_TIM_CHANNEL_CH1,&TIM_OC_Initstruct);
}

/**
  * @brief  Configure DMA transfer
  * @param  None
  * @retval None
  */
static void APP_ConfigDMATIM1Reload(void)
{
  LL_DMA_InitTypeDef DMA_InitType ={0};

  LL_AHB1_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_DMA1);
 
  /* Configure DMA function parameters */
  DMA_InitType.SrcAddress    = (uint32_t)&TIM1DataBuff;             /* Source Address Settings */
  DMA_InitType.DstAddress    = (uint32_t)&(TIM1->DMAR);             /* Dest Address Settings */
  DMA_InitType.Direction     = LL_DMA_DIRECTION_MEMORY_TO_PERIPH;   /* Memory to Periph mode */
  DMA_InitType.SrcIncMode    = LL_DMA_SRC_ADDR_INC;                 /* Enable Src increment mode */
  DMA_InitType.DstIncMode    = LL_DMA_DST_ADDR_FIX;                 /* Disable Dst increment mode */
  DMA_InitType.SrcWidth      = LL_DMA_SRC_WIDTH_WORD;               /* Source data width is 32 bits */
  DMA_InitType.DstWidth      = LL_DMA_DST_WIDTH_WORD;               /* Dest data width is 32 bits */
  DMA_InitType.Priority      = LL_DMA_PRIORITY_0;                   /* Channel priority is 0 */
 /* DMA_InitType.SrcBurstLen   = LL_DMA_SRC_BURST_LEN_1;  */
  DMA_InitType.DstBurstLen   = LL_DMA_DST_BURST_LEN_4;              /* DST Burst Length is 4 */
 /* DMA_InitType.SrcHandshakeType = LL_DMA_SRC_HANDSHAKE_TYPE_HARD; */
  DMA_InitType.DstHandshakeType = LL_DMA_DST_HANDSHAKE_TYPE_HARD; 
 
  /* Initialize DMA */
  if (LL_DMA_Init(DMA1, LL_DMA_CHANNEL_1, &DMA_InitType) != SUCCESS)
  {
    APP_ErrorHandler();
  }

  LL_DMA_SetDstPeriphMap(DMA1, LL_DMA_CHANNEL_1, LL_DMA_DST_PERIPH_MAP_1);
  LL_SYSCFG_SetDMARemap(DMA1,LL_SYSCFG_DMA_PERIPH_MAP_1,LL_SYSCFG_DMA_MAP_TIM1_UP);

  /* Enable DMA channel 1 interrupt request */
  NVIC_EnableIRQ(DMA1_Channel1_IRQn);
  NVIC_SetPriority(DMA1_Channel1_IRQn,0);

  /* Clear Block1 Flag */  
  LL_DMA_ClearFlag_BLOCK1(DMA1); 

  /* Enable DMA Block Interrupt */
  LL_DMA_EnableIT_BLOCK(DMA1,LL_DMA_CHANNEL_1);

  /* Enable DMA Interrupt */
  LL_DMA_EnableIT(DMA1,LL_DMA_CHANNEL_1);

  LL_DMA_SetBlockLength(DMA1,LL_DMA_CHANNEL_1,2);

  /* DMA Enable */
  LL_DMA_Enable(DMA1);
  
  /* Enable DMA channel 1 */
  LL_DMA_EnableChannel(DMA1,LL_DMA_CHANNEL_1);
}

/**
  * @brief  TIM update interrupt callback function
  * @param  None
  * @retval None
  */
void APP_UpdateCallback(void)
{ 
  /* toggle LED */
  BSP_LED_Toggle(LED_GREEN);
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
