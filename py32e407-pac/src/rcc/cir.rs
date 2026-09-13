#[doc = "Register `CIR` reader"]
pub type R = crate::R<CirSpec>;
#[doc = "Register `CIR` writer"]
pub type W = crate::W<CirSpec>;
#[doc = "Field `LSIRDYF` reader - "]
pub type LsirdyfR = crate::BitReader;
#[doc = "Field `LSERDYF` reader - "]
pub type LserdyfR = crate::BitReader;
#[doc = "Field `HSIRDYF` reader - "]
pub type HsirdyfR = crate::BitReader;
#[doc = "Field `HSERDYF` reader - "]
pub type HserdyfR = crate::BitReader;
#[doc = "Field `PLLRDYF` reader - "]
pub type PllrdyfR = crate::BitReader;
#[doc = "Field `HSI48RDYF` reader - "]
pub type Hsi48rdyfR = crate::BitReader;
#[doc = "Field `CSSF` reader - "]
pub type CssfR = crate::BitReader;
#[doc = "Field `LSIRDYIE` reader - "]
pub type LsirdyieR = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - "]
pub type LsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYIE` reader - "]
pub type LserdyieR = crate::BitReader;
#[doc = "Field `LSERDYIE` writer - "]
pub type LserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYIE` reader - "]
pub type HsirdyieR = crate::BitReader;
#[doc = "Field `HSIRDYIE` writer - "]
pub type HsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYIE` reader - "]
pub type HserdyieR = crate::BitReader;
#[doc = "Field `HSERDYIE` writer - "]
pub type HserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYIE` reader - "]
pub type PllrdyieR = crate::BitReader;
#[doc = "Field `PLLRDYIE` writer - "]
pub type PllrdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSI48RDYIE` reader - "]
pub type Hsi48rdyieR = crate::BitReader;
#[doc = "Field `HSI48RDYIE` writer - "]
pub type Hsi48rdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDYC` reader - "]
pub type LsirdycR = crate::BitReader;
#[doc = "Field `LSIRDYC` writer - "]
pub type LsirdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYC` reader - "]
pub type LserdycR = crate::BitReader;
#[doc = "Field `LSERDYC` writer - "]
pub type LserdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYC` reader - "]
pub type HsirdycR = crate::BitReader;
#[doc = "Field `HSIRDYC` writer - "]
pub type HsirdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYC` reader - "]
pub type HserdycR = crate::BitReader;
#[doc = "Field `HSERDYC` writer - "]
pub type HserdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYC` reader - "]
pub type PllrdycR = crate::BitReader;
#[doc = "Field `PLLRDYC` writer - "]
pub type PllrdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSI48RDYC` reader - "]
pub type Hsi48rdycR = crate::BitReader;
#[doc = "Field `HSI48RDYC` writer - "]
pub type Hsi48rdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSC` reader - "]
pub type CsscR = crate::BitReader;
#[doc = "Field `CSSC` writer - "]
pub type CsscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LsirdyfR {
        LsirdyfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LserdyfR {
        LserdyfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HsirdyfR {
        HsirdyfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HserdyfR {
        HserdyfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PllrdyfR {
        PllrdyfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> Hsi48rdyfR {
        Hsi48rdyfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cssf(&self) -> CssfR {
        CssfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LsirdyieR {
        LsirdyieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LserdyieR {
        LserdyieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HsirdyieR {
        HsirdyieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HserdyieR {
        HserdyieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PllrdyieR {
        PllrdyieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> Hsi48rdyieR {
        Hsi48rdyieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lsirdyc(&self) -> LsirdycR {
        LsirdycR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lserdyc(&self) -> LserdycR {
        LserdycR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn hsirdyc(&self) -> HsirdycR {
        HsirdycR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn hserdyc(&self) -> HserdycR {
        HserdycR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pllrdyc(&self) -> PllrdycR {
        PllrdycR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn hsi48rdyc(&self) -> Hsi48rdycR {
        Hsi48rdycR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cssc(&self) -> CsscR {
        CsscR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LsirdyieW<'_, CirSpec> {
        LsirdyieW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LserdyieW<'_, CirSpec> {
        LserdyieW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HsirdyieW<'_, CirSpec> {
        HsirdyieW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HserdyieW<'_, CirSpec> {
        HserdyieW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PllrdyieW<'_, CirSpec> {
        PllrdyieW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hsi48rdyie(&mut self) -> Hsi48rdyieW<'_, CirSpec> {
        Hsi48rdyieW::new(self, 13)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LsirdycW<'_, CirSpec> {
        LsirdycW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LserdycW<'_, CirSpec> {
        LserdycW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HsirdycW<'_, CirSpec> {
        HsirdycW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HserdycW<'_, CirSpec> {
        HserdycW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PllrdycW<'_, CirSpec> {
        PllrdycW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn hsi48rdyc(&mut self) -> Hsi48rdycW<'_, CirSpec> {
        Hsi48rdycW::new(self, 21)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cssc(&mut self) -> CsscW<'_, CirSpec> {
        CsscW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CirSpec;
impl crate::RegisterSpec for CirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir::R`](R) reader structure"]
impl crate::Readable for CirSpec {}
#[doc = "`write(|w| ..)` method takes [`cir::W`](W) writer structure"]
impl crate::Writable for CirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIR to value 0"]
impl crate::Resettable for CirSpec {}
