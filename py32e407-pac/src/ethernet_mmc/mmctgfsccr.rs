#[doc = "Register `MMCTGFSCCR` reader"]
pub type R = crate::R<MmctgfsccrSpec>;
#[doc = "Register `MMCTGFSCCR` writer"]
pub type W = crate::W<MmctgfsccrSpec>;
#[doc = "Field `TGFSCC` reader - Transmitted good frames after a single collision counter"]
pub type TgfsccR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames after a single collision counter"]
    #[inline(always)]
    pub fn tgfscc(&self) -> TgfsccR {
        TgfsccR::new(self.bits)
    }
}
impl W {}
#[doc = "Ethernet MMC transmitted good frames after a single collision counter\n\nYou can [`read`](crate::Reg::read) this register and get [`mmctgfsccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmctgfsccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmctgfsccrSpec;
impl crate::RegisterSpec for MmctgfsccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctgfsccr::R`](R) reader structure"]
impl crate::Readable for MmctgfsccrSpec {}
#[doc = "`write(|w| ..)` method takes [`mmctgfsccr::W`](W) writer structure"]
impl crate::Writable for MmctgfsccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMCTGFSCCR to value 0"]
impl crate::Resettable for MmctgfsccrSpec {}
