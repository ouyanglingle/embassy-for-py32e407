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
uint8_t aTxBuffer[15] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15};
uint8_t aRxBuffer[15] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

uint8_t         *pBuffPtr   = NULL;
__IO uint16_t   XferCount   = 0;
__IO uint32_t   Devaddress  = 0;
__IO uint32_t   State       = I2C_STATE_READY;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void APP_ConfigI2c(void);
static void APP_I2cTransmit(uint16_t DevAddress, uint8_t *pData, uint16_t Size);
static void APP_I2cReceive(uint16_t DevAddress, uint8_t *pData, uint16_t Size);
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

  /* Configure I2C */
  APP_ConfigI2c();

#ifdef BOARD_IN_MASTER_MODE
  /* Initialize BUTTON */
  BSP_PB_Init(BUTTON_KEY,BUTTON_MODE_GPIO);

  /* Wait for the button to be pressed */
  while(BSP_PB_GetState(BUTTON_KEY) == 0);

  /* Master transmits data */
  APP_I2cTransmit(I2C_ADDRESS, (uint8_t *)aTxBuffer, sizeof(aTxBuffer));

  /* Wait for master to finish sending data */
  while (State != I2C_STATE_READY);

  /* Delay to wait for the slave to prepare data */
  LL_mDelay(5);

  /* Master receives data */
  APP_I2cReceive(I2C_ADDRESS, (uint8_t *)aRxBuffer, sizeof(aRxBuffer));

#else
  /* Slave receives data */
  APP_I2cReceive(I2C_ADDRESS, (uint8_t *)aRxBuffer, sizeof(aRxBuffer));

  /* Wait for slave to complete receiving data */
  while (State != I2C_STATE_READY);

  /* Slave transmits data */
  APP_I2cTransmit(I2C_ADDRESS, (uint8_t *)aTxBuffer, sizeof(aTxBuffer));
#endif
  /* Wait for master to finish receiving data */
  while (State != I2C_STATE_READY);

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

/**
  * @brief  I2C transmission function
  * @param  DevAddress：Slave address
  * @param  pData：Pointer to data to be sent
  * @param  Size：Size of data to be sent
  * @retval None
  */
static void APP_I2cTransmit(uint16_t DevAddress, uint8_t *pData, uint16_t Size)
{
  /* Assign slave address, data to be sent, data size, and state to global variables */
  pBuffPtr    = pData;
  XferCount   = Size;
  Devaddress  = DevAddress;
  State       = I2C_STATE_BUSY_TX;

  /* Clear POS bit */
  LL_I2C_DisableBitPOS(I2C3);

#ifdef BOARD_IN_MASTER_MODE
  /* Generate start condition */
  LL_I2C_GenerateStartCondition(I2C3);
  while(LL_I2C_IsActiveFlag_SB(I2C3) != 1);
  
  /* Send slave address */
  LL_I2C_TransmitData8(I2C3, (Devaddress & (uint8_t)(~0x01)));
  while(LL_I2C_IsActiveFlag_ADDR(I2C3) != 1);
  LL_I2C_ClearFlag_ADDR(I2C3);
  
  /* Send data */
  while(XferCount > 0)
  {
    while(LL_I2C_IsActiveFlag_TXE(I2C3) != 1);
    LL_I2C_TransmitData8(I2C3, *pBuffPtr);
    pBuffPtr++;
    XferCount--;
    
    if ((LL_I2C_IsActiveFlag_BTF(I2C3) == 1) && (XferCount != 0U))
    {
      LL_I2C_TransmitData8(I2C3, *pBuffPtr);
      pBuffPtr++;
      XferCount--;
    }
    while(LL_I2C_IsActiveFlag_BTF(I2C3) != 1);
  }
  
  /* Generate stop condition */
  LL_I2C_GenerateStopCondition(I2C3);
#else
  /* Enable Address Acknowledge */
  LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_ACK);

  /* Wait ADDR flag and check address match code and direction ************/
  while (!LL_I2C_IsActiveFlag_ADDR(I2C3))
  {
  }

  /* Clear ADDR flag value in ISR register */
  LL_I2C_ClearFlag_ADDR(I2C3);

  while (XferCount > 0U)
  {
    /* Wait until TXE flag is set */
    while(!LL_I2C_IsActiveFlag_TXE(I2C3));

    /* Write data to DR */
    LL_I2C_TransmitData8(I2C3, *pBuffPtr);
    /* Increment Buffer pointer */
    pBuffPtr++;
    /* Update counter */
    XferCount--;
    
    if((LL_I2C_IsActiveFlag_BTF(I2C3) == 1) && (XferCount != 0))
    {
      /* Write data to DR */
      LL_I2C_TransmitData8(I2C3, *pBuffPtr);
      /* Increment Buffer pointer */
      pBuffPtr++;
      /* Update counter */
      XferCount--;
    }
  }

  /* Wait until AF flag is set */
  while(!LL_I2C_IsActiveFlag_AF(I2C3));
  /* Clear AF flag */
  LL_I2C_ClearFlag_AF(I2C3);
  
  /* Disable Address Acknowledge */
  LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_NACK);
#endif
  State = I2C_STATE_READY;
}

/**
  * @brief  I2C reception function
  * @param  DevAddress：Slave address
  * @param  pData：Pointer to data to be received
  * @param  Size：Size of data to be received
  * @retval None
  */
static void APP_I2cReceive(uint16_t DevAddress, uint8_t *pData, uint16_t Size)
{
  /* Assign slave address, data to be sent, and data size to global variables */
  pBuffPtr    = pData;
  XferCount   = Size;
  Devaddress  = DevAddress;
  State       = I2C_STATE_BUSY_RX;

  /* Clear POS bit */
  LL_I2C_DisableBitPOS(I2C3);

#ifdef BOARD_IN_MASTER_MODE
  /* Generate start condition */
  LL_I2C_GenerateStartCondition(I2C3);
  while(LL_I2C_IsActiveFlag_SB(I2C3) != 1);
  
  /* Send slave address */
  LL_I2C_TransmitData8(I2C3, (Devaddress | 0x1));
  while(LL_I2C_IsActiveFlag_ADDR(I2C3) != 1);
  
  /* Receive data */
  if (XferCount == 0U)
  {
    LL_I2C_ClearFlag_ADDR(I2C3);
    LL_I2C_GenerateStopCondition(I2C3);
  }
  else if(XferCount == 1U)
  {
    LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_NACK);
    
    __disable_irq();
    LL_I2C_ClearFlag_ADDR(I2C3);
    LL_I2C_GenerateStopCondition(I2C3);
    __enable_irq();
  }
  else if(XferCount == 2U)
  {
    LL_I2C_EnableBitPOS(I2C3);
    
    __disable_irq();
    LL_I2C_ClearFlag_ADDR(I2C3);
    LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_NACK);
    __enable_irq();
  }
  else
  {
    LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_ACK);
    LL_I2C_ClearFlag_ADDR(I2C3);
  }
  
  while (XferCount > 0U)
  {
    if (XferCount <= 3U)
    {
      if (XferCount == 1U)
      {
        while(LL_I2C_IsActiveFlag_RXNE(I2C3) != 1);
        *pBuffPtr = LL_I2C_ReceiveData8(I2C3);
        pBuffPtr++;
        XferCount--;
      }
      else if (XferCount == 2U)
      {
        while(LL_I2C_IsActiveFlag_BTF(I2C3) != 1);
        
        __disable_irq();
        LL_I2C_GenerateStopCondition(I2C3);
        *pBuffPtr = LL_I2C_ReceiveData8(I2C3);
        pBuffPtr++;
        XferCount--;
        __enable_irq();
        
        *pBuffPtr = LL_I2C_ReceiveData8(I2C3);
        pBuffPtr++;
        XferCount--;
      }
      else
      {
        while(LL_I2C_IsActiveFlag_BTF(I2C3) != 1);
        
        LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_NACK);
        
        __disable_irq();
        *pBuffPtr = LL_I2C_ReceiveData8(I2C3);
        pBuffPtr++;
        XferCount--;
        while(LL_I2C_IsActiveFlag_BTF(I2C3) != 1);
        LL_I2C_GenerateStopCondition(I2C3);
        *pBuffPtr = LL_I2C_ReceiveData8(I2C3);
        pBuffPtr++;
        XferCount--;
        __enable_irq();
        
        *pBuffPtr = LL_I2C_ReceiveData8(I2C3);
        pBuffPtr++;
        XferCount--;
      }
    }
    else
    {
      while(LL_I2C_IsActiveFlag_RXNE(I2C3) != 1);
      
      *pBuffPtr = LL_I2C_ReceiveData8(I2C3);
       pBuffPtr++;
       XferCount--;
      
      if (LL_I2C_IsActiveFlag_BTF(I2C3) == 1)
      {
        *pBuffPtr = LL_I2C_ReceiveData8(I2C3);
        pBuffPtr++;
        XferCount--;
      }
    }
  }
#else
  /* Enable Address Acknowledge */
  LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_ACK);

  /* (1) Wait ADDR flag and check address match code and direction ************/
  while (!LL_I2C_IsActiveFlag_ADDR(I2C3))
  {
  }

  /* Clear ADDR flag value in ISR register */
  LL_I2C_ClearFlag_ADDR(I2C3);

  while (XferCount > 0U)
  {
    /* Wait until RXNE flag is set */
    while(!LL_I2C_IsActiveFlag_RXNE(I2C3));

    /* Read data from DR */
    *pBuffPtr = LL_I2C_ReceiveData8(I2C3);
    /* Increment Buffer pointer */
    pBuffPtr++;
    /* Update counter */
    XferCount--;
    
    if((LL_I2C_IsActiveFlag_BTF(I2C3) == 1) && (XferCount != 0))
    {
      /* Read data from DR */
      *pBuffPtr = LL_I2C_ReceiveData8(I2C3);
      /* Increment Buffer pointer */
      pBuffPtr++;
      /* Update counter */
      XferCount--;
    }
  }

  /* Wait until STOP flag is set */
  while(!LL_I2C_IsActiveFlag_STOP(I2C3));
  /* Clear STOP flag */
  LL_I2C_ClearFlag_STOP(I2C3);
  
  /* Disable Address Acknowledge */
  LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_NACK);
#endif
  State = I2C_STATE_READY;
}

/**
  * @brief  Data verification function
  * @param  None
  * @retval None
  */
static void APP_CheckEndOfTransfer(void)
{
  /* Compare the transmitted data with the received data */
  if(APP_Buffercmp8((uint8_t*)aTxBuffer, (uint8_t*)aRxBuffer, sizeof(aRxBuffer)))
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
    LL_mDelay(500);
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
