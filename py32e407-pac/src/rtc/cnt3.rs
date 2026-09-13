#[doc = "Register `CNT3` reader"]
pub type R = crate::R<Cnt3Spec>;
#[doc = "Register `CNT3` writer"]
pub type W = crate::W<Cnt3Spec>;
#[doc = "Field `RTC_CNT` reader - RTC counter byte3"]
pub type RtcCntR = crate::FieldReader;
#[doc = "Field `RTC_CNT` writer - RTC counter byte3"]
pub type RtcCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RTC counter byte3"]
    #[inline(always)]
    pub fn rtc_cnt(&self) -> RtcCntR {
        RtcCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC counter byte3"]
    #[inline(always)]
    pub fn rtc_cnt(&mut self) -> RtcCntW<'_, Cnt3Spec> {
        RtcCntW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cnt3Spec;
impl crate::RegisterSpec for Cnt3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cnt3::R`](R) reader structure"]
impl crate::Readable for Cnt3Spec {}
#[doc = "`write(|w| ..)` method takes [`cnt3::W`](W) writer structure"]
impl crate::Writable for Cnt3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNT3 to value 0"]
impl crate::Resettable for Cnt3Spec {}
