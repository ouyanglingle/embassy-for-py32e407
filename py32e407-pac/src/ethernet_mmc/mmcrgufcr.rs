#[doc = "Register `MMCRGUFCR` reader"]
pub type R = crate::R<MmcrgufcrSpec>;
#[doc = "Register `MMCRGUFCR` writer"]
pub type W = crate::W<MmcrgufcrSpec>;
#[doc = "Field `RGUFC` reader - Received good unicast frames counter"]
pub type RgufcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received good unicast frames counter"]
    #[inline(always)]
    pub fn rgufc(&self) -> RgufcR {
        RgufcR::new(self.bits)
    }
}
impl W {}
#[doc = "MMC received good unicast frames counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmcrgufcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmcrgufcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcrgufcrSpec;
impl crate::RegisterSpec for MmcrgufcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrgufcr::R`](R) reader structure"]
impl crate::Readable for MmcrgufcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mmcrgufcr::W`](W) writer structure"]
impl crate::Writable for MmcrgufcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMCRGUFCR to value 0"]
impl crate::Resettable for MmcrgufcrSpec {}
