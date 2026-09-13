#[doc = "Register `STATUSTFR` reader"]
pub type R = crate::R<StatustfrSpec>;
#[doc = "Register `STATUSTFR` writer"]
pub type W = crate::W<StatustfrSpec>;
#[doc = "Field `STATUS` reader - Status for IntTfr Interrupt"]
pub type StatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Status for IntTfr Interrupt"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 0x3f) as u8)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`statustfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statustfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatustfrSpec;
impl crate::RegisterSpec for StatustfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statustfr::R`](R) reader structure"]
impl crate::Readable for StatustfrSpec {}
#[doc = "`write(|w| ..)` method takes [`statustfr::W`](W) writer structure"]
impl crate::Writable for StatustfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUSTFR to value 0"]
impl crate::Resettable for StatustfrSpec {}
