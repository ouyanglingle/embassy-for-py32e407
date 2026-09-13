#[doc = "Register `ENG_DBCFG` reader"]
pub type R = crate::R<EngDbcfgSpec>;
#[doc = "Register `ENG_DBCFG` writer"]
pub type W = crate::W<EngDbcfgSpec>;
#[doc = "Field `SRC_TR_WIDTH` reader - "]
pub type SrcTrWidthR = crate::FieldReader;
#[doc = "Field `SRC_TR_WIDTH` writer - "]
pub type SrcTrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRC_MSIZE` reader - "]
pub type SrcMsizeR = crate::FieldReader;
#[doc = "Field `SRC_MSIZE` writer - "]
pub type SrcMsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DMA_EN` reader - "]
pub type DmaEnR = crate::BitReader;
#[doc = "Field `DMA_EN` writer - "]
pub type DmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIPERTEXT_REQ` reader - "]
pub type CipertextReqR = crate::BitReader;
#[doc = "Field `CIPERTEXT_REQ` writer - "]
pub type CipertextReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn src_tr_width(&self) -> SrcTrWidthR {
        SrcTrWidthR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn src_msize(&self) -> SrcMsizeR {
        SrcMsizeR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cipertext_req(&self) -> CipertextReqR {
        CipertextReqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn src_tr_width(&mut self) -> SrcTrWidthW<'_, EngDbcfgSpec> {
        SrcTrWidthW::new(self, 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn src_msize(&mut self) -> SrcMsizeW<'_, EngDbcfgSpec> {
        SrcMsizeW::new(self, 3)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DmaEnW<'_, EngDbcfgSpec> {
        DmaEnW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cipertext_req(&mut self) -> CipertextReqW<'_, EngDbcfgSpec> {
        CipertextReqW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eng_dbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eng_dbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EngDbcfgSpec;
impl crate::RegisterSpec for EngDbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eng_dbcfg::R`](R) reader structure"]
impl crate::Readable for EngDbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`eng_dbcfg::W`](W) writer structure"]
impl crate::Writable for EngDbcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENG_DBCFG to value 0"]
impl crate::Resettable for EngDbcfgSpec {}
