#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `CTE` reader - clear TAMP event"]
pub type CteR = crate::BitReader;
#[doc = "Field `CTE` writer - clear TAMP event"]
pub type CteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTI` reader - clear TAMP interrupt"]
pub type CtiR = crate::BitReader;
#[doc = "Field `CTI` writer - clear TAMP interrupt"]
pub type CtiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPIE` reader - TAMP interrupt enable"]
pub type TpieR = crate::BitReader;
#[doc = "Field `TPIE` writer - TAMP interrupt enable"]
pub type TpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEF` reader - TAMP event flag"]
pub type TefR = crate::BitReader;
#[doc = "Field `TIF` reader - TAMP interrupt flag"]
pub type TifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - clear TAMP event"]
    #[inline(always)]
    pub fn cte(&self) -> CteR {
        CteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clear TAMP interrupt"]
    #[inline(always)]
    pub fn cti(&self) -> CtiR {
        CtiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP interrupt enable"]
    #[inline(always)]
    pub fn tpie(&self) -> TpieR {
        TpieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - TAMP event flag"]
    #[inline(always)]
    pub fn tef(&self) -> TefR {
        TefR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TAMP interrupt flag"]
    #[inline(always)]
    pub fn tif(&self) -> TifR {
        TifR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clear TAMP event"]
    #[inline(always)]
    pub fn cte(&mut self) -> CteW<'_, CsrSpec> {
        CteW::new(self, 0)
    }
    #[doc = "Bit 1 - clear TAMP interrupt"]
    #[inline(always)]
    pub fn cti(&mut self) -> CtiW<'_, CsrSpec> {
        CtiW::new(self, 1)
    }
    #[doc = "Bit 2 - TAMP interrupt enable"]
    #[inline(always)]
    pub fn tpie(&mut self) -> TpieW<'_, CsrSpec> {
        TpieW::new(self, 2)
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
