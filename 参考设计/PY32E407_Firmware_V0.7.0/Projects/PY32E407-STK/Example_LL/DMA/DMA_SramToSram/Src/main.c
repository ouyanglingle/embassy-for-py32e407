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
#define BUFFER_SIZE              32

/* Private variables ---------------------------------------------------------*/
uint32_t aSRC_Const_Buffer[BUFFER_SIZE];       /* Data transmission source buffer */
uint32_t aDST_Buffer[BUFFER_SIZE];             /* Data transmission destination buffer */
__IO uint32_t transferCompleteDetected=0;      /* When the transmission is completed, this position 1 */
__IO uint32_t transferErrorDetected=0;         /* When a transmission error occurs, this position 1 */
__IO uint32_t transferFailedDetected=0;        /* When there is an error in transmitting data, this position 1 */

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_DmaConfig(void);

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

  /* Initialize data for DMA source buffer */
  for(uint8_t i=0; i<BUFFER_SIZE; i++)
  {
    aSRC_Const_Buffer[i]=i;
  }

  /* Configuring DMA */
  APP_DmaConfig();

  while (1)
  {   
    /* DMA transfer completed, but data is incorrect */
    if(transferFailedDetected == 1 && transferCompleteDetected == 1)
    {
      while(1)
      {
        BSP_LED_Toggle(LED_GREEN);
        LL_mDelay(500);
      }
    }

    /* DMA transmission completed and data correct */
    if(transferFailedDetected == 0 && transferCompleteDetected == 1)
    {
      BSP_LED_On(LED_GREEN);
      while(1)
      {
      }
    }

    /* DMA transmission error */
    if(transferErrorDetected == 1)
    {
      BSP_LED_On(LED_GREEN);
      while(1)
      {
        BSP_LED_Toggle(LED_GREEN);
        LL_mDelay(500);
      }
    } 
  }
}

/**
  * @brief  DMA configuration function
  * @param  None
  * @retval None
  */
static void APP_DmaConfig(void)
{
  LL_DMA_InitTypeDef DMA_Initstruct = {0};

  /* Enable DMA clock */
  LL_AHB1_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_DMA1);

  /* Configure DMA function parameters */
  DMA_Initstruct.SrcAddress    = (uint32_t)&aSRC_Const_Buffer;       /* Source Address Settings */
  DMA_Initstruct.DstAddress    = (uint32_t)&aDST_Buffer;             /* Dst Address Settings */
  DMA_Initstruct.Direction     = LL_DMA_DIRECTION_MEMORY_TO_MEMORY;  /* Memory to Memory mode */
  DMA_Initstruct.SrcIncMode    = LL_DMA_SRC_ADDR_INC;                /* Disable Src increment mode */
  DMA_Initstruct.DstIncMode    = LL_DMA_DST_ADDR_INC;                /* Enable Dst increment mode */
  DMA_Initstruct.SrcWidth      = LL_DMA_SRC_WIDTH_WORD;              /* Source data width is 32 bits */
  DMA_Initstruct.DstWidth      = LL_DMA_DST_WIDTH_WORD;              /* Dst data width is 32 bits */
  DMA_Initstruct.Priority      = LL_DMA_PRIORITY_0;                  /* Channel priority is 0 */
/*  DMA_Initstruct.SrcHandshakeType = LL_DMA_SRC_HANDSHAKE_TYPE_HARD;  */
/*  DMA_Initstruct.DstHandshakeType = LL_DMA_DST_HANDSHAKE_TYPE_HARD;  */
/*  DMA_Initstruct.SrcBurstLen      = LL_DMA_SRC_BURST_LEN_1;          */
/*  DMA_Initstruct.DstBurstLen      = LL_DMA_DST_BURST_LEN_1;          */
  
  /* Initialize DMA */
  if (LL_DMA_Init(DMA1, LL_DMA_CHANNEL_1, &DMA_Initstruct) != SUCCESS)
  {
    APP_ErrorHandler();
  }

  /* Enable interrupt */
  LL_DMA_EnableIT(DMA1,LL_DMA_CHANNEL_1);
  LL_DMA_EnableIT_TFR(DMA1, LL_DMA_CHANNEL_1);
  LL_DMA_EnableIT_ERR(DMA1, LL_DMA_CHANNEL_1);

  NVIC_SetPriority(DMA1_Channel1_IRQn, 0);
  NVIC_EnableIRQ(DMA1_Channel1_IRQn);

  LL_DMA_ClearFlag_TFR1(DMA1);
  LL_DMA_ClearFlag_BLOCK1(DMA1);
  LL_DMA_ClearFlag_HALFBLOCK1(DMA1);
  LL_DMA_ClearFlag_SRCTRAN1(DMA1);
  LL_DMA_ClearFlag_DSTTRAN1(DMA1);

  /* Set Block Length */
  LL_DMA_SetBlockLength(DMA1,LL_DMA_CHANNEL_1,BUFFER_SIZE);

  /* Enable DMA */
  LL_DMA_Enable(DMA1);

  /* Enable DMA Channel1 */
  LL_DMA_EnableChannel(DMA1, LL_DMA_CHANNEL_1);
}

/**
  * @brief  DMA transfer completion callback function
  * @param  None
  * @retval None
  */
void APP_TransferCompleteCallback(void)
{
  for(uint16_t i=0 ; i<BUFFER_SIZE; i++)
  {
    if(aDST_Buffer[i] != aSRC_Const_Buffer[i])
    {
      transferFailedDetected=1;
      break;
    }
  }
  transferCompleteDetected=1;
}

/**
  * @brief  DMA transfer error callback function
  * @param  None
  * @retval None
  */
void APP_TransferErrorCallback(void)
{
  transferErrorDetected = 1;
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
