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
#define HID_KBD_USAGE_CAPSLOCK        0x39 /* Keyboard Caps Lock */
#define HID_KBD_USAGE_LCAPSLOCK       0x82 /* Keyboard Locking Caps Lock */

/* Private variables ---------------------------------------------------------*/
USBD_HandleTypeDef USBD_Device;
/*!< keyboard report */
static uint8_t uhKeyboard_buf[8] = {0};
__IO   uint8_t ueRemotewakeup = 0;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/

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
  
  /* Initialize BUTTON */
  BSP_PB_Init(BUTTON_USER, BUTTON_MODE_EXTI);
  
  /* Initialize debug UART (used for printf) */
  BSP_UART_Config();

  /* Init Device Library */
  USBD_Init(&USBD_Device, &HID_Desc, 0); 
  
  /* Add Supported Class */
  USBD_RegisterClass(&USBD_Device, &USBD_CUSTOM_HID);
  
  /* Add Custom HID callbacks */
  USBD_CUSTOM_HID_RegisterInterface(&USBD_Device, &USBD_CustomHID_fops);

  /* Start Device Process */
  USBD_Start(&USBD_Device);
  
  while (1)
  {
    /* Wait for the button press to active remote wakeup */
    while(ueRemotewakeup != 1);
    ueRemotewakeup = 0;
    
    /* Send remote wake up */
    USB_ActivateRemoteWakeup(hUSB1.Instance);
    HAL_Delay(10);
    USB_DeActivateRemoteWakeup(hUSB1.Instance);
    
    /* Restore the state before suspend */
    USBD_LL_Resume(&USBD_Device);
    
    uhKeyboard_buf[2] = HID_KBD_USAGE_CAPSLOCK;
    USBD_CUSTOM_HID_SendReport(&USBD_Device, (uint8_t *)&uhKeyboard_buf, 8);
    HAL_Delay(1000);
    uhKeyboard_buf[2] = HID_KBD_USAGE_LCAPSLOCK;
    USBD_CUSTOM_HID_SendReport(&USBD_Device, (uint8_t *)&uhKeyboard_buf, 8);
    HAL_Delay(1000);
  }
}

void HAL_GPIO_EXTI_Callback(uint16_t GPIO_Pin)
{
  /* Restore phyclk */
  __HAL_USBDEV_UNGATE_PHYCLOCK(&hUSB1);
  
  /* Active remote wake up */
  ueRemotewakeup = 1;
}

/**
  * @brief  System Clock Configuration
  * @param  None
  * @retval None
  */
void APP_SystemClockConfig(void)
{
  RCC_OscInitTypeDef  OscInitstruct = {0};
  RCC_ClkInitTypeDef  ClkInitstruct = {0};
  
  OscInitstruct.OscillatorType  = RCC_OSCILLATORTYPE_HSE | RCC_OSCILLATORTYPE_HSI | RCC_OSCILLATORTYPE_LSE | 
                                  RCC_OSCILLATORTYPE_LSI | RCC_OSCILLATORTYPE_HSI48M;
  OscInitstruct.HSEState        = RCC_HSE_OFF;                              /* Close HSE */
/* OscInitstruct.HSEFreq         = RCC_HSE_16_32MHz; */                       /* Choose HSE frequency of 16-32MHz */
  OscInitstruct.HSI48MState     = RCC_HSI48M_ON;                            /* Enable HSI48M */
  OscInitstruct.HSIState        = RCC_HSI_ON;                               /* Enable HSI */
  OscInitstruct.LSEState        = RCC_LSE_OFF;                              /* Close LSE */
/* OscInitstruct.LSEDriver       = RCC_LSEDRIVE_HIGH; */                    /* Drive capability level: high */
  OscInitstruct.LSIState        = RCC_LSI_OFF;                              /* Close LSI */
  OscInitstruct.PLL.PLLState    = RCC_PLL_ON;                               /* Enable PLL */
  OscInitstruct.PLL.PLLSource   = RCC_PLLSOURCE_HSI_DIV2;                   /* PLL clock source selection HSI/2 */
  OscInitstruct.PLL.PLLPrediv   = RCC_PLL_PREDIV_DIV1;                      /* PLL clock Prediv factor */
  OscInitstruct.PLL.PLLMUL      = 12;                                       /* PLL clock source 12-fold frequency */
  OscInitstruct.PLL.PLLPostdiv  = RCC_PLL_POSTDIV_DIV1;                     /* PLL clock Postdiv factor */
  /* Configure oscillator */
  if(HAL_RCC_OscConfig(&OscInitstruct) != HAL_OK)
  {
    APP_ErrorHandler();
  }
  
  ClkInitstruct.ClockType       = RCC_CLOCKTYPE_SYSCLK | RCC_CLOCKTYPE_HCLK | RCC_CLOCKTYPE_PCLK1 | RCC_CLOCKTYPE_PCLK2;
  ClkInitstruct.SYSCLKSource    = RCC_SYSCLKSOURCE_PLLCLK;              /* System clock selection PLLCLK */
  ClkInitstruct.AHBCLKDivider   = RCC_SYSCLK_DIV1;                      /* AHB clock 1 division */
  ClkInitstruct.APB1CLKDivider  = RCC_HCLK_DIV2;                        /* APB1 clock 2 division */
  ClkInitstruct.APB2CLKDivider  = RCC_HCLK_DIV2;                        /* APB2 clock 2 division */
  /* Configure Clock */
  if(HAL_RCC_ClockConfig(&ClkInitstruct, FLASH_LATENCY_4) != HAL_OK)
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
