#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `SW` reader - "]
pub type SwR = crate::FieldReader;
#[doc = "Field `SW` writer - "]
pub type SwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWS` reader - "]
pub type SwsR = crate::FieldReader;
#[doc = "Field `SWS` writer - "]
pub type SwsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HPRE` reader - "]
pub type HpreR = crate::FieldReader;
#[doc = "Field `HPRE` writer - "]
pub type HpreW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPRE1` reader - "]
pub type Ppre1R = crate::FieldReader;
#[doc = "Field `PPRE1` writer - "]
pub type Ppre1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PPRE2` reader - "]
pub type Ppre2R = crate::FieldReader;
#[doc = "Field `PPRE2` writer - "]
pub type Ppre2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADCPRE` reader - "]
pub type AdcpreR = crate::FieldReader;
#[doc = "Field `ADCPRE` writer - "]
pub type AdcpreW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USBPRE` reader - "]
pub type UsbpreR = crate::FieldReader;
#[doc = "Field `USBPRE` writer - "]
pub type UsbpreW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRAM2PRE` reader - SRAM2PRE"]
pub type Sram2preR = crate::BitReader;
#[doc = "Field `SRAM2PRE` writer - SRAM2PRE"]
pub type Sram2preW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCO` reader - "]
pub type McoR = crate::FieldReader;
#[doc = "Field `MCO` writer - "]
pub type McoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sws(&self) -> SwsR {
        SwsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn hpre(&self) -> HpreR {
        HpreR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ppre1(&self) -> Ppre1R {
        Ppre1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn ppre2(&self) -> Ppre2R {
        Ppre2R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn adcpre(&self) -> AdcpreR {
        AdcpreR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn usbpre(&self) -> UsbpreR {
        UsbpreR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - SRAM2PRE"]
    #[inline(always)]
    pub fn sram2pre(&self) -> Sram2preR {
        Sram2preR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn mco(&self) -> McoR {
        McoR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sw(&mut self) -> SwW<'_, CfgrSpec> {
        SwW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sws(&mut self) -> SwsW<'_, CfgrSpec> {
        SwsW::new(self, 2)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HpreW<'_, CfgrSpec> {
        HpreW::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ppre1(&mut self) -> Ppre1W<'_, CfgrSpec> {
        Ppre1W::new(self, 8)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn ppre2(&mut self) -> Ppre2W<'_, CfgrSpec> {
        Ppre2W::new(self, 11)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn adcpre(&mut self) -> AdcpreW<'_, CfgrSpec> {
        AdcpreW::new(self, 16)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn usbpre(&mut self) -> UsbpreW<'_, CfgrSpec> {
        UsbpreW::new(self, 20)
    }
    #[doc = "Bit 23 - SRAM2PRE"]
    #[inline(always)]
    pub fn sram2pre(&mut self) -> Sram2preW<'_, CfgrSpec> {
        Sram2preW::new(self, 23)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn mco(&mut self) -> McoW<'_, CfgrSpec> {
        McoW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
