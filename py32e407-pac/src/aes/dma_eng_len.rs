#[doc = "Register `DMA_ENG_LEN` reader"]
pub type R = crate::R<DmaEngLenSpec>;
#[doc = "Register `DMA_ENG_LEN` writer"]
pub type W = crate::W<DmaEngLenSpec>;
#[doc = "Field `ENGRESS_DMA_DATA_LENGTH` reader - "]
pub type EngressDmaDataLengthR = crate::FieldReader<u32>;
#[doc = "Field `ENGRESS_DMA_DATA_LENGTH` writer - "]
pub type EngressDmaDataLengthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn engress_dma_data_length(&self) -> EngressDmaDataLengthR {
        EngressDmaDataLengthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn engress_dma_data_length(&mut self) -> EngressDmaDataLengthW<'_, DmaEngLenSpec> {
        EngressDmaDataLengthW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_eng_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_eng_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaEngLenSpec;
impl crate::RegisterSpec for DmaEngLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_eng_len::R`](R) reader structure"]
impl crate::Readable for DmaEngLenSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_eng_len::W`](W) writer structure"]
impl crate::Writable for DmaEngLenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_ENG_LEN to value 0"]
impl crate::Resettable for DmaEngLenSpec {}
