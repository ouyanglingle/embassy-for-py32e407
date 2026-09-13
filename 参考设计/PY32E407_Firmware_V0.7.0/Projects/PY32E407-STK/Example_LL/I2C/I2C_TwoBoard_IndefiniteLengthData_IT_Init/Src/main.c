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
#define I2C_ADDRESS        0xA0     /* Local/Slave address */
#define I2C_SPEEDCLOCK     100000   /* Communication speed : 100K */
#define I2C_STATE_READY    0        /* Ready state */
#define I2C_STATE_BUSY_TX  1        /* Transmission state */
#define I2C_STATE_BUSY_RX  2        /* Reception state */

/* Private variables ---------------------------------------------------------*/
uint8_t aTxBuffer[100] = {0};

uint8_t         *pBuffPtr   = NULL;
__IO uint16_t   XferCount   = 0;
__IO uint32_t   Devaddress  = 0;
__IO uint32_t   State       = I2C_STATE_READY;

#ifndef BOARD_IN_MASTER_MODE
#define RX_MAX_LEN 200              /* Single frame data, maximum received data length */
uint32_t RxLen = 0;                 /* Single frame data, actual received data length */
uint8_t RxBuffer[RX_MAX_LEN] = {0}; /* Receive buffer */
__IO uint8_t RevOkFlag = 0;         /* Single frame data received completion flag */
#endif

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_ConfigI2c(void);
#ifdef BOARD_IN_MASTER_MODE
static void APP_I2cTransmit_IT(uint16_t DevAddress, uint8_t *pData, uint16_t Size);
#else
static void APP_I2cReceive_IT(void);
#endif

/**
  * @brief  Main program.
  * @param  None
  * @retval int
  */
int main(void)
{
  uint32_t i = 0;
  
  /* Enable SYSCFG and PWR clock */
  LL_APB2_GRP1_EnableClock(LL_APB2_GRP1_PERIPH_SYSCFG);
  LL_APB1_GRP1_EnableClock(LL_APB1_GRP1_PERIPH_PWR);
  
  /* 3 bits for pre-emption priority, 0 bits for subpriority */
  NVIC_SetPriorityGrouping(NVIC_PRIORITYGROUP_3);

  /* Configure system clock */
  APP_SystemClockConfig();

  /* Initialize LED */
  BSP_LED_Init(LED_GREEN);

  /* Configure I2C */
  APP_ConfigI2c();

#ifdef BOARD_IN_MASTER_MODE
  /* Initialize BUTTON */
  BSP_PB_Init(BUTTON_KEY,BUTTON_MODE_GPIO);

  /* Wait for the button to be pressed */
  while(BSP_PB_GetState(BUTTON_KEY) == 0);

  for(i = 0; i < 10; i++)
  {
    aTxBuffer[i] = i;
  }
  /* Master transmits data */
  APP_I2cTransmit_IT(I2C_ADDRESS, (uint8_t *)aTxBuffer, 10);
  
  /* Wait for master to finish sending data */
  while (State != I2C_STATE_READY);
  
  /* Delay purpose: Wait for the slave to receive and print the received data */
  LL_mDelay(200);
  for(i = 0; i < 100; i++)
  {
    aTxBuffer[i] = i + 1;
  }
  /* Master transmits data */
  APP_I2cTransmit_IT(I2C_ADDRESS, (uint8_t *)aTxBuffer, 100);
  
  /* Wait for master to finish sending data */
  while (State != I2C_STATE_READY);
  
  /* Delay purpose: Wait for the slave to receive and print the received data */
  LL_mDelay(200);
  for(i = 0; i < 10; i++)
  {
    aTxBuffer[i] = i;
  }
  /* Master transmits data */
  APP_I2cTransmit_IT(I2C_ADDRESS, (uint8_t *)aTxBuffer, 10);
  
  /* Wait for master to finish sending data */
  while (State != I2C_STATE_READY);
#else
  /* Initialize USART */
  BSP_UART_Config();
  
  /* Slave receives data */
  APP_I2cReceive_IT();
#endif

  while (1)
  {
#ifndef BOARD_IN_MASTER_MODE
    if (RevOkFlag == 1)
    {
      for (i = 0; i < RxLen; i++)
      {
        printf("%d ", RxBuffer[i]);
      }
      RxLen = 0;
      RevOkFlag = 0;
    }
#endif
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
  * @brief  I2C configuration function
  * @param  None
  * @retval None
  */
static void APP_ConfigI2c(void)
{
  LL_GPIO_InitTypeDef GPIO_InitStruct = {0};
  LL_I2C_InitTypeDef I2C_InitStruct = {0};

  /* Enable GPIOC peripheral clock */
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOC);

  /* Enable I2C3 peripheral clock */
  LL_APB1_GRP1_EnableClock(LL_APB1_GRP1_PERIPH_I2C3);

  /**I2C GPIO Configuration
  PC8  -------> I2C3_SCL
  PC9  -------> I2C3_SDA
  */
  /* Configure SCL pin: Alternative function, High speed, Open-drain, Pull-up */  
  GPIO_InitStruct.Pin        = LL_GPIO_PIN_8;
  GPIO_InitStruct.Mode       = LL_GPIO_MODE_ALTERNATE;
  GPIO_InitStruct.Speed      = LL_GPIO_SPEED_FREQ_HIGH;
  GPIO_InitStruct.OutputType = LL_GPIO_OUTPUT_OPENDRAIN;
  GPIO_InitStruct.Pull       = LL_GPIO_PULL_UP;
  GPIO_InitStruct.Alternate  = LL_GPIO_AF_8;
  LL_GPIO_Init(GPIOC, &GPIO_InitStruct);

  /* Configure SDA pin: Alternative function, High speed, Open-drain, Pull-up */
  GPIO_InitStruct.Pin        = LL_GPIO_PIN_9;
  GPIO_InitStruct.Mode       = LL_GPIO_MODE_ALTERNATE;
  GPIO_InitStruct.Speed      = LL_GPIO_SPEED_FREQ_HIGH;
  GPIO_InitStruct.OutputType = LL_GPIO_OUTPUT_OPENDRAIN;
  GPIO_InitStruct.Pull       = LL_GPIO_PULL_UP;
  GPIO_InitStruct.Alternate  = LL_GPIO_AF_8;
  LL_GPIO_Init(GPIOC, &GPIO_InitStruct);
  
  /*  Reset I2C  */
  LL_APB1_GRP1_ForceReset(LL_APB1_GRP1_PERIPH_I2C3);
  LL_APB1_GRP1_ReleaseReset(LL_APB1_GRP1_PERIPH_I2C3);
  
  /* Enable NVIC interrupt for I2C */
  NVIC_SetPriority(I2C3_EV_IRQn, 0);
  NVIC_EnableIRQ(I2C3_EV_IRQn);
  
  NVIC_SetPriority(I2C3_ER_IRQn, 0);
  NVIC_EnableIRQ(I2C3_ER_IRQn);

  /* I2C initialization */
  I2C_InitStruct.PeripheralMode  = LL_I2C_MODE_I2C;
  I2C_InitStruct.ClockSpeed      = I2C_SPEEDCLOCK;
  I2C_InitStruct.DutyCycle       = LL_I2C_DUTYCYCLE_16_9;
  I2C_InitStruct.DataHoldTimeSel = LL_I2C_DATA_HOLD_TIME_HARDWARE;
  I2C_InitStruct.OwnAddress1     = I2C_ADDRESS;
  I2C_InitStruct.TypeAcknowledge = LL_I2C_NACK;
  I2C_InitStruct.OwnAddrSize     = LL_I2C_OWNADDRESS1_7BIT;
  LL_I2C_Init(I2C3, &I2C_InitStruct);
}

#ifdef BOARD_IN_MASTER_MODE

/**
  * @brief  I2C transmission function
  * @param  DevAddress：Slave address
  * @param  pData：Pointer to data to be sent
  * @param  Size：Size of data to be sent
  * @retval None
  */
static void APP_I2cTransmit_IT(uint16_t DevAddress, uint8_t *pData, uint16_t Size)
{
  /* Clear POS bit */
  LL_I2C_DisableBitPOS(I2C3);
  
  /* Assign slave address, data to be sent, data size, and state to global variables */
  pBuffPtr    = pData;
  XferCount   = Size;
  Devaddress  = DevAddress;
  State       = I2C_STATE_BUSY_TX;

  /* Generate start condition */
  LL_I2C_GenerateStartCondition(I2C3);

  /* Enable interrupt */
  LL_I2C_EnableIT_EVT(I2C3);
  LL_I2C_EnableIT_BUF(I2C3);
  LL_I2C_EnableIT_ERR(I2C3);
}

/**
  * @brief  I2C interrupt callback function
  * @param  None
  * @retval None
  */
void APP_MasterI2cIRQCallback(void)
{
  /* Set SB flag(used for master reception) */
  if ((LL_I2C_IsActiveFlag_SB(I2C3) == 1) && (LL_I2C_IsEnabledIT_EVT(I2C3) == 1))
  {
    /* Send slave address + direction bit */
    if (State == I2C_STATE_BUSY_TX)
    {
      LL_I2C_TransmitData8(I2C3, (Devaddress & (uint8_t)(~0x01)));
    }
    else
    {
      LL_I2C_TransmitData8(I2C3, (Devaddress | 0x1));
    }
  }
  /* Set ADDR flag */
  else if ((LL_I2C_IsActiveFlag_ADDR(I2C3) == 1) && (LL_I2C_IsEnabledIT_EVT(I2C3) == 1))
  {
    if (State == I2C_STATE_BUSY_RX)
    {
      if (XferCount == 0U)
      {
        LL_I2C_ClearFlag_ADDR(I2C3);
        LL_I2C_GenerateStopCondition(I2C3);
      }
      else if (XferCount == 1U)
      {
        LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_NACK);
        LL_I2C_ClearFlag_ADDR(I2C3);
        LL_I2C_GenerateStopCondition(I2C3);
      }
      else if (XferCount == 2U)
      {
        LL_I2C_EnableBitPOS(I2C3);
        LL_I2C_ClearFlag_ADDR(I2C3);
        LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_NACK);
      }
      else
      {
        LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_ACK);
        LL_I2C_ClearFlag_ADDR(I2C3);
      }
    }
    else
    {
      LL_I2C_ClearFlag_ADDR(I2C3);
    }
  }
  /* Master send direction */
  else if (LL_I2C_GetTransferDirection(I2C3) == LL_I2C_DIRECTION_WRITE)
  {
    /* Set TXE flag, BTF flag is not set */
    if ((LL_I2C_IsActiveFlag_TXE(I2C3) == 1) && (LL_I2C_IsEnabledIT_BUF(I2C3) == 1) && (LL_I2C_IsActiveFlag_BTF(I2C3) == 0))
    {
      if (XferCount == 0U)
      {
        LL_I2C_DisableIT_BUF(I2C3);
      }
      else
      {
        LL_I2C_TransmitData8(I2C3, *pBuffPtr);
        pBuffPtr++;
        XferCount--;
      }
    }
    /* Set BTF flag */
    else if ((LL_I2C_IsActiveFlag_BTF(I2C3) == 1) && (LL_I2C_IsEnabledIT_EVT(I2C3) == 1))
    {
      if (XferCount != 0U)
      {
        LL_I2C_TransmitData8(I2C3, *pBuffPtr);
        pBuffPtr++;
        XferCount--;
      }
      else
      {
        LL_I2C_DisableIT_EVT(I2C3);
        LL_I2C_DisableIT_BUF(I2C3);
        LL_I2C_DisableIT_ERR(I2C3);
        
        LL_I2C_GenerateStopCondition(I2C3);
        State = I2C_STATE_READY;
      }
    }
  }
  /* Master receive direction */
  else
  {

  }
}
#else

/**
  * @brief  I2C reception function
  * @param  None
  * @retval None
  */
static void APP_I2cReceive_IT()
{
  /* Clear POS bit */
  LL_I2C_DisableBitPOS(I2C3);

  /* Enable acknowledge */
  LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_ACK);

  /* Enable interrupt */
  LL_I2C_EnableIT_EVT(I2C3);
  LL_I2C_EnableIT_BUF(I2C3);
  LL_I2C_EnableIT_ERR(I2C3);
}

/**
  * @brief  I2C interrupt callback function
  * @param  None
  * @retval None
  */
void APP_SlaveIRQCallback(void)
{
  /* Set ADDR flag (used for slave reception) */
  if ((LL_I2C_IsActiveFlag_ADDR(I2C3) == 1) && (LL_I2C_IsEnabledIT_EVT(I2C3) == 1))
  {
    LL_I2C_ClearFlag_ADDR(I2C3);
  }
  /* Set STOP flag */
  else if (LL_I2C_IsActiveFlag_STOP(I2C3) == 1)
  {
    
    LL_I2C_ClearFlag_STOP(I2C3);
    
    RevOkFlag = 1;
  }
  /* Slave Receive */
  else
  {
    /* Set RXNE flag, BTF flag is not set */
    if ((LL_I2C_IsActiveFlag_RXNE(I2C3) == 1) && (LL_I2C_IsEnabledIT_BUF(I2C3) == 1) && (LL_I2C_IsActiveFlag_BTF(I2C3) == 0))
    {
      RxBuffer[RxLen++] = LL_I2C_ReceiveData8(I2C3);
    }
    /* Set BTF flag */
    else if ((LL_I2C_IsActiveFlag_BTF(I2C3) == 1) && (LL_I2C_IsEnabledIT_EVT(I2C3) == 1))
    {
      RxBuffer[RxLen++] = LL_I2C_ReceiveData8(I2C3);
    }
  }
}

/**
  * @brief  After the I2C master receives the last byte, send NACK to the slave, slave NACK 
  * @param  None
  * @retval None
  */
void APP_SlaveIRQCallback_NACK(void)
{
  if ((LL_I2C_IsActiveFlag_AF(I2C3) == 1) && (LL_I2C_IsEnabledIT_ERR(I2C3) == 1))
  {
    if ((XferCount == 0) && (State == I2C_STATE_BUSY_TX))
    {
      LL_I2C_DisableIT_EVT(I2C3);
      LL_I2C_DisableIT_BUF(I2C3);
      LL_I2C_DisableIT_ERR(I2C3);
      
      LL_I2C_ClearFlag_AF(I2C3);
      
      LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_NACK);
      
      State = I2C_STATE_READY;
    }
  }
}
#endif

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
