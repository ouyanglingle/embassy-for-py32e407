#[doc = "Register `DMA_ING_LEN` reader"]
pub type R = crate::R<DmaIngLenSpec>;
#[doc = "Register `DMA_ING_LEN` writer"]
pub type W = crate::W<DmaIngLenSpec>;
#[doc = "Field `INGRESS_DMA_DATA_LENGTH` reader - "]
pub type IngressDmaDataLengthR = crate::FieldReader<u32>;
#[doc = "Field `INGRESS_DMA_DATA_LENGTH` writer - "]
pub type IngressDmaDataLengthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ingress_dma_data_length(&self) -> IngressDmaDataLengthR {
        IngressDmaDataLengthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ingress_dma_data_length(&mut self) -> IngressDmaDataLengthW<'_, DmaIngLenSpec> {
        IngressDmaDataLengthW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ing_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ing_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIngLenSpec;
impl crate::RegisterSpec for DmaIngLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ing_len::R`](R) reader structure"]
impl crate::Readable for DmaIngLenSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_ing_len::W`](W) writer structure"]
impl crate::Writable for DmaIngLenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_ING_LEN to value 0"]
impl crate::Resettable for DmaIngLenSpec {}
