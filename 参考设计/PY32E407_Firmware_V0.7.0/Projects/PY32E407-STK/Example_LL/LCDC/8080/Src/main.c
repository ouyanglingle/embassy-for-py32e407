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
#define CMD_MemoryWrite   0x2C
#define COLOR_BLUE        0x001F
#define COLUMN            280
#define ROW               320
           
#define LCD_BL_PIN        LL_GPIO_PIN_13   
#define LCD_RST_PIN       LL_GPIO_PIN_11

/* Private variables ---------------------------------------------------------*/
uint16_t DataTxBuf[15];
uint16_t CmdTxBuf[10];
uint8_t  ID[4];

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_LCDCWriteData(LCDC_TypeDef *LCDCx,uint16_t Data);
static void APP_LCDCWriteCommand(LCDC_TypeDef *LCDCx,uint16_t Command);
static void APP_LCDCConfig(void);
static void APP_GpioConfig(void);

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

  LL_APB2_GRP1_EnableClock(LL_APB2_GRP1_PERIPH_LCDC);

  /* Gpio Config */
  APP_GpioConfig();

  APP_LCDCConfig();

  CmdTxBuf[0] = CMD_MemoryWrite;
  DataTxBuf[0] = COLOR_BLUE;

  APP_LCDCWriteCommand(LCDC0,CMD_MemoryWrite);

  for(uint32_t i = 0; i < COLUMN * ROW; i++)
  {
    APP_LCDCWriteData(LCDC0,COLOR_BLUE);
  }
  
  /* Infinite loop */
  while (1)
  {
  }
}

/**
  * @brief  GPIO Config 
  * @param  None
  * @retval None
  */
static void APP_GpioConfig(void)
{
  LL_GPIO_InitTypeDef GPIO_InitType= {0};

  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOC);
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOD);
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOE);
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOF);

  /* Initlize LCDC D2/D3/RD/WR/CS/D13/D14/D15/D0/D1 pin */
  GPIO_InitType.Pin        = LL_GPIO_PIN_0|LL_GPIO_PIN_1|LL_GPIO_PIN_4|LL_GPIO_PIN_5|LL_GPIO_PIN_7|LL_GPIO_PIN_8| LL_GPIO_PIN_9 |LL_GPIO_PIN_10|LL_GPIO_PIN_14|LL_GPIO_PIN_15;
  GPIO_InitType.Mode       = LL_GPIO_MODE_ALTERNATE;
  GPIO_InitType.Alternate  = LL_GPIO_AF12_LCDC; 
  GPIO_InitType.Speed      = LL_GPIO_SPEED_FREQ_HIGH;
  GPIO_InitType.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  GPIO_InitType.Pull       = LL_GPIO_PULL_NO;
  LL_GPIO_Init(GPIOD,&GPIO_InitType);

  /* Initlize LCDC D4/D5/D6/D7/D8/D9/D10/D11/D12 pin */
  GPIO_InitType.Pin        = LL_GPIO_PIN_7|LL_GPIO_PIN_8|LL_GPIO_PIN_9|LL_GPIO_PIN_10|LL_GPIO_PIN_11| LL_GPIO_PIN_12|LL_GPIO_PIN_13|LL_GPIO_PIN_14| LL_GPIO_PIN_15;
  GPIO_InitType.Mode       = LL_GPIO_MODE_ALTERNATE;
  GPIO_InitType.Alternate  = LL_GPIO_AF12_LCDC; 
  GPIO_InitType.Speed      = LL_GPIO_SPEED_FREQ_HIGH;
  GPIO_InitType.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  GPIO_InitType.Pull       = LL_GPIO_PULL_NO;
  LL_GPIO_Init(GPIOE,&GPIO_InitType);

  /* Initlize LCDC RS pin*/
  GPIO_InitType.Pin        = LL_GPIO_PIN_4;
  GPIO_InitType.Mode       = LL_GPIO_MODE_ALTERNATE;
  GPIO_InitType.Alternate  = LL_GPIO_AF12_LCDC; 
  GPIO_InitType.Speed      = LL_GPIO_SPEED_FREQ_HIGH;
  GPIO_InitType.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  GPIO_InitType.Pull       = LL_GPIO_PULL_NO;
  LL_GPIO_Init(GPIOF,&GPIO_InitType);

  /* LCD BL and RST pin */
  GPIO_InitType.Pin        = LCD_BL_PIN | LCD_RST_PIN;
  GPIO_InitType.Mode       = LL_GPIO_MODE_OUTPUT;
  GPIO_InitType.Speed      = LL_GPIO_SPEED_FREQ_HIGH;
  GPIO_InitType.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  GPIO_InitType.Pull       = LL_GPIO_PULL_NO;
  LL_GPIO_Init(GPIOC,&GPIO_InitType);

  LL_GPIO_SetOutputPin(GPIOC, LCD_RST_PIN);
  LL_GPIO_SetOutputPin(GPIOC, LCD_BL_PIN);
}

/**
  * @brief  LCD moudle reset 
  * @param  None
  * @retval None
  */
static void APP_LCDReset(void)
{
  LL_GPIO_ResetOutputPin(GPIOC, LL_GPIO_PIN_11);
  LL_mDelay(50);
  LL_GPIO_SetOutputPin(GPIOC, LL_GPIO_PIN_11);
  LL_mDelay(120);
}

/**
  * @brief  LCDC back light control 
  * @param  BlState: back light state
  * @retval None
  */
static void APP_LCDBackLight(uint8_t BlState)
{
  if(BlState == RESET)
  {
    LL_GPIO_ResetOutputPin(GPIOC, LL_GPIO_PIN_13);
  }
  else
  {
    LL_GPIO_SetOutputPin(GPIOC, LL_GPIO_PIN_13);
  }
}

/**
  * @brief  Initlize LCD moudle
  * @param  None
  * @retval None
  */
static void APP_LCDConfig(void)
{
  /* Power control B*/
  APP_LCDCWriteCommand(LCDC0,0xCF);
  APP_LCDCWriteData(LCDC0,0x00);
  APP_LCDCWriteData(LCDC0,0x81);
  APP_LCDCWriteData(LCDC0,0x30);

  /* Power on sequence control */
  APP_LCDCWriteCommand(LCDC0,0xED);
  APP_LCDCWriteData(LCDC0,0x64);
  APP_LCDCWriteData(LCDC0,0x03);
  APP_LCDCWriteData(LCDC0,0x12);
  APP_LCDCWriteData(LCDC0,0x81);

  /* Power timing control A */
  APP_LCDCWriteCommand(LCDC0,0xE8);
  APP_LCDCWriteData(LCDC0,0x85);
  APP_LCDCWriteData(LCDC0,0x10);
  APP_LCDCWriteData(LCDC0,0x78);

  /* Power control A */
  APP_LCDCWriteCommand(LCDC0,0xCB);
  APP_LCDCWriteData(LCDC0,0x39);
  APP_LCDCWriteData(LCDC0,0x2C);
  APP_LCDCWriteData(LCDC0,0x00);
  APP_LCDCWriteData(LCDC0,0x34);
  APP_LCDCWriteData(LCDC0,0x02);

  /* Pump ratio control */
  APP_LCDCWriteCommand(LCDC0,0xF7);
  APP_LCDCWriteData(LCDC0,0x20);

  /* Driver timing control B */
  APP_LCDCWriteCommand(LCDC0,0xEA);
  APP_LCDCWriteData(LCDC0,0x00);
  APP_LCDCWriteData(LCDC0,0x00);

  /* Frame Rate Control (In normal mode/full colors) */
  APP_LCDCWriteCommand(LCDC0,0xB1);

  APP_LCDCWriteData(LCDC0,0x00);
  APP_LCDCWriteData(LCDC0,0x1B);

  /* Display function control */
  APP_LCDCWriteCommand(LCDC0,0xB6);
  APP_LCDCWriteData(LCDC0,0x0A);
  APP_LCDCWriteData(LCDC0,0xA2);

  /* Power control 1 */
  APP_LCDCWriteCommand(LCDC0,0xC0);
  APP_LCDCWriteData(LCDC0,0x35);

  /* Power control 2 */
  APP_LCDCWriteCommand(LCDC0,0xC1);
  APP_LCDCWriteData(LCDC0,0x11);

  /* VCOM control 1 */
  APP_LCDCWriteCommand(LCDC0,0xC5);
  APP_LCDCWriteData(LCDC0,0x45);
  APP_LCDCWriteData(LCDC0,0x45);

  /* VCOM control 2 */
  APP_LCDCWriteCommand(LCDC0,0xC7);
  APP_LCDCWriteData(LCDC0,0xA2);

  /* Enable 3G */
  APP_LCDCWriteCommand(LCDC0,0xF2);
  APP_LCDCWriteData(LCDC0,0x00);

  /* Gamma set */
  APP_LCDCWriteCommand(LCDC0,0x26);
  APP_LCDCWriteData(LCDC0,0x01);

  /* Positive Gamma correction */
  APP_LCDCWriteCommand(LCDC0,0xE0);
  APP_LCDCWriteData(LCDC0,0x0F);
  APP_LCDCWriteData(LCDC0,0x26);
  APP_LCDCWriteData(LCDC0,0x24);
  APP_LCDCWriteData(LCDC0,0x0B);
  APP_LCDCWriteData(LCDC0,0x0E);
  APP_LCDCWriteData(LCDC0,0x09);
  APP_LCDCWriteData(LCDC0,0x54);
  APP_LCDCWriteData(LCDC0,0xA8);
  APP_LCDCWriteData(LCDC0,0x46);
  APP_LCDCWriteData(LCDC0,0x0C);
  APP_LCDCWriteData(LCDC0,0x17);
  APP_LCDCWriteData(LCDC0,0x09);
  APP_LCDCWriteData(LCDC0,0x0F);
  APP_LCDCWriteData(LCDC0,0x07);
  APP_LCDCWriteData(LCDC0,0x00);

  /* Negative Gamma correction */
  CmdTxBuf[0] = 0xE1;
  APP_LCDCWriteCommand(LCDC0,0xE1);

  APP_LCDCWriteData(LCDC0,0x00);
  APP_LCDCWriteData(LCDC0,0x19);
  APP_LCDCWriteData(LCDC0,0x1B);
  APP_LCDCWriteData(LCDC0,0x04);
  APP_LCDCWriteData(LCDC0,0x10);
  APP_LCDCWriteData(LCDC0,0x07);
  APP_LCDCWriteData(LCDC0,0x2A);
  APP_LCDCWriteData(LCDC0,0x47);
  APP_LCDCWriteData(LCDC0,0x39);
  APP_LCDCWriteData(LCDC0,0x03);
  APP_LCDCWriteData(LCDC0,0x06);
  APP_LCDCWriteData(LCDC0,0x06);
  APP_LCDCWriteData(LCDC0,0x30);
  APP_LCDCWriteData(LCDC0,0x38);
  APP_LCDCWriteData(LCDC0,0x0F);

  /* Memory access control set */
  APP_LCDCWriteCommand(LCDC0,0x36);
  APP_LCDCWriteData(LCDC0,0xC8);

  /* Column address control set: 0~239 */
  APP_LCDCWriteCommand(LCDC0,0x2A);
  APP_LCDCWriteData(LCDC0,0x00);
  APP_LCDCWriteData(LCDC0,0x00);
  APP_LCDCWriteData(LCDC0,0x00);
  APP_LCDCWriteData(LCDC0,0xEF);

  /* Page address control set: 0~319 */
  APP_LCDCWriteCommand(LCDC0,0x2B);
  APP_LCDCWriteData(LCDC0,0x00);
  APP_LCDCWriteData(LCDC0,0x00);
  APP_LCDCWriteData(LCDC0,0x01);
  APP_LCDCWriteData(LCDC0,0x3F);

  /* Pixel fotmat set */
  APP_LCDCWriteCommand(LCDC0,0x3A);
  APP_LCDCWriteData(LCDC0,0x55);

  /* Sleep out */
  APP_LCDCWriteCommand(LCDC0,0x11);

  /* Display on */
  APP_LCDCWriteCommand(LCDC0,0x29);
}

/**
  * @brief  LCDC Config
  * @param  None
  * @retval None
  */
static void APP_LCDCConfig(void)
{
  LL_LCDC_SetMode(LCDC0,LL_LCDC_MODE_8080);
 
  LL_LCDC_Enable(LCDC0);

  /* LCD Reset */
  APP_LCDReset();

  /* LCD Get ID */
  APP_LCDCWriteCommand(LCDC0,0xD3);
  ID[0] = LL_LCDC_ReadData_8b(LCDC0);
  ID[1] = LL_LCDC_ReadData_8b(LCDC0);
  ID[2] = LL_LCDC_ReadData_8b(LCDC0);
  ID[3] = LL_LCDC_ReadData_8b(LCDC0);
  
  /* LCD back light on */
  APP_LCDBackLight(RESET);

  APP_LCDConfig();
}

/**
  * @brief  LCD Write Data
  * @param  None
  * @retval None
  */
static void APP_LCDCWriteData(LCDC_TypeDef *LCDCx,uint16_t Data)
{
  while(LL_LCDC_IsActiveFlag_BUSY(LCDCx)==1);
  LL_LCDC_WriteData_16b(LCDCx,Data);
  while(LL_LCDC_IsActiveFlag_BUSY(LCDCx)==1);
}

/**
  * @brief  LCD Write Command 
  * @param  None
  * @retval None
  */
static void APP_LCDCWriteCommand(LCDC_TypeDef *LCDCx,uint16_t Command)
{
  while(LL_LCDC_IsActiveFlag_BUSY(LCDCx)==1);
  LL_LCDC_WriteCommand_16b(LCDCx,Command);
  while(LL_LCDC_IsActiveFlag_BUSY(LCDCx)==1);
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
