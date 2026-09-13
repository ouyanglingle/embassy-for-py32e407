#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `CMPMCF` reader - Compare match clear flag"]
pub type CmpmcfR = crate::BitReader;
#[doc = "Field `CMPMCF` writer - Compare match clear flag"]
pub type CmpmcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRMCF` reader - Autoreload match clear flag"]
pub type ArrmcfR = crate::BitReader;
#[doc = "Field `ARRMCF` writer - Autoreload match clear flag"]
pub type ArrmcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIGCF` reader - External trigger valid edge clear flag"]
pub type ExttrigcfR = crate::BitReader;
#[doc = "Field `EXTTRIGCF` writer - External trigger valid edge clear flag"]
pub type ExttrigcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPOKCF` reader - Compare register update OK clear flag"]
pub type CmpokcfR = crate::BitReader;
#[doc = "Field `CMPOKCF` writer - Compare register update OK clear flag"]
pub type CmpokcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARROKCF` reader - Autoreload register update OK clear flag"]
pub type ArrokcfR = crate::BitReader;
#[doc = "Field `ARROKCF` writer - Autoreload register update OK clear flag"]
pub type ArrokcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPCF` reader - Direction change to UP clear flag"]
pub type UpcfR = crate::BitReader;
#[doc = "Field `UPCF` writer - Direction change to UP clear flag"]
pub type UpcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNCF` reader - Direction change to down clear flag"]
pub type DowncfR = crate::BitReader;
#[doc = "Field `DOWNCF` writer - Direction change to down clear flag"]
pub type DowncfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Compare match clear flag"]
    #[inline(always)]
    pub fn cmpmcf(&self) -> CmpmcfR {
        CmpmcfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match clear flag"]
    #[inline(always)]
    pub fn arrmcf(&self) -> ArrmcfR {
        ArrmcfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger valid edge clear flag"]
    #[inline(always)]
    pub fn exttrigcf(&self) -> ExttrigcfR {
        ExttrigcfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare register update OK clear flag"]
    #[inline(always)]
    pub fn cmpokcf(&self) -> CmpokcfR {
        CmpokcfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK clear flag"]
    #[inline(always)]
    pub fn arrokcf(&self) -> ArrokcfR {
        ArrokcfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direction change to UP clear flag"]
    #[inline(always)]
    pub fn upcf(&self) -> UpcfR {
        UpcfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Direction change to down clear flag"]
    #[inline(always)]
    pub fn downcf(&self) -> DowncfR {
        DowncfR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare match clear flag"]
    #[inline(always)]
    pub fn cmpmcf(&mut self) -> CmpmcfW<'_, IcrSpec> {
        CmpmcfW::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match clear flag"]
    #[inline(always)]
    pub fn arrmcf(&mut self) -> ArrmcfW<'_, IcrSpec> {
        ArrmcfW::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge clear flag"]
    #[inline(always)]
    pub fn exttrigcf(&mut self) -> ExttrigcfW<'_, IcrSpec> {
        ExttrigcfW::new(self, 2)
    }
    #[doc = "Bit 3 - Compare register update OK clear flag"]
    #[inline(always)]
    pub fn cmpokcf(&mut self) -> CmpokcfW<'_, IcrSpec> {
        CmpokcfW::new(self, 3)
    }
    #[doc = "Bit 4 - Autoreload register update OK clear flag"]
    #[inline(always)]
    pub fn arrokcf(&mut self) -> ArrokcfW<'_, IcrSpec> {
        ArrokcfW::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP clear flag"]
    #[inline(always)]
    pub fn upcf(&mut self) -> UpcfW<'_, IcrSpec> {
        UpcfW::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down clear flag"]
    #[inline(always)]
    pub fn downcf(&mut self) -> DowncfW<'_, IcrSpec> {
        DowncfW::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
