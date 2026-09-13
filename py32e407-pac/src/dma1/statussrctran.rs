#[doc = "Register `STATUSSRCTRAN` reader"]
pub type R = crate::R<StatussrctranSpec>;
#[doc = "Register `STATUSSRCTRAN` writer"]
pub type W = crate::W<StatussrctranSpec>;
#[doc = "Field `STATUS` reader - Status for IntSrcTran Interrupt"]
pub type StatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Status for IntSrcTran Interrupt"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 0x3f) as u8)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`statussrctran::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statussrctran::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatussrctranSpec;
impl crate::RegisterSpec for StatussrctranSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statussrctran::R`](R) reader structure"]
impl crate::Readable for StatussrctranSpec {}
#[doc = "`write(|w| ..)` method takes [`statussrctran::W`](W) writer structure"]
impl crate::Writable for StatussrctranSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUSSRCTRAN to value 0"]
impl crate::Resettable for StatussrctranSpec {}
