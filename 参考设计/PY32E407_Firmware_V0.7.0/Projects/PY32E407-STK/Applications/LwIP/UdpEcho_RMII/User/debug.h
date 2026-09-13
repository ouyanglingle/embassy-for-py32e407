/**
  ******************************************************************************
  * @file    debug.h
  * @author  MCU Application Team
  * @brief   This file is used to print log messagese.
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
#ifndef _DEBUG_H
#define _DEBUG_H

/**
* @name Debug print 
* @{
*/

#ifndef PRINT_DEBUG_ENABLE
#define PRINT_DEBUG_ENABLE		1		/* Printing debugging information */
#endif

#ifndef PRINT_ERR_ENABLE
#define PRINT_ERR_ENABLE			1 	/* Printing error information */
#endif

#ifndef PRINT_INFO_ENABLE
#define PRINT_INFO_ENABLE			0		/* Print Personal Information */
#endif

#if PRINT_DEBUG_ENABLE
#define PRINT_DEBUG(fmt, args...) 	 do{(printf("\r\n[DEBUG] >> "), printf(fmt, ##args));}while(0)     
#else
#define PRINT_DEBUG(fmt, args...)	     
#endif

#if PRINT_ERR_ENABLE
#define PRINT_ERR(fmt, args...) 	 do{(printf("\r\n[ERR] >> "), printf(fmt, ##args));}while(0)     
#else
#define PRINT_ERR(fmt, args...)	       
#endif

#if PRINT_INFO_ENABLE
#define PRINT_INFO(fmt, args...) 	 do{(printf("\r\n[INFO] >> "), printf(fmt, ##args));}while(0)     
#else
#define PRINT_INFO(fmt, args...)	       
#endif

/**@} */
	
/* Calling different stdint.h files for different compilers */
#if defined(__ICCARM__) || defined(__CC_ARM) || defined(__GNUC__)
    #include <stdint.h>
#endif

/* Assert */
#define AssertCalled(char,int) 	printf("\nError:%s,%d\r\n",char,int)
#define ASSERT(x)   if((x)==0)  AssertCalled(__FILE__,__LINE__)
  
typedef enum 
{
	ASSERT_ERR = 0,								/* Error */
	ASSERT_SUCCESS = !ASSERT_ERR	/* Success */
} Assert_ErrorStatus;

typedef enum 
{
	FALSE = 0,		/* False */
	TRUE = !FALSE	/* True */
}bool;

#endif /* __DEBUG_H */

/************************ (C) COPYRIGHT Puya *****END OF FILE******************/
