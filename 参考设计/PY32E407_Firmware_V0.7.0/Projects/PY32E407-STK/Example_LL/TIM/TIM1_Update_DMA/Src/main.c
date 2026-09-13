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
uint32_t TIM1ReloadCounter[9] = {900-1,800-1,700-1,600-1,500-1,400-1,300-1,200-1,100-1};

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_ConfigTIM1(void);
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

  while (1)
  {
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
  TIM1CountInit.RepetitionCounter   = 0;                        /* RepetitionCounter value：0    */
  
  /* Initialize TIM1 */
  LL_TIM_Init(TIM1,&TIM1CountInit);

  /* Clear update flag */
  LL_TIM_ClearFlag_UPDATE(TIM1);
    
  /* Enable UPDATE interrupt */
  LL_TIM_EnableIT_UPDATE(TIM1);

  /* Enable UPDATE DMA request */
  LL_TIM_EnableDMAReq_UPDATE(TIM1);

  /* Enable UPDATE interrupt request */
  NVIC_EnableIRQ(TIM1_UP_TIM10_IRQn);
  NVIC_SetPriority(TIM1_UP_TIM10_IRQn,0);
  
  /* Enable TIM1 */
  LL_TIM_EnableCounter(TIM1);
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
  DMA_InitType.SrcAddress    = (uint32_t)&TIM1ReloadCounter;       /* Source Address Settings */
  DMA_InitType.DstAddress    = (uint32_t)&(TIM1->ARR);             /* Dest Address Settings */
  DMA_InitType.Direction     = LL_DMA_DIRECTION_MEMORY_TO_PERIPH;  /* Periph to Memory mode */
  DMA_InitType.SrcIncMode    = LL_DMA_SRC_ADDR_INC;                /* Disable Src increment mode */
  DMA_InitType.DstIncMode    = LL_DMA_DST_ADDR_FIX;                /* Enable Dst increment mode */
  DMA_InitType.SrcWidth      = LL_DMA_SRC_WIDTH_WORD;              /* Source data width is 32 bits */
  DMA_InitType.DstWidth      = LL_DMA_DST_WIDTH_WORD;              /* Dest data width is 32 bits */
  DMA_InitType.Priority      = LL_DMA_PRIORITY_0;                  /* Channel priority is 0 */
 /* DMA_InitType.SrcBurstLen   = LL_DMA_SRC_BURST_LEN_1;  */
  DMA_InitType.DstBurstLen   = LL_DMA_DST_BURST_LEN_1; 
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

  /* Enable DMA Block Interrupt */
  LL_DMA_EnableIT_BLOCK(DMA1,LL_DMA_CHANNEL_1);

  /* Enable DMA Interrupt */
  LL_DMA_EnableIT(DMA1,LL_DMA_CHANNEL_1);

  /* Set Block Length 9 */
  LL_DMA_SetBlockLength(DMA1,LL_DMA_CHANNEL_1,9);

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
