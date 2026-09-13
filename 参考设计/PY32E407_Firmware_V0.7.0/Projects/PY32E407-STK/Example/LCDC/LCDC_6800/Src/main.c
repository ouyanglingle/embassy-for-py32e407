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
LCDC_HandleTypeDef hLCDC0;
uint8_t DataTxBuf[4];
uint8_t CmdTxBuf[2];
uint8_t CmdRxBuf[2];

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_LCDCConfig(void);
static void APP_LCDInit(void);
static void APP_LCD_Write8b(uint8_t *pData, uint32_t Size);

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

  /* Initialize LCDC0 */
  APP_LCDCConfig();
  
  APP_LCDInit();
  
  DataTxBuf[0] = 'P';
  DataTxBuf[1] = 'U';
  DataTxBuf[2] = 'Y';
  DataTxBuf[3] = 'A';
  APP_LCD_Write8b(DataTxBuf, 4);
  
  /* Infinite loop */
  while (1)
  {
  }
}

/**
  * @brief  Initlize LCD moudle
  * @param  None
  * @retval None
  */
static void APP_LCDInit(void)
{
  /* Function set */
  CmdTxBuf[0] = 0x38;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  /* Wait screen Busy = 0 */
  do
  {
    HAL_LCDC_ReadCmd(&hLCDC0, CmdRxBuf, 1, 1000);
  }while((CmdRxBuf[0] & 0x80) != 0);
  
  /* Clear disaply */
  CmdTxBuf[0] = 0x01;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  /* Wait screen Busy = 0 */
  do
  {
    HAL_LCDC_ReadCmd(&hLCDC0, CmdRxBuf, 1, 1000);
  }while((CmdRxBuf[0] & 0x80) != 0);
  
  /* Disaply on */
  CmdTxBuf[0] = 0x0F;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  /* Wait screen Busy = 0 */
  do
  {
    HAL_LCDC_ReadCmd(&hLCDC0, CmdRxBuf, 1, 1000);
  }while((CmdRxBuf[0] & 0x80) != 0);
}

/**
  * @brief  Write 8 bit data 
  * @param  pData: pointer to data buffer
  * @param  Size: amount of data to be sent
  * @retval None
  */
static void APP_LCD_Write8b(uint8_t *pData, uint32_t Size)
{
  for(uint32_t i = 0; i < Size; i++)
  {
    HAL_LCDC_WriteData(&hLCDC0, pData, 1, 1000);
    /* Wait screen Busy = 0 */
    do
    {
      HAL_LCDC_ReadCmd(&hLCDC0, CmdRxBuf, 1, 1000);
    }while((CmdRxBuf[0] & 0x80) != 0);
    pData += 1;
  }
}

/**
  * @brief  LCDC Configuration
  * @param  None
  * @retval None
  */
static void APP_LCDCConfig(void)
{
  hLCDC0.Instance = LCDC0;
  hLCDC0.Init.Mode = LCDC_MODE_6800;      /* Select 8080 mode */
  hLCDC0.Init.Write.AddSet = 16 - 1;      /* Address setup phase is 16 pclks */
  hLCDC0.Init.Write.DataSt = 256 - 1;     /* Data-phase is 256 pclks */
  hLCDC0.Init.Write.DataHold = 5 - 2;     /* Data hold phase is 5 pclks */
  hLCDC0.Init.Read.AddSet = 16 - 1;       /* Address setup phase is 16 pclks */
  hLCDC0.Init.Read.DataSt = 256 - 1;      /* Data-phase is 256 pclks */
  hLCDC0.Init.Read.DataHold = 5 - 2;      /* Data hold phase is 5 pclks */
  hLCDC0.Init.DataSize = LCDC_DATASIZE_8BIT;
  hLCDC0.Init.CmdSize = LCDC_CMDSIZE_8BIT;
  
  if(HAL_LCDC_Init(&hLCDC0) != HAL_OK)
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
