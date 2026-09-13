#[doc = "Register `ALR2` reader"]
pub type R = crate::R<Alr2Spec>;
#[doc = "Register `ALR2` writer"]
pub type W = crate::W<Alr2Spec>;
#[doc = "Field `RTC_ALR` reader - RTC alarm byte2"]
pub type RtcAlrR = crate::FieldReader;
#[doc = "Field `RTC_ALR` writer - RTC alarm byte2"]
pub type RtcAlrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RTC alarm byte2"]
    #[inline(always)]
    pub fn rtc_alr(&self) -> RtcAlrR {
        RtcAlrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC alarm byte2"]
    #[inline(always)]
    pub fn rtc_alr(&mut self) -> RtcAlrW<'_, Alr2Spec> {
        RtcAlrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`alr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alr2Spec;
impl crate::RegisterSpec for Alr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`alr2::R`](R) reader structure"]
impl crate::Readable for Alr2Spec {}
#[doc = "`write(|w| ..)` method takes [`alr2::W`](W) writer structure"]
impl crate::Writable for Alr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALR2 to value 0"]
impl crate::Resettable for Alr2Spec {}
