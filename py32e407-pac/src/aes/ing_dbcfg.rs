#[doc = "Register `ING_DBCFG` reader"]
pub type R = crate::R<IngDbcfgSpec>;
#[doc = "Register `ING_DBCFG` writer"]
pub type W = crate::W<IngDbcfgSpec>;
#[doc = "Field `DST_TR_WIDTH` reader - "]
pub type DstTrWidthR = crate::FieldReader;
#[doc = "Field `DST_TR_WIDTH` writer - "]
pub type DstTrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DST_MSIZE` reader - "]
pub type DstMsizeR = crate::FieldReader;
#[doc = "Field `DST_MSIZE` writer - "]
pub type DstMsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DMA_EN` reader - "]
pub type DmaEnR = crate::BitReader;
#[doc = "Field `DMA_EN` writer - "]
pub type DmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLAINTEXT_REQ` reader - "]
pub type PlaintextReqR = crate::BitReader;
#[doc = "Field `PLAINTEXT_REQ` writer - "]
pub type PlaintextReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dst_tr_width(&self) -> DstTrWidthR {
        DstTrWidthR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn dst_msize(&self) -> DstMsizeR {
        DstMsizeR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn plaintext_req(&self) -> PlaintextReqR {
        PlaintextReqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dst_tr_width(&mut self) -> DstTrWidthW<'_, IngDbcfgSpec> {
        DstTrWidthW::new(self, 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn dst_msize(&mut self) -> DstMsizeW<'_, IngDbcfgSpec> {
        DstMsizeW::new(self, 3)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DmaEnW<'_, IngDbcfgSpec> {
        DmaEnW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn plaintext_req(&mut self) -> PlaintextReqW<'_, IngDbcfgSpec> {
        PlaintextReqW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ing_dbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ing_dbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IngDbcfgSpec;
impl crate::RegisterSpec for IngDbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ing_dbcfg::R`](R) reader structure"]
impl crate::Readable for IngDbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ing_dbcfg::W`](W) writer structure"]
impl crate::Writable for IngDbcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ING_DBCFG to value 0"]
impl crate::Resettable for IngDbcfgSpec {}
