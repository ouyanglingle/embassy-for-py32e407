#[doc = "Register `DIV1` reader"]
pub type R = crate::R<Div1Spec>;
#[doc = "Register `DIV1` writer"]
pub type W = crate::W<Div1Spec>;
#[doc = "Field `RTC_DIV` reader - RTC clock divider byte1"]
pub type RtcDivR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RTC clock divider byte1"]
    #[inline(always)]
    pub fn rtc_div(&self) -> RtcDivR {
        RtcDivR::new(self.bits)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`div1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Div1Spec;
impl crate::RegisterSpec for Div1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`div1::R`](R) reader structure"]
impl crate::Readable for Div1Spec {}
#[doc = "`write(|w| ..)` method takes [`div1::W`](W) writer structure"]
impl crate::Writable for Div1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIV1 to value 0"]
impl crate::Resettable for Div1Spec {}
