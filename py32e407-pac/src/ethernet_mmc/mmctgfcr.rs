#[doc = "Register `MMCTGFCR` reader"]
pub type R = crate::R<MmctgfcrSpec>;
#[doc = "Register `MMCTGFCR` writer"]
pub type W = crate::W<MmctgfcrSpec>;
#[doc = "Field `TGFC` reader - Transmitted good frames counter"]
pub type TgfcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames counter"]
    #[inline(always)]
    pub fn tgfc(&self) -> TgfcR {
        TgfcR::new(self.bits)
    }
}
impl W {}
#[doc = "Ethernet MMC transmitted good frames counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmctgfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmctgfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmctgfcrSpec;
impl crate::RegisterSpec for MmctgfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctgfcr::R`](R) reader structure"]
impl crate::Readable for MmctgfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mmctgfcr::W`](W) writer structure"]
impl crate::Writable for MmctgfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMCTGFCR to value 0"]
impl crate::Resettable for MmctgfcrSpec {}
