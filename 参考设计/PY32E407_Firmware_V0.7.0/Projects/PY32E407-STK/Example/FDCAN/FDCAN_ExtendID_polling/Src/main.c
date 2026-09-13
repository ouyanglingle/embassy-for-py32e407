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
FDCAN_HandleTypeDef FdcanHandle;
uint8_t TxData[64] = {0};
FDCAN_FilterTypeDef FilterConfig;
FDCAN_RxHeaderTypeDef RxHeader;
uint8_t RxData[64] = {0};
FDCAN_TxHeaderTypeDef TxHeader;
uint8_t DLCtoBytes[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 16, 20, 24, 32, 48, 64};

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);

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

  /* Initialize debug UART (used for printf) */
  BSP_UART_Config();
  
  /* CAN init */
  FdcanHandle.Instance = FDCAN2;
  FdcanHandle.Init.FrameFormat = FDCAN_FRAME_FD_BRS;
  FdcanHandle.Init.Mode = FDCAN_MODE_NORMAL;
  FdcanHandle.Init.AutoRetransmission = ENABLE;
  FdcanHandle.Init.TransmitPause = DISABLE;
  FdcanHandle.Init.ProtocolException = ENABLE;
  FdcanHandle.Init.NominalPrescaler = 1;   /* 500kbps */
  FdcanHandle.Init.NominalSyncJumpWidth = 8;
  FdcanHandle.Init.NominalTimeSeg1 = 31;
  FdcanHandle.Init.NominalTimeSeg2 = 8;
  FdcanHandle.Init.DataPrescaler = 1;   /* 2Mbps */
  FdcanHandle.Init.DataSyncJumpWidth = 2;
  FdcanHandle.Init.DataTimeSeg1 = 7;
  FdcanHandle.Init.DataTimeSeg2 = 2;
  FdcanHandle.Init.MessageRAMOffset = 0;
  FdcanHandle.Init.StdFiltersNbr = 0;
  FdcanHandle.Init.ExtFiltersNbr = 1;
  FdcanHandle.Init.RxFifo0ElmtsNbr = 1;
  FdcanHandle.Init.RxFifo0ElmtSize = FDCAN_DATA_BYTES_64;
  FdcanHandle.Init.RxFifo1ElmtsNbr = 0;
  FdcanHandle.Init.RxBuffersNbr = 0;
  FdcanHandle.Init.TxEventsNbr = 0;
  FdcanHandle.Init.TxBuffersNbr = 0;
  FdcanHandle.Init.TxFifoQueueElmtsNbr = 1;
  FdcanHandle.Init.TxFifoQueueMode = FDCAN_TX_FIFO_OPERATION;
  FdcanHandle.Init.TxElmtSize = FDCAN_DATA_BYTES_64;
  HAL_FDCAN_Init(&FdcanHandle);
  
  /* Configure Rx filter */
  FilterConfig.IdType = FDCAN_EXTENDED_ID;
  FilterConfig.FilterIndex = 0;
  FilterConfig.FilterType = FDCAN_FILTER_MASK;
  FilterConfig.FilterConfig = FDCAN_FILTER_TO_RXFIFO0;
  FilterConfig.FilterID1 = 0x12345678;
  FilterConfig.FilterID2 = 0x1FFFFFFF; /* For acceptance, MessageID and FilterID1 must match exactly */
  HAL_FDCAN_ConfigFilter(&FdcanHandle, &FilterConfig);
  
  /* Configure global filter to reject all non-matching frames */
  HAL_FDCAN_ConfigGlobalFilter(&FdcanHandle, FDCAN_REJECT, FDCAN_REJECT, FDCAN_REJECT_REMOTE, FDCAN_REJECT_REMOTE);
  
  /* Prepare Tx Header */
  TxHeader.Identifier = 0x12345678;
  TxHeader.IdType = FDCAN_EXTENDED_ID;
  TxHeader.TxFrameType = FDCAN_DATA_FRAME;
  TxHeader.DataLength = FDCAN_DLC_BYTES_64;
  TxHeader.ErrorStateIndicator = FDCAN_ESI_ACTIVE;
  TxHeader.BitRateSwitch = FDCAN_BRS_ON;
  TxHeader.FDFormat = FDCAN_FD_CAN;
  TxHeader.TxEventFifoControl = FDCAN_NO_TX_EVENTS;
  TxHeader.MessageMarker = 0;
  
  /* Sending data initialization */
  for (i = 0; i <64; i++)
  {
    TxData[i] = i;
  }
  
  /* Start the FDCAN module */
  HAL_FDCAN_Start(&FdcanHandle);

  /* Add messages to TX FIFO */
  HAL_FDCAN_AddMessageToTxFifoQ(&FdcanHandle, &TxHeader, &TxData[0]);
  
  /* Infinite loop */
  while (1)
  {
    /* Wait for two messages received */
    while (HAL_FDCAN_GetRxFifoFillLevel(&FdcanHandle, FDCAN_RX_FIFO0) < 1) {}
    
    /* Retrieve Rx messages from RX FIFO0 */
    HAL_FDCAN_GetRxMessage(&FdcanHandle, FDCAN_RX_FIFO0, &RxHeader, RxData);
    
    /* Print received data */
    for (i = 0; i < DLCtoBytes[RxHeader.DataLength >> 16]; i++)
    {
      printf("%x ", RxData[i]);
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
  OscInitstruct.PLL.PLLState    = RCC_PLL_ON;                               /* Close PLL */
  OscInitstruct.PLL.PLLSource   = RCC_PLLSOURCE_HSI_DIV2;                   /* PLL clock source selection HSI/2 */
  OscInitstruct.PLL.PLLPrediv   = RCC_PLL_PREDIV_DIV1;                      /* PLL clock Prediv factor */
  OscInitstruct.PLL.PLLMUL      = 15;                                       /* PLL clock source 15-fold frequency */
  OscInitstruct.PLL.PLLPostdiv  = RCC_PLL_POSTDIV_DIV2;                     /* PLL clock Postdiv factor */
  /* Configure oscillator */
  if(HAL_RCC_OscConfig(&OscInitstruct) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  
  ClkInitstruct.ClockType       = RCC_CLOCKTYPE_SYSCLK | RCC_CLOCKTYPE_HCLK | RCC_CLOCKTYPE_PCLK1 | RCC_CLOCKTYPE_PCLK2;
  ClkInitstruct.SYSCLKSource    = RCC_SYSCLKSOURCE_PLLCLK;              /* System clock selection PLL */
  ClkInitstruct.AHBCLKDivider   = RCC_SYSCLK_DIV1;                      /* AHB clock 1 division */
  ClkInitstruct.APB1CLKDivider  = RCC_HCLK_DIV1;                        /* APB1 clock 1 division */
  ClkInitstruct.APB2CLKDivider  = RCC_HCLK_DIV2;                        /* APB2 clock 2 division */
  /* Configure Clock */
  if(HAL_RCC_ClockConfig(&ClkInitstruct, FLASH_LATENCY_3) != HAL_OK)
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
