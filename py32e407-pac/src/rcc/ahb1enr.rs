#[doc = "Register `AHB1ENR` reader"]
pub type R = crate::R<Ahb1enrSpec>;
#[doc = "Register `AHB1ENR` writer"]
pub type W = crate::W<Ahb1enrSpec>;
#[doc = "Field `DMA1EN` reader - "]
pub type Dma1enR = crate::BitReader;
#[doc = "Field `DMA1EN` writer - "]
pub type Dma1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2EN` reader - "]
pub type Dma2enR = crate::BitReader;
#[doc = "Field `DMA2EN` writer - "]
pub type Dma2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMEN` reader - "]
pub type SramenR = crate::BitReader;
#[doc = "Field `SRAMEN` writer - "]
pub type SramenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCEN` reader - "]
pub type FmcenR = crate::BitReader;
#[doc = "Field `FMCEN` writer - "]
pub type FmcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORDICEN` reader - "]
pub type CordicenR = crate::BitReader;
#[doc = "Field `CORDICEN` writer - "]
pub type CordicenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - "]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - "]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESMCEN` reader - "]
pub type EsmcenR = crate::BitReader;
#[doc = "Field `ESMCEN` writer - "]
pub type EsmcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma1en(&self) -> Dma1enR {
        Dma1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma2en(&self) -> Dma2enR {
        Dma2enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sramen(&self) -> SramenR {
        SramenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fmcen(&self) -> FmcenR {
        FmcenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cordicen(&self) -> CordicenR {
        CordicenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn esmcen(&self) -> EsmcenR {
        EsmcenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> Dma1enW<'_, Ahb1enrSpec> {
        Dma1enW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> Dma2enW<'_, Ahb1enrSpec> {
        Dma2enW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sramen(&mut self) -> SramenW<'_, Ahb1enrSpec> {
        SramenW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FmcenW<'_, Ahb1enrSpec> {
        FmcenW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cordicen(&mut self) -> CordicenW<'_, Ahb1enrSpec> {
        CordicenW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, Ahb1enrSpec> {
        CrcenW::new(self, 6)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn esmcen(&mut self) -> EsmcenW<'_, Ahb1enrSpec> {
        EsmcenW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1enrSpec;
impl crate::RegisterSpec for Ahb1enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1enr::R`](R) reader structure"]
impl crate::Readable for Ahb1enrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure"]
impl crate::Writable for Ahb1enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB1ENR to value 0"]
impl crate::Resettable for Ahb1enrSpec {}
