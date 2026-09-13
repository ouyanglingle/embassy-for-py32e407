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
#define BUFFER_SIZE 32

/* Private variables ---------------------------------------------------------*/
DMA_HandleTypeDef     DmaHandle;
uint32_t aSRC_Buffer[BUFFER_SIZE] = {0};       /* Data transfer source buffer */
uint32_t aDST_Buffer[BUFFER_SIZE] = {0};       /* Data transfer destination buffer */
__IO uint32_t transferCompleteDetected=0;      /* When the transmission is complete,the bit set to 1 */
__IO uint32_t transferErrorDetected=0;         /* When a transmission error occurs,the bit set to 1 */
__IO uint32_t transferFailedDetected=0;        /* When data of transmission error,the bit set to 1 */

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_DmaConfig(void);
static void APP_TransferComplete(DMA_HandleTypeDef *DmaHandle);
static void APP_TransferError(DMA_HandleTypeDef *DmaHandle);

/**
  * @brief  Main program.
  * @retval int
  */
int main(void)
{
  uint32_t i = 0;
  
  /* Reset of all peripherals, Initializes the Systick. */
  HAL_Init();
  
  /* Configure system clock */
  APP_SystemClockConfig(); 

  /* Initialize LED */
  BSP_LED_Init(LED_GREEN);

  /* Initialize DMA source buffer */
  for (i = 0; i < BUFFER_SIZE; i++)
  {
    aSRC_Buffer[i] = i;
  }
  
  /* Configure DMA */
  APP_DmaConfig();
  
  /* Start the DMA Transfer with interrupt enabled. */
  if (HAL_DMA_Start_IT(&DmaHandle, (uint32_t)&aSRC_Buffer, (uint32_t)&aDST_Buffer, BUFFER_SIZE) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  
  /* Infinite loop */
  while (1)
  {
    /* DMA transfer complete,but data error */
    if(transferFailedDetected == 1 && transferCompleteDetected == 1)
    {
      while(1)
      {
        BSP_LED_Toggle(LED_GREEN);
        HAL_Delay(500);
      }
    }

    /* DMA transfer complete,and data is correct */
    if(transferFailedDetected == 0 && transferCompleteDetected == 1)
    {
      BSP_LED_On(LED_GREEN);
      while(1)
      {
      }
    }

    /* DMA transfer error */
    if(transferErrorDetected == 1)
    {
      while(1)
      {
        BSP_LED_Toggle(LED_GREEN);
        HAL_Delay(500);
      }
    }
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
  * @brief  Configure DMA
  * @param  None
  * @retval None
  */
static void APP_DmaConfig(void)
{
  /*Enable DMA1 clock*/
  __HAL_RCC_DMA1_CLK_ENABLE();
  
  DmaHandle.Instance = DMA1_Channel1;
  DmaHandle.Init.Direction = DMA_MEMORY_TO_MEMORY;
  DmaHandle.Init.SrcWidth = DMA_SRC_WIDTH_WORD;
  DmaHandle.Init.DstWidth = DMA_DST_WIDTH_WORD;
  /* DmaHandle.Init.SrcBurstLen = DMA_SRC_BURST_LEN_1; */
  /* DmaHandle.Init.DstBurstLen = DMA_DST_BURST_LEN_1; */
  DmaHandle.Init.SrcInc = DMA_SRC_ADDR_INC;
  DmaHandle.Init.DstInc = DMA_DST_ADDR_INC;
  DmaHandle.Init.Mode = DMA_NORMAL;
  /* DmaHandle.Init.SrcReload = DMA_SRC_RELOAD_DISABLE; */
  /* DmaHandle.Init.DstReload = DMA_DST_RELOAD_DISABLE; */
  /* DmaHandle.Init.SrcHandshakeType = DMA_SRC_HS_TYPE_HARD; */
  /* DmaHandle.Init.DstHandshakeType = DMA_DST_HS_TYPE_HARD; */
  DmaHandle.Init.Priority = DMA_PRIORITY_1;
  DmaHandle.Init.BlockMaxLen = DMA_BLOCK_DEFAULT_MAX_LEN;
  if (HAL_DMA_Init(&DmaHandle) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  
  /* Select the callback function to call after error transmission and correct transmission */
  HAL_DMA_RegisterCallback(&DmaHandle, HAL_DMA_XFER_CPLT_CB_ID, APP_TransferComplete);
  HAL_DMA_RegisterCallback(&DmaHandle, HAL_DMA_XFER_ERROR_CB_ID, APP_TransferError);
  
  /* DMA channel 1 interrupt enable */
  HAL_NVIC_SetPriority(DMA1_Channel1_IRQn, 1, 0);
  HAL_NVIC_EnableIRQ(DMA1_Channel1_IRQn);
}

/**
  * @brief  DMA transfer completion function
  * @param  DmaHandle：DMA handle
  * @retval None
  */
static void APP_TransferComplete(DMA_HandleTypeDef *DmaHandle)
{
  for(uint32_t i=0 ; i<BUFFER_SIZE; i++)
  {
    if(aDST_Buffer[i] != aSRC_Buffer[i])
    {
      transferFailedDetected=1;
      break;
    }
  }
  transferCompleteDetected=1;
}

/**
  * @brief  DMA transfer error function
  * @param  DmaHandle：DMA handle
  * @retval None
  */
static void APP_TransferError(DMA_HandleTypeDef *DmaHandle)
{
  transferErrorDetected = 1;
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
