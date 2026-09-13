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
#define BUFFER_SIZE    39   /* 9 u32 + 1 u16 + 1 u8 */

/* Private variables ---------------------------------------------------------*/
static const uint8_t aDataBuffer[BUFFER_SIZE] =
{
  0x21, 0x10, 0x00, 0x00, 0x63, 0x30, 0x42, 0x20, 0xa5, 0x50, 0x84, 0x40,
  0xe7, 0x70, 0xc6, 0x60, 0x4a, 0xa1, 0x29, 0x91, 0x8c, 0xc1, 0x6b, 0xb1,
  0xce, 0xe1, 0xad, 0xd1, 0x31, 0x12, 0xef, 0xf1, 0x52, 0x22, 0x73, 0x32,
  0xa1, 0xb2, 0xc3
};
/* Used for storing CRC Value */
uint32_t CRCValue = 0;
/* Expected CRC Value */
uint32_t ExpectedCRCValue = 0xA9866043;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static uint32_t APP_CalculateCRC(uint32_t BufferLength);
static void APP_CRC_Init(void);
static void CheckCRCResultValue(void);

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

  /* Initialize CRC */
  APP_CRC_Init();

  /* Calculate CRC */
  CRCValue = APP_CalculateCRC(BUFFER_SIZE);

  /* Check if CRC computed result value is equal to expected one */
  CheckCRCResultValue();

  /* Infinite loop */
  while (1)
  {
  }
}

/**
  * @brief CRC Initialization Function
  * @param None
  * @retval None
  */
static void APP_CRC_Init(void)
{
  /* Peripheral clock enable */
  LL_AHB1_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_CRC);

  LL_CRC_SetInputDataReverseMode(CRC, LL_CRC_INDATA_REVERSE_NONE);
  LL_CRC_SetOutputDataReverseMode(CRC, LL_CRC_OUTDATA_REVERSE_NONE);
  LL_CRC_SetPolynomialCoef(CRC, LL_CRC_DEFAULT_CRC32_POLY);
  LL_CRC_SetPolynomialSize(CRC, LL_CRC_POLYLENGTH_32B);
  LL_CRC_SetInitialData(CRC, LL_CRC_DEFAULT_CRC_INITVALUE);
}

/**
  * @brief  This function performs CRC calculation on BufferSize bytes from input data buffer aDataBuffer.
  * @param  BufferSize Nb of bytes to be processed for CRC calculation
  * @retval 32-bit CRC value computed on input data buffer
  */
static uint32_t APP_CalculateCRC(uint32_t BufferSize)
{
  register uint32_t data = 0;
  register uint32_t index = 0;

  /* Compute the CRC of Data Buffer array*/
  for (index = 0; index < (BufferSize / 4); index++)
  {
    data = (uint32_t)((aDataBuffer[4 * index + 3] << 24) | (aDataBuffer[4 * index + 2] << 16) | (aDataBuffer[4 * index + 1] << 8) | aDataBuffer[4 * index]);
    LL_CRC_FeedData32(CRC, data);
  }

  /* Last bytes specific handling */
  if ((BUFFER_SIZE % 4) != 0)
  {
    if (BUFFER_SIZE % 4 == 1)
    {
      LL_CRC_FeedData8(CRC, aDataBuffer[4 * index]);
    }
    if (BUFFER_SIZE % 4 == 2)
    {
      LL_CRC_FeedData16(CRC, (uint16_t)((aDataBuffer[4 * index + 1] << 8) | aDataBuffer[4 * index]));
    }
    if (BUFFER_SIZE % 4 == 3)
    {
      LL_CRC_FeedData16(CRC, (uint16_t)((aDataBuffer[4 * index + 1] << 8) | aDataBuffer[4 * index]));
      LL_CRC_FeedData8(CRC, aDataBuffer[4 * index + 2]);
    }
  }

  /* return CRC Calculation result */
  return (LL_CRC_ReadData32(CRC));
}

/**
  * @brief  Check CRC computation result value.
  * @param  None
  * @retval None
  */
static void CheckCRCResultValue(void)
{
  /* Compare the CRC value to the Expected one */
  if (CRCValue != ExpectedCRCValue)
  {
    /* Wrong CRC value: Set LED2 to Blinking mode (Error) */
    BSP_LED_Off(LED_GREEN);
  }
  else
  {
    /* Right CRC value: Turn LED2 on */
    BSP_LED_On(LED_GREEN);
  }
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
  * @param  file：Pointer to the source file name
  * @param  line：assert_param error line source number
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
