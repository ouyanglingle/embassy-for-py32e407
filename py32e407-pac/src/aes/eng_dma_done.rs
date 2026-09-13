#[doc = "Register `ENG_DMA_DONE` reader"]
pub type R = crate::R<EngDmaDoneSpec>;
#[doc = "Register `ENG_DMA_DONE` writer"]
pub type W = crate::W<EngDmaDoneSpec>;
#[doc = "Field `DMA_NGRESS_DONE` reader - "]
pub type DmaNgressDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_ngress_done(&self) -> DmaNgressDoneR {
        DmaNgressDoneR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eng_dma_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eng_dma_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EngDmaDoneSpec;
impl crate::RegisterSpec for EngDmaDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eng_dma_done::R`](R) reader structure"]
impl crate::Readable for EngDmaDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`eng_dma_done::W`](W) writer structure"]
impl crate::Writable for EngDmaDoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENG_DMA_DONE to value 0"]
impl crate::Resettable for EngDmaDoneSpec {}
