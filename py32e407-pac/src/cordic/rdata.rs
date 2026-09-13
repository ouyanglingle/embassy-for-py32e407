#[doc = "Register `RDATA` reader"]
pub type R = crate::R<RdataSpec>;
#[doc = "Register `RDATA` writer"]
pub type W = crate::W<RdataSpec>;
#[doc = "Field `RES` reader - Function result"]
pub type ResR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Function result"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(self.bits)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdataSpec;
impl crate::RegisterSpec for RdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdata::R`](R) reader structure"]
impl crate::Readable for RdataSpec {}
#[doc = "`write(|w| ..)` method takes [`rdata::W`](W) writer structure"]
impl crate::Writable for RdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RDATA to value 0"]
impl crate::Resettable for RdataSpec {}
