/**
******************************************************************************
  * @file    : USB_HOST/MSC_Standalone/USB_Host/App/file_operations.c
  * @author  : MCD Application Team
  * @brief   : Write/read file on the disk.
  ******************************************************************************
  * @attention
  *
  * Copyright (c) 2019 STMicroelectronics.
  * All rights reserved.
  *
  * This software is licensed under terms that can be found in the LICENSE file
  * in the root directory of this software component.
  * If no LICENSE file comes with this software, it is provided AS-IS.
  *
  ******************************************************************************
  */
/* Includes ------------------------------------------------------------------*/
#include "file_operations.h"
#include "main.h"

/* Private typedef -----------------------------------------------------------*/
static DWORD scratch[FF_MAX_SS / 4];

/* Private define ------------------------------------------------------------*/
FIL       MyFile;
FRESULT   res;
uint32_t  bytesWritten;
uint8_t   rtext[200];
uint8_t   wtext[] = "USB Host Library : Mass Storage Example";

/* Private macro -------------------------------------------------------------*/
/* Private variables ---------------------------------------------------------*/
/* Private function prototypes -----------------------------------------------*/
/* Private functions ---------------------------------------------------------*/

/**
* @brief  Files operations: Read/Write and compare
* @param  None
* @retval Operation result
*/
uint8_t msc_file_operations(void)
{
  uint32_t bytesread;
  uint8_t retvalue = 0;

  USBH_UsrLog("INFO : FatFs Initialized\n");

  if(f_open(&MyFile, "2:USBDemo.txt",FA_CREATE_ALWAYS | FA_WRITE) != FR_OK)
  {
    USBH_ErrLog("Cannot Open 'USBDemo.txt' file \n");
    retvalue = 1;
  }
  else
  {
    USBH_UsrLog("Write Text:");
    USBH_UsrLog("INFO : 'USBDemo.txt' opened for write");
    res= f_write (&MyFile, wtext, sizeof(wtext), (void *)&bytesWritten);
    f_close(&MyFile);

    if((bytesWritten == 0) || (res != FR_OK)) /*EOF or Error*/
    {
      USBH_ErrLog("Cannot Write on the  'USBDemo.txt' file \n");
      retvalue = 1;
    }
    else
    {
      if(f_open(&MyFile, "2:USBDemo.txt", FA_READ) != FR_OK)
      {
        USBH_ErrLog("Cannot Open 'USBDemo.txt' file for read.\n");
        retvalue = 1;
      }
      else
      {
        USBH_UsrLog("INFO : Text written on the 'USBDemo.txt' file\n");

        res = f_read(&MyFile, rtext, sizeof(rtext), (void *)&bytesread);

        if((bytesread == 0) || (res != FR_OK)) /*EOF or Error*/
        {
          USBH_ErrLog("Cannot Read from the  'USBDemo.txt' file \n");
          retvalue = 1;
        }
        else
        {
          USBH_UsrLog("Read Text:");
          USBH_UsrLog((char *)rtext);
          USBH_UsrLog("\n");
        }
        f_close(&MyFile);
      }
      /* Compare read data with the expected data */
      if((bytesread == bytesWritten))
      {
        USBH_UsrLog("INFO : FatFs data compare SUCCES");
      }
      else
      {
        USBH_ErrLog("FatFs data compare ERROR");
        USBH_ErrLog("\n");
        retvalue = 1;
      }
    }
  }
  return (retvalue);
}

/**
  * @brief  Initializes a Drive
  * @retval DSTATUS: Operation status
  */
DSTATUS USB_disk_initialize()
{
  /* CAUTION : USB Host library has to be initialized in the application */

  return 0;
}

/**
  * @brief  Gets Disk Status
  * @retval DSTATUS: Operation status
  */
DSTATUS USB_disk_status()
{
  DRESULT res = RES_ERROR;

  if(USBH_MSC_UnitIsReady(&hUSBH, 0))
  {
    res = RES_OK;
  }
  else
  {
    res = RES_ERROR;
  }

  return res;
}
/**
  * @brief  Reads Sector(s)
  * @param  lun : lun id
  * @param  *buff: Data buffer to store read data
  * @param  sector: Sector address (LBA)
  * @param  count: Number of sectors to read (1..128)
  * @retval DRESULT: Operation result
  */
DRESULT USB_disk_read(BYTE *buff, DWORD sector, UINT count)
{
  DRESULT res = RES_ERROR;
  MSC_LUNTypeDef info;
  USBH_StatusTypeDef  status = USBH_OK;

  if ((DWORD)buff & 3)
  {
    while ((count--)&&(status == USBH_OK))
    {
      status = USBH_MSC_Read(&hUSBH, 0, sector + count, (uint8_t *)scratch, 1);

      if(status == USBH_OK)
      {
        memcpy (&buff[count * FF_MAX_SS] ,scratch, FF_MAX_SS);
      }
      else
      {
        break;
      }
    }
  }
  else
  {
    status = USBH_MSC_Read(&hUSBH, 0, sector, buff, count);
  }

  if(status == USBH_OK)
  {
    res = RES_OK;
  }
  else
  {
    USBH_MSC_GetLUNInfo(&hUSBH, 0, &info);

    switch (info.sense.asc)
    {
      case SCSI_ASC_LOGICAL_UNIT_NOT_READY:
      case SCSI_ASC_MEDIUM_NOT_PRESENT:
      case SCSI_ASC_NOT_READY_TO_READY_CHANGE:
        USBH_ErrLog ("USB Disk is not ready!");
        res = RES_NOTRDY;
        break;

      default:
        res = RES_ERROR;
        break;
    }
  }

  return res;
}

/**
  * @brief  Writes Sector(s)
  * @param  lun : lun id
  * @param  *buff: Data to be written
  * @param  sector: Sector address (LBA)
  * @param  count: Number of sectors to write (1..128)
  * @retval DRESULT: Operation result
  */

DRESULT USB_disk_write(const BYTE *buff, DWORD sector, UINT count)
{
  DRESULT res = RES_ERROR;
  MSC_LUNTypeDef info;
  USBH_StatusTypeDef  status = USBH_OK;

  if ((DWORD)buff & 3)
  {
    while (count--)
    {
      memcpy (scratch, &buff[count * FF_MAX_SS], FF_MAX_SS);

      status = USBH_MSC_Write(&hUSBH, 0, sector + count, (BYTE *)scratch, 1) ;
      if(status == USBH_FAIL)
      {
        break;
      }
    }
  }
  else
  {
    status = USBH_MSC_Write(&hUSBH, 0, sector, (BYTE *)buff, count);
  }

  if(status == USBH_OK)
  {
    res = RES_OK;
  }
  else
  {
    USBH_MSC_GetLUNInfo(&hUSBH, 0, &info);

    switch (info.sense.asc)
    {
      case SCSI_ASC_WRITE_PROTECTED:
        USBH_ErrLog("USB Disk is Write protected!");
        res = RES_WRPRT;
      break;

      case SCSI_ASC_LOGICAL_UNIT_NOT_READY:
      case SCSI_ASC_MEDIUM_NOT_PRESENT:
      case SCSI_ASC_NOT_READY_TO_READY_CHANGE:
        USBH_ErrLog("USB Disk is not ready!");
        res = RES_NOTRDY;
      break;

      default:
        res = RES_ERROR;
      break;
    }
  }

  return res;
}

/**
  * @brief  I/O control operation
  * @param  cmd: Control code
  * @param  *buff: Buffer to send/receive control data
  * @retval DRESULT: Operation result
  */
DRESULT USBH_ioctl(BYTE cmd, void *buff)
{
  DRESULT res = RES_ERROR;
  MSC_LUNTypeDef info;

  switch (cmd)
  {
    /* Make sure that no pending write process */
    case CTRL_SYNC:
      res = RES_OK;
    break;

    /* Get number of sectors on the disk (DWORD) */
    case GET_SECTOR_COUNT :
      if(USBH_MSC_GetLUNInfo(&hUSBH, 0, &info) == USBH_OK)
      {
        *(DWORD*)buff = info.capacity.block_nbr;
        res = RES_OK;
      }
      else
      {
        res = RES_ERROR;
      }
    break;

    /* Get R/W sector size (WORD) */
    case GET_SECTOR_SIZE :
      if(USBH_MSC_GetLUNInfo(&hUSBH, 0, &info) == USBH_OK)
      {
        *(DWORD*)buff = info.capacity.block_size;
        res = RES_OK;
      }
      else
      {
        res = RES_ERROR;
      }
    break;

      /* Get erase block size in unit of sector (DWORD) */
    case GET_BLOCK_SIZE :

      if(USBH_MSC_GetLUNInfo(&hUSBH, 0, &info) == USBH_OK)
      {
        *(DWORD*)buff = info.capacity.block_size;
        res = RES_OK;
      }
      else
      {
        res = RES_ERROR;
      }
    break;

    default:
      res = RES_PARERR;
    break;
  }

  return res;
}

