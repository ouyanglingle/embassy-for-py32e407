#[doc = "Register `INTSTS` reader"]
pub type R = crate::R<IntstsSpec>;
#[doc = "Register `INTSTS` writer"]
pub type W = crate::W<IntstsSpec>;
#[doc = "Field `CAD` reader - desc CAD"]
pub type CadR = crate::BitReader;
#[doc = "Field `CAD` writer - desc CAD"]
pub type CadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE` reader - desc RE"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - desc RE"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD` reader - desc CD"]
pub type CdR = crate::BitReader;
#[doc = "Field `CD` writer - desc CD"]
pub type CdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTO` reader - desc DTO"]
pub type DtoR = crate::BitReader;
#[doc = "Field `DTO` writer - desc DTO"]
pub type DtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDR` reader - desc TXDR"]
pub type TxdrR = crate::BitReader;
#[doc = "Field `TXDR` writer - desc TXDR"]
pub type TxdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDR` reader - desc RXDR"]
pub type RxdrR = crate::BitReader;
#[doc = "Field `RXDR` writer - desc RXDR"]
pub type RxdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCRC` reader - desc RCRC"]
pub type RcrcR = crate::BitReader;
#[doc = "Field `RCRC` writer - desc RCRC"]
pub type RcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRC` reader - desc DCRC"]
pub type DcrcR = crate::BitReader;
#[doc = "Field `DCRC` writer - desc DCRC"]
pub type DcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTO_BAR` reader - desc RTO_BAR"]
pub type RtoBarR = crate::BitReader;
#[doc = "Field `RTO_BAR` writer - desc RTO_BAR"]
pub type RtoBarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRTO_BDS` reader - desc DRTO_BDS"]
pub type DrtoBdsR = crate::BitReader;
#[doc = "Field `DRTO_BDS` writer - desc DRTO_BDS"]
pub type DrtoBdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTO` reader - desc HTO"]
pub type HtoR = crate::BitReader;
#[doc = "Field `HTO` writer - desc HTO"]
pub type HtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRUN` reader - desc FRUN"]
pub type FrunR = crate::BitReader;
#[doc = "Field `FRUN` writer - desc FRUN"]
pub type FrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HLE` reader - desc HLE"]
pub type HleR = crate::BitReader;
#[doc = "Field `HLE` writer - desc HLE"]
pub type HleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBE` reader - desc SBE"]
pub type SbeR = crate::BitReader;
#[doc = "Field `SBE` writer - desc SBE"]
pub type SbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACD` reader - desc ACD"]
pub type AcdR = crate::BitReader;
#[doc = "Field `ACD` writer - desc ACD"]
pub type AcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBE` reader - desc EBE"]
pub type EbeR = crate::BitReader;
#[doc = "Field `EBE` writer - desc EBE"]
pub type EbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOINT` reader - desc SDIOINT"]
pub type SdiointR = crate::BitReader;
#[doc = "Field `SDIOINT` writer - desc SDIOINT"]
pub type SdiointW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CAD"]
    #[inline(always)]
    pub fn cad(&self) -> CadR {
        CadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RE"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CD"]
    #[inline(always)]
    pub fn cd(&self) -> CdR {
        CdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc DTO"]
    #[inline(always)]
    pub fn dto(&self) -> DtoR {
        DtoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TXDR"]
    #[inline(always)]
    pub fn txdr(&self) -> TxdrR {
        TxdrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RXDR"]
    #[inline(always)]
    pub fn rxdr(&self) -> RxdrR {
        RxdrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RCRC"]
    #[inline(always)]
    pub fn rcrc(&self) -> RcrcR {
        RcrcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DCRC"]
    #[inline(always)]
    pub fn dcrc(&self) -> DcrcR {
        DcrcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RTO_BAR"]
    #[inline(always)]
    pub fn rto_bar(&self) -> RtoBarR {
        RtoBarR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc DRTO_BDS"]
    #[inline(always)]
    pub fn drto_bds(&self) -> DrtoBdsR {
        DrtoBdsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HTO"]
    #[inline(always)]
    pub fn hto(&self) -> HtoR {
        HtoR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc FRUN"]
    #[inline(always)]
    pub fn frun(&self) -> FrunR {
        FrunR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc HLE"]
    #[inline(always)]
    pub fn hle(&self) -> HleR {
        HleR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc SBE"]
    #[inline(always)]
    pub fn sbe(&self) -> SbeR {
        SbeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc ACD"]
    #[inline(always)]
    pub fn acd(&self) -> AcdR {
        AcdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EBE"]
    #[inline(always)]
    pub fn ebe(&self) -> EbeR {
        EbeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc SDIOINT"]
    #[inline(always)]
    pub fn sdioint(&self) -> SdiointR {
        SdiointR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CAD"]
    #[inline(always)]
    pub fn cad(&mut self) -> CadW<'_, IntstsSpec> {
        CadW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RE"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<'_, IntstsSpec> {
        ReW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CD"]
    #[inline(always)]
    pub fn cd(&mut self) -> CdW<'_, IntstsSpec> {
        CdW::new(self, 2)
    }
    #[doc = "Bit 3 - desc DTO"]
    #[inline(always)]
    pub fn dto(&mut self) -> DtoW<'_, IntstsSpec> {
        DtoW::new(self, 3)
    }
    #[doc = "Bit 4 - desc TXDR"]
    #[inline(always)]
    pub fn txdr(&mut self) -> TxdrW<'_, IntstsSpec> {
        TxdrW::new(self, 4)
    }
    #[doc = "Bit 5 - desc RXDR"]
    #[inline(always)]
    pub fn rxdr(&mut self) -> RxdrW<'_, IntstsSpec> {
        RxdrW::new(self, 5)
    }
    #[doc = "Bit 6 - desc RCRC"]
    #[inline(always)]
    pub fn rcrc(&mut self) -> RcrcW<'_, IntstsSpec> {
        RcrcW::new(self, 6)
    }
    #[doc = "Bit 7 - desc DCRC"]
    #[inline(always)]
    pub fn dcrc(&mut self) -> DcrcW<'_, IntstsSpec> {
        DcrcW::new(self, 7)
    }
    #[doc = "Bit 8 - desc RTO_BAR"]
    #[inline(always)]
    pub fn rto_bar(&mut self) -> RtoBarW<'_, IntstsSpec> {
        RtoBarW::new(self, 8)
    }
    #[doc = "Bit 9 - desc DRTO_BDS"]
    #[inline(always)]
    pub fn drto_bds(&mut self) -> DrtoBdsW<'_, IntstsSpec> {
        DrtoBdsW::new(self, 9)
    }
    #[doc = "Bit 10 - desc HTO"]
    #[inline(always)]
    pub fn hto(&mut self) -> HtoW<'_, IntstsSpec> {
        HtoW::new(self, 10)
    }
    #[doc = "Bit 11 - desc FRUN"]
    #[inline(always)]
    pub fn frun(&mut self) -> FrunW<'_, IntstsSpec> {
        FrunW::new(self, 11)
    }
    #[doc = "Bit 12 - desc HLE"]
    #[inline(always)]
    pub fn hle(&mut self) -> HleW<'_, IntstsSpec> {
        HleW::new(self, 12)
    }
    #[doc = "Bit 13 - desc SBE"]
    #[inline(always)]
    pub fn sbe(&mut self) -> SbeW<'_, IntstsSpec> {
        SbeW::new(self, 13)
    }
    #[doc = "Bit 14 - desc ACD"]
    #[inline(always)]
    pub fn acd(&mut self) -> AcdW<'_, IntstsSpec> {
        AcdW::new(self, 14)
    }
    #[doc = "Bit 15 - desc EBE"]
    #[inline(always)]
    pub fn ebe(&mut self) -> EbeW<'_, IntstsSpec> {
        EbeW::new(self, 15)
    }
    #[doc = "Bit 16 - desc SDIOINT"]
    #[inline(always)]
    pub fn sdioint(&mut self) -> SdiointW<'_, IntstsSpec> {
        SdiointW::new(self, 16)
    }
}
#[doc = "desc INTSTS\n\nYou can [`read`](crate::Reg::read) this register and get [`intsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstsSpec;
impl crate::RegisterSpec for IntstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsts::R`](R) reader structure"]
impl crate::Readable for IntstsSpec {}
#[doc = "`write(|w| ..)` method takes [`intsts::W`](W) writer structure"]
impl crate::Writable for IntstsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for IntstsSpec {}
