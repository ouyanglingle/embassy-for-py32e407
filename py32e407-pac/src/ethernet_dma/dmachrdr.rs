#[doc = "Register `DMACHRDR` reader"]
pub type R = crate::R<DmachrdrSpec>;
#[doc = "Register `DMACHRDR` writer"]
pub type W = crate::W<DmachrdrSpec>;
#[doc = "Field `HRDAP` reader - Host receive descriptor address pointer"]
pub type HrdapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host receive descriptor address pointer"]
    #[inline(always)]
    pub fn hrdap(&self) -> HrdapR {
        HrdapR::new(self.bits)
    }
}
impl W {}
#[doc = "Ethernet DMA current host receive descriptor register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmachrdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmachrdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmachrdrSpec;
impl crate::RegisterSpec for DmachrdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmachrdr::R`](R) reader structure"]
impl crate::Readable for DmachrdrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmachrdr::W`](W) writer structure"]
impl crate::Writable for DmachrdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACHRDR to value 0"]
impl crate::Resettable for DmachrdrSpec {}
