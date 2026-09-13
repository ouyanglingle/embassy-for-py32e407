#[doc = "Register `CLKCR` reader"]
pub type R = crate::R<ClkcrSpec>;
#[doc = "Register `CLKCR` writer"]
pub type W = crate::W<ClkcrSpec>;
#[doc = "Field `CLKDIV` reader - desc CLKDIV"]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - desc CLKDIV"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLKEN` reader - desc CLKEN"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - desc CLKEN"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSAV` reader - desc PWRSAV"]
pub type PwrsavR = crate::BitReader;
#[doc = "Field `PWRSAV` writer - desc PWRSAV"]
pub type PwrsavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIDBUS` reader - desc WIDBUS"]
pub type WidbusR = crate::FieldReader;
#[doc = "Field `WIDBUS` writer - desc WIDBUS"]
pub type WidbusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SMPCLKSEL` reader - desc SMPCLKSEL"]
pub type SmpclkselR = crate::BitReader;
#[doc = "Field `SMPCLKSEL` writer - desc SMPCLKSEL"]
pub type SmpclkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPEN` reader - desc SMPEN"]
pub type SmpenR = crate::BitReader;
#[doc = "Field `SMPEN` writer - desc SMPEN"]
pub type SmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSEL` reader - desc CKSEL"]
pub type CkselR = crate::BitReader;
#[doc = "Field `CKSEL` writer - desc CKSEL"]
pub type CkselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - desc CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - desc CLKEN"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PWRSAV"]
    #[inline(always)]
    pub fn pwrsav(&self) -> PwrsavR {
        PwrsavR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - desc WIDBUS"]
    #[inline(always)]
    pub fn widbus(&self) -> WidbusR {
        WidbusR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - desc SMPCLKSEL"]
    #[inline(always)]
    pub fn smpclksel(&self) -> SmpclkselR {
        SmpclkselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc SMPEN"]
    #[inline(always)]
    pub fn smpen(&self) -> SmpenR {
        SmpenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc CKSEL"]
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, ClkcrSpec> {
        ClkdivW::new(self, 0)
    }
    #[doc = "Bit 8 - desc CLKEN"]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<'_, ClkcrSpec> {
        ClkenW::new(self, 8)
    }
    #[doc = "Bit 9 - desc PWRSAV"]
    #[inline(always)]
    pub fn pwrsav(&mut self) -> PwrsavW<'_, ClkcrSpec> {
        PwrsavW::new(self, 9)
    }
    #[doc = "Bits 10:11 - desc WIDBUS"]
    #[inline(always)]
    pub fn widbus(&mut self) -> WidbusW<'_, ClkcrSpec> {
        WidbusW::new(self, 10)
    }
    #[doc = "Bit 12 - desc SMPCLKSEL"]
    #[inline(always)]
    pub fn smpclksel(&mut self) -> SmpclkselW<'_, ClkcrSpec> {
        SmpclkselW::new(self, 12)
    }
    #[doc = "Bit 13 - desc SMPEN"]
    #[inline(always)]
    pub fn smpen(&mut self) -> SmpenW<'_, ClkcrSpec> {
        SmpenW::new(self, 13)
    }
    #[doc = "Bit 14 - desc CKSEL"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CkselW<'_, ClkcrSpec> {
        CkselW::new(self, 14)
    }
}
#[doc = "desc CLKCR\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkcrSpec;
impl crate::RegisterSpec for ClkcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcr::R`](R) reader structure"]
impl crate::Readable for ClkcrSpec {}
#[doc = "`write(|w| ..)` method takes [`clkcr::W`](W) writer structure"]
impl crate::Writable for ClkcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKCR to value 0"]
impl crate::Resettable for ClkcrSpec {}
