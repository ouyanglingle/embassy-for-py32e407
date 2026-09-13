#[doc = "Register `AHB1RSTR` reader"]
pub type R = crate::R<Ahb1rstrSpec>;
#[doc = "Register `AHB1RSTR` writer"]
pub type W = crate::W<Ahb1rstrSpec>;
#[doc = "Field `DMA1RST` reader - "]
pub type Dma1rstR = crate::BitReader;
#[doc = "Field `DMA1RST` writer - "]
pub type Dma1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2RST` reader - "]
pub type Dma2rstR = crate::BitReader;
#[doc = "Field `DMA2RST` writer - "]
pub type Dma2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORDICRST` reader - "]
pub type CordicrstR = crate::BitReader;
#[doc = "Field `CORDICRST` writer - "]
pub type CordicrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRST` reader - "]
pub type CrcrstR = crate::BitReader;
#[doc = "Field `CRCRST` writer - "]
pub type CrcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESMCRST` reader - "]
pub type EsmcrstR = crate::BitReader;
#[doc = "Field `ESMCRST` writer - "]
pub type EsmcrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma1rst(&self) -> Dma1rstR {
        Dma1rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma2rst(&self) -> Dma2rstR {
        Dma2rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cordicrst(&self) -> CordicrstR {
        CordicrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn crcrst(&self) -> CrcrstR {
        CrcrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn esmcrst(&self) -> EsmcrstR {
        EsmcrstR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma1rst(&mut self) -> Dma1rstW<'_, Ahb1rstrSpec> {
        Dma1rstW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma2rst(&mut self) -> Dma2rstW<'_, Ahb1rstrSpec> {
        Dma2rstW::new(self, 1)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cordicrst(&mut self) -> CordicrstW<'_, Ahb1rstrSpec> {
        CordicrstW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn crcrst(&mut self) -> CrcrstW<'_, Ahb1rstrSpec> {
        CrcrstW::new(self, 6)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn esmcrst(&mut self) -> EsmcrstW<'_, Ahb1rstrSpec> {
        EsmcrstW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1rstrSpec;
impl crate::RegisterSpec for Ahb1rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rstr::R`](R) reader structure"]
impl crate::Readable for Ahb1rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1rstr::W`](W) writer structure"]
impl crate::Writable for Ahb1rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB1RSTR to value 0"]
impl crate::Resettable for Ahb1rstrSpec {}
