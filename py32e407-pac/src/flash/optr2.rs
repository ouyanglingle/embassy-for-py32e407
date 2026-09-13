#[doc = "Register `OPTR2` reader"]
pub type R = crate::R<Optr2Spec>;
#[doc = "Register `OPTR2` writer"]
pub type W = crate::W<Optr2Spec>;
#[doc = "Field `NBOOT1` reader - "]
pub type Nboot1R = crate::BitReader;
#[doc = "Field `NBOOT1` writer - "]
pub type Nboot1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRST_MODE` reader - "]
pub type NrstModeR = crate::FieldReader;
#[doc = "Field `NRST_MODE` writer - "]
pub type NrstModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SRAM_PE` reader - "]
pub type SramPeR = crate::BitReader;
#[doc = "Field `SRAM_PE` writer - "]
pub type SramPeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCMSRAM_RST` reader - "]
pub type CcmsramRstR = crate::BitReader;
#[doc = "Field `CCMSRAM_RST` writer - "]
pub type CcmsramRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSWBOOT0` reader - "]
pub type Nswboot0R = crate::BitReader;
#[doc = "Field `NSWBOOT0` writer - "]
pub type Nswboot0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBOOT0` reader - "]
pub type Nboot0R = crate::BitReader;
#[doc = "Field `NBOOT0` writer - "]
pub type Nboot0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_LOCK` reader - "]
pub type BootLockR = crate::BitReader;
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn nboot1(&self) -> Nboot1R {
        Nboot1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn nrst_mode(&self) -> NrstModeR {
        NrstModeR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sram_pe(&self) -> SramPeR {
        SramPeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ccmsram_rst(&self) -> CcmsramRstR {
        CcmsramRstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nswboot0(&self) -> Nswboot0R {
        Nswboot0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nboot0(&self) -> Nboot0R {
        Nboot0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn boot_lock(&self) -> BootLockR {
        BootLockR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn nboot1(&mut self) -> Nboot1W<'_, Optr2Spec> {
        Nboot1W::new(self, 8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn nrst_mode(&mut self) -> NrstModeW<'_, Optr2Spec> {
        NrstModeW::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sram_pe(&mut self) -> SramPeW<'_, Optr2Spec> {
        SramPeW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ccmsram_rst(&mut self) -> CcmsramRstW<'_, Optr2Spec> {
        CcmsramRstW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nswboot0(&mut self) -> Nswboot0W<'_, Optr2Spec> {
        Nswboot0W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nboot0(&mut self) -> Nboot0W<'_, Optr2Spec> {
        Nboot0W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`optr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Optr2Spec;
impl crate::RegisterSpec for Optr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optr2::R`](R) reader structure"]
impl crate::Readable for Optr2Spec {}
#[doc = "`write(|w| ..)` method takes [`optr2::W`](W) writer structure"]
impl crate::Writable for Optr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPTR2 to value 0"]
impl crate::Resettable for Optr2Spec {}
