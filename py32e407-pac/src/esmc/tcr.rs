#[doc = "Register `TCR` reader"]
pub type R = crate::R<TcrSpec>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TcrSpec>;
#[doc = "Field `HOLD` reader - setup/hold time"]
pub type HoldR = crate::FieldReader;
#[doc = "Field `HOLD` writer - setup/hold time"]
pub type HoldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HIGH` reader - high time"]
pub type HighR = crate::FieldReader;
#[doc = "Field `HIGH` writer - high time"]
pub type HighW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - setup/hold time"]
    #[inline(always)]
    pub fn hold(&self) -> HoldR {
        HoldR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - high time"]
    #[inline(always)]
    pub fn high(&self) -> HighR {
        HighR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - setup/hold time"]
    #[inline(always)]
    pub fn hold(&mut self) -> HoldW<'_, TcrSpec> {
        HoldW::new(self, 0)
    }
    #[doc = "Bits 4:7 - high time"]
    #[inline(always)]
    pub fn high(&mut self) -> HighW<'_, TcrSpec> {
        HighW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcrSpec;
impl crate::RegisterSpec for TcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TcrSpec {}
