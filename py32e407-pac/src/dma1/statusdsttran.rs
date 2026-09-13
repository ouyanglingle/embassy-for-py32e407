#[doc = "Register `STATUSDSTTRAN` reader"]
pub type R = crate::R<StatusdsttranSpec>;
#[doc = "Register `STATUSDSTTRAN` writer"]
pub type W = crate::W<StatusdsttranSpec>;
#[doc = "Field `STATUS` reader - Status for IntDstTran Interrupt"]
pub type StatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Status for IntDstTran Interrupt"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 0x3f) as u8)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`statusdsttran::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusdsttran::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusdsttranSpec;
impl crate::RegisterSpec for StatusdsttranSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusdsttran::R`](R) reader structure"]
impl crate::Readable for StatusdsttranSpec {}
#[doc = "`write(|w| ..)` method takes [`statusdsttran::W`](W) writer structure"]
impl crate::Writable for StatusdsttranSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUSDSTTRAN to value 0"]
impl crate::Resettable for StatusdsttranSpec {}
