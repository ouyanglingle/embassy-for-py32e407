#[doc = "Register `ALR1` reader"]
pub type R = crate::R<Alr1Spec>;
#[doc = "Register `ALR1` writer"]
pub type W = crate::W<Alr1Spec>;
#[doc = "Field `RTC_ALR` reader - RTC alarm byte1"]
pub type RtcAlrR = crate::FieldReader;
#[doc = "Field `RTC_ALR` writer - RTC alarm byte1"]
pub type RtcAlrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RTC alarm byte1"]
    #[inline(always)]
    pub fn rtc_alr(&self) -> RtcAlrR {
        RtcAlrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC alarm byte1"]
    #[inline(always)]
    pub fn rtc_alr(&mut self) -> RtcAlrW<'_, Alr1Spec> {
        RtcAlrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`alr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alr1Spec;
impl crate::RegisterSpec for Alr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`alr1::R`](R) reader structure"]
impl crate::Readable for Alr1Spec {}
#[doc = "`write(|w| ..)` method takes [`alr1::W`](W) writer structure"]
impl crate::Writable for Alr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALR1 to value 0"]
impl crate::Resettable for Alr1Spec {}
