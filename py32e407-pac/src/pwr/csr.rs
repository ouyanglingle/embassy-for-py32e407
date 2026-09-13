#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `CWUF1` reader - "]
pub type Cwuf1R = crate::BitReader;
#[doc = "Field `CWUF1` writer - "]
pub type Cwuf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF2` reader - "]
pub type Cwuf2R = crate::BitReader;
#[doc = "Field `CWUF2` writer - "]
pub type Cwuf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF3` reader - "]
pub type Cwuf3R = crate::BitReader;
#[doc = "Field `CWUF3` writer - "]
pub type Cwuf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF4` reader - "]
pub type Cwuf4R = crate::BitReader;
#[doc = "Field `CWUF4` writer - "]
pub type Cwuf4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF5` reader - "]
pub type Cwuf5R = crate::BitReader;
#[doc = "Field `CWUF5` writer - "]
pub type Cwuf5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSBF` reader - "]
pub type CsbfR = crate::BitReader;
#[doc = "Field `CSBF` writer - "]
pub type CsbfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cwuf1(&self) -> Cwuf1R {
        Cwuf1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cwuf2(&self) -> Cwuf2R {
        Cwuf2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cwuf3(&self) -> Cwuf3R {
        Cwuf3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cwuf4(&self) -> Cwuf4R {
        Cwuf4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cwuf5(&self) -> Cwuf5R {
        Cwuf5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn csbf(&self) -> CsbfR {
        CsbfR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cwuf1(&mut self) -> Cwuf1W<'_, CsrSpec> {
        Cwuf1W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cwuf2(&mut self) -> Cwuf2W<'_, CsrSpec> {
        Cwuf2W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cwuf3(&mut self) -> Cwuf3W<'_, CsrSpec> {
        Cwuf3W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cwuf4(&mut self) -> Cwuf4W<'_, CsrSpec> {
        Cwuf4W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cwuf5(&mut self) -> Cwuf5W<'_, CsrSpec> {
        Cwuf5W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn csbf(&mut self) -> CsbfW<'_, CsrSpec> {
        CsbfW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
