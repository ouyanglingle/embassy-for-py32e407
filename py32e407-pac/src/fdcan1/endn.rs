#[doc = "Register `ENDN` reader"]
pub type R = crate::R<EndnSpec>;
#[doc = "Register `ENDN` writer"]
pub type W = crate::W<EndnSpec>;
#[doc = "Field `ETV` reader - Endian Register"]
pub type EtvR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Endian Register"]
    #[inline(always)]
    pub fn etv(&self) -> EtvR {
        EtvR::new(self.bits)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`endn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EndnSpec;
impl crate::RegisterSpec for EndnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endn::R`](R) reader structure"]
impl crate::Readable for EndnSpec {}
#[doc = "`write(|w| ..)` method takes [`endn::W`](W) writer structure"]
impl crate::Writable for EndnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENDN to value 0"]
impl crate::Resettable for EndnSpec {}
