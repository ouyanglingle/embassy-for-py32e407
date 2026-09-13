#[doc = "Register `DIV0` reader"]
pub type R = crate::R<Div0Spec>;
#[doc = "Register `DIV0` writer"]
pub type W = crate::W<Div0Spec>;
#[doc = "Field `RTC_DIV` reader - RTC clock divider byte0"]
pub type RtcDivR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RTC clock divider byte0"]
    #[inline(always)]
    pub fn rtc_div(&self) -> RtcDivR {
        RtcDivR::new(self.bits)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`div0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Div0Spec;
impl crate::RegisterSpec for Div0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`div0::R`](R) reader structure"]
impl crate::Readable for Div0Spec {}
#[doc = "`write(|w| ..)` method takes [`div0::W`](W) writer structure"]
impl crate::Writable for Div0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIV0 to value 0"]
impl crate::Resettable for Div0Spec {}
