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
#define BOARD_IN_MASTER_MODE
#define DATA_LENGTH       16

/* Private variables ---------------------------------------------------------*/
LL_I2S_InitTypeDef I2S_InitStruct = {0};

uint16_t      TxBuff[16] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16};
uint16_t      *pTxBuffPtr;
__IO uint16_t TxXferCount;

uint16_t RxBuff[16] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
uint16_t      *pRxBuffPtr;
__IO uint16_t RxXferCount;

__IO ITStatus I2sReady = RESET;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_ConfigI2SPin(void);
static void APP_I2STransmit_IT(uint16_t *pData, uint16_t Size);
static void APP_I2SReceive_IT(uint16_t *pData, uint16_t Size);
static void APP_CheckEndOfTransfer(void);
static uint8_t APP_Buffercmp8(uint8_t* pBuffer1, uint8_t* pBuffer2, uint8_t BufferLength);
static void APP_LedBlinking(void);

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

  /* Initialize BUTTON */
  BSP_PB_Init(BUTTON_KEY,BUTTON_MODE_GPIO);

  /* Wait for the button to be pressed */
  while(BSP_PB_GetState(BUTTON_KEY) == 0);

  /* Configure I2S1 pins */
  APP_ConfigI2SPin();

  /* Enable I2S1 clock */
  LL_APB2_GRP1_EnableClock(LL_APB2_GRP1_PERIPH_SPI1);

  /* Initialize I2S1 */
#ifdef BOARD_IN_MASTER_MODE
  I2S_InitStruct.Mode              = LL_I2S_MODE_MASTER_TX; /* Master transmit */
#else
  I2S_InitStruct.Mode              = LL_I2S_MODE_SLAVE_RX;  /* Slave receive */
#endif
  I2S_InitStruct.Standard          = LL_I2S_STANDARD_PHILIPS;
  I2S_InitStruct.DataFormat        = LL_I2S_DATAFORMAT_16B;
  I2S_InitStruct.MCLKOutput        = LL_I2S_MCLK_OUTPUT_DISABLE;
  I2S_InitStruct.AudioFreq         = LL_I2S_AUDIOFREQ_8K;
  I2S_InitStruct.ClockPolarity     = LL_I2S_POLARITY_HIGH;
  LL_I2S_Init(SPI1, &I2S_InitStruct);

#ifdef BOARD_IN_MASTER_MODE
  /* Transmit data as master */
  APP_I2STransmit_IT((uint16_t *)TxBuff, DATA_LENGTH);
#else
  /* Receive data as slave */
  APP_I2SReceive_IT((uint16_t *)RxBuff, DATA_LENGTH);
#endif
  /* Wait for transmit completion */
  while (I2sReady != SET)
  {
  }
  I2sReady = RESET;

#ifdef BOARD_IN_MASTER_MODE
  /* Wait for the slave to be ready for data transmission */
  LL_mDelay(1);
#endif

  /* Initialize I2S1 */
  LL_I2S_Disable(SPI1); /* Disable to complete reinitialization */
  /* Initialize I2S1 */
#ifdef BOARD_IN_MASTER_MODE
  I2S_InitStruct.Mode              = LL_I2S_MODE_MASTER_RX; /* Master Receive */
#else
  I2S_InitStruct.Mode              = LL_I2S_MODE_SLAVE_TX;  /* Slave transmit */
#endif
  I2S_InitStruct.Standard          = LL_I2S_STANDARD_PHILIPS;
  I2S_InitStruct.DataFormat        = LL_I2S_DATAFORMAT_16B;
  I2S_InitStruct.MCLKOutput        = LL_I2S_MCLK_OUTPUT_DISABLE;
  I2S_InitStruct.AudioFreq         = LL_I2S_AUDIOFREQ_8K;
  I2S_InitStruct.ClockPolarity     = LL_I2S_POLARITY_HIGH;
  LL_I2S_Init(SPI1, &I2S_InitStruct);
  
#ifdef BOARD_IN_MASTER_MODE
  /* Receive data as master */
  APP_I2SReceive_IT((uint16_t *)RxBuff, DATA_LENGTH);
#else
  /* Transmit data as slave */
  APP_I2STransmit_IT((uint16_t *)TxBuff, DATA_LENGTH);
#endif
  /* Wait for receive completion  */
  while (I2sReady != SET)
  {
  }
  I2sReady = RESET;

  /* Check the received data */
  APP_CheckEndOfTransfer();

  while (1)
  {
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
  * @brief  Configure I2S1 pins
  * @param  None
  * @retval None
  */
static void APP_ConfigI2SPin(void)
{
  LL_GPIO_InitTypeDef GPIO_InitStruct = {0};

  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOA);

  /**I2S1 pin configuration
  PA04 ------> I2S1_WS
  PA05 ------> I2S1_CK
  PA07 ------> I2S1_SD
  */
  /* GPIO configured as I2S1 WS, CK, SD*/
  GPIO_InitStruct.Pin = (LL_GPIO_PIN_4 | LL_GPIO_PIN_5 | LL_GPIO_PIN_7);
  GPIO_InitStruct.Mode = LL_GPIO_MODE_ALTERNATE;
  GPIO_InitStruct.Speed = LL_GPIO_SPEED_FREQ_HIGH;
  GPIO_InitStruct.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  GPIO_InitStruct.Pull = LL_GPIO_PULL_NO;
  GPIO_InitStruct.Alternate = LL_GPIO_AF_5;
  LL_GPIO_Init(GPIOA, &GPIO_InitStruct);

  /* Enable SPI NVIC interrupts */
  NVIC_SetPriority(SPI1_IRQn,0);
  NVIC_EnableIRQ(SPI1_IRQn);
}

/**
  * @brief  I2S1 transmission function using Interrupt
  * @param  None
  * @retval None
  */
static void APP_I2STransmit_IT(uint16_t *pData, uint16_t Size)
{
  /* Record the transmission data and count */
  pTxBuffPtr = pData;
  TxXferCount = Size;

  /* Enable transmit data register empty interrupt and error interrupt */
  LL_I2S_EnableIT_TXE(SPI1);
  LL_I2S_EnableIT_ERR(SPI1);

  /* Enable I2S1 */
  LL_I2S_Enable(SPI1);
}

/**
  * @brief  I2S1 reception function using Interrupt
  * @param  None
  * @retval None
  */
static void APP_I2SReceive_IT(uint16_t *pData, uint16_t Size)
{
  /* Record the received data and count */
  pRxBuffPtr = pData;
  RxXferCount = Size;

  /* Enable receive data register not empty interrupt and error interrupt */
  LL_I2S_EnableIT_RXNE(SPI1);
  LL_I2S_EnableIT_ERR(SPI1);

  /* Enable I2S1 */
  LL_I2S_Enable(SPI1);
}

/**
  * @brief  I2S1 interrupt callback function
  * @param  None
  * @retval None
  */
void APP_I2sIRQCallback(void)
{
  /* Receive data */
  if ((LL_I2S_IsActiveFlag_OVR(SPI1) == 0) &&
      (LL_I2S_IsActiveFlag_RXNE(SPI1) == 1) && (LL_I2S_IsEnabledIT_RXNE(SPI1) == 1))
  {
    (*pRxBuffPtr) = LL_I2S_ReceiveData16(SPI1);
    pRxBuffPtr++;
    RxXferCount--;

    if (RxXferCount == 0U)
    {
      LL_I2S_DisableIT_RXNE(SPI1);
      LL_I2S_DisableIT_ERR(SPI1);
      I2sReady = SET;
    }
    return;
  }

  /* Send data */
  if ((LL_I2S_IsActiveFlag_TXE(SPI1) == 1) && (LL_I2S_IsEnabledIT_TXE(SPI1) == 1))
  {
    LL_I2S_TransmitData16(SPI1, (*pTxBuffPtr));
    pTxBuffPtr++;
    TxXferCount--;
    
    if (TxXferCount == 0U)
    {
      LL_I2S_DisableIT_TXE(SPI1);
      LL_I2S_DisableIT_ERR(SPI1);
      I2sReady = SET;
    }
    return;
  }

  /* Error handling */
  if (LL_I2S_IsEnabledIT_ERR(SPI1) == 1)
  {
    if((LL_I2S_IsActiveFlag_OVR(SPI1) == 1) || (LL_I2S_IsActiveFlag_UDR(SPI1) == 1))
    {
      APP_ErrorHandler();
    }
  }
}

/**
  * @brief  Wait for the end of transfer and verify data
  * @param  None
  * @retval None
  */
static void APP_CheckEndOfTransfer(void)
{
  /* Compare the transmitted data with the received data */
  if((APP_Buffercmp8((uint8_t*)TxBuff, (uint8_t*)RxBuff, DATA_LENGTH)))
  {
    /* Error handling */
    APP_LedBlinking();
  }
  else
  {
    /* If data received successfully, turn on the LED */
    BSP_LED_On(LED_GREEN);
  }
}

/**
  * @brief  Character comparison function
  * @param  pBuffer1：Pointer to the first buffer to be compared
  * @param  pBuffer2：Pointer to the second buffer to be compared
  * @param  BufferLength：Number of characters to compare
  * @retval 0: buffers are the same; 1: buffers are different
  */
static uint8_t APP_Buffercmp8(uint8_t* pBuffer1, uint8_t* pBuffer2, uint8_t BufferLength)
{
  while (BufferLength--)
  {
    if (*pBuffer1 != *pBuffer2)
    {
      return 1;
    }
    pBuffer1++;
    pBuffer2++;
  }

  return 0;
}

/**
  * @brief  LED blinking function
  * @param  None
  * @retval None
  */
static void APP_LedBlinking(void)
{
  while (1)
  {
    BSP_LED_Toggle(LED_GREEN); 
    LL_mDelay(200);
  }
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
