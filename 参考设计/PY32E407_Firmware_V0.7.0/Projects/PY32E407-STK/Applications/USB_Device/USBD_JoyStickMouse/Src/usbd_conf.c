/**
  ******************************************************************************
  * @file    usbd_conf.c
  * @author  MCU Application Team
  * @brief   USB Device configuration and interface file
  ******************************************************************************
  * @attention
  *
  * <h2><center>&copy; Copyright (c) 2024 Puya Semiconductor Co.
  * All rights reserved.</center></h2>
  *
  * This software component is licensed by Puya under Ultimate Liberty license
  * SLA0044, the "License"; You may not use this file except in compliance with
  * the License. You may obtain a copy of the License at:
  *                      www.st.com/SLA0044
  *
  ******************************************************************************
  * @attention
  *
  * <h2><center>&copy; Copyright (c) 2015 STMicroelectronics.
  * All rights reserved.</center></h2>
  *
  * This software component is licensed by ST under Ultimate Liberty license
  * SLA0044, the "License"; You may not use this file except in compliance with
  * the License. You may obtain a copy of the License at:
  *                      www.st.com/SLA0044
  *
  ******************************************************************************
  */

/* Includes ------------------------------------------------------------------*/
#include "usbd_core.h"

/* Private typedef -----------------------------------------------------------*/
/* Private define ------------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private variables ---------------------------------------------------------*/
USBDEV_HandleTypeDef hUSB1;

/* Private function prototypes -----------------------------------------------*/
/* Private functions ---------------------------------------------------------*/

/*******************************************************************************
                       LL Driver Callbacks (USBD -> USB Device Library)
*******************************************************************************/

/**
  * @brief  SetupStage callback.
  * @param  husbd: USBD handle
  * @retval None
  */
void HAL_USBDEV_SetupStageCallback(USBDEV_HandleTypeDef * husbd)
{
  USBD_LL_SetupStage((USBD_HandleTypeDef *) husbd->pData,
                     (uint8_t *) husbd->Setup);
}

/**
  * @brief  DataOut Stage callback.
  * @param  husbd: USBD handle
  * @param  epnum: Endpoint Number
  * @retval None
  */
void HAL_USBDEV_DataOutStageCallback(USBDEV_HandleTypeDef * husbd, uint8_t epnum)
{
  USBD_LL_DataOutStage((USBD_HandleTypeDef *) husbd->pData, epnum,
                       husbd->OUT_ep[epnum].xfer_buff);
}

/**
  * @brief  DataIn Stage callback.
  * @param  husbd: USBD handle
  * @param  epnum: Endpoint Number
  * @retval None
  */
void HAL_USBDEV_DataInStageCallback(USBDEV_HandleTypeDef * husbd, uint8_t epnum)
{
  USBD_LL_DataInStage((USBD_HandleTypeDef *) husbd->pData, epnum,
                      husbd->IN_ep[epnum].xfer_buff);
}

/**
  * @brief  SOF callback.
  * @param  husbd: USBD handle
  * @retval None
  */
void HAL_USBDEV_SOFCallback(USBDEV_HandleTypeDef * husbd)
{
  USBD_LL_SOF((USBD_HandleTypeDef *) husbd->pData);
}

/**
  * @brief  Reset callback.
  * @param  husbd: USBD handle
  * @retval None
  */
void HAL_USBDEV_ResetCallback(USBDEV_HandleTypeDef * husbd)
{
  USBD_LL_SetSpeed((USBD_HandleTypeDef *) husbd->pData, USBD_SPEED_FULL);
  /* Reset Device */
  USBD_LL_Reset((USBD_HandleTypeDef *) husbd->pData);
}

/**
  * @brief  Suspend callback.
  * @param  husbd: USBD handle
  * @retval None
  */
void HAL_USBDEV_SuspendCallback(USBDEV_HandleTypeDef * husbd)
{
  USBD_LL_Suspend((USBD_HandleTypeDef *) husbd->pData);
}

/**
  * @brief  Resume callback.
  * @param  husbd: USBD handle
  * @retval None
  */
void HAL_USBDEV_ResumeCallback(USBDEV_HandleTypeDef * husbd)
{
  USBD_LL_Resume((USBD_HandleTypeDef *) husbd->pData);
}

/**
  * @brief  ISOOUTIncomplete callback.
  * @param  husbd: USBD handle
  * @param  epnum: Endpoint Number
  * @retval None
  */
void HAL_USBDEV_ISOOUTIncompleteCallback(USBDEV_HandleTypeDef * husbd, uint8_t epnum)
{
  USBD_LL_IsoOUTIncomplete((USBD_HandleTypeDef *) husbd->pData, epnum);
}

/**
  * @brief  ISOINIncomplete callback.
  * @param  husbd: USBD handle
  * @param  epnum: Endpoint Number
  * @retval None
  */
void HAL_USBDEV_ISOINIncompleteCallback(USBDEV_HandleTypeDef * husbd, uint8_t epnum)
{
  USBD_LL_IsoINIncomplete((USBD_HandleTypeDef *) husbd->pData, epnum);
}

/**
  * @brief  ConnectCallback callback.
  * @param  husbd: USBD handle
  * @retval None
  */
void HAL_USBDEV_ConnectCallback(USBDEV_HandleTypeDef * husbd)
{
  USBD_LL_DevConnected((USBD_HandleTypeDef *) husbd->pData);
}

/**
  * @brief  Disconnect callback.
  * @param  husbd: USBD handle
  * @retval None
  */
void HAL_USBDEV_DisconnectCallback(USBDEV_HandleTypeDef * husbd)
{
  USBD_LL_DevDisconnected((USBD_HandleTypeDef *) husbd->pData);
}

/*******************************************************************************
                       LL Driver Interface (USB Device Library --> USBD)
*******************************************************************************/

/**
  * @brief  Initializes the Low Level portion of the Device driver.
  * @param  pdev: Device handle
  * @retval USBD Status
  */
USBD_StatusTypeDef USBD_LL_Init(USBD_HandleTypeDef *pdev)
{
  /* Set LL Driver parameters */
  hUSB1.Instance = USB1_OTG_FS;
  hUSB1.Init.dev_endpoints = 4;
  hUSB1.Init.low_power_enable = 0;
  hUSB1.Init.Sof_enable = 0;
  hUSB1.Init.speed = USBDEV_SPEED_FULL;
  hUSB1.Init.vbus_sensing_enable = 0;
  /* Link The driver to the stack */
  hUSB1.pData = pdev;
  pdev->pData = &hUSB1;
  /* Initialize LL Driver */
  HAL_USBDEV_Init(&hUSB1);

  HAL_USBDEVEx_SetRxFiFo(&hUSB1, 0x80);
  HAL_USBDEVEx_SetTxFiFo(&hUSB1, 0, 0x10);
  HAL_USBDEVEx_SetTxFiFo(&hUSB1, 1, 0x20);
  HAL_USBDEVEx_SetTxFiFo(&hUSB1, 2, 0x20);
  HAL_USBDEVEx_SetTxFiFo(&hUSB1, 3, 0x20);
  return USBD_OK;
}

/**
  * @brief  De-Initializes the Low Level portion of the Device driver.
  * @param  pdev: Device handle
  * @retval USBD Status
  */
USBD_StatusTypeDef USBD_LL_DeInit(USBD_HandleTypeDef * pdev)
{
  HAL_USBDEV_DeInit((USBDEV_HandleTypeDef *) pdev->pData);
  return USBD_OK;
}

/**
  * @brief  Starts the Low Level portion of the Device driver.
  * @param  pdev: Device handle
  * @retval USBD Status
  */
USBD_StatusTypeDef USBD_LL_Start(USBD_HandleTypeDef * pdev)
{
  HAL_USBDEV_Start((USBDEV_HandleTypeDef *) pdev->pData);
  return USBD_OK;
}

/**
  * @brief  Stops the Low Level portion of the Device driver.
  * @param  pdev: Device handle
  * @retval USBD Status
  */
USBD_StatusTypeDef USBD_LL_Stop(USBD_HandleTypeDef * pdev)
{
  HAL_USBDEV_Stop((USBDEV_HandleTypeDef *) pdev->pData);
  return USBD_OK;
}

/**
  * @brief  Opens an endpoint of the Low Level Driver.
  * @param  pdev: Device handle
  * @param  ep_addr: Endpoint Number
  * @param  ep_type: Endpoint Type
  * @param  ep_mps: Endpoint Max Packet Size
  * @retval USBD Status
  */
USBD_StatusTypeDef USBD_LL_OpenEP(USBD_HandleTypeDef * pdev,
                                  uint8_t ep_addr,
                                  uint8_t ep_type, uint16_t ep_mps)
{
  HAL_USBDEV_EP_Open((USBDEV_HandleTypeDef *) pdev->pData, ep_addr, ep_mps, ep_type);

  return USBD_OK;
}

/**
  * @brief  Closes an endpoint of the Low Level Driver.
  * @param  pdev: Device handle
  * @param  ep_addr: Endpoint Number
  * @retval USBD Status
  */
USBD_StatusTypeDef USBD_LL_CloseEP(USBD_HandleTypeDef * pdev, uint8_t ep_addr)
{
  HAL_USBDEV_EP_Close((USBDEV_HandleTypeDef *) pdev->pData, ep_addr);
  return USBD_OK;
}

/**
  * @brief  Flushes an endpoint of the Low Level Driver.
  * @param  pdev: Device handle
  * @param  ep_addr: Endpoint Number
  * @retval USBD Status
  */
USBD_StatusTypeDef USBD_LL_FlushEP(USBD_HandleTypeDef * pdev, uint8_t ep_addr)
{
  HAL_USBDEV_EP_Flush((USBDEV_HandleTypeDef *) pdev->pData, ep_addr);
  return USBD_OK;
}

/**
  * @brief  Sets a Stall condition on an endpoint of the Low Level Driver.
  * @param  pdev: Device handle
  * @param  ep_addr: Endpoint Number
  * @retval USBD Status
  */
USBD_StatusTypeDef USBD_LL_StallEP(USBD_HandleTypeDef * pdev, uint8_t ep_addr)
{
  HAL_USBDEV_EP_SetStall((USBDEV_HandleTypeDef *) pdev->pData, ep_addr);
  return USBD_OK;
}

/**
  * @brief  Clears a Stall condition on an endpoint of the Low Level Driver.
  * @param  pdev: Device handle
  * @param  ep_addr: Endpoint Number
  * @retval USBD Status
  */
USBD_StatusTypeDef USBD_LL_ClearStallEP(USBD_HandleTypeDef * pdev,
                                        uint8_t ep_addr)
{
  HAL_USBDEV_EP_ClrStall((USBDEV_HandleTypeDef *) pdev->pData, ep_addr);
  return USBD_OK;
}

/**
  * @brief  Returns Stall condition.
  * @param  pdev: Device handle
  * @param  ep_addr: Endpoint Number
  * @retval Stall (1: Yes, 0: No)
  */
uint8_t USBD_LL_IsStallEP(USBD_HandleTypeDef * pdev, uint8_t ep_addr)
{
  USBDEV_HandleTypeDef *husbd = (USBDEV_HandleTypeDef *) pdev->pData;

  if ((ep_addr & 0x80) == 0x80)
  {
    return husbd->IN_ep[ep_addr & 0x7F].is_stall;
  }
  else
  {
    return husbd->OUT_ep[ep_addr & 0x7F].is_stall;
  }
}

/**
  * @brief  Assigns a USB address to the device.
  * @param  pdev: Device handle
  * @param  ep_addr: Endpoint Number
  * @retval USBD Status
  */
USBD_StatusTypeDef USBD_LL_SetUSBAddress(USBD_HandleTypeDef * pdev,
                                         uint8_t dev_addr)
{
  HAL_USBDEV_SetAddress((USBDEV_HandleTypeDef *) pdev->pData, dev_addr);
  return USBD_OK;
}

/**
  * @brief  Transmits data over an endpoint.
  * @param  pdev: Device handle
  * @param  ep_addr: Endpoint Number
  * @param  pbuf: Pointer to data to be sent
  * @param  size: Data size
  * @retval USBD Status
  */
USBD_StatusTypeDef USBD_LL_Transmit(USBD_HandleTypeDef * pdev,
                                    uint8_t ep_addr,
                                    uint8_t * pbuf, uint16_t size)
{
  HAL_USBDEV_EP_Transmit((USBDEV_HandleTypeDef *) pdev->pData, ep_addr, pbuf, size);
  return USBD_OK;
}

/**
  * @brief  Prepares an endpoint for reception.
  * @param  pdev: Device handle
  * @param  ep_addr: Endpoint Number
  * @param  pbuf: Pointer to data to be received
  * @param  size: Data size
  * @retval USBD Status
  */
USBD_StatusTypeDef USBD_LL_PrepareReceive(USBD_HandleTypeDef * pdev,
                                          uint8_t ep_addr,
                                          uint8_t * pbuf, uint16_t size)
{
  HAL_USBDEV_EP_Receive((USBDEV_HandleTypeDef *) pdev->pData, ep_addr, pbuf, size);
  return USBD_OK;
}

/**
  * @brief  Returns the last transferred packet size.
  * @param  pdev: Device handle
  * @param  ep_addr: Endpoint Number
  * @retval Recived Data Size
  */
uint32_t USBD_LL_GetRxDataSize(USBD_HandleTypeDef * pdev, uint8_t ep_addr)
{
  return HAL_USBDEV_EP_GetRxCount((USBDEV_HandleTypeDef *) pdev->pData, ep_addr);
}

/**
  * @brief  Delays routine for the USB Device Library.
  * @param  Delay: Delay in ms
  * @retval None
  */
void USBD_LL_Delay(uint32_t Delay)
{
  HAL_Delay(Delay);
}
/************************ (C) COPYRIGHT Puya *****END OF FILE******************/

