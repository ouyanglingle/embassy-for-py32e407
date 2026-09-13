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
#define DELTA             (int32_t)0x00001000       /* Max residual error for sines, with 6 cycle precision:
                                                       2^-19 max residual error, ie 31-19=12 LSB, ie <0x1000 */
#define PASS              0
#define FAIL              1

/* Private variables ---------------------------------------------------------*/
/* Array of angles in Q1.31 format, regularly incremented from 0 to 2*pi */
static int32_t Angles[64] =
{
  0x00000000, 0x04000000, 0x08000000, 0x0C000000,
  0x10000000, 0x14000000, 0x18000000, 0x1C000000,
  0x20000000, 0x24000000, 0x28000000, 0x2C000000,
  0x30000000, 0x34000000, 0x38000000, 0x3C000000,
  0x40000000, 0x44000000, 0x48000000, 0x4C000000,
  0x50000000, 0x54000000, 0x58000000, 0x5C000000,
  0x60000000, 0x64000000, 0x68000000, 0x6C000000,
  0x70000000, 0x74000000, 0x78000000, 0x7C000000,
  0x80000000, 0x84000000, 0x88000000, 0x8C000000,
  0x90000000, 0x94000000, 0x98000000, 0x9C000000,
  0xA0000000, 0xA4000000, 0xA8000000, 0xAC000000,
  0xB0000000, 0xB4000000, 0xB8000000, 0xBC000000,
  0xC0000000, 0xC4000000, 0xC8000000, 0xCC000000,
  0xD0000000, 0xD4000000, 0xD8000000, 0xDC000000,
  0xE0000000, 0xE4000000, 0xE8000000, 0xEC000000,
  0xF0000000, 0xF4000000, 0xF8000000, 0xFC000000
};
/* Array of reference sines in Q1.31 format */
static int32_t RefSin[64] =
{
  0x00000000, 0x0C8BD35E, 0x18F8B83C, 0x25280C5D,
  0x30FBC54D, 0x3C56BA70, 0x471CECE6, 0x5133CC94,
  0x5A827999, 0x62F201AC, 0x6A6D98A4, 0x70E2CBC6,
  0x7641AF3C, 0x7A7D055B, 0x7D8A5F3F, 0x7F62368F,
  0x80000000, 0x7F62368F, 0x7D8A5F3F, 0x7A7D055B,
  0x7641AF3C, 0x70E2CBC6, 0x6A6D98A4, 0x62F201AC,
  0x5A827999, 0x5133CC94, 0x471CECE6, 0x3C56BA70,
  0x30FBC54D, 0x25280C5D, 0x18F8B83C, 0x0C8BD35E,
  0x00000000, 0xF3742CA2, 0xE70747C4, 0xDAD7F3A3,
  0xCF043AB3, 0xC3A94590, 0xB8E3131A, 0xAECC336C,
  0xA57D8667, 0x9D0DFE54, 0x9592675C, 0x8F1D343A,
  0x89BE50C4, 0x8582FAA5, 0x8275A0C1, 0x809DC971,
  0x80000000, 0x809DC971, 0x8275A0C1, 0x8582FAA5,
  0x89BE50C4, 0x8F1D343A, 0x9592675C, 0x9D0DFE54,
  0xA57D8667, 0xAECC336C, 0xB8E3131A, 0xC3A94590,
  0xCF043AB3, 0xDAD7F3A3, 0xE70747C4, 0xF3742CA2
};

/* Array of calculated sines in Q1.31 format */
static int32_t CalculatedSin[64];
__IO int CordicReadFlags = 0;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_DmaConfig(void);
static uint32_t Check_Residual_Error(int32_t VarA, int32_t VarB, uint32_t MaxError);

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

  LL_AHB1_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_CORDIC);

  LL_CORDIC_Config(CORDIC,LL_CORDIC_FUNCTION_SINE,LL_CORDIC_PRECISION_6CYCLES,LL_CORDIC_SCALE_0,LL_CORDIC_NBWRITE_1,LL_CORDIC_NBREAD_1,LL_CORDIC_INSIZE_32BITS,LL_CORDIC_OUTSIZE_32BITS);

  /* DMA Config */
  APP_DmaConfig();
  
  CordicReadFlags = 0;

  /* Enable DMA Channel1 */
  LL_DMA_EnableChannel(DMA1, LL_DMA_CHANNEL_1);

  /* Enable DMA Channel2 */
  LL_DMA_EnableChannel(DMA1, LL_DMA_CHANNEL_2);

  /* Wait Cordic Read DMA complete */
  while(CordicReadFlags == 0);

  CordicReadFlags = 0;

  /* Compare CORDIC results to the reference values */
  for (uint32_t i = 0; i < 64; i++)
  {
    if (Check_Residual_Error(CalculatedSin[i], RefSin[i], DELTA) == FAIL)
    {
      APP_ErrorHandler();
    }
  }
 
  /* Correct CORDIC output values: Turn LED on */
  BSP_LED_On(LED_GREEN);

  while (1)
  {
  }
}

/**
  * @brief  Cordic Read Callback
  * @param  None
  * @retval None
  */
void APP_CordicReadCallback(void)
{
  CordicReadFlags = 1;
}

/**
  * @brief  DMA Config
  * @param  None
  * @retval None
  */
static void APP_DmaConfig(void)
{
  LL_DMA_InitTypeDef DMA_InitstructR = {0};
  LL_DMA_InitTypeDef DMA_InitstructW = {0};

  LL_AHB1_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_DMA1);

  /* Configure DMA function parameters */
  DMA_InitstructR.SrcAddress    = (uint32_t)&CORDIC->RDATA;           /* Source Address Settings */
  DMA_InitstructR.DstAddress    = (uint32_t)&CalculatedSin;           /* Dest Address Settings */
  DMA_InitstructR.Direction     = LL_DMA_DIRECTION_PERIPH_TO_MEMORY;  /* Periph to Memory mode */
  DMA_InitstructR.SrcIncMode    = LL_DMA_SRC_ADDR_FIX;                /* Disable Src increment mode */
  DMA_InitstructR.DstIncMode    = LL_DMA_DST_ADDR_INC;                /* Enable Dst increment mode */
  DMA_InitstructR.SrcWidth      = LL_DMA_SRC_WIDTH_WORD;              /* Source data width is 32 bits */
  DMA_InitstructR.DstWidth      = LL_DMA_DST_WIDTH_WORD;              /* Dest data width is 32 bits */
  DMA_InitstructR.Priority      = LL_DMA_PRIORITY_1;                  /* Channel priority is 0 */
  DMA_InitstructR.SrcHandshakeType = LL_DMA_SRC_HANDSHAKE_TYPE_HARD;  
/*  DMA_InitstructR.DstHandshakeType = LL_DMA_DST_HANDSHAKE_TYPE_HARD;  */
  DMA_InitstructR.SrcBurstLen      = LL_DMA_SRC_BURST_LEN_1;          
/*  DMA_InitstructR.DstBurstLen      = LL_DMA_DST_BURST_LEN_1;          */

  /* Initialize DMA */
  if (LL_DMA_Init(DMA1, LL_DMA_CHANNEL_1, &DMA_InitstructR) != SUCCESS)
  {
    APP_ErrorHandler();
  }

  LL_DMA_SetSrcPeriphMap(DMA1, LL_DMA_CHANNEL_1, LL_DMA_SRC_PERIPH_MAP_1);
  LL_SYSCFG_SetDMARemap(DMA1,LL_SYSCFG_DMA_PERIPH_MAP_1,LL_SYSCFG_DMA_MAP_Cordic_Read);

  /* Configure DMA function parameters */
  DMA_InitstructW.SrcAddress    = (uint32_t)&Angles;                  /* Source Address Settings */
  DMA_InitstructW.DstAddress    = (uint32_t)&CORDIC->WDATA;           /* Dest Address Settings */
  DMA_InitstructW.Direction     = LL_DMA_DIRECTION_MEMORY_TO_PERIPH;  /* Periph to Memory mode */
  DMA_InitstructW.SrcIncMode    = LL_DMA_SRC_ADDR_INC;                /* Disable Src increment mode */
  DMA_InitstructW.DstIncMode    = LL_DMA_DST_ADDR_FIX;                /* Enable Dst increment mode */
  DMA_InitstructW.SrcWidth      = LL_DMA_SRC_WIDTH_WORD;              /* Source data width is 32 bits */
  DMA_InitstructW.DstWidth      = LL_DMA_DST_WIDTH_WORD;              /* Dest data width is 32 bits */
  DMA_InitstructW.Priority      = LL_DMA_PRIORITY_0;                  /* Channel priority is 0 */
/*  DMA_InitstructW.SrcHandshakeType = LL_DMA_SRC_HANDSHAKE_TYPE_HARD;  */
  DMA_InitstructW.DstHandshakeType = LL_DMA_DST_HANDSHAKE_TYPE_HARD;  
/*  DMA_InitstructW.SrcBurstLen      = LL_DMA_SRC_BURST_LEN_1;          */
  DMA_InitstructW.DstBurstLen      = LL_DMA_DST_BURST_LEN_1;          

  /* Initialize DMA */
  if (LL_DMA_Init(DMA1, LL_DMA_CHANNEL_2, &DMA_InitstructW) != SUCCESS)
  {
    APP_ErrorHandler();
  }

  LL_DMA_SetDstPeriphMap(DMA1, LL_DMA_CHANNEL_2, LL_DMA_DST_PERIPH_MAP_2);
  LL_SYSCFG_SetDMARemap(DMA1,LL_SYSCFG_DMA_PERIPH_MAP_2,LL_SYSCFG_DMA_MAP_Cordic_Write);

  /* Enable Block Interrupt */
  LL_DMA_EnableIT_BLOCK(DMA1,LL_DMA_CHANNEL_1);

  /* Enable Interrupt */
  LL_DMA_EnableIT(DMA1,LL_DMA_CHANNEL_1);

  LL_DMA_ClearFlag_BLOCK1(DMA1);
  LL_CORDIC_EnableDMAReq_RD(CORDIC);

  NVIC_SetPriority(DMA1_Channel1_IRQn, 0);
  NVIC_EnableIRQ(DMA1_Channel1_IRQn);

  /* Enable Block Interrupt */
  LL_DMA_EnableIT_BLOCK(DMA1,LL_DMA_CHANNEL_2);

  /* Enable Interrupt */
  LL_DMA_EnableIT(DMA1,LL_DMA_CHANNEL_2);

  LL_DMA_ClearFlag_BLOCK2(DMA1);
  LL_CORDIC_EnableDMAReq_WR(CORDIC);

  NVIC_SetPriority(DMA1_Channel2_IRQn, 1);
  NVIC_EnableIRQ(DMA1_Channel2_IRQn);

  LL_DMA_SetBlockLength(DMA1,LL_DMA_CHANNEL_1,64);
  LL_DMA_SetBlockLength(DMA1,LL_DMA_CHANNEL_2,64);
  LL_DMA_Enable(DMA1);
}

/**
  * @brief  Check delta between two values is below threshold
  * @param  VarA First input variable
  * @param  VarB Second input variable
  * @param  MaxError Maximum delta allowed between VarA and VarB
  * @retval Status
  *           PASS: Delta is below threshold
  *           FAIL: Delta is above threshold
  */
static uint32_t Check_Residual_Error(int32_t VarA, int32_t VarB, uint32_t MaxError)
{
  uint32_t status = PASS;

  if ((VarA - VarB) >= 0)
  {
    if ((VarA - VarB) > MaxError)
    {
      status = FAIL;
    }
  }
  else
  {
    if ((VarB - VarA) > MaxError)
    {
      status = FAIL;
    }
  }

  return status;
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

  /* Set APB1 & APB2 prescaler: PCLK1 = HCLK, PCLK2 = 1/2HCLK */
  LL_RCC_SetAPB1Prescaler(LL_RCC_APB1_DIV_1);
  LL_RCC_SetAPB2Prescaler(LL_RCC_APB2_DIV_1);
  
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
