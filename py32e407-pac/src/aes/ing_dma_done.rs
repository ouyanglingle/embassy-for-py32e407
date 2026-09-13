#[doc = "Register `ING_DMA_DONE` reader"]
pub type R = crate::R<IngDmaDoneSpec>;
#[doc = "Register `ING_DMA_DONE` writer"]
pub type W = crate::W<IngDmaDoneSpec>;
#[doc = "Field `DMA_INGRESS_DONE` reader - "]
pub type DmaIngressDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_ingress_done(&self) -> DmaIngressDoneR {
        DmaIngressDoneR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ing_dma_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ing_dma_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IngDmaDoneSpec;
impl crate::RegisterSpec for IngDmaDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ing_dma_done::R`](R) reader structure"]
impl crate::Readable for IngDmaDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`ing_dma_done::W`](W) writer structure"]
impl crate::Writable for IngDmaDoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ING_DMA_DONE to value 0"]
impl crate::Resettable for IngDmaDoneSpec {}
