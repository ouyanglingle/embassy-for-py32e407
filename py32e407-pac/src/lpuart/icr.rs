#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `PECF` reader - Check value error flag reset"]
pub type PecfR = crate::BitReader;
#[doc = "Field `PECF` writer - Check value error flag reset"]
pub type PecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECF` reader - Frame error flag reset"]
pub type FecfR = crate::BitReader;
#[doc = "Field `FECF` writer - Frame error flag reset"]
pub type FecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NECF` reader - Clear the START bit noise flag"]
pub type NecfR = crate::BitReader;
#[doc = "Field `NECF` writer - Clear the START bit noise flag"]
pub type NecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORECF` reader - Overrun error flag reset"]
pub type OrecfR = crate::BitReader;
#[doc = "Field `ORECF` writer - Overrun error flag reset"]
pub type OrecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLECF` reader - Clear idle flag"]
pub type IdlecfR = crate::BitReader;
#[doc = "Field `IDLECF` writer - Clear idle flag"]
pub type IdlecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCF` reader - Transfer completion flag reset"]
pub type TccfR = crate::BitReader;
#[doc = "Field `TCCF` writer - Transfer completion flag reset"]
pub type TccfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSCF` reader - CTS flag reset"]
pub type CtscfR = crate::BitReader;
#[doc = "Field `CTSCF` writer - CTS flag reset"]
pub type CtscfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMCF` reader - Address matching flag reset"]
pub type CmcfR = crate::BitReader;
#[doc = "Field `CMCF` writer - Address matching flag reset"]
pub type CmcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUCF` reader - Low power wake-up flag reset"]
pub type WucfR = crate::BitReader;
#[doc = "Field `WUCF` writer - Low power wake-up flag reset"]
pub type WucfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Check value error flag reset"]
    #[inline(always)]
    pub fn pecf(&self) -> PecfR {
        PecfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame error flag reset"]
    #[inline(always)]
    pub fn fecf(&self) -> FecfR {
        FecfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear the START bit noise flag"]
    #[inline(always)]
    pub fn necf(&self) -> NecfR {
        NecfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error flag reset"]
    #[inline(always)]
    pub fn orecf(&self) -> OrecfR {
        OrecfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear idle flag"]
    #[inline(always)]
    pub fn idlecf(&self) -> IdlecfR {
        IdlecfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer completion flag reset"]
    #[inline(always)]
    pub fn tccf(&self) -> TccfR {
        TccfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS flag reset"]
    #[inline(always)]
    pub fn ctscf(&self) -> CtscfR {
        CtscfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Address matching flag reset"]
    #[inline(always)]
    pub fn cmcf(&self) -> CmcfR {
        CmcfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Low power wake-up flag reset"]
    #[inline(always)]
    pub fn wucf(&self) -> WucfR {
        WucfR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Check value error flag reset"]
    #[inline(always)]
    pub fn pecf(&mut self) -> PecfW<'_, IcrSpec> {
        PecfW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame error flag reset"]
    #[inline(always)]
    pub fn fecf(&mut self) -> FecfW<'_, IcrSpec> {
        FecfW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear the START bit noise flag"]
    #[inline(always)]
    pub fn necf(&mut self) -> NecfW<'_, IcrSpec> {
        NecfW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error flag reset"]
    #[inline(always)]
    pub fn orecf(&mut self) -> OrecfW<'_, IcrSpec> {
        OrecfW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear idle flag"]
    #[inline(always)]
    pub fn idlecf(&mut self) -> IdlecfW<'_, IcrSpec> {
        IdlecfW::new(self, 4)
    }
    #[doc = "Bit 6 - Transfer completion flag reset"]
    #[inline(always)]
    pub fn tccf(&mut self) -> TccfW<'_, IcrSpec> {
        TccfW::new(self, 6)
    }
    #[doc = "Bit 9 - CTS flag reset"]
    #[inline(always)]
    pub fn ctscf(&mut self) -> CtscfW<'_, IcrSpec> {
        CtscfW::new(self, 9)
    }
    #[doc = "Bit 17 - Address matching flag reset"]
    #[inline(always)]
    pub fn cmcf(&mut self) -> CmcfW<'_, IcrSpec> {
        CmcfW::new(self, 17)
    }
    #[doc = "Bit 20 - Low power wake-up flag reset"]
    #[inline(always)]
    pub fn wucf(&mut self) -> WucfW<'_, IcrSpec> {
        WucfW::new(self, 20)
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
