/* -----------------------------------------------------------------------------
 * Copyright (c) 2014 - 2019 ARM Ltd.
 *
 * This software is provided 'as-is', without any express or implied warranty.
 * In no event will the authors be held liable for any damages arising from
 * the use of this software. Permission is granted to anyone to use this
 * software for any purpose, including commercial applications, and to alter
 * it and redistribute it freely, subject to the following restrictions:
 *
 * 1. The origin of this software must not be misrepresented; you must not
 *    claim that you wrote the original software. If you use this software in
 *    a product, an acknowledgment in the product documentation would be
 *    appreciated but is not required.
 *
 * 2. Altered source versions must be plainly marked as such, and must not be
 *    misrepresented as being the original software.
 *
 * 3. This notice may not be removed or altered from any source distribution.
 *
 *
 * $Date:        2021-7-1
 * $Revision:    V1.0.0
 *
 * Project:      Flash Programming Functions for Puya PY32E407xx Flash
 * --------------------------------------------------------------------------- */

/* History:
 *  Version 1.0.0
 *    Initial release
 */

#include "FlashOS.h" // FlashOS Structures

typedef volatile unsigned char    vu8;
typedef          unsigned char     u8;
typedef volatile unsigned short   vu16;
typedef          unsigned short    u16;
typedef volatile unsigned long    vu32;
typedef          unsigned long     u32;

#define HW32_REG(ADDRESS)     ( * ((volatile unsigned           int * )(ADDRESS)))

#define HW16_REG(ADDRESS)     ( * ((volatile unsigned short     int * )(ADDRESS)))

#define HW8_REG(ADDRESS)      ( * ((volatile unsigned          char * )(ADDRESS)))

#define M32(ADDRESS)          HW32_REG(ADDRESS)

#define M16(ADDRESS)          HW16_REG(ADDRESS)

#define M8(ADDRESS)           HW8_REG(ADDRESS)


#define SET_BIT(REG, BIT)     ((REG) |= (BIT))
#define CLEAR_BIT(REG, BIT)   ((REG) &= ~(BIT))
#define READ_BIT(REG, BIT)    ((REG) & (BIT))
#define CLEAR_REG(REG)        ((REG) = (0x0))
#define WRITE_REG(REG, VAL)   ((REG) = (VAL))
#define READ_REG(REG)         ((REG))
#define MODIFY_REG(REG, CLEARMASK, SETMASK)  WRITE_REG((REG), (((READ_REG(REG)) & (~(CLEARMASK))) | (SETMASK)))



typedef struct
{
   vu32 ACR;              /*!< FLASH ACR Register,                 Address offset: 0x00  */
   vu32 RESERVED1;         
   vu32 KEYR;             /*!< FLASH KEYR Register,                Address offset: 0x08  */
   vu32 OPTKEYR;          /*!< FLASH OPTKEYR Register,             Address offset: 0x0C  */
   vu32 SR;               /*!< FLASH SR Register,                  Address offset: 0x10  */
   vu32 CR;               /*!< FLASH CR Register,                  Address offset: 0x14  */
   vu32 ECCR;             /*!< FLASH ECCR Register,                Address offset: 0x18  */
   vu32 RESERVED2;         
   vu32 OPTR1;            /*!< FLASH OPTR1 Register,               Address offset: 0x20  */
   vu32 OPTR2;            /*!< FLASH OPTR2 Register,               Address offset: 0x24  */
   vu32 RESERVED3;         
   vu32 WRPR;             /*!< FLASH WRPR Register,                Address offset: 0x2C  */
   vu32 PCROP0SR;         /*!< FLASH PCROP0SR Register,            Address offset: 0x30  */
   vu32 PCROP0ER;         /*!< FLASH PCROP0ER Register,            Address offset: 0x34  */
   vu32 PCROP1SR;         /*!< FLASH PCROP1SR Register,            Address offset: 0x38  */
   vu32 PCROP1ER;         /*!< FLASH PCROP1ER Register,            Address offset: 0x3C  */
} FLASH_TypeDef;

typedef struct
{
   vu32 KR;               /*!< IWDG KR Register,                   Address offset: 0x0  */
   vu32 PR;               /*!< IWDG PR Register,                   Address offset: 0x4  */
   vu32 RLR;              /*!< IWDG RLR Register,                  Address offset: 0x8  */
   vu32 SR;               /*!< IWDG SR Register,                   Address offset: 0xC  */
}IWDG_TypeDef;


typedef struct
{
   vu32 CR;               /*!< WWDR CR Register,                   Address offset: 0x0  */
   vu32 CFR;              /*!< WWDG CFR Register,                  Address offset: 0x4  */
   vu32 SR;               /*!< WWDG SR Register,                   Address offset: 0x8  */
}WWDG_TypeDef;



#define FLASH_BASE                         (0x08000000UL)                         /*!< FLASH base address */
#define FLASH_END                          (0x0807FFFFUL)                         /*!< FLASH end address */
#define FLASH_SIZE                         (FLASH_END - FLASH_BASE + 1)
#define FLASH_PAGE_SIZE                    (0x00000200UL)                         /*!< FLASH Page Size, 512 Bytes */
#define FLASH_PAGE_NB                      (FLASH_SIZE / FLASH_PAGE_SIZE)
#define FLASH_SECTOR_SIZE                  (0x00001000UL)                         /*!< FLASH Sector Size, 4096 Bytes */
#define FLASH_SECTOR_NB                    (FLASH_SIZE / FLASH_SECTOR_SIZE)
#define FLASH_BLOCK_SIZE                   (0x00010000UL)                         /*!< FLASH Block  Size, 65536 Bytes */
#define FLASH_BLOCK_NB                     (FLASH_SIZE / FLASH_BLOCK_SIZE)
              
#define WWDG_BASE                          (0x40002C00)
#define IWDG_BASE                          (0x40003000)
#define RCC_BASE                           (0x40021000)
#define FLASH_R_BASE                       (0x40022000)                          /*!< Flash registers base address */
#define OTP_BASE                            0x1FFF7000
#define OB_BASE                            (0x1FFF7400)                         /*!< Flash Option Bytes base address */
#define UID_BASE                           (0x1FFF7E00)                         /*!< Unique device ID register base address */
#define FLASH_BANK_SIZE                    (0x00040000)
#define FLASH_BANK_NB (FLASH_SIZE / FLASH_BANK_SIZE)
#define GET_FLASH_BANK_SIZE

#define FLASH                                  ((FLASH_TypeDef *) FLASH_R_BASE)
#define IWDG                                   ((IWDG_TypeDef *) IWDG_BASE)
#define RCC                                    ((RCC_TypeDef *) RCC_BASE)
#define WWDG                                   ((WWDG_TypeDef *) WWDG_BASE)


#define FLASH_ACR_LATENCY_Pos                     (0U)
#define FLASH_ACR_LATENCY_Msk                     (0xFUL<<FLASH_ACR_LATENCY_Pos)                    /*!< 0x0000000F */
#define FLASH_ACR_LATENCY                         FLASH_ACR_LATENCY_Msk                             /*!< Latency */
#define FLASH_ACR_LATENCY_0                       (0x1UL<<FLASH_ACR_LATENCY_Pos)                    /*!< 0x00000001 */
#define FLASH_ACR_LATENCY_1                       (0x2UL<<FLASH_ACR_LATENCY_Pos)                    /*!< 0x00000002 */
#define FLASH_ACR_LATENCY_2                       (0x4UL<<FLASH_ACR_LATENCY_Pos)                    /*!< 0x00000004 */
#define FLASH_ACR_LATENCY_3                       (0x8UL<<FLASH_ACR_LATENCY_Pos)                    /*!< 0x00000008 */


#define FLASH_SR_EOP_Pos                          (0U)
#define FLASH_SR_EOP_Msk                          (0x1UL<<FLASH_SR_EOP_Pos)                         /*!< 0x00000001 */
#define FLASH_SR_EOP                              FLASH_SR_EOP_Msk                                  /*!< End of operation */
#define FLASH_SR_WRPERR_Pos                       (4U)
#define FLASH_SR_WRPERR_Msk                       (0x1UL<<FLASH_SR_WRPERR_Pos)                      /*!< 0x00000010 */
#define FLASH_SR_WRPERR                           FLASH_SR_WRPERR_Msk                               /*!< Write protection error */
#define FLASH_SR_USRLOCK_Pos                      (13U)
#define FLASH_SR_USRLOCK_Msk                      (0x1UL<<FLASH_SR_USRLOCK_Pos)                     /*!< 0x00002000 */
#define FLASH_SR_USRLOCK                          FLASH_SR_USRLOCK_Msk                              /*!< Userdata Lock  */
#define FLASH_SR_RDERR_Pos                        (14U)
#define FLASH_SR_RDERR_Msk                        (0x1UL<<FLASH_SR_RDERR_Pos)                       /*!< 0x00004000 */
#define FLASH_SR_RDERR                            FLASH_SR_RDERR_Msk                                
#define FLASH_SR_OPTVERR_Pos                      (15U)
#define FLASH_SR_OPTVERR_Msk                      (0x1UL<<FLASH_SR_OPTVERR_Pos)                     /*!< 0x00008000 */
#define FLASH_SR_OPTVERR                          FLASH_SR_OPTVERR_Msk                              /*!< Option and trimming bits loading validity error */
#define FLASH_SR_BSY0_Pos                         (16U)
#define FLASH_SR_BSY0_Msk                         (0x1UL<<FLASH_SR_BSY0_Pos)                        /*!< 0x00010000 */
#define FLASH_SR_BSY0                             FLASH_SR_BSY0_Msk                                 /*!< Bank0 Busy */
#define FLASH_SR_BSY1_Pos                         (17U)
#define FLASH_SR_BSY1_Msk                         (0x1UL<<FLASH_SR_BSY1_Pos)                        /*!< 0x00020000 */
#define FLASH_SR_BSY1                             FLASH_SR_BSY1_Msk                                 /*!< Bank1 Busy */


#define FLASH_CR_PG_Pos                           (0U)
#define FLASH_CR_PG_Msk                           (0x1UL<<FLASH_CR_PG_Pos)                          /*!< 0x00000001 */
#define FLASH_CR_PG                               FLASH_CR_PG_Msk                                   /*!< Page Program */
#define FLASH_CR_PER_Pos                          (1U)
#define FLASH_CR_PER_Msk                          (0x1UL<<FLASH_CR_PER_Pos)                         /*!< 0x00000002 */
#define FLASH_CR_PER                              FLASH_CR_PER_Msk                                  /*!< Page Erase */
#define FLASH_CR_MER0_Pos                         (2U)
#define FLASH_CR_MER0_Msk                         (0x1UL<<FLASH_CR_MER0_Pos)                        /*!< 0x00000004 */
#define FLASH_CR_MER0                             FLASH_CR_MER0_Msk                                 /*!< Bank 0 Mass Erase */
#define FLASH_CR_MER1_Pos                         (3U)
#define FLASH_CR_MER1_Msk                         (0x1UL<<FLASH_CR_MER1_Pos)                        /*!< 0x00000008 */
#define FLASH_CR_MER1                             FLASH_CR_MER1_Msk                                 /*!< Bank 1 Mass Erase */
#define FLASH_CR_SER_Pos                          (11U)
#define FLASH_CR_SER_Msk                          (0x1UL<<FLASH_CR_SER_Pos)                         /*!< 0x00000800 */
#define FLASH_CR_SER                              FLASH_CR_SER_Msk                                  /*!< Sector Erase */
#define FLASH_CR_BER_Pos                          (12U)
#define FLASH_CR_BER_Msk                          (0x1UL<<FLASH_CR_BER_Pos)                         /*!< 0x00001000 */
#define FLASH_CR_BER                              FLASH_CR_BER_Msk                                  /*!< Block Erase */
#define FLASH_CR_OPTSTRT_Pos                      (17U)
#define FLASH_CR_OPTSTRT_Msk                      (0x1UL<<FLASH_CR_OPTSTRT_Pos)                     /*!< 0x00020000 */
#define FLASH_CR_OPTSTRT                          FLASH_CR_OPTSTRT_Msk                              /*!< Option bytes Programming Start */
#define FLASH_CR_PGSTRT_Pos                       (19U)
#define FLASH_CR_PGSTRT_Msk                       (0x1UL<<FLASH_CR_PGSTRT_Pos)                      /*!< 0x00080000 */
#define FLASH_CR_PGSTRT                           FLASH_CR_PGSTRT_Msk                               /*!< Programming Start */
#define FLASH_CR_EOPIE_Pos                        (24U)
#define FLASH_CR_EOPIE_Msk                        (0x1UL<<FLASH_CR_EOPIE_Pos)                       /*!< 0x01000000 */
#define FLASH_CR_EOPIE                            FLASH_CR_EOPIE_Msk                                /*!< End of operation interrupt enable */
#define FLASH_CR_ERRIE_Pos                        (25U)
#define FLASH_CR_ERRIE_Msk                        (0x1UL<<FLASH_CR_ERRIE_Pos)                       /*!< 0x02000000 */
#define FLASH_CR_ERRIE                            FLASH_CR_ERRIE_Msk                                /*!< Error interrupt enable */
#define FLASH_CR_RDERRIE_Pos                      (26U)
#define FLASH_CR_RDERRIE_Msk                      (0x1UL<<FLASH_CR_RDERRIE_Pos)                     /*!< 0x04000000 */
#define FLASH_CR_RDERRIE                          FLASH_CR_RDERRIE_Msk                              
#define FLASH_CR_OBL_LAUNCH_Pos                   (27U)
#define FLASH_CR_OBL_LAUNCH_Msk                   (0x1UL<<FLASH_CR_OBL_LAUNCH_Pos)                  /*!< 0x08000000 */
#define FLASH_CR_OBL_LAUNCH                       FLASH_CR_OBL_LAUNCH_Msk                           /*!< Force the option bytes loading */
#define FLASH_CR_OPTLOCK_Pos                      (30U)
#define FLASH_CR_OPTLOCK_Msk                      (0x1UL<<FLASH_CR_OPTLOCK_Pos)                     /*!< 0x40000000 */
#define FLASH_CR_OPTLOCK                          FLASH_CR_OPTLOCK_Msk                              /*!< Option Lock */
#define FLASH_CR_LOCK_Pos                         (31U)
#define FLASH_CR_LOCK_Msk                         (0x1UL<<FLASH_CR_LOCK_Pos)                        /*!< 0x80000000 */
#define FLASH_CR_LOCK                             FLASH_CR_LOCK_Msk                                 /*!< Lock */

#define FLASH_OPTR1_RDP_Pos                       (0U)
#define FLASH_OPTR1_RDP_Msk                       (0xFFUL<<FLASH_OPTR1_RDP_Pos)                     /*!< 0x000000FF */
#define FLASH_OPTR1_RDP                           FLASH_OPTR1_RDP_Msk                               /*!< Read Protection */
#define FLASH_OPTR1_IWDG_STOP_Pos                 (9U)
#define FLASH_OPTR1_IWDG_STOP_Msk                 (0x1UL<<FLASH_OPTR1_IWDG_STOP_Pos)                /*!< 0x00000200 */
#define FLASH_OPTR1_IWDG_STOP                     FLASH_OPTR1_IWDG_STOP_Msk                         
#define FLASH_OPTR1_IWDG_STDBY_Pos                (10U)
#define FLASH_OPTR1_IWDG_STDBY_Msk                (0x1UL<<FLASH_OPTR1_IWDG_STDBY_Pos)               /*!< 0x00000400 */
#define FLASH_OPTR1_IWDG_STDBY                    FLASH_OPTR1_IWDG_STDBY_Msk                        
#define FLASH_OPTR1_IWDG_SW_Pos                   (12U)
#define FLASH_OPTR1_IWDG_SW_Msk                   (0x1UL<<FLASH_OPTR1_IWDG_SW_Pos)                  /*!< 0x00001000 */
#define FLASH_OPTR1_IWDG_SW                       FLASH_OPTR1_IWDG_SW_Msk                           /*!< IWDG Software Enable */

#define FLASH_OPTR1_WWDG_SW_Pos                   (15U)
#define FLASH_OPTR1_WWDG_SW_Msk                   (0x1UL<<FLASH_OPTR1_WWDG_SW_Pos)                  /*!< 0x00008000 */
#define FLASH_OPTR1_WWDG_SW                       FLASH_OPTR1_WWDG_SW_Msk                           

#define FLASH_KEY1_Pos                            (0U)
#define FLASH_KEY1_Msk                            (0x45670123UL << FLASH_KEY1_Pos)                  /*!< 0x45670123 */
#define FLASH_KEY1                                FLASH_KEY1_Msk                                    /*!< Flash program erase key1 */
#define FLASH_KEY2_Pos                            (0U)
#define FLASH_KEY2_Msk                            (0xCDEF89ABUL << FLASH_KEY2_Pos)                  /*!< 0xCDEF89AB */
#define FLASH_KEY2                                FLASH_KEY2_Msk                                    /*!< Flash program erase key2: used with FLASH_PEKEY1 to unlock the write access to the FPEC. */

#define FLASH_OPTKEY1_Pos                         (0U)
#define FLASH_OPTKEY1_Msk                         (0x08192A3BUL << FLASH_OPTKEY1_Pos)               /*!< 0x08192A3B */
#define FLASH_OPTKEY1                             FLASH_OPTKEY1_Msk                                 /*!< Flash option key1 */
#define FLASH_OPTKEY2_Pos                         (0U)
#define FLASH_OPTKEY2_Msk                         (0x4C5D6E7FUL << FLASH_OPTKEY2_Pos)               /*!< 0x4C5D6E7F */
#define FLASH_OPTKEY2                             FLASH_OPTKEY2_Msk                                 /*!< Flash option key2: used with FLASH_OPTKEY1 to unlock the write access to the option byte block */




int ErasePage(unsigned long adr);

/*
 *  Initialize Flash Programming Functions
 *    Parameter:      adr:  Device Base Address
 *                    clk:  Clock Frequency (Hz)
 *                    fnc:  Function Code (1 - Erase, 2 - Program, 3 - Verify)
 *    Return Value:   0 - OK,  1 - Failed
 */

#ifdef FLASH_MEM
int Init(unsigned long adr, unsigned long clk, unsigned long fnc)
{
    FLASH->KEYR = FLASH_KEY1; // Unlock Flash
    FLASH->KEYR = FLASH_KEY2;

    CLEAR_BIT(FLASH->ACR, FLASH_ACR_LATENCY); // Zero Wait State, no Prefetch
    FLASH->SR |= FLASH_SR_EOP;                // Clear FLASH_EOP

    if ((FLASH->OPTR1 & FLASH_OPTR1_IWDG_SW) == 0x00) // Test if IWDG is running (IWDG in HW mode)
    {
        // Set IWDG time out to ~32.768 second
        IWDG->KR = 0x5555; // Enable write access to IWDG_PR and IWDG_RLR
        IWDG->PR = 0x06;   // Set prescaler to 256
        IWDG->RLR = 0xFFF; // Set reload value to 4095
    }

    return (0);
}
#endif

#ifdef FLASH_OB
int Init(unsigned long adr, unsigned long clk, unsigned long fnc)
{
    FLASH->KEYR = FLASH_KEY1; // Unlock Flash
    FLASH->KEYR = FLASH_KEY2;

    FLASH->OPTKEYR = FLASH_OPTKEY1; // Unlock Option Bytes
    FLASH->OPTKEYR = FLASH_OPTKEY2;

    CLEAR_BIT(FLASH->ACR, FLASH_ACR_LATENCY); // Zero Wait State, no Prefetch
    FLASH->SR |= FLASH_SR_EOP;                // Clear FLASH_EOP

    if ((FLASH->OPTR1 & FLASH_OPTR1_IWDG_SW) == 0x00) // Test if IWDG is running (IWDG in HW mode)
    {
        // Set IWDG time out to ~32.768 second
        IWDG->KR = 0x5555; // Enable write access to IWDG_PR and IWDG_RLR
        IWDG->PR = 0x06;   // Set prescaler to 256
        IWDG->RLR = 0xFFF; // Set reload value to 4095
    }

    return (0);
}
#endif

/*
 *  De-Initialize Flash Programming Functions
 *    Parameter:      fnc:  Function Code (1 - Erase, 2 - Program, 3 - Verify)
 *    Return Value:   0 - OK,  1 - Failed
 */

#ifdef FLASH_MEM
int UnInit(unsigned long fnc)
{
    FLASH->CR |= FLASH_CR_LOCK; // Lock Flash

    return (0);
}
#endif

#ifdef FLASH_OB
int UnInit(unsigned long fnc)
{
    FLASH->CR |= FLASH_CR_LOCK;    // Lock Flash
    FLASH->CR |= FLASH_CR_OPTLOCK; // Lock Option Bytes

    return (0);
}
#endif

/*
 *  Erase Sector in Flash Memory
 *    Parameter:      adr:  Sector Address
 *    Return Value:   0 - OK,  1 - Failed
 */

#ifdef FLASH_MEM
#ifdef FLASH_OTP
int EraseSector(unsigned long adr)
{
    return ErasePage(adr); // Done
}
#else
int EraseSector(unsigned long adr)
{
    while ((FLASH->SR & FLASH_SR_BSY0) || (FLASH->SR & FLASH_SR_BSY1))
    {
        IWDG->KR = 0xAAAA; // Reload IWDG
    }

    FLASH->SR |= FLASH_SR_EOP; // Clear FLASH_EOP

    FLASH->CR |= FLASH_CR_SER; // Sector Erase Enabled
    FLASH->CR |= FLASH_CR_EOPIE;

    M32(adr) = 0xFFFFFFFF; // Sector Address

    __asm("DSB");

    while ((FLASH->SR & FLASH_SR_BSY0) || (FLASH->SR & FLASH_SR_BSY1))
    {
        IWDG->KR = 0xAAAA; // Reload IWDG
    }

    FLASH->CR &= ~FLASH_CR_SER;   // Sector Erase Disabled
    FLASH->CR &= ~FLASH_CR_EOPIE; // Reset FLASH_EOPIE

    if (FLASH_SR_EOP != (FLASH->SR & FLASH_SR_EOP)) // Check for FLASH_SR_EOP
    {
        FLASH->SR |= FLASH_SR_EOP;
        return (1); // Failed
    }

    SET_BIT(FLASH->SR, FLASH_SR_EOP);

    return (0); // Done
}
#endif
#endif

#ifdef FLASH_OB
int EraseSector(unsigned long adr)
{
    /* erase sector is not needed for
       - Flash Option bytes
    */
    return (0); // Done
}
#endif

/*
 *  Program Page in Flash Memory
 *    Parameter:      adr:  Page Start Address
 *                    sz:   Page Size
 *                    buf:  Page Data
 *    Return Value:   0 - OK,  1 - Failed
 */

#ifdef FLASH_MEM
int ProgramPage(unsigned long adr, unsigned long sz, unsigned char *buf)
{
    while ((FLASH->SR & FLASH_SR_BSY0) || (FLASH->SR & FLASH_SR_BSY1))
    {
        IWDG->KR = 0xAAAA; // Reload IWDG
    }

    sz = (sz + (FLASH_PAGE_SIZE - 1)) & ~(FLASH_PAGE_SIZE - 1);

    FLASH->SR |= FLASH_SR_EOP; // Reset FLASH_EOP

    while (sz)
    {
        FLASH->CR |= FLASH_CR_PG; // Programming Enabled
        FLASH->CR |= FLASH_CR_EOPIE;

        for (u8 i = 0; i < FLASH_PAGE_SIZE / 4; i++)
        {

            M32(adr + i * 4) = M32(buf + i * 4); // Program the first word of the Double Word

            if (i == (FLASH_PAGE_SIZE / 4 - 2))
            {
                FLASH->CR |= FLASH_CR_PGSTRT;
            }
        }

        __asm("DSB");

        while ((FLASH->SR & FLASH_SR_BSY0) || (FLASH->SR & FLASH_SR_BSY1))
        {
            IWDG->KR = 0xAAAA; // Reload IWDG
        }

        FLASH->CR &= ~FLASH_CR_PG;    // Programming Disabled
        FLASH->CR &= ~FLASH_CR_EOPIE; // Reset FLASH_EOPIE

        FLASH->SR |= FLASH_SR_EOP;

        adr += FLASH_PAGE_SIZE; // Go to next Page
        buf += FLASH_PAGE_SIZE;
        sz -= FLASH_PAGE_SIZE;
    }

    return (0); // Done
}
#endif // FLASH_OTP

#ifdef FLASH_OB
int ProgramPage(unsigned long adr, unsigned long sz, unsigned char *buf)
{
    u32 optr1;
    u32 optr2;
    u32 wrpr;
    u32 pcrop0sr;
    u32 pcrop0er;
    u32 pcrop1sr;
    u32 pcrop1er;

    optr1 = M32(buf + 0x00);
    optr2 = M32(buf + 0x04);
    wrpr = M32(buf + 0x0C);
    pcrop0sr = M32(buf + 0x10);
    pcrop0er = M32(buf + 0x14);
    pcrop1sr = M32(buf + 0x18);
    pcrop1er = M32(buf + 0x1C);

    FLASH->SR |= FLASH_SR_EOP; // Clear FLASH_EOP

    FLASH->OPTR1 = (optr1 & 0x0000FFFF); // Write OPTR1 values
    FLASH->OPTR2 = (optr2 & 0x0000FFFF); // Write OPTR2 values
    FLASH->WRPR = (wrpr & 0x0000FFFF);
    FLASH->PCROP0SR = (pcrop0sr & 0x0000FFFF);
    FLASH->PCROP0ER = (pcrop0er & 0x0000FFFF);
    FLASH->PCROP1SR = (pcrop1sr & 0x0000FFFF);
    FLASH->PCROP1ER = (pcrop1er & 0x0000FFFF); // Write WRPR values

    FLASH->CR |= FLASH_CR_OPTSTRT;
    FLASH->CR |= FLASH_CR_EOPIE;

    HW32_REG(0x1FFF7400) = 0xFFFFFFFF;
    __asm("DSB");

    while ((FLASH->SR & FLASH_SR_BSY0) || (FLASH->SR & FLASH_SR_BSY1))
    {
        IWDG->KR = 0xAAAA; // Reload IWDG
    }

    FLASH->CR &= ~FLASH_CR_OPTSTRT; // Programming Disabled
    FLASH->CR &= ~FLASH_CR_EOPIE;   // Reset FLASH_EOPIE

    return (0); // Done
}
#endif //  FLASH_OB


int ErasePage(unsigned long adr)
{

    FLASH->SR |= FLASH_SR_EOP; // Clear FLASH_EOP

    FLASH->CR |= FLASH_CR_PER; // PAGE Erase Enabled
    FLASH->CR |= FLASH_CR_EOPIE;
    M32(adr) = 0xFF; // PAGE Address
    __asm("DSB");

    while ((FLASH->SR & FLASH_SR_BSY0) || (FLASH->SR & FLASH_SR_BSY1))
    {
        IWDG->KR = 0xAAAA; // Reload IWDG
    }

    FLASH->CR &= ~FLASH_CR_PER;   // Sector Erase Disabled
    FLASH->CR &= ~FLASH_CR_EOPIE; // Reset FLASH_EOPIE

    if (FLASH_SR_EOP != (FLASH->SR & FLASH_SR_EOP)) // Check for FLASH_SR_EOP
    {
        FLASH->SR |= FLASH_SR_EOP;
        return (1); // Failed
    }

    return (0); // Done
}
