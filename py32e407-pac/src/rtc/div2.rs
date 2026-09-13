#[doc = "Register `DIV2` reader"]
pub type R = crate::R<Div2Spec>;
#[doc = "Register `DIV2` writer"]
pub type W = crate::W<Div2Spec>;
#[doc = "Field `RTC_DIV` reader - RTC clock divider byte2"]
pub type RtcDivR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - RTC clock divider byte2"]
    #[inline(always)]
    pub fn rtc_div(&self) -> RtcDivR {
        RtcDivR::new(self.bits & 0x0f)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`div2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Div2Spec;
impl crate::RegisterSpec for Div2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`div2::R`](R) reader structure"]
impl crate::Readable for Div2Spec {}
#[doc = "`write(|w| ..)` method takes [`div2::W`](W) writer structure"]
impl crate::Writable for Div2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIV2 to value 0"]
impl crate::Resettable for Div2Spec {}
