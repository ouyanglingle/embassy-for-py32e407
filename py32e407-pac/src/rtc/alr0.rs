#[doc = "Register `ALR0` reader"]
pub type R = crate::R<Alr0Spec>;
#[doc = "Register `ALR0` writer"]
pub type W = crate::W<Alr0Spec>;
#[doc = "Field `RTC_ALR` reader - RTC alarm byte0"]
pub type RtcAlrR = crate::FieldReader;
#[doc = "Field `RTC_ALR` writer - RTC alarm byte0"]
pub type RtcAlrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RTC alarm byte0"]
    #[inline(always)]
    pub fn rtc_alr(&self) -> RtcAlrR {
        RtcAlrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC alarm byte0"]
    #[inline(always)]
    pub fn rtc_alr(&mut self) -> RtcAlrW<'_, Alr0Spec> {
        RtcAlrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`alr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alr0Spec;
impl crate::RegisterSpec for Alr0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`alr0::R`](R) reader structure"]
impl crate::Readable for Alr0Spec {}
#[doc = "`write(|w| ..)` method takes [`alr0::W`](W) writer structure"]
impl crate::Writable for Alr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALR0 to value 0"]
impl crate::Resettable for Alr0Spec {}
