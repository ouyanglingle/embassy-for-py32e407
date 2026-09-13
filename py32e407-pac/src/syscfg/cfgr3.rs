#[doc = "Register `CFGR3` reader"]
pub type R = crate::R<Cfgr3Spec>;
#[doc = "Register `CFGR3` writer"]
pub type W = crate::W<Cfgr3Spec>;
#[doc = "Field `DMA1_MAP1` reader - DMA1 ch1 mapping"]
pub type Dma1Map1R = crate::FieldReader;
#[doc = "Field `DMA1_MAP1` writer - DMA1 ch1 mapping"]
pub type Dma1Map1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DMA1_MAP2` reader - DMA1 ch2 mapping"]
pub type Dma1Map2R = crate::FieldReader;
#[doc = "Field `DMA1_MAP2` writer - DMA1 ch2 mapping"]
pub type Dma1Map2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DMA1_MAP3` reader - DMA1 ch3 mapping"]
pub type Dma1Map3R = crate::FieldReader;
#[doc = "Field `DMA1_MAP3` writer - DMA1 ch3 mapping"]
pub type Dma1Map3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DMA1_MAP4` reader - DMA1 ch4 mapping"]
pub type Dma1Map4R = crate::FieldReader;
#[doc = "Field `DMA1_MAP4` writer - DMA1 ch4 mapping"]
pub type Dma1Map4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - DMA1 ch1 mapping"]
    #[inline(always)]
    pub fn dma1_map1(&self) -> Dma1Map1R {
        Dma1Map1R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - DMA1 ch2 mapping"]
    #[inline(always)]
    pub fn dma1_map2(&self) -> Dma1Map2R {
        Dma1Map2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - DMA1 ch3 mapping"]
    #[inline(always)]
    pub fn dma1_map3(&self) -> Dma1Map3R {
        Dma1Map3R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - DMA1 ch4 mapping"]
    #[inline(always)]
    pub fn dma1_map4(&self) -> Dma1Map4R {
        Dma1Map4R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA1 ch1 mapping"]
    #[inline(always)]
    pub fn dma1_map1(&mut self) -> Dma1Map1W<'_, Cfgr3Spec> {
        Dma1Map1W::new(self, 0)
    }
    #[doc = "Bits 8:14 - DMA1 ch2 mapping"]
    #[inline(always)]
    pub fn dma1_map2(&mut self) -> Dma1Map2W<'_, Cfgr3Spec> {
        Dma1Map2W::new(self, 8)
    }
    #[doc = "Bits 16:22 - DMA1 ch3 mapping"]
    #[inline(always)]
    pub fn dma1_map3(&mut self) -> Dma1Map3W<'_, Cfgr3Spec> {
        Dma1Map3W::new(self, 16)
    }
    #[doc = "Bits 24:30 - DMA1 ch4 mapping"]
    #[inline(always)]
    pub fn dma1_map4(&mut self) -> Dma1Map4W<'_, Cfgr3Spec> {
        Dma1Map4W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr3Spec;
impl crate::RegisterSpec for Cfgr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr3::R`](R) reader structure"]
impl crate::Readable for Cfgr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure"]
impl crate::Writable for Cfgr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for Cfgr3Spec {}
