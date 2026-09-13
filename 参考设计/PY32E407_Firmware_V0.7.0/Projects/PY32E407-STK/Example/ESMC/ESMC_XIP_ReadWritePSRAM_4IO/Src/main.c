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
#define BUFFERSIZE  1024
#define Address 0x90000000

/* Private variables ---------------------------------------------------------*/
ESMC_HandleTypeDef hesmc;
ESMC_InitTypeDef ESMC_initTypeDef;

uint32_t aRxBuffer[BUFFERSIZE];
uint32_t aTxBuffer[BUFFERSIZE];

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_EsmcInit(void);
static HAL_StatusTypeDef APP_DataCheck(uint32_t *pSrc, uint32_t *pDst, uint32_t length);
static void APP_XIPReadConfig(void);
static void APP_XIPWriteConfig(void);

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

  BSP_LED_Init(LED_GREEN);

  /* Initialize ESMC */
  APP_EsmcInit();
  
  /* Configure ESMC to work in XIP Write mode */
  APP_XIPWriteConfig();

  /* Configure ESMC to work in XIP Read mode */
  APP_XIPReadConfig();

  for(int i=0;i<BUFFERSIZE;i++)
  {
    aTxBuffer[i]=0xffffffff;
  }
  
  for(int32_t i=0;i<BUFFERSIZE;i++)
  {
    *(uint32_t*)(Address+i*4) = aTxBuffer[i];
  }

  for(int32_t i=0;i<BUFFERSIZE;i++)
  {
     aRxBuffer[i] = *(uint32_t*)(Address+i*4);
  }

  /* Compare the read data with write data */
  if (HAL_OK != APP_DataCheck(aRxBuffer, aTxBuffer, BUFFERSIZE))
  {
    BSP_LED_Off(LED_GREEN);
    APP_ErrorHandler();
  }  

  for(int i=0;i<BUFFERSIZE;i++)
  {
    aTxBuffer[i]=i;
  }
  
  for(int32_t i=0;i<BUFFERSIZE;i++)
  {
    *(uint32_t*)(Address+i*4) = aTxBuffer[i];
  }

  for(int32_t i=0;i<BUFFERSIZE;i++)
  {
     aRxBuffer[i] = *(uint32_t*)(Address+i*4);
  }

  /* Compare the read data with write data */
  if (HAL_OK != APP_DataCheck(aRxBuffer, aTxBuffer, BUFFERSIZE))
  {
    BSP_LED_Off(LED_GREEN);
    APP_ErrorHandler();
  }

  BSP_LED_On(LED_GREEN);
  while(1)
  {
  }
}

/**
  * @brief  ESMC initialization function
  * @param  None
  * @retval None
  */
static void APP_EsmcInit()
{
  hesmc.Instance=ESMC;
  ESMC_initTypeDef.ClockPrescaler = 3;
  ESMC_initTypeDef.ClockMode = ESMC_CLOCK_MODE_0;
  ESMC_initTypeDef.DualFlash = ESMC_DUALFLASH_DISABLE;
  ESMC_initTypeDef.ChipSelectHighTime = ESMC_CS_HIGH_TIME_1_CYCLE;
  ESMC_initTypeDef.ChipSelectHoldTime = ESMC_CS_HOLD_TIME_2_CYCLE;
  hesmc.Init=ESMC_initTypeDef;
  HAL_ESMC_Init(&hesmc);
}

/**
  * @brief  XIP Write Config
  * @param  None
  * @retval None
  */
static void APP_XIPWriteConfig()
{
  ESMC_CommandTypeDef cmdStc={0};

  cmdStc.TransferFormat    = ESMC_TRANSFER_FORMAT_QUAD;
  cmdStc.InstructionMode   = ESMC_INSTRUCTION_SINGLE_LINE;
  cmdStc.InstrcutionLen    = ESMC_INSTRCUTION_LEN_1_BYTE;
  cmdStc.Instruction       = 0x38;
  cmdStc.AddressMode       = ESMC_ADDRESS_MULTI_LINES;
  cmdStc.AddressSize       = ESMC_ADDRESS_24_BITS;
  cmdStc.DQSMode           = ESMC_DQS_DISABLE;
  cmdStc.AlternateByteMode = ESMC_ALTERNATE_BYTES_DISABLE;
  cmdStc.DdrMode           = ESMC_DDR_DISABLE;
  cmdStc.DummyCycles       = 0x0;
  cmdStc.CSPinSel          = ESMC_SELECT_PIN_CS0;
  cmdStc.XspiDualLatency   = ESMC_XSPI_DUALLATENCY_DISABLE;
  HAL_ESMC_MemoryWrite(&hesmc, &cmdStc);
}

/**
  * @brief  XIP Read Config
  * @param  None
  * @retval None
  */
static void APP_XIPReadConfig()
{
  ESMC_CommandTypeDef cmdStc={0};

  cmdStc.TransferFormat    = ESMC_TRANSFER_FORMAT_QUAD;
  cmdStc.InstructionMode   = ESMC_INSTRUCTION_SINGLE_LINE;
  cmdStc.InstrcutionLen    = ESMC_INSTRCUTION_LEN_1_BYTE;
  cmdStc.Instruction       = 0xeb;
  cmdStc.AddressMode       = ESMC_ADDRESS_MULTI_LINES;
  cmdStc.AddressSize       = ESMC_ADDRESS_24_BITS;
  cmdStc.AlternateByteMode = ESMC_ALTERNATE_BYTES_DISABLE;
  cmdStc.AlternateByte     = 0xff;
  cmdStc.DdrMode           = ESMC_DDR_DISABLE;
  cmdStc.DummyCycles       = 0x6;
  cmdStc.CSPinSel          = ESMC_SELECT_PIN_CS0;
  cmdStc.DQSMode           = ESMC_DQS_DISABLE;
  cmdStc.SampleShifting    = ESMC_SAMPLE_SHIFTING_1_CYCLE;
  cmdStc.XspiDualLatency   = ESMC_XSPI_DUALLATENCY_DISABLE;
  HAL_ESMC_MemoryMapped(&hesmc, &cmdStc);
}

/**
  * @brief  Data comparison function
  * @param  pSrc   Source data pointer
  * @param  pDst   Target data pointer
  * @param  length Data length
  * @retval Error Code
  */
static HAL_StatusTypeDef APP_DataCheck(uint32_t *pSrc, uint32_t *pDst, uint32_t length)
{
  HAL_StatusTypeDef status = HAL_OK;

  for(uint32_t i=0; i<length; i++)
  {
    if(pSrc[i]!=pDst[i])
    {
      status = HAL_ERROR;
      break;
    }
  }

  return status;
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
