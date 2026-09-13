#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Field `DUAL` reader - Dual ADC mode selection"]
pub type DualR = crate::FieldReader;
#[doc = "Field `DUAL` writer - Dual ADC mode selection"]
pub type DualW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VREFBUFEN` reader - VrefBuffer enable"]
pub type VrefbufenR = crate::BitReader;
#[doc = "Field `VREFBUFEN` writer - VrefBuffer enable"]
pub type VrefbufenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFBUFSEL` reader - VREFBUF output voltage select"]
pub type VrefbufselR = crate::FieldReader;
#[doc = "Field `VREFBUFSEL` writer - VREFBUF output voltage select"]
pub type VrefbufselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VREFEN` reader - VREFINT enable"]
pub type VrefenR = crate::BitReader;
#[doc = "Field `VREFEN` writer - VREFINT enable"]
pub type VrefenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSENSESEL` reader - VTS selection"]
pub type VsenseselR = crate::BitReader;
#[doc = "Field `VSENSESEL` writer - VTS selection"]
pub type VsenseselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBATSEL` reader - VBAT selection"]
pub type VbatselR = crate::BitReader;
#[doc = "Field `VBATSEL` writer - VBAT selection"]
pub type VbatselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRMODE` reader - ADC power mode"]
pub type PwrmodeR = crate::FieldReader;
#[doc = "Field `PWRMODE` writer - ADC power mode"]
pub type PwrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VREFSEL` reader - ADC reference selection"]
pub type VrefselR = crate::BitReader;
#[doc = "Field `VREFSEL` writer - ADC reference selection"]
pub type VrefselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Dual ADC mode selection"]
    #[inline(always)]
    pub fn dual(&self) -> DualR {
        DualR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - VrefBuffer enable"]
    #[inline(always)]
    pub fn vrefbufen(&self) -> VrefbufenR {
        VrefbufenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - VREFBUF output voltage select"]
    #[inline(always)]
    pub fn vrefbufsel(&self) -> VrefbufselR {
        VrefbufselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 22 - VREFINT enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VrefenR {
        VrefenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - VTS selection"]
    #[inline(always)]
    pub fn vsensesel(&self) -> VsenseselR {
        VsenseselR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBAT selection"]
    #[inline(always)]
    pub fn vbatsel(&self) -> VbatselR {
        VbatselR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - ADC power mode"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PwrmodeR {
        PwrmodeR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 28 - ADC reference selection"]
    #[inline(always)]
    pub fn vrefsel(&self) -> VrefselR {
        VrefselR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dual ADC mode selection"]
    #[inline(always)]
    pub fn dual(&mut self) -> DualW<'_, CcrSpec> {
        DualW::new(self, 0)
    }
    #[doc = "Bit 5 - VrefBuffer enable"]
    #[inline(always)]
    pub fn vrefbufen(&mut self) -> VrefbufenW<'_, CcrSpec> {
        VrefbufenW::new(self, 5)
    }
    #[doc = "Bits 6:7 - VREFBUF output voltage select"]
    #[inline(always)]
    pub fn vrefbufsel(&mut self) -> VrefbufselW<'_, CcrSpec> {
        VrefbufselW::new(self, 6)
    }
    #[doc = "Bit 22 - VREFINT enable"]
    #[inline(always)]
    pub fn vrefen(&mut self) -> VrefenW<'_, CcrSpec> {
        VrefenW::new(self, 22)
    }
    #[doc = "Bit 23 - VTS selection"]
    #[inline(always)]
    pub fn vsensesel(&mut self) -> VsenseselW<'_, CcrSpec> {
        VsenseselW::new(self, 23)
    }
    #[doc = "Bit 24 - VBAT selection"]
    #[inline(always)]
    pub fn vbatsel(&mut self) -> VbatselW<'_, CcrSpec> {
        VbatselW::new(self, 24)
    }
    #[doc = "Bits 25:26 - ADC power mode"]
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PwrmodeW<'_, CcrSpec> {
        PwrmodeW::new(self, 25)
    }
    #[doc = "Bit 28 - ADC reference selection"]
    #[inline(always)]
    pub fn vrefsel(&mut self) -> VrefselW<'_, CcrSpec> {
        VrefselW::new(self, 28)
    }
}
#[doc = "ADC common control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {}
