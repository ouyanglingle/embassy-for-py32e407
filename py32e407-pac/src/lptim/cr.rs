#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ENABLE` reader - LPTIM enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - LPTIM enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNGSTRT` reader - LPTIM start in Single mode"]
pub type SngstrtR = crate::BitReader;
#[doc = "Field `SNGSTRT` writer - LPTIM start in Single mode"]
pub type SngstrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTSTRT` reader - Timer start in Continuous mode"]
pub type CntstrtR = crate::BitReader;
#[doc = "Field `CNTSTRT` writer - Timer start in Continuous mode"]
pub type CntstrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTRST` reader - Counter reset"]
pub type CountrstR = crate::BitReader;
#[doc = "Field `COUNTRST` writer - Counter reset"]
pub type CountrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTARE` reader - Reset after read enable"]
pub type RstareR = crate::BitReader;
#[doc = "Field `RSTARE` writer - Reset after read enable"]
pub type RstareW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPTIM enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPTIM start in Single mode"]
    #[inline(always)]
    pub fn sngstrt(&self) -> SngstrtR {
        SngstrtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer start in Continuous mode"]
    #[inline(always)]
    pub fn cntstrt(&self) -> CntstrtR {
        CntstrtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter reset"]
    #[inline(always)]
    pub fn countrst(&self) -> CountrstR {
        CountrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset after read enable"]
    #[inline(always)]
    pub fn rstare(&self) -> RstareR {
        RstareR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - LPTIM start in Single mode"]
    #[inline(always)]
    pub fn sngstrt(&mut self) -> SngstrtW<'_, CrSpec> {
        SngstrtW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer start in Continuous mode"]
    #[inline(always)]
    pub fn cntstrt(&mut self) -> CntstrtW<'_, CrSpec> {
        CntstrtW::new(self, 2)
    }
    #[doc = "Bit 3 - Counter reset"]
    #[inline(always)]
    pub fn countrst(&mut self) -> CountrstW<'_, CrSpec> {
        CountrstW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset after read enable"]
    #[inline(always)]
    pub fn rstare(&mut self) -> RstareW<'_, CrSpec> {
        RstareW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
