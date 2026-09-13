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
uint8_t TxData[] = {0x10, 0x32, 0x54, 0x76, 0x98, 0x00, 0x11, 0x22,
                    0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00};
FDCAN_FilterTypeDef FilterConfig;
FDCAN_RxHeaderTypeDef RxHeader;
uint8_t RxData[16];
FDCAN_TxHeaderTypeDef TxHeader;
__IO uint32_t RxFifo0Notificationflag = 0;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static uint32_t APP_BufferCmp8b(uint8_t* pBuffer1, uint8_t* pBuffer2, uint16_t BufferLength);

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

  /* Initialize debug UART (used for printf) */
  BSP_UART_Config();
  
  /* CAN init */
  FdcanHandle.Instance = FDCAN1;
  FdcanHandle.Init.FrameFormat = FDCAN_FRAME_CLASSIC;
  FdcanHandle.Init.Mode = FDCAN_MODE_NORMAL;
  FdcanHandle.Init.AutoRetransmission = ENABLE;
  FdcanHandle.Init.TransmitPause = DISABLE;
  FdcanHandle.Init.ProtocolException = ENABLE;
  FdcanHandle.Init.NominalPrescaler = 1;   /* 500kbps */
  FdcanHandle.Init.NominalSyncJumpWidth = 8;
  FdcanHandle.Init.NominalTimeSeg1 = 31;
  FdcanHandle.Init.NominalTimeSeg2 = 8;
  FdcanHandle.Init.MessageRAMOffset = 0;
  FdcanHandle.Init.StdFiltersNbr = 1;
  FdcanHandle.Init.ExtFiltersNbr = 0;
  FdcanHandle.Init.RxFifo0ElmtsNbr = 2;
  FdcanHandle.Init.RxFifo0ElmtSize = FDCAN_DATA_BYTES_8;
  FdcanHandle.Init.RxFifo1ElmtsNbr = 0;
  FdcanHandle.Init.RxBuffersNbr = 0;
  FdcanHandle.Init.TxEventsNbr = 0;
  FdcanHandle.Init.TxBuffersNbr = 0;
  FdcanHandle.Init.TxFifoQueueElmtsNbr = 2;
  FdcanHandle.Init.TxFifoQueueMode = FDCAN_TX_FIFO_OPERATION;
  FdcanHandle.Init.TxElmtSize = FDCAN_DATA_BYTES_8;
  HAL_FDCAN_Init(&FdcanHandle);
  
  /* Configure Rx filter */
  FilterConfig.IdType = FDCAN_STANDARD_ID;
  FilterConfig.FilterIndex = 0;
  FilterConfig.FilterType = FDCAN_FILTER_MASK;
  FilterConfig.FilterConfig = FDCAN_FILTER_TO_RXFIFO0;
  FilterConfig.FilterID1 = 0x111;
  FilterConfig.FilterID2 = 0x7FF; /* For acceptance, MessageID and FilterID1 must match exactly */
  HAL_FDCAN_ConfigFilter(&FdcanHandle, &FilterConfig);
  
  /* Configure global filter to reject all non-matching frames */
  HAL_FDCAN_ConfigGlobalFilter(&FdcanHandle, FDCAN_REJECT, FDCAN_REJECT, FDCAN_REJECT_REMOTE, FDCAN_REJECT_REMOTE);
  
  /* Configure Rx FIFO 0 watermark to 2 */
  HAL_FDCAN_ConfigFifoWatermark(&FdcanHandle, FDCAN_CFG_RX_FIFO0, 2);
  /* Activate Rx FIFO 0 notification */
  HAL_FDCAN_ActivateNotification(&FdcanHandle, (FDCAN_IT_RX_FIFO0_MESSAGE_LOST | FDCAN_IT_RX_FIFO0_FULL | \
                                          FDCAN_IT_RX_FIFO0_WATERMARK | FDCAN_IT_RX_FIFO0_NEW_MESSAGE), 0);
  
  /* Prepare Tx Header */
  TxHeader.Identifier = 0x111;
  TxHeader.IdType = FDCAN_STANDARD_ID;
  TxHeader.TxFrameType = FDCAN_DATA_FRAME;
  TxHeader.DataLength = FDCAN_DLC_BYTES_8;
  TxHeader.ErrorStateIndicator = FDCAN_ESI_ACTIVE;
  TxHeader.BitRateSwitch = FDCAN_BRS_OFF;
  TxHeader.FDFormat = FDCAN_CLASSIC_CAN;
  TxHeader.TxEventFifoControl = FDCAN_NO_TX_EVENTS;
  TxHeader.MessageMarker = 0;
  
  /* Start the FDCAN module */
  HAL_FDCAN_Start(&FdcanHandle);

  /* Add messages to TX FIFO */
  HAL_FDCAN_AddMessageToTxFifoQ(&FdcanHandle, &TxHeader, &TxData[0]);
  HAL_FDCAN_AddMessageToTxFifoQ(&FdcanHandle, &TxHeader, &TxData[8]);
  
  /* Infinite loop */
  while (1)
  {
    /* check on the watermark notification flag */
    if (RxFifo0Notificationflag == 1)
    {
        /* Retrieve Rx messages from RX FIFO0 */
      HAL_FDCAN_GetRxMessage(&FdcanHandle, FDCAN_RX_FIFO0, &RxHeader, RxData);
      HAL_FDCAN_GetRxMessage(&FdcanHandle, FDCAN_RX_FIFO0, &RxHeader, &RxData[8]);
      
      /* Compare payload to expected data */
      if (APP_BufferCmp8b(TxData, RxData, 16) != 0)
      {
        APP_ErrorHandler();
      }
      else
      {
        printf("receive success\r\n");
      }
      
      RxFifo0Notificationflag = 0;
    }
  }
}

/**
  * @brief Compares two buffers.
  * @param 
  *  - pBuffer1, pBuffer2: buffers to be compared.
  *  - BufferLength: buffer's length
  * @retval
  *   0: pBuffer1 identical to pBuffer2
  *   1: pBuffer1 differs from pBuffer2
  */
static uint32_t APP_BufferCmp8b(uint8_t* pBuffer1, uint8_t* pBuffer2, uint16_t BufferLength)
{
  while(BufferLength--)
  {
    if(*pBuffer1 != *pBuffer2)
    {
      return 1;
    }
    
    pBuffer1++;
    pBuffer2++;
  }
  return 0;
}

/**
  * @brief  Rx FIFO 0 callback.
  * @param  hfdcan pointer to an FDCAN_HandleTypeDef structure that contains
  *         the configuration information for the specified FDCAN.
  * @param  RxFifo0ITs indicates which Rx FIFO 0 interrupts are signaled.
  *         This parameter can be any combination of @arg FDCAN_Rx_Fifo0_Interrupts.
  * @retval None
  */
void HAL_FDCAN_RxFifo0Callback(FDCAN_HandleTypeDef *hfdcan, uint32_t RxFifo0ITs)
{
  if((RxFifo0ITs & FDCAN_IT_RX_FIFO0_WATERMARK) != RESET)
  {
    RxFifo0Notificationflag = 1;
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
