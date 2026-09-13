#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `HSION` reader - "]
pub type HsionR = crate::BitReader;
#[doc = "Field `HSION` writer - "]
pub type HsionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDY` reader - "]
pub type HsirdyR = crate::BitReader;
#[doc = "Field `HSIKERON` reader - HSIKERON"]
pub type HsikeronR = crate::BitReader;
#[doc = "Field `HSIKERON` writer - HSIKERON"]
pub type HsikeronW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSITRIM` reader - "]
pub type HsitrimR = crate::FieldReader;
#[doc = "Field `HSITRIM` writer - "]
pub type HsitrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HSICAL_L` reader - "]
pub type HsicalLR = crate::FieldReader;
#[doc = "Field `HSEON` reader - "]
pub type HseonR = crate::BitReader;
#[doc = "Field `HSEON` writer - "]
pub type HseonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - "]
pub type HserdyR = crate::BitReader;
#[doc = "Field `HSEBYP` reader - "]
pub type HsebypR = crate::BitReader;
#[doc = "Field `HSEBYP` writer - "]
pub type HsebypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSON` reader - "]
pub type CssonR = crate::BitReader;
#[doc = "Field `CSSON` writer - "]
pub type CssonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSICAL_H` reader - "]
pub type HsicalHR = crate::FieldReader;
#[doc = "Field `PLLON` reader - "]
pub type PllonR = crate::BitReader;
#[doc = "Field `PLLON` writer - "]
pub type PllonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDY` reader - "]
pub type PllrdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hsion(&self) -> HsionR {
        HsionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HsirdyR {
        HsirdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSIKERON"]
    #[inline(always)]
    pub fn hsikeron(&self) -> HsikeronR {
        HsikeronR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HsitrimR {
        HsitrimR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn hsical_l(&self) -> HsicalLR {
        HsicalLR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn hseon(&self) -> HseonR {
        HseonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hserdy(&self) -> HserdyR {
        HserdyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HsebypR {
        HsebypR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn csson(&self) -> CssonR {
        CssonR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn hsical_h(&self) -> HsicalHR {
        HsicalHR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pllon(&self) -> PllonR {
        PllonR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PllrdyR {
        PllrdyR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hsion(&mut self) -> HsionW<'_, CrSpec> {
        HsionW::new(self, 0)
    }
    #[doc = "Bit 2 - HSIKERON"]
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HsikeronW<'_, CrSpec> {
        HsikeronW::new(self, 2)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HsitrimW<'_, CrSpec> {
        HsitrimW::new(self, 3)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn hseon(&mut self) -> HseonW<'_, CrSpec> {
        HseonW::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HsebypW<'_, CrSpec> {
        HsebypW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn csson(&mut self) -> CssonW<'_, CrSpec> {
        CssonW::new(self, 19)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pllon(&mut self) -> PllonW<'_, CrSpec> {
        PllonW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
