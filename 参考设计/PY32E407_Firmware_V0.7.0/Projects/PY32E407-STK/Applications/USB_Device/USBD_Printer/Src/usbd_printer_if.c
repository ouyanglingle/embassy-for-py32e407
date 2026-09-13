/**
  ******************************************************************************
  * @file    usbd_printer_if.c
  * @author  MCU Application Team
  * @brief   Generic media access Layer.
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
#include "usbd_printer_if.h"

/** @addtogroup PY32_USB_DEVICE_LIBRARY
  * @{
  */


/** @defgroup USBD_PRNT
  * @brief usbd core module
  * @{
  */

/** @defgroup USBD_PRNT_Private_TypesDefinitions
  * @{
  */
/**
  * @}
  */


/** @defgroup USBD_PRNT_Private_Defines
  * @{
  */
/**
  * @}
  */


/** @defgroup USBD_PRNT_Private_Macros
  * @{
  */

/**
  * @}
  */


/** @defgroup USBD_PRNT_Private_FunctionPrototypes
  * @{
  */

static int8_t PRNT_Init(void);
static int8_t PRNT_DeInit(void);
static int8_t PRNT_Control_req(uint8_t req, uint8_t *pbuf, uint16_t *length);
static int8_t PRNT_Receive(uint8_t *pbuf, uint32_t *Len);

/*printer Private function prototypes*/
void PRNT_PageEndManager(uint8_t *Buf, uint32_t Len);

USBD_PRNT_ItfTypeDef USBD_PRNT_fops =
{
  PRNT_Init,
  PRNT_DeInit,
  PRNT_Control_req,
  PRNT_Receive
};

static uint8_t PRNT_DEVICE_ID[] =
{
  0x00,
  '0','M','F','G',':','P','u','y','a',';',
  'C','M','D',':','T','S','P','L',';',
  'M','D','L',':','P','u','y','a',';',
  'C','L','S',':','P','R','I','N','T','E','R',';',
  0x00
};

static uint8_t PrntRxBuf[64] = {0};
extern USBD_HandleTypeDef USBD_Device;

/* Private functions ---------------------------------------------------------*/

/**
  * @brief  PRNT_Init
  *         Initializes the PRNT media low layer
  * @param  None
  * @retval Result of the operation: USBD_OK if all operations are OK else USBD_FAIL
  */
static int8_t PRNT_Init(void)
{
  USBD_PRNT_SetRxBuffer(&USBD_Device, (uint8_t *)&PrntRxBuf);
  
  return (0);
}

/**
  * @brief  PRNT_DeInit
  *         DeInitializes the PRNT media low layer
  * @param  None
  * @retval Result of the operation: USBD_OK if all operations are OK else USBD_FAIL
  */
static int8_t PRNT_DeInit(void)
{
  /*
     Add your deinitialization code here
  */
  return (0);
}


/**
  * @brief  PRNT_Receive
  *         Data received over USB OUT endpoint are sent over PRNT interface
  *         through this function.
  *
  *         @note
  *         This function will issue a NAK packet on any OUT packet received on
  *         USB endpoint until exiting this function. If you exit this function
  *         before transfer is complete on PRNT interface (ie. using DMA controller)
  *         it will result in receiving more data while previous ones are still
  *         not sent.
  *
  * @param  Buf: Buffer of data to be received
  * @param  Len: Number of data received (in bytes)
  * @retval Result of the operation: USBD_OK if all operations are OK else USBD_FAIL
  */
static int8_t PRNT_Receive(uint8_t *Buf, uint32_t *Len)
{
  printf("Receive:");
  for (int i = 0; i < sizeof(PrntRxBuf); i++)
  {
    printf("%d ", PrntRxBuf[i]);
  }
  printf("\n"); 
  
  /* Prepare Out endpoint to receive next packet */
  (void)USBD_PRNT_ReceivePacket(&USBD_Device);
  
  UNUSED(Buf);
  UNUSED(Len);

  return (0);
}


/**
  * @brief  PRNT_Control_req
  *         Manage the  PRNT class requests
  * @param  req: Command code
  * @param  pbuf: Buffer containing command data (request parameters)
  * @param  length: Number of data to be sent (in bytes)
  * @retval Result of the operation: USBD_OK if all operations are OK else USBD_FAIL
  */
static int8_t PRNT_Control_req(uint8_t req, uint8_t *pbuf, uint16_t *length)
{
  uint32_t i = 0U;

  /* Check on the setup request value */
  switch (req)
  {
    /* Get Printer Device ID request */
    case PRNT_GET_DEVICE_ID:
      /* Not using for loop here due to MISRA-C2012-Rule-16.1 */
      while (i < sizeof(PRNT_DEVICE_ID))
      {
        pbuf[i] = PRNT_DEVICE_ID[i];
        i++;
      }
      *length = (uint16_t)i;
      break;

    /* Get Printer current status */
    case PRNT_GET_PORT_STATUS:
      pbuf[0] =  PRNT_STATUS_PAPER_EMPTY |
                 PRNT_STATUS_SELECTED |
                 PRNT_STATUS_NO_ERROR;

      *length = 1U;
      break;

    /* Printer SOFT RESET request: cleanup pending tasks */
    case PRNT_SOFT_RESET:
      break;

    default:
      /* Unknown commands are not managed */
      break;
  }

  return (0);
}

/**
  * @}
  */

/**
  * @}
  */

/**
  * @}
  */

