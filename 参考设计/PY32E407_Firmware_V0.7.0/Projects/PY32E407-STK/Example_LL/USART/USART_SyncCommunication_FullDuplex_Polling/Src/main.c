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
/* Send buffer definition */
uint8_t aTxBuffer[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15};
uint8_t ubNbDataToTransmit = sizeof(aTxBuffer);
__IO uint8_t ubTransmitIndex = 0;

/* Receive buffer definition */
uint8_t aRxBuffer[sizeof(aTxBuffer)] = {0};
uint8_t ubNbDataToReceive = sizeof(aTxBuffer);
__IO uint8_t ubReceiveIndex = 0;

/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
#ifdef TRANSMITTER_BOARD
static void APP_ConfigUsart(void);
static void APP_UsartTransmitReceive(void);
#else
static void APP_ConfigSPI(void);
static void APP_SpiTransmitReceive(void);
#endif
static void APP_WaitAndCheckEndOfTransfer(void);
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

#ifdef TRANSMITTER_BOARD
  /* Initialize USART */
  APP_ConfigUsart();

  /* Initialization button */
  BSP_PB_Init(BUTTON_KEY,BUTTON_MODE_GPIO);

  /* Wait for the button to be pressed */
  while(BSP_PB_GetState(BUTTON_KEY) == 0);

  /* USART transfer */
  APP_UsartTransmitReceive();
#else
  /* Initialize SPI */
  APP_ConfigSPI();

  /* Enable SPI1 */
  LL_SPI_Enable(SPI1);
  APP_SpiTransmitReceive();
#endif

  /* Wait for the transfer to end and check the received data */
  APP_WaitAndCheckEndOfTransfer();

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

#ifdef TRANSMITTER_BOARD
/**
  * @brief  USART1 configuration function
  * @param  None
  * @retval None
  */
static void APP_ConfigUsart(void)
{
  LL_GPIO_InitTypeDef GPIO_InitStruct = {0};
  LL_USART_InitTypeDef USART_InitStruct = {0};

  /* Enable GPIOA clock */
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOA);
  /* Enable USART1 clock */
  LL_APB2_GRP1_EnableClock(LL_APB2_GRP1_PERIPH_USART1);
      
  /**USART1 GPIO Configuration
  PA08    ------> USART1_CK
  PA09    ------> USART1_TX
  PA10    ------> USART1_RX
  */
  /* Config Tx, Rx Pin */
  GPIO_InitStruct.Pin        = (LL_GPIO_PIN_8 | LL_GPIO_PIN_9 | LL_GPIO_PIN_10);
  /* Select alternate function mode */
  GPIO_InitStruct.Mode       = LL_GPIO_MODE_ALTERNATE;
  /* Set output speed */
  GPIO_InitStruct.Speed      = LL_GPIO_SPEED_FREQ_VERY_HIGH;
  /* Set output type to push pull */
  GPIO_InitStruct.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  /* Enable pull up */
  GPIO_InitStruct.Pull       = LL_GPIO_PULL_UP;
  /* Set alternate function to USART1 function  */
  GPIO_InitStruct.Alternate  = LL_GPIO_AF_7;
  /* Initialize GPIOA */
  LL_GPIO_Init(GPIOA,&GPIO_InitStruct);

  /* Set USART1 feature */
  /* Set baud rate */
  USART_InitStruct.BaudRate            = 115200;
  /* set word length to 8 bits: Start bit, 8 data bits, n stop bits */
  USART_InitStruct.DataWidth           = LL_USART_DATAWIDTH_8B;
  /* 1 stop bit */
  USART_InitStruct.StopBits            = LL_USART_STOPBITS_1;
  /* Parity control disabled  */
  USART_InitStruct.Parity              = LL_USART_PARITY_NONE;
  /* Set transmit and receive direction */
  USART_InitStruct.TransferDirection   = LL_USART_DIRECTION_TX_RX;
  USART_InitStruct.OverSampling        = LL_USART_OVERSAMPLING_16;

  /* Initialize USART */
  LL_USART_Init(USART1, &USART_InitStruct);
  /* Clock configuration : Phase 1 edges, Polarity Low, Last Bit Clock output enabled */
  LL_USART_ConfigClock(USART1, LL_USART_PHASE_1EDGE, LL_USART_POLARITY_LOW, LL_USART_LASTCLKPULSE_OUTPUT);

  LL_USART_ConfigSyncMode(USART1);
  LL_USART_Enable(USART1);
}

/**
  * @brief  USART transmit and receive function
  * @param  None
  * @retval None
  */
static void APP_UsartTransmitReceive(void)
{
  uint32_t             txallowed = 1U;

  /* transmit data */
  LL_USART_TransmitData8(USART1, aTxBuffer[ubTransmitIndex++]);

  /* Data is sent and received in polling mode */
  while((ubReceiveIndex < ubNbDataToReceive) || (ubTransmitIndex < ubNbDataToTransmit))
  {
    /* wait to send data(The following conditions are required to start sending data：send data register is empty，
       there's still data to transfer ，Receive completely) */
    if ((LL_USART_IsActiveFlag_TXE(USART1) == 1) && (ubTransmitIndex < ubNbDataToTransmit) && txallowed == 1U)
    {
      LL_USART_TransmitData8(USART1, aTxBuffer[ubTransmitIndex++]);
      txallowed = 0U;
    }

    /* wait to receive data(The following conditions are required to start receiving data：receive data register is empty，
       there's still data to receive ，send completely) */
    if((LL_USART_IsActiveFlag_RXNE(USART1) == 1) && (ubReceiveIndex < ubNbDataToReceive))
    {
      aRxBuffer[ubReceiveIndex++] = LL_USART_ReceiveData8(USART1);
      txallowed = 1U;
    }
  }
}
#else
/**
  * @brief  SPI1 configuration function
  * @param  None
  * @retval None
  */
static void APP_ConfigSPI(void)
{
  LL_SPI_InitTypeDef SPI_InitStruct = {0};
  LL_GPIO_InitTypeDef GPIO_InitStruct = {0};

  /* Enable SPI1 clock */
  LL_APB2_GRP1_EnableClock(LL_APB2_GRP1_PERIPH_SPI1);
  /* Enable GPIOA clock */
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOA);

  /** SPI1 Pin config
  PA05   ------> SPI1_SCK
  PA06   ------> SPI1_MISO
  PA07   ------> SPI1_MOSI
  */
  /* SPI SCK GPIO pin configuration  */
  GPIO_InitStruct.Pin = LL_GPIO_PIN_5;
  GPIO_InitStruct.Mode = LL_GPIO_MODE_ALTERNATE;
  GPIO_InitStruct.Speed = LL_GPIO_SPEED_FREQ_HIGH;
  GPIO_InitStruct.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  GPIO_InitStruct.Pull = LL_GPIO_PULL_DOWN;
  GPIO_InitStruct.Alternate = LL_GPIO_AF_5;
  LL_GPIO_Init(GPIOA, &GPIO_InitStruct);

  /* Configure GPIO as SPI: MISO/MOSI*/
  GPIO_InitStruct.Pin = (LL_GPIO_PIN_6 | LL_GPIO_PIN_7);
  GPIO_InitStruct.Mode = LL_GPIO_MODE_ALTERNATE;
  GPIO_InitStruct.Speed = LL_GPIO_SPEED_FREQ_HIGH;
  GPIO_InitStruct.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  GPIO_InitStruct.Pull = LL_GPIO_PULL_UP;
  GPIO_InitStruct.Alternate = LL_GPIO_AF_5;
  LL_GPIO_Init(GPIOA, &GPIO_InitStruct);

  /* SPI1 parameter configuration*/
  SPI_InitStruct.TransferDirection = LL_SPI_FULL_DUPLEX;
  SPI_InitStruct.Mode              = LL_SPI_MODE_SLAVE;
  SPI_InitStruct.DataWidth         = LL_SPI_DATAWIDTH_8BIT;
  SPI_InitStruct.ClockPolarity     = LL_SPI_POLARITY_LOW;
  SPI_InitStruct.ClockPhase        = LL_SPI_PHASE_1EDGE;
  SPI_InitStruct.NSS               = LL_SPI_NSS_SOFT;
  SPI_InitStruct.BaudRate          = LL_SPI_BAUDRATEPRESCALER_DIV256;
  SPI_InitStruct.BitOrder          = LL_SPI_LSB_FIRST;
  SPI_InitStruct.CRCCalculation    = LL_SPI_CRCCALCULATION_DISABLE;
  SPI_InitStruct.CRCPoly           = 0x0U;
  LL_SPI_Init(SPI1, &SPI_InitStruct);
}

/**
  * @brief  SPI1 receive and send data program
  * @param  None
  * @retval None
  */
static void APP_SpiTransmitReceive(void)
{
  uint32_t             txallowed = 1U;

  /* Write data to data register(after start communicating，perform send) */
  LL_SPI_TransmitData8(SPI1, aTxBuffer[ubTransmitIndex++]);

  /* Data is sent and received in polling mode */
  while((ubReceiveIndex < ubNbDataToReceive) || (ubTransmitIndex < ubNbDataToTransmit))
  {
    /* wait to send data(The following conditions are required to start sending data：send data register is empty，
       there's still data to transfer ，Receive completely) */
    if ((LL_SPI_IsActiveFlag_TXE(SPI1) == 1) && (ubTransmitIndex < ubNbDataToTransmit) && txallowed == 1U)
    {
      LL_SPI_TransmitData8(SPI1, aTxBuffer[ubTransmitIndex++]);
      txallowed = 0U;
    }

    /* wait to receive data(The following conditions are required to start receiving data：receive data register is empty，
       there's still data to receive ，send completely) */
    if((LL_SPI_IsActiveFlag_RXNE(SPI1) == 1) && (ubReceiveIndex < ubNbDataToReceive))
    {
      aRxBuffer[ubReceiveIndex++] = LL_SPI_ReceiveData8(SPI1);
      txallowed = 1U;
    }
  }
}
#endif

/**
  * @brief  Wait transfer to be completed and check the received data
  * @param  None
  * @retval None
  */
static void APP_WaitAndCheckEndOfTransfer(void)
{
  /* 1 - Wait transfer to be completed  */
  while (ubTransmitIndex != ubNbDataToTransmit)
  {
  }
  /* 2 - Wait data to be received completely */
  while (ubNbDataToReceive > ubReceiveIndex)
  {
  }
  /* 2 - Compare sent and received data */
  if(APP_Buffercmp8((uint8_t*)aTxBuffer, (uint8_t*)aRxBuffer, ubNbDataToTransmit))
  {
    /* error handling */
    APP_LedBlinking();
  }
  else
  {
    /* If the data is received correctly,then the LED will be on */
    BSP_LED_On(LED_GREEN);
  }
}

/**
  * @brief  Character comparison function
  * @param  pBuffer1：Pointer to buffer 1 that to be compared
  * @param  pBuffer2：Pointer to buffer 2 that to be compared
  * @param  BufferLength：The number of characters to be compared
  * @retval 0: The comparison value is the same; 1: The comparison value is different
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
  * @brief  LED blinking
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
