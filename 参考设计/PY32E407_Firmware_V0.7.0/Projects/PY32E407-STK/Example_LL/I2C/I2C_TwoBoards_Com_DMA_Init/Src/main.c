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
static void APP_I2cTransmit_DMA(uint16_t DevAddress, uint8_t *pData, uint16_t Size);
static void APP_I2cReceive_DMA(uint16_t DevAddress, uint8_t *pData, uint16_t Size);
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
  APP_I2cTransmit_DMA(I2C_ADDRESS, (uint8_t *)aTxBuffer, sizeof(aTxBuffer));

  /* Wait for master to finish sending data */
  while (State != I2C_STATE_READY);

  /* Delay to wait for the slave to prepare data */
  LL_mDelay(5);

  /* Master receives data */
  APP_I2cReceive_DMA(I2C_ADDRESS, (uint8_t *)aRxBuffer, sizeof(aRxBuffer));

#else
  /* Slave receives data */
  APP_I2cReceive_DMA(I2C_ADDRESS, (uint8_t *)aRxBuffer, sizeof(aRxBuffer));

  /* Wait for slave to complete receiving data */
  while (State != I2C_STATE_READY);

  /* Slave transmits data */
  APP_I2cTransmit_DMA(I2C_ADDRESS, (uint8_t *)aTxBuffer, sizeof(aTxBuffer));
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
  LL_DMA_InitTypeDef DMA_InitStruct = {0};

  /* Enable GPIOC peripheral clock */
  LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOC);

  /* Enable I2C3 peripheral clock */
  LL_APB1_GRP1_EnableClock(LL_APB1_GRP1_PERIPH_I2C3);
  
  /* Enable DMA clock */
  LL_AHB1_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_DMA1);
  
  /* Enable SYSCFG clock */
  LL_APB2_GRP1_EnableClock(LL_APB2_GRP1_PERIPH_SYSCFG);

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
  
  /* Configure DMA request mapping */
  LL_DMA_SetDstPeriphMap(DMA1, LL_DMA_CHANNEL_1, LL_DMA_DST_PERIPH_MAP_1);
  LL_SYSCFG_SetDMARemap(DMA1, LL_SYSCFG_DMA_PERIPH_MAP_1, LL_SYSCFG_DMA_MAP_I2C3_TX);

  LL_DMA_SetSrcPeriphMap(DMA1, LL_DMA_CHANNEL_2, LL_DMA_SRC_PERIPH_MAP_2);
  LL_SYSCFG_SetDMARemap(DMA1, LL_SYSCFG_DMA_PERIPH_MAP_2, LL_SYSCFG_DMA_MAP_I2C3_RX);
  
  /* Initialize DMA channel 1 */
  DMA_InitStruct.Direction         = LL_DMA_DIRECTION_MEMORY_TO_PERIPH;
  DMA_InitStruct.DstIncMode        = LL_DMA_DST_ADDR_FIX;
  DMA_InitStruct.SrcIncMode        = LL_DMA_SRC_ADDR_INC;
  DMA_InitStruct.DstWidth          = LL_DMA_DST_WIDTH_BYTE;
  DMA_InitStruct.SrcWidth          = LL_DMA_SRC_WIDTH_BYTE;
  DMA_InitStruct.Priority          = LL_DMA_PRIORITY_1;
  DMA_InitStruct.DstBurstLen       = LL_DMA_DST_BURST_LEN_1;
  DMA_InitStruct.DstHandshakeType  = LL_DMA_DST_HANDSHAKE_TYPE_HARD;
  LL_DMA_Init(DMA1, LL_DMA_CHANNEL_1, &DMA_InitStruct);
  
  /* Initialize DMA channel 2 */
  DMA_InitStruct.Direction         = LL_DMA_DIRECTION_PERIPH_TO_MEMORY;
  DMA_InitStruct.DstIncMode        = LL_DMA_DST_ADDR_INC;
  DMA_InitStruct.SrcIncMode        = LL_DMA_SRC_ADDR_FIX;
  DMA_InitStruct.DstWidth          = LL_DMA_DST_WIDTH_BYTE;
  DMA_InitStruct.SrcWidth          = LL_DMA_SRC_WIDTH_BYTE;
  DMA_InitStruct.Priority          = LL_DMA_PRIORITY_0;
  DMA_InitStruct.SrcBurstLen       = LL_DMA_SRC_BURST_LEN_1;
  DMA_InitStruct.SrcHandshakeType  = LL_DMA_SRC_HANDSHAKE_TYPE_HARD;
  LL_DMA_Init(DMA1, LL_DMA_CHANNEL_2, &DMA_InitStruct);
  
  /* Enable NVIC interrupt for DMA */
  NVIC_SetPriority(DMA1_Channel1_IRQn, 1);
  NVIC_EnableIRQ(DMA1_Channel1_IRQn);
  
  NVIC_SetPriority(DMA1_Channel2_IRQn, 0);
  NVIC_EnableIRQ(DMA1_Channel2_IRQn);

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
static void APP_I2cTransmit_DMA(uint16_t DevAddress, uint8_t *pData, uint16_t Size)
{
  /* Clear POS bit */
  LL_I2C_DisableBitPOS(I2C3);
  
  /* Assign slave address, data to be sent, data size, and state to global variables */
  pBuffPtr    = pData;
  XferCount   = Size;
  Devaddress  = DevAddress;
  State       = I2C_STATE_BUSY_TX;
  
  /* Enable DMA transfer interrupt */
  LL_DMA_DisableChannel(DMA1, LL_DMA_CHANNEL_1);
  
  LL_DMA_ClearFlag_TFR1(DMA1);
  LL_DMA_SetBlockLength(DMA1, LL_DMA_CHANNEL_1, Size);
  uint32_t dataRegAddr = LL_I2C_DMA_GetRegAddr(I2C3);
  LL_DMA_ConfigAddresses(DMA1, LL_DMA_CHANNEL_1, (uint32_t)pData, dataRegAddr);
  
  LL_DMA_DisableIT_HALFBLOCK(DMA1, LL_DMA_CHANNEL_1);
  LL_DMA_EnableIT_TFR(DMA1, LL_DMA_CHANNEL_1);
  LL_DMA_EnableIT_ERR(DMA1, LL_DMA_CHANNEL_1);
  
  LL_DMA_Enable(DMA1);
  LL_DMA_EnableChannel(DMA1, LL_DMA_CHANNEL_1);
  
  /* Enable acknowledge */
  LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_ACK);
  
#ifdef BOARD_IN_MASTER_MODE
  /* Generate start condition */
  LL_I2C_GenerateStartCondition(I2C3);
  while(LL_I2C_IsActiveFlag_SB(I2C3) != 1);
  /* Send slave address */
  LL_I2C_TransmitData8(I2C3, (Devaddress & (uint8_t)(~0x01)));
#endif
  while(LL_I2C_IsActiveFlag_ADDR(I2C3) != 1);
  LL_I2C_ClearFlag_ADDR(I2C3);
  
  /* Enable I2C EVT and ERR interrupts */
  LL_I2C_EnableIT_EVT(I2C3);
  LL_I2C_EnableIT_ERR(I2C3);
  
  /* Enable DMA request */
  LL_I2C_EnableDMAReq_TX(I2C3);
}

/**
  * @brief  I2C reception function
  * @param  DevAddress：Slave address
  * @param  pData：Pointer to data to be received
  * @param  Size：Size of data to be received
  * @retval None
  */
static void APP_I2cReceive_DMA(uint16_t DevAddress, uint8_t *pData, uint16_t Size)
{
  /* Clear POS bit */
  LL_I2C_DisableBitPOS(I2C3);
  
  /* Assign slave address, data to be sent, and data size to global variables */
  pBuffPtr    = pData;
  XferCount   = Size;
  Devaddress  = DevAddress;
  State       = I2C_STATE_BUSY_RX;
  
  /* Enable DMA transfer interrupt */
  LL_DMA_DisableChannel(DMA1, LL_DMA_CHANNEL_2);
  
  LL_DMA_ClearFlag_TFR2(DMA1);
  LL_DMA_SetBlockLength(DMA1, LL_DMA_CHANNEL_2, Size);
  uint32_t dataRegAddr = LL_I2C_DMA_GetRegAddr(I2C3);
  LL_DMA_ConfigAddresses(DMA1, LL_DMA_CHANNEL_2, dataRegAddr, (uint32_t)pData);
  
  LL_DMA_DisableIT_HALFBLOCK(DMA1, LL_DMA_CHANNEL_2);
  LL_DMA_EnableIT_TFR(DMA1, LL_DMA_CHANNEL_2);
  LL_DMA_EnableIT_ERR(DMA1, LL_DMA_CHANNEL_2);
  
  LL_DMA_Enable(DMA1);
  LL_DMA_EnableChannel(DMA1, LL_DMA_CHANNEL_2);
  
  /* Enable acknowledge */
  LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_ACK);

#ifdef BOARD_IN_MASTER_MODE
  /* Generate start condition */
  LL_I2C_GenerateStartCondition(I2C3);
#endif
  /* Enable I2C EVT and ERR interrupts */
  LL_I2C_EnableIT_EVT(I2C3);
  LL_I2C_EnableIT_ERR(I2C3);
  
  /* Enable DMA request */
  LL_I2C_EnableDMAReq_RX(I2C3);
}

#ifdef BOARD_IN_MASTER_MODE
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
    if (State == I2C_STATE_BUSY_RX)
    {
      LL_I2C_TransmitData8(I2C3, (Devaddress | 0x1));
    }
  }
  /* Set ADDR flag (used for master reception) */
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
      }
      else if (XferCount == 2U)
      {
        LL_I2C_EnableBitPOS(I2C3);
        LL_I2C_ClearFlag_ADDR(I2C3);
        LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_NACK);
        LL_I2C_EnableLastDMA(I2C3);
      }
      else
      {
        LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_ACK);
        LL_I2C_EnableLastDMA(I2C3);
        LL_I2C_ClearFlag_ADDR(I2C3);
      }
    }
  }
  /* Master send direction */
  else if (LL_I2C_GetTransferDirection(I2C3) == LL_I2C_DIRECTION_WRITE)
  {
    /* If DMA transmission is ongoing, do not check BTF flag */
    if (LL_I2C_IsEnabledDMAReq_TX(I2C3) != 1)
    {
      /* Set BTF flag */
      if ((LL_I2C_IsActiveFlag_BTF(I2C3) == 1) && (LL_I2C_IsEnabledIT_EVT(I2C3) == 1))
      {
        if (XferCount == 0U)
        {
          LL_I2C_DisableIT_EVT(I2C3);
          LL_I2C_DisableIT_ERR(I2C3);
          
          LL_I2C_GenerateStopCondition(I2C3);
          State = I2C_STATE_READY;
        }
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
  /* Set STOP flag (used for slave reception) */
  else if (LL_I2C_IsActiveFlag_STOP(I2C3) == 1)
  {
    if ((XferCount == 0) && (State == I2C_STATE_BUSY_TX))
    {
      LL_I2C_DisableIT_EVT(I2C3);
      LL_I2C_DisableIT_BUF(I2C3);
    }
    else
    {
      LL_I2C_DisableIT_EVT(I2C3);
      LL_I2C_DisableIT_BUF(I2C3);
      LL_I2C_DisableIT_ERR(I2C3);
    }
    
    LL_I2C_ClearFlag_STOP(I2C3);
    
    LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_NACK);
    
    /* Data not fully received, receive data failed */
    if (XferCount != 0U)
    {
      if ((LL_I2C_IsActiveFlag_BTF(I2C3) == 1))
      {
        *pBuffPtr = LL_I2C_ReceiveData8(I2C3);
        pBuffPtr++;
        XferCount--;
      }
      
      if ((LL_I2C_IsActiveFlag_RXNE(I2C3) == 1))
      {
        *pBuffPtr = LL_I2C_ReceiveData8(I2C3);
        pBuffPtr++;
        XferCount--;
      }
      if (XferCount != 0U)
      {
        /*  Handle data reception failure */
      }
    }
    
    if (State == I2C_STATE_BUSY_RX)
    {
       State = I2C_STATE_READY;
    }
  }
  /* Slave Transmit */
  else if (State == I2C_STATE_BUSY_TX)
  {
  }
  /* Slave Receive */
  else
  {
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
      LL_I2C_DisableIT_ERR(I2C3);
      
      LL_I2C_ClearFlag_AF(I2C3);
      
      LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_NACK);
      
      State = I2C_STATE_READY;
    }
  }
}

#endif
/**
  * @brief  DMA interrupt callback function
  * @param  None
  * @retval None
  */
void APP_DmaIRQCallback(void)
{
  /* Transmission completed handling */
  if (State == I2C_STATE_BUSY_TX)
  {
    if ((LL_DMA_IsActiveFlag_TFR1(DMA1) == 1) && (LL_DMA_IsEnabledIT_TFR(DMA1, LL_DMA_CHANNEL_1) == 1))
    {
      /* Disable transfer complete interrupt */
      LL_DMA_DisableIT_TFR(DMA1, LL_DMA_CHANNEL_1);
      LL_DMA_DisableIT_ERR(DMA1, LL_DMA_CHANNEL_1);
      
      /* Clear transfer complete flag */
      LL_DMA_ClearFlag_TFR1(DMA1);
      
      /* Disable I2C EVT and ERR interrupts */
      LL_I2C_DisableIT_EVT(I2C3);
      LL_I2C_DisableIT_ERR(I2C3);
      
      /* Disable I2C DMA requests */
      LL_I2C_DisableDMAReq_TX(I2C3);
      
      XferCount = 0U;
      
      /* Enable I2C EVT and ERR interrupts */
      LL_I2C_EnableIT_EVT(I2C3);
      LL_I2C_EnableIT_ERR(I2C3);
    }
  }
  /* Reception completed handling */
  else
  {
    if ((LL_DMA_IsActiveFlag_TFR2(DMA1) == 1) && (LL_DMA_IsEnabledIT_TFR(DMA1, LL_DMA_CHANNEL_2) == 1))
    {
      /* Disable transfer complete interrupt */
      LL_DMA_DisableIT_TFR(DMA1, LL_DMA_CHANNEL_2);
      LL_DMA_DisableIT_ERR(DMA1, LL_DMA_CHANNEL_2);
      
      /* Clear transfer complete flag */
      LL_DMA_ClearFlag_TFR2(DMA1);
      
      /* Disable I2C EVT and ERR interrupts */
      LL_I2C_DisableIT_EVT(I2C3);
      LL_I2C_DisableIT_ERR(I2C3);
#ifdef BOARD_IN_MASTER_MODE
      /* I2C handling when transfer is complete */
      if (XferCount == (uint16_t)1)
      {
        LL_I2C_AcknowledgeNextData(I2C3, LL_I2C_NACK);
      }
      LL_I2C_GenerateStopCondition(I2C3);
      LL_I2C_DisableLastDMA(I2C3);
      
      LL_I2C_DisableDMAReq_RX(I2C3);
      XferCount = 0U;
      State = I2C_STATE_READY;
#else
      /* Disable I2C DMA requests */
      LL_I2C_DisableDMAReq_RX(I2C3);
      XferCount = 0U;

      /* Enable I2C EVT and ERR interrupts */
      LL_I2C_EnableIT_EVT(I2C3);
      LL_I2C_EnableIT_ERR(I2C3);
#endif
    }
  }
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
