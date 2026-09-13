#[doc = "Register `CNT0` reader"]
pub type R = crate::R<Cnt0Spec>;
#[doc = "Register `CNT0` writer"]
pub type W = crate::W<Cnt0Spec>;
#[doc = "Field `RTC_CNT` reader - RTC counter byte0"]
pub type RtcCntR = crate::FieldReader;
#[doc = "Field `RTC_CNT` writer - RTC counter byte0"]
pub type RtcCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RTC counter byte0"]
    #[inline(always)]
    pub fn rtc_cnt(&self) -> RtcCntR {
        RtcCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC counter byte0"]
    #[inline(always)]
    pub fn rtc_cnt(&mut self) -> RtcCntW<'_, Cnt0Spec> {
        RtcCntW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cnt0Spec;
impl crate::RegisterSpec for Cnt0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cnt0::R`](R) reader structure"]
impl crate::Readable for Cnt0Spec {}
#[doc = "`write(|w| ..)` method takes [`cnt0::W`](W) writer structure"]
impl crate::Writable for Cnt0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNT0 to value 0"]
impl crate::Resettable for Cnt0Spec {}
