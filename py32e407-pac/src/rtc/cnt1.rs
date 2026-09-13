#[doc = "Register `CNT1` reader"]
pub type R = crate::R<Cnt1Spec>;
#[doc = "Register `CNT1` writer"]
pub type W = crate::W<Cnt1Spec>;
#[doc = "Field `RTC_CNT` reader - RTC counter byte1"]
pub type RtcCntR = crate::FieldReader;
#[doc = "Field `RTC_CNT` writer - RTC counter byte1"]
pub type RtcCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RTC counter byte1"]
    #[inline(always)]
    pub fn rtc_cnt(&self) -> RtcCntR {
        RtcCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC counter byte1"]
    #[inline(always)]
    pub fn rtc_cnt(&mut self) -> RtcCntW<'_, Cnt1Spec> {
        RtcCntW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cnt1Spec;
impl crate::RegisterSpec for Cnt1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cnt1::R`](R) reader structure"]
impl crate::Readable for Cnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`cnt1::W`](W) writer structure"]
impl crate::Writable for Cnt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNT1 to value 0"]
impl crate::Resettable for Cnt1Spec {}
