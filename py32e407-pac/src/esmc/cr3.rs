#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `ADDR8BIT` reader - 8bit address"]
pub type Addr8bitR = crate::BitReader;
#[doc = "Field `ADDR8BIT` writer - 8bit address"]
pub type Addr8bitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR16BIT` reader - 16bit address"]
pub type Addr16bitR = crate::BitReader;
#[doc = "Field `ADDR16BIT` writer - 16bit address"]
pub type Addr16bitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR32BIT` reader - 32bit address"]
pub type Addr32bitR = crate::BitReader;
#[doc = "Field `ADDR32BIT` writer - 32bit address"]
pub type Addr32bitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_ADDR` reader - no address enable"]
pub type NoAddrR = crate::BitReader;
#[doc = "Field `NO_ADDR` writer - no address enable"]
pub type NoAddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XSPI_RWDS` reader - xSPI RWDS enable"]
pub type XspiRwdsR = crate::BitReader;
#[doc = "Field `XSPI_RWDS` writer - xSPI RWDS enable"]
pub type XspiRwdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_EXT` reader - command extend enable"]
pub type CmdExtR = crate::BitReader;
#[doc = "Field `CMD_EXT` writer - command extend enable"]
pub type CmdExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_DATA` reader - no data enable"]
pub type NoDataR = crate::BitReader;
#[doc = "Field `NO_DATA` writer - no data enable"]
pub type NoDataW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 8bit address"]
    #[inline(always)]
    pub fn addr8bit(&self) -> Addr8bitR {
        Addr8bitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 16bit address"]
    #[inline(always)]
    pub fn addr16bit(&self) -> Addr16bitR {
        Addr16bitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 32bit address"]
    #[inline(always)]
    pub fn addr32bit(&self) -> Addr32bitR {
        Addr32bitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no address enable"]
    #[inline(always)]
    pub fn no_addr(&self) -> NoAddrR {
        NoAddrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - xSPI RWDS enable"]
    #[inline(always)]
    pub fn xspi_rwds(&self) -> XspiRwdsR {
        XspiRwdsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - command extend enable"]
    #[inline(always)]
    pub fn cmd_ext(&self) -> CmdExtR {
        CmdExtR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - no data enable"]
    #[inline(always)]
    pub fn no_data(&self) -> NoDataR {
        NoDataR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 8bit address"]
    #[inline(always)]
    pub fn addr8bit(&mut self) -> Addr8bitW<'_, Cr3Spec> {
        Addr8bitW::new(self, 0)
    }
    #[doc = "Bit 1 - 16bit address"]
    #[inline(always)]
    pub fn addr16bit(&mut self) -> Addr16bitW<'_, Cr3Spec> {
        Addr16bitW::new(self, 1)
    }
    #[doc = "Bit 2 - 32bit address"]
    #[inline(always)]
    pub fn addr32bit(&mut self) -> Addr32bitW<'_, Cr3Spec> {
        Addr32bitW::new(self, 2)
    }
    #[doc = "Bit 3 - no address enable"]
    #[inline(always)]
    pub fn no_addr(&mut self) -> NoAddrW<'_, Cr3Spec> {
        NoAddrW::new(self, 3)
    }
    #[doc = "Bit 4 - xSPI RWDS enable"]
    #[inline(always)]
    pub fn xspi_rwds(&mut self) -> XspiRwdsW<'_, Cr3Spec> {
        XspiRwdsW::new(self, 4)
    }
    #[doc = "Bit 5 - command extend enable"]
    #[inline(always)]
    pub fn cmd_ext(&mut self) -> CmdExtW<'_, Cr3Spec> {
        CmdExtW::new(self, 5)
    }
    #[doc = "Bit 7 - no data enable"]
    #[inline(always)]
    pub fn no_data(&mut self) -> NoDataW<'_, Cr3Spec> {
        NoDataW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for Cr3Spec {}
