/**
  ******************************************************************************
  * @file    usbd_customhid_if.c
  * @author  MCU Application Team
  * @brief   USB Device Custom HID interface file.
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
#include "usbd_customhid_if.h"
/* Private typedef -----------------------------------------------------------*/
/* Private define ------------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static int8_t CUSTOM_HID_Init(void);
static int8_t CUSTOM_HID_DeInit(void);
static int8_t CUSTOM_HID_OutEvent(uint8_t event_idx, uint8_t state);

/* Private variables ---------------------------------------------------------*/
__ALIGN_BEGIN static uint8_t CUSTOM_HID_ReportDesc[USBD_CUSTOM_HID_REPORT_DESC_SIZE] __ALIGN_END =
{
  0x05, 0x01,        /* Usage Page (Generic Desktop Ctrls)                                                     */
  0x09, 0x06,        /* Usage (Keyboard)                                                                       */
  0xA1, 0x01,        /* Collection (Application)                                                               */
  0x05, 0x07,        /*   Usage Page (Kbrd/Keypad)                                                             */
  0x19, 0xE0,        /*   Usage Minimum (LeftControl)                                                          */
  0x29, 0xE7,        /*   Usage Maximum (Right GUI)                                                            */
  0x15, 0x00,        /*   Logical Minimum (0)                                                                  */
  0x25, 0x01,        /*   Logical Maximum (1)                                                                  */
  0x75, 0x01,        /*   Report Size (1)                                                                      */
  0x95, 0x08,        /*   Report Count (8)                                                                     */
  0x81, 0x02,        /*   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)                 */
  0x95, 0x01,        /*   Report Count (1)                                                                     */
  0x75, 0x08,        /*   Report Size (8)                                                                      */
  0x81, 0x03,        /*   Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)                */
  0x95, 0x05,        /*   Report Count (5)                                                                     */
  0x75, 0x01,        /*   Report Size (1)                                                                      */
  0x05, 0x08,        /*   Usage Page (LEDs)                                                                    */
  0x19, 0x01,        /*   Usage Minimum (Num Lock)                                                             */
  0x29, 0x05,        /*   Usage Maximum (Kana)                                                                 */
  0x91, 0x02,        /*   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)   */
  0x95, 0x01,        /*   Report Count (1)                                                                     */
  0x75, 0x03,        /*   Report Size (3)                                                                      */
  0x91, 0x03,        /*   Output (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)  */
  0x95, 0x06,        /*   Report Count (6)                                                                     */
  0x75, 0x08,        /*   Report Size (8)                                                                      */
  0x15, 0x00,        /*   Logical Minimum (0)                                                                  */
  0x25, 0xFF,        /*   Logical Maximum (255)                                                                */
  0x05, 0x07,        /*   Usage Page (Kbrd/Keypad)                                                             */
  0x19, 0x00,        /*   Usage Minimum (0x00)                                                                 */
  0x29, 0x65,        /*   Usage Maximum (0x65)                                                                 */
  0x81, 0x00,        /*   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)               */
  0xC0,              /* End Collection                                                                         */
}; 

USBD_CUSTOM_HID_ItfTypeDef USBD_CustomHID_fops =
{
  CUSTOM_HID_ReportDesc,
  CUSTOM_HID_Init,
  CUSTOM_HID_DeInit,
  CUSTOM_HID_OutEvent,
};

/* Private functions ---------------------------------------------------------*/

/**
  * @brief  CUSTOM_HID_Init
  *         Initializes the CUSTOM HID media low layer
  * @param  None
  * @retval Result of the operation: USBD_OK if all operations are OK else USBD_FAIL
  */
static int8_t CUSTOM_HID_Init(void)
{

  return (0);
}

/**
  * @brief  CUSTOM_HID_DeInit
  *         DeInitializes the CUSTOM HID media low layer
  * @param  None
  * @retval Result of the operation: USBD_OK if all operations are OK else USBD_FAIL
  */
static int8_t CUSTOM_HID_DeInit(void)
{
  /*
     Add your deinitialization code here
  */
  return (0);
}


/**
  * @brief  CUSTOM_HID_Control
  *         Manage the CUSTOM HID class events
  * @param  event_idx: event index
  * @param  state: event state
  * @retval Result of the operation: USBD_OK if all operations are OK else USBD_FAIL
  */
static int8_t CUSTOM_HID_OutEvent(uint8_t event_idx, uint8_t state)
{

  return (0);
}
/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
