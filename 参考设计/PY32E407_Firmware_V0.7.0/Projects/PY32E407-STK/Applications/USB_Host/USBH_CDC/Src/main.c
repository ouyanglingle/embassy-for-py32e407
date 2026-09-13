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
#define RX_BUFF_SIZE   128
#define TX_BUFF_SIZE   96

/* Private variables ---------------------------------------------------------*/
USBH_HandleTypeDef          hUSBH;
CDC_ApplicationTypeDef      App_State = APPLICATION_IDLE;
CDC_LineCodingTypeDef LineCoding;
uint8_t CDC_RX_Buffer[RX_BUFF_SIZE];
uint8_t CDC_RX_Flag = 0;
uint8_t CDC_TX_Buffer[TX_BUFF_SIZE];
uint8_t CDC_TX_Flag = 0;

/* Private user code ---------------------------------------------------------*/
/* Private macro -------------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
static void APP_SystemClockConfig(void);
static void USBH_UserProcess(USBH_HandleTypeDef *phost, uint8_t id);
static void USB_demo(USBH_HandleTypeDef *phost);

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

  /* Initialize LED */
  BSP_LED_Init(LED_GREEN);
  
  /* Initialize EXTI GPIO */
  BSP_PB_Init(BUTTON_USER,BUTTON_MODE_EXTI);

  /* Initialize debug UART (used for printf) */
  BSP_UART_Config();

  /* Init Host Library */
  USBH_Init(&hUSBH, USBH_UserProcess, 0);
  
  /* Add Supported Class */
  USBH_RegisterClass(&hUSBH, USBH_CDC_CLASS);
  
  /* Start Host Process */
  USBH_Start(&hUSBH);

  /* Infinite loop */
  while (1)
  {
    /* USB Host Background task */
    USBH_Process(&hUSBH);
    
    /* USB user task */
    USB_demo(&hUSBH);

  }
}

/**
  * @brief  User USB host demo.
  * @param  phost: USB HOST handle
  * @retval None
  */
static void USB_demo(USBH_HandleTypeDef *phost)
{
  if (App_State == APPLICATION_READY)
  {
    USBH_CDC_Stop(&hUSBH);
    memset(CDC_RX_Buffer, 0, RX_BUFF_SIZE);
    USBH_CDC_Receive(&hUSBH, CDC_RX_Buffer, RX_BUFF_SIZE);
    App_State = APPLICATION_RUNNING;
  }
  else if (App_State == APPLICATION_RUNNING)
  {
    if(CDC_RX_Flag == 1)
    {
      printf("\r\nreceive data :\r\n");
      uint16_t i = 0;
      for(i = 0; i < RX_BUFF_SIZE; i++)
      {
        printf("%c",CDC_RX_Buffer[i]);
      }
      printf("\r\n");
      CDC_RX_Flag = 0;
    }
    if(CDC_TX_Flag == 1)
    {
      memset(CDC_TX_Buffer, 0x5A, TX_BUFF_SIZE);
      printf("Sending data ...\r\n");
      USBH_CDC_Transmit(&hUSBH, CDC_TX_Buffer, TX_BUFF_SIZE);
      CDC_TX_Flag = 0;
    }
  }
  else if(App_State == APPLICATION_DISCONNECT)
  {
    App_State = APPLICATION_IDLE;
  }
  else
  {}
}

/**
  * @brief  GPIO EXTI callbacks
  * @param  GPIO_Pin: EXTI GPIO Pin
  * @retval None
  */
void HAL_GPIO_EXTI_Callback(uint16_t GPIO_Pin)
{
  if(GPIO_Pin == USER_BUTTON_PIN)
  {
    CDC_TX_Flag = 1;
  }
}

/**
  * @brief  CDC Receive callbacks
  * @param  phost: USB HOST handle
  * @retval None
  */
void USBH_CDC_ReceiveCallback(USBH_HandleTypeDef *phost)
{
  CDC_RX_Flag = 1;
  USBH_CDC_Receive(&hUSBH, CDC_RX_Buffer, RX_BUFF_SIZE);
}

/**
  * @brief  CDC Transmit callbacks
  * @param  phost: USB HOST handle
  * @retval None
  */
void USBH_CDC_TransmitCallback(USBH_HandleTypeDef *phost)
{
  printf(">> Data sent\r\n" );
}

/**
  * @brief  User Process
  * @param  phost: Host Handle
  * @param  id: Host Library user message ID
  * @retval None
  */
static void USBH_UserProcess(USBH_HandleTypeDef *phost, uint8_t id)
{
    switch(id)
    {
    case HOST_USER_SELECT_CONFIGURATION:
      break;

    case HOST_USER_DISCONNECTION:
      App_State = APPLICATION_DISCONNECT;
      break;

    case HOST_USER_CLASS_ACTIVE:
      USBH_CDC_GetLineCoding(&hUSBH, &LineCoding);
      App_State = APPLICATION_READY;
      printf("CDC deceive connect\r\n");
      USB_demo(&hUSBH);
      break;

    case HOST_USER_CONNECTION:
      App_State = APPLICATION_START;
      break;

    default:
      break;
    }
}

/**
  * @brief  System Clock Configuration
  * @param  None
  * @retval None
  */
static void APP_SystemClockConfig(void)
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
