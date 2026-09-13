#[doc = "Register `CNT2` reader"]
pub type R = crate::R<Cnt2Spec>;
#[doc = "Register `CNT2` writer"]
pub type W = crate::W<Cnt2Spec>;
#[doc = "Field `RTC_CNT` reader - RTC counter byte2"]
pub type RtcCntR = crate::FieldReader;
#[doc = "Field `RTC_CNT` writer - RTC counter byte2"]
pub type RtcCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RTC counter byte2"]
    #[inline(always)]
    pub fn rtc_cnt(&self) -> RtcCntR {
        RtcCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC counter byte2"]
    #[inline(always)]
    pub fn rtc_cnt(&mut self) -> RtcCntW<'_, Cnt2Spec> {
        RtcCntW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cnt2Spec;
impl crate::RegisterSpec for Cnt2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cnt2::R`](R) reader structure"]
impl crate::Readable for Cnt2Spec {}
#[doc = "`write(|w| ..)` method takes [`cnt2::W`](W) writer structure"]
impl crate::Writable for Cnt2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNT2 to value 0"]
impl crate::Resettable for Cnt2Spec {}
