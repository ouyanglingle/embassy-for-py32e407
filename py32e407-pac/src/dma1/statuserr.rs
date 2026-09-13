#[doc = "Register `STATUSERR` reader"]
pub type R = crate::R<StatuserrSpec>;
#[doc = "Register `STATUSERR` writer"]
pub type W = crate::W<StatuserrSpec>;
#[doc = "Field `STATUS` reader - Status for IntErr Interrupt"]
pub type StatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Status for IntErr Interrupt"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 0x3f) as u8)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`statuserr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statuserr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatuserrSpec;
impl crate::RegisterSpec for StatuserrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statuserr::R`](R) reader structure"]
impl crate::Readable for StatuserrSpec {}
#[doc = "`write(|w| ..)` method takes [`statuserr::W`](W) writer structure"]
impl crate::Writable for StatuserrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUSERR to value 0"]
impl crate::Resettable for StatuserrSpec {}
