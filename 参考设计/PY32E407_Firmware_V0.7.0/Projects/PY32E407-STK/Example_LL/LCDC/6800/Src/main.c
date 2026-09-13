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
/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_LCDCWriteData(LCDC_TypeDef *LCDCx,uint8_t Data);
static void APP_LCDCWriteCommand(LCDC_TypeDef *LCDCx,uint8_t Command);
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

  /* Set Mode 6800 */
  LL_LCDC_SetMode(LCDC0,LL_LCDC_MODE_6800);

  LL_LCDC_SetWriteDataHoldTime(LCDC0,3);
  LL_LCDC_SetReadDataHoldTime(LCDC0,3);

  /* Enable LCDC */
  LL_LCDC_Enable(LCDC0);

  /* Function set */
  APP_LCDCWriteCommand(LCDC0,0x38);

  /* Wait screen Busy = 0 */
  while((LL_LCDC_ReadCommand_8b(LCDC0)&0x80)!=0);

  /* Clear disaply */
  APP_LCDCWriteCommand(LCDC0,0x01);

  /* Wait screen Busy = 0 */
  while((LL_LCDC_ReadCommand_8b(LCDC0)&0x80)!=0);

  /* Disaply on */
  APP_LCDCWriteCommand(LCDC0,0x0f);

  /* Wait screen Busy = 0 */
  while((LL_LCDC_ReadCommand_8b(LCDC0)&0x80)!=0);

  APP_LCDCWriteData(LCDC0,'P');

  /* Wait screen Busy = 0 */
  while((LL_LCDC_ReadCommand_8b(LCDC0)&0x80)!=0);
  
  APP_LCDCWriteData(LCDC0,'U');

  /* Wait screen Busy = 0 */
  while((LL_LCDC_ReadCommand_8b(LCDC0)&0x80)!=0);
 
  APP_LCDCWriteData(LCDC0,'Y');
  /* Wait screen Busy = 0 */
  while((LL_LCDC_ReadCommand_8b(LCDC0)&0x80)!=0);

  APP_LCDCWriteData(LCDC0,'A');

  /* Wait screen Busy = 0 */
  while((LL_LCDC_ReadCommand_8b(LCDC0)&0x80)!=0);

  while (1)
  {
  }
}

/**
  * @brief  Gpio Config
  * @param  None
  * @retval None
  */
static void APP_GpioConfig(void)
{
  LL_GPIO_InitTypeDef GPIO_InitType= {0};

  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOD);
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOE); 
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOF);

  /* Set LCDC RS pin */
  GPIO_InitType.Pin        = LL_GPIO_PIN_4;
  GPIO_InitType.Mode       = LL_GPIO_MODE_ALTERNATE;
  GPIO_InitType.Alternate  = LL_GPIO_AF12_LCDC; 
  GPIO_InitType.Speed      = LL_GPIO_SPEED_FREQ_HIGH;
  GPIO_InitType.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  GPIO_InitType.Pull       = LL_GPIO_PULL_NO;
  LL_GPIO_Init(GPIOF,&GPIO_InitType);

  /* Initlize LCDC D2/D3/RW/EN/CS/D0/D1 pin */
  GPIO_InitType.Pin        = LL_GPIO_PIN_0 | LL_GPIO_PIN_1 | LL_GPIO_PIN_4 | LL_GPIO_PIN_5 | LL_GPIO_PIN_7 | LL_GPIO_PIN_14 | LL_GPIO_PIN_15;
  GPIO_InitType.Mode       = LL_GPIO_MODE_ALTERNATE;
  GPIO_InitType.Alternate  = LL_GPIO_AF12_LCDC; 
  GPIO_InitType.Speed      = LL_GPIO_SPEED_FREQ_HIGH;
  GPIO_InitType.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  GPIO_InitType.Pull       = LL_GPIO_PULL_NO;
  LL_GPIO_Init(GPIOD,&GPIO_InitType);

  /* Initlize LCDC D4/D5/D6/D7 pin */
  GPIO_InitType.Pin        = LL_GPIO_PIN_7 | LL_GPIO_PIN_8 | LL_GPIO_PIN_9 | LL_GPIO_PIN_10;
  GPIO_InitType.Mode       = LL_GPIO_MODE_ALTERNATE;
  GPIO_InitType.Alternate  = LL_GPIO_AF12_LCDC; 
  GPIO_InitType.Speed      = LL_GPIO_SPEED_FREQ_HIGH;
  GPIO_InitType.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  GPIO_InitType.Pull       = LL_GPIO_PULL_NO;
  LL_GPIO_Init(GPIOE,&GPIO_InitType);
}

/**
  * @brief  LCD Write Data
  * @param  LCDCx LCDC
  * @param  Data Data to Write
  * @retval None
  */
static void APP_LCDCWriteData(LCDC_TypeDef *LCDCx,uint8_t Data)
{
  while(LL_LCDC_IsActiveFlag_BUSY(LCDCx)==1);
  LL_LCDC_WriteData_8b(LCDCx,Data);
  while(LL_LCDC_IsActiveFlag_BUSY(LCDCx)==1);
}

/**
  * @brief  LCD Write Command
  * @param  LCDCx LCDC
  * @param  Command Command to Write
  * @retval None
  */
static void APP_LCDCWriteCommand(LCDC_TypeDef *LCDCx,uint8_t Command)
{
  while(LL_LCDC_IsActiveFlag_BUSY(LCDCx)==1);
  LL_LCDC_WriteCommand_8b(LCDCx,Command);
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
