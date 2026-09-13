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
#define CMD_MemoryWrite   0x2C
#define COLOR_BLUE        0x001F
#define COLUMN            280
#define ROW               320
#define LCD_BL_ON         GPIO_PIN_RESET

/* Private variables ---------------------------------------------------------*/
LCDC_HandleTypeDef hLCDC0;
uint8_t DataTxBuf[15];
uint8_t CmdTxBuf[10];

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_LCDCConfig(void);

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
  
  /* Full LCD gram */
  CmdTxBuf[0] = CMD_MemoryWrite;
  *(uint16_t *)DataTxBuf = COLOR_BLUE;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);

  for(uint32_t i = 0; i < COLUMN * ROW; i++)
  {
    HAL_LCDC_WriteData(&hLCDC0, (uint8_t *)DataTxBuf, 1, 1000);
    while(HAL_LCDC_GetState(&hLCDC0) != HAL_LCDC_STATE_READY);
  }
  
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
  /* Power control B*/
  CmdTxBuf[0] = 0xCF;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x00;
  DataTxBuf[1] = 0x81;
  DataTxBuf[2] = 0x30;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 3, 1000);
  HAL_Delay(5);
  
  /* Power on sequence control */
  CmdTxBuf[0] = 0xED;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x64;
  DataTxBuf[1] = 0x03;
  DataTxBuf[2] = 0x12;
  DataTxBuf[3] = 0x81;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 4, 1000);
  HAL_Delay(5);
  
  /* Power timing control A */
  CmdTxBuf[0] = 0xE8;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x85;
  DataTxBuf[1] = 0x10;
  DataTxBuf[2] = 0x78;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 3, 1000);
  HAL_Delay(5);
  
  /* Power control A */
  CmdTxBuf[0] = 0xCB;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x39;
  DataTxBuf[1] = 0x2C;
  DataTxBuf[2] = 0x00;
  DataTxBuf[3] = 0x34;
  DataTxBuf[4] = 0x02;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 5, 1000);
  HAL_Delay(5);
  
  /* Pump ratio control */
  CmdTxBuf[0] = 0xF7;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x20;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 1, 1000);
  HAL_Delay(5);
  
  /* Driver timing control B */
  CmdTxBuf[0] = 0xEA;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x00;
  DataTxBuf[1] = 0x00;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 2, 1000);
  HAL_Delay(5);
  
  /* Frame Rate Control (In normal mode/full colors) */
  CmdTxBuf[0] = 0xB1;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x00;
  DataTxBuf[1] = 0x1B;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 2, 1000);
  HAL_Delay(5);
  
  /* Display function control */
  CmdTxBuf[0] = 0xB6;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x0A;
  DataTxBuf[1] = 0xA2;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 2, 1000);
  HAL_Delay(5);
  
  /* Power control 1 */
  CmdTxBuf[0] = 0xC0;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x35;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 1, 1000);
  HAL_Delay(5);
  
  /* Power control 2 */
  CmdTxBuf[0] = 0xC1;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x11;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 1, 1000);
  HAL_Delay(5);
  
  /* VCOM control 1 */
  CmdTxBuf[0] = 0xC5;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x45;
  DataTxBuf[1] = 0x45;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 2, 1000);
  HAL_Delay(5);
  
  /* VCOM control 2 */
  CmdTxBuf[0] = 0xC7;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0xA2;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 1, 1000);
  HAL_Delay(5);
  
  /* Enable 3G */
  CmdTxBuf[0] = 0xF2;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x00;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 1, 1000);
  HAL_Delay(5);
  
  /* Gamma set */
  CmdTxBuf[0] = 0x26;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x01;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 1, 1000);
  HAL_Delay(5);
  
  /* Positive Gamma correction */
  CmdTxBuf[0] = 0xE0;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x0F;
  DataTxBuf[1] = 0x26;
  DataTxBuf[2] = 0x24;
  DataTxBuf[3] = 0x0B;
  DataTxBuf[4] = 0x0E;
  DataTxBuf[5] = 0x09;
  DataTxBuf[6] = 0x54;
  DataTxBuf[7] = 0xA8;
  DataTxBuf[8] = 0x46;
  DataTxBuf[9] = 0x0C;
  DataTxBuf[10] = 0x17;
  DataTxBuf[11] = 0x09;
  DataTxBuf[12] = 0x0F;
  DataTxBuf[13] = 0x07;
  DataTxBuf[14] = 0x00;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 15, 1000);
  HAL_Delay(5);
  
  /* Negative Gamma correction */
  CmdTxBuf[0] = 0xE1;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x00;
  DataTxBuf[1] = 0x19;
  DataTxBuf[2] = 0x1B;
  DataTxBuf[3] = 0x04;
  DataTxBuf[4] = 0x10;
  DataTxBuf[5] = 0x07;
  DataTxBuf[6] = 0x2A;
  DataTxBuf[7] = 0x47;
  DataTxBuf[8] = 0x39;
  DataTxBuf[9] = 0x03;
  DataTxBuf[10] = 0x06;
  DataTxBuf[11] = 0x06;
  DataTxBuf[12] = 0x30;
  DataTxBuf[13] = 0x38;
  DataTxBuf[14] = 0x0F;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 15, 1000);
  HAL_Delay(5);
  
  /* Memory access control set */
  CmdTxBuf[0] = 0x36;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0xC8;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 1, 1000);
  HAL_Delay(5);
  
  /* Column address control set: 0~239 */
  CmdTxBuf[0] = 0x2A;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x00;
  DataTxBuf[1] = 0x00;
  DataTxBuf[2] = 0x00;
  DataTxBuf[3] = 0xEF;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 4, 1000);
  HAL_Delay(5);
  
  /* Page address control set: 0~319 */
  CmdTxBuf[0] = 0x2B;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x00;
  DataTxBuf[1] = 0x00;
  DataTxBuf[2] = 0x01;
  DataTxBuf[3] = 0x3F;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 4, 1000);
  HAL_Delay(5);
  
  /* Pixel fotmat set */
  CmdTxBuf[0] = 0x3A;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  DataTxBuf[0] = 0x55;
  HAL_LCDC_WriteData(&hLCDC0, DataTxBuf, 1, 1000);
  HAL_Delay(5);
  
  /* Sleep out */
  CmdTxBuf[0] = 0x11;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
  HAL_Delay(5);
  
  /* Display on */
  CmdTxBuf[0] = 0x29;
  HAL_LCDC_WriteCmd(&hLCDC0, CmdTxBuf, 1, 1000);
}


/**
  * @brief  LCD moudle reset 
  * @param  None
  * @retval None
  */
static void APP_LCDReset(void)
{
  HAL_GPIO_WritePin(GPIOA, GPIO_PIN_5, GPIO_PIN_RESET);
  HAL_Delay(500);
  HAL_GPIO_WritePin(GPIOA, GPIO_PIN_5, GPIO_PIN_SET);
}

/**
  * @brief  LCDC back light control 
  * @param  BlState: back light state
  * @retval None
  */
static void APP_LCDBackLight(uint8_t BlState)
{
  HAL_GPIO_WritePin(GPIOA, GPIO_PIN_4, (GPIO_PinState)BlState);
}

/**
  * @brief  LCDC Configuration
  * @param  None
  * @retval None
  */
static void APP_LCDCConfig(void)
{
  hLCDC0.Instance = LCDC0;
  hLCDC0.Init.Mode = LCDC_MODE_8080;      /* Select 8080 mode */
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
  
  /* LCD Reset */
  APP_LCDReset();
  
  /* LCD back light on */
  APP_LCDBackLight(LCD_BL_ON);
  
  /* LCD init */
  APP_LCDInit();
  
  hLCDC0.Instance = LCDC0;
  hLCDC0.Init.Mode = LCDC_MODE_8080;      /* Select 8080 mode */
  hLCDC0.Init.Write.AddSet = 16 - 1;      /* Address setup phase is 16 pclks */
  hLCDC0.Init.Write.DataSt = 256 - 1;     /* Data-phase is 256 pclks */
  hLCDC0.Init.Write.DataHold = 5 - 2;     /* Data hold phase is 5 pclks */
  hLCDC0.Init.Read.AddSet = 16 - 1;       /* Address setup phase is 16 pclks */
  hLCDC0.Init.Read.DataSt = 256 - 1;      /* Data-phase is 256 pclks */
  hLCDC0.Init.Read.DataHold = 5 - 2;      /* Data hold phase is 5 pclks */
  hLCDC0.Init.DataSize = LCDC_DATASIZE_16BIT;
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
