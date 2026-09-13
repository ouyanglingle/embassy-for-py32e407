#[doc = "Register `CFGR4` reader"]
pub type R = crate::R<Cfgr4Spec>;
#[doc = "Register `CFGR4` writer"]
pub type W = crate::W<Cfgr4Spec>;
#[doc = "Field `DMA1_MAP5` reader - DMA1 ch5 mapping"]
pub type Dma1Map5R = crate::FieldReader;
#[doc = "Field `DMA1_MAP5` writer - DMA1 ch5 mapping"]
pub type Dma1Map5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DMA1_MAP6` reader - DMA1 ch6 mapping"]
pub type Dma1Map6R = crate::FieldReader;
#[doc = "Field `DMA1_MAP6` writer - DMA1 ch6 mapping"]
pub type Dma1Map6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DMA2_MAP1` reader - DMA2 ch1 mapping"]
pub type Dma2Map1R = crate::FieldReader;
#[doc = "Field `DMA2_MAP1` writer - DMA2 ch1 mapping"]
pub type Dma2Map1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DMA2_MAP2` reader - DMA2 ch2 mapping"]
pub type Dma2Map2R = crate::FieldReader;
#[doc = "Field `DMA2_MAP2` writer - DMA2 ch2 mapping"]
pub type Dma2Map2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - DMA1 ch5 mapping"]
    #[inline(always)]
    pub fn dma1_map5(&self) -> Dma1Map5R {
        Dma1Map5R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - DMA1 ch6 mapping"]
    #[inline(always)]
    pub fn dma1_map6(&self) -> Dma1Map6R {
        Dma1Map6R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - DMA2 ch1 mapping"]
    #[inline(always)]
    pub fn dma2_map1(&self) -> Dma2Map1R {
        Dma2Map1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - DMA2 ch2 mapping"]
    #[inline(always)]
    pub fn dma2_map2(&self) -> Dma2Map2R {
        Dma2Map2R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA1 ch5 mapping"]
    #[inline(always)]
    pub fn dma1_map5(&mut self) -> Dma1Map5W<'_, Cfgr4Spec> {
        Dma1Map5W::new(self, 0)
    }
    #[doc = "Bits 8:14 - DMA1 ch6 mapping"]
    #[inline(always)]
    pub fn dma1_map6(&mut self) -> Dma1Map6W<'_, Cfgr4Spec> {
        Dma1Map6W::new(self, 8)
    }
    #[doc = "Bits 16:22 - DMA2 ch1 mapping"]
    #[inline(always)]
    pub fn dma2_map1(&mut self) -> Dma2Map1W<'_, Cfgr4Spec> {
        Dma2Map1W::new(self, 16)
    }
    #[doc = "Bits 24:30 - DMA2 ch2 mapping"]
    #[inline(always)]
    pub fn dma2_map2(&mut self) -> Dma2Map2W<'_, Cfgr4Spec> {
        Dma2Map2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr4Spec;
impl crate::RegisterSpec for Cfgr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr4::R`](R) reader structure"]
impl crate::Readable for Cfgr4Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr4::W`](W) writer structure"]
impl crate::Writable for Cfgr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR4 to value 0"]
impl crate::Resettable for Cfgr4Spec {}
