#[doc = "Register `ALR3` reader"]
pub type R = crate::R<Alr3Spec>;
#[doc = "Register `ALR3` writer"]
pub type W = crate::W<Alr3Spec>;
#[doc = "Field `RTC_ALR` reader - RTC alarm byte3"]
pub type RtcAlrR = crate::FieldReader;
#[doc = "Field `RTC_ALR` writer - RTC alarm byte3"]
pub type RtcAlrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RTC alarm byte3"]
    #[inline(always)]
    pub fn rtc_alr(&self) -> RtcAlrR {
        RtcAlrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC alarm byte3"]
    #[inline(always)]
    pub fn rtc_alr(&mut self) -> RtcAlrW<'_, Alr3Spec> {
        RtcAlrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`alr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alr3Spec;
impl crate::RegisterSpec for Alr3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`alr3::R`](R) reader structure"]
impl crate::Readable for Alr3Spec {}
#[doc = "`write(|w| ..)` method takes [`alr3::W`](W) writer structure"]
impl crate::Writable for Alr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALR3 to value 0"]
impl crate::Resettable for Alr3Spec {}
