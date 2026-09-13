#[doc = "Register `MMCTGFMSCCR` reader"]
pub type R = crate::R<MmctgfmsccrSpec>;
#[doc = "Register `MMCTGFMSCCR` writer"]
pub type W = crate::W<MmctgfmsccrSpec>;
#[doc = "Field `TGFMSCC` reader - Transmitted good frames after more than a single collision counter"]
pub type TgfmsccR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames after more than a single collision counter"]
    #[inline(always)]
    pub fn tgfmscc(&self) -> TgfmsccR {
        TgfmsccR::new(self.bits)
    }
}
impl W {}
#[doc = "Ethernet MMC transmitted good frames after more than a single collision\n\nYou can [`read`](crate::Reg::read) this register and get [`mmctgfmsccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmctgfmsccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmctgfmsccrSpec;
impl crate::RegisterSpec for MmctgfmsccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctgfmsccr::R`](R) reader structure"]
impl crate::Readable for MmctgfmsccrSpec {}
#[doc = "`write(|w| ..)` method takes [`mmctgfmsccr::W`](W) writer structure"]
impl crate::Writable for MmctgfmsccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMCTGFMSCCR to value 0"]
impl crate::Resettable for MmctgfmsccrSpec {}
