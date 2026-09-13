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
#define COUNTOF(__BUFFER__)   (sizeof(__BUFFER__) / sizeof(*(__BUFFER__)))
#define TXSTARTMESSAGESIZE    (COUNTOF(aTxStartMessage) - 1)
#define TXENDMESSAGESIZE      (COUNTOF(aTxEndMessage) - 1)

/* Private variables ---------------------------------------------------------*/
uint8_t aRxBuffer[12] = {0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff};
uint8_t aTxStartMessage[] = "\n\r LPUART Hyperterminal communication based on interrupt\n\r Enter 12 characters using keyboard :\n\r";
uint8_t aTxEndMessage[] = "\n\r Example Finished\n\r";

uint8_t *TxBuff = NULL;
__IO uint16_t TxCount = 0;

uint8_t *RxBuff = NULL;
__IO uint16_t RxCount = 0;

__IO ITStatus LpuartReady = RESET;

uint32_t  EieFlags = 0;
uint32_t  ErrorFlags = 0;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_ConfigLpuart(void);
static void APP_LpuartTransmit_IT(LPUART_TypeDef *LPLPUARTx, uint8_t *pData, uint16_t Size);
static void APP_LpuartReceive_IT(LPUART_TypeDef *LPLPUARTx, uint8_t *pData, uint16_t Size);
static void APP_WaitToReady(void);

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

  /* LPUART configuration */
  APP_ConfigLpuart();

 /* Start the transmission process */
  APP_LpuartTransmit_IT(LPUART, (uint8_t*)aTxStartMessage, TXSTARTMESSAGESIZE);
  APP_WaitToReady();
  
  /* Put LPUART peripheral in reception process */
  APP_LpuartReceive_IT(LPUART, (uint8_t *)aRxBuffer, 12);
  APP_WaitToReady();

  /* Send the received Buffer */
  APP_LpuartTransmit_IT(LPUART, (uint8_t*)aRxBuffer, 12);
  APP_WaitToReady();
  
  /* Send the End Message */
  APP_LpuartTransmit_IT(LPUART, (uint8_t*)aTxEndMessage, TXENDMESSAGESIZE);
  APP_WaitToReady();
  
  if(EieFlags)
  {
    while (1)
    {
      /* If some error occurs during transmission, the LED blinking
         and the test failed */
      BSP_LED_Toggle(LED_GREEN); 
      LL_mDelay(500);
    }
  }
  else
  {
    /* Turn on LED if test passes then enter infinite loop */
    BSP_LED_On(LED_GREEN);
  }

  /* Infinite loop */
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
  * @brief  Wait transfer complete
  * @param  None
  * @retval None
  */
static void APP_WaitToReady(void)
{
  while (LpuartReady != SET);
  
  LpuartReady = RESET;
}

/**
  * @brief  LPUART1 configuration function
  * @param  None
  * @retval None
  */
static void APP_ConfigLpuart(void)
{
  LL_GPIO_InitTypeDef GPIO_InitStruct = {0};
  LL_LPUART_InitTypeDef LPUART_InitStruct = {0};

  /* Enable GPIOC clock */
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOC);
  /* Enable LPUART1 clock */
  LL_APB1_GRP2_EnableClock(LL_APB1_GRP2_PERIPH_LPUART1);
  
  /**LPUART1 GPIO Configuration
  PC0     ------> LPUART_RX
  PC1     ------> LPUART_TX
  */
  /* Config Tx, Rx Pin */
  GPIO_InitStruct.Pin        = (LL_GPIO_PIN_0 | LL_GPIO_PIN_1);
  /* Select alternate function mode */
  GPIO_InitStruct.Mode       = LL_GPIO_MODE_ALTERNATE;
  /* Set output speed */
  GPIO_InitStruct.Speed      = LL_GPIO_SPEED_FREQ_VERY_HIGH;
  /* Set output type to push pull */
  GPIO_InitStruct.OutputType = LL_GPIO_OUTPUT_PUSHPULL;
  /* Enable pull up */
  GPIO_InitStruct.Pull       = LL_GPIO_PULL_UP;
  /* Set alternate function to LPUART1 function  */
  GPIO_InitStruct.Alternate  = LL_GPIO_AF_8;
  /* Initialize GPIOC */
  LL_GPIO_Init(GPIOC,&GPIO_InitStruct);

  /* Set LPUART1 interrupt priority  */
  NVIC_SetPriority(LPUART_IRQn, 0);
  /* Enable LPUART1 interrupt request */
  NVIC_EnableIRQ(LPUART_IRQn);

  /* Set LPUART feature */
  LPUART_InitStruct.PrescalerValue      = LL_LPUART_PRESCALER_DIV1;
  /* Set baud rate */
  LPUART_InitStruct.BaudRate            = 9600;
  /* set word length to 8 bits: Start bit, 8 data bits, n stop bits */
  LPUART_InitStruct.DataWidth           = LL_LPUART_DATAWIDTH_8B;
  /* 1 stop bit */
  LPUART_InitStruct.StopBits            = LL_LPUART_STOPBITS_1;
  /* Parity control disabled  */
  LPUART_InitStruct.Parity              = LL_LPUART_PARITY_NONE;
  /* Set transmit and receive direction */
  LPUART_InitStruct.TransferDirection   = LL_LPUART_DIRECTION_TX_RX;
  LPUART_InitStruct.HardwareFlowControl = LL_LPUART_HWCONTROL_NONE;

  /* Initialize LPUART */
  LL_LPUART_Init(LPUART, &LPUART_InitStruct);
  LL_LPUART_Enable(LPUART);
}

/**
  * @brief  LPUART transmit function
  * @param  LPUARTx：LPUART module
  * @param  pData：transmit buffer
  * @param  Size：Size of the transmit buffer
  * @retval None
  */
static void APP_LpuartTransmit_IT(LPUART_TypeDef *LPUARTx, uint8_t *pData, uint16_t Size)
{
  TxBuff = pData;
  TxCount = Size;

  /* Enable transmit data register empty interrupt */
  LL_LPUART_EnableIT_TXE(LPUARTx);
}

/**
  * @brief  LPUART receive function
  * @param  LPUARTx：LPUART module, can be LPUART1 or LPUART2
  * @param  pData：receive buffer
  * @param  Size：Size of the receive buffer
  * @retval None
  */
static void APP_LpuartReceive_IT(LPUART_TypeDef *LPUARTx, uint8_t *pData, uint16_t Size)
{
  RxBuff = pData;
  RxCount = Size;

  /* Enable parity error interrupt */
  LL_LPUART_EnableIT_PE(LPUARTx);
  /* Enable Error Interrupt */
  LL_LPUART_EnableIT_ERROR(LPUARTx);
  /* Enable receive data register not empty interrupt */
  LL_LPUART_EnableIT_RXNE(LPUARTx);
  /* Enable receive and begins searching for a start bit */
  LL_LPUART_EnableDirectionRx(LPUARTx);
}

/**
  * @brief  LPUART interrupt callback function
  * @param  LPUARTx：LPUART module, can be LPUART1 or LPUART2
  * @retval None
  */
void APP_LpuartIRQCallback(LPUART_TypeDef *LPUARTx)

{
  /* Check SR register PE,FE,ORE,NE bit */
  ErrorFlags = (LL_LPUART_IsActiveFlag_PE(LPUARTx)  | LL_LPUART_IsActiveFlag_FE(LPUARTx) | \
                LL_LPUART_IsActiveFlag_ORE(LPUARTx) | LL_LPUART_IsActiveFlag_NE(LPUARTx));
  if (ErrorFlags == RESET)
  {
    /* The receive data register is not empty */
    if ((LL_LPUART_IsActiveFlag_RXNE(LPUARTx) != RESET) && (LL_LPUART_IsEnabledIT_RXNE(LPUARTx) != RESET))
    {
      *RxBuff = LL_LPUART_ReceiveData8(LPUARTx);
       RxBuff++;

      if (--RxCount == 0U)
      {
        LL_LPUART_DisableIT_RXNE(LPUARTx);
        LL_LPUART_DisableIT_PE(LPUARTx);
        LL_LPUART_DisableIT_ERROR(LPUARTx);
        LL_LPUART_DisableDirectionRx(LPUARTx);

        LpuartReady = SET;
      }
      return;
    }
  }

  /* An error occurred during receiving data */
  if (ErrorFlags != RESET)
  {
    /* Parity error */
    if(LL_LPUART_IsActiveFlag_PE(LPUARTx))
    {
      LL_LPUART_ClearFlag_PE(LPUARTx);
    }
    /* Frame error */
    if(LL_LPUART_IsActiveFlag_FE(LPUARTx))
    {
      LL_LPUART_ClearFlag_FE(LPUARTx);
    }
    /* Noise error */
    if(LL_LPUART_IsActiveFlag_NE(LPUARTx))
    {
      LL_LPUART_ClearFlag_NE(LPUARTx);
    }
    /* Overrun error */
    if(LL_LPUART_IsActiveFlag_ORE(LPUARTx))
    {
      LL_LPUART_ClearFlag_ORE(LPUARTx);
    }

    /* Error callback function */
    APP_LpuartErrorCallback();
  }

  /* The transmit data register is not empty */
  if ((LL_LPUART_IsActiveFlag_TXE(LPUARTx) != RESET) && (LL_LPUART_IsEnabledIT_TXE(LPUARTx) != RESET))
  {
    LL_LPUART_TransmitData8(LPUARTx, *TxBuff);
    TxBuff++;

    if (--TxCount == 0U)
    {
      LL_LPUART_DisableIT_TXE(LPUARTx);
      LL_LPUART_EnableIT_TC(LPUARTx);
    }

    return;
  }

  /* Transmit complete */
  if ((LL_LPUART_IsActiveFlag_TC(LPUARTx) != RESET) && (LL_LPUART_IsEnabledIT_TC(LPUARTx) != RESET))
  {
    LL_LPUART_DisableIT_TC(LPUARTx);
    LpuartReady = SET;

    return;
  }
}

/**
  * @brief  LPUART Error handling function
  * @param  None
  * @retval None
  */
void APP_LpuartErrorCallback(void)
{
  EieFlags = ErrorFlags;
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
