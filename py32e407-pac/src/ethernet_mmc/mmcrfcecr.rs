#[doc = "Register `MMCRFCECR` reader"]
pub type R = crate::R<MmcrfcecrSpec>;
#[doc = "Register `MMCRFCECR` writer"]
pub type W = crate::W<MmcrfcecrSpec>;
#[doc = "Field `RFCFC` reader - Received frames with CRC error counter"]
pub type RfcfcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received frames with CRC error counter"]
    #[inline(always)]
    pub fn rfcfc(&self) -> RfcfcR {
        RfcfcR::new(self.bits)
    }
}
impl W {}
#[doc = "Ethernet MMC received frames with CRC error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmcrfcecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmcrfcecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcrfcecrSpec;
impl crate::RegisterSpec for MmcrfcecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrfcecr::R`](R) reader structure"]
impl crate::Readable for MmcrfcecrSpec {}
#[doc = "`write(|w| ..)` method takes [`mmcrfcecr::W`](W) writer structure"]
impl crate::Writable for MmcrfcecrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMCRFCECR to value 0"]
impl crate::Resettable for MmcrfcecrSpec {}
