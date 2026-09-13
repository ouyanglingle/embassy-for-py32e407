#[doc = "Register `INTMASK` reader"]
pub type R = crate::R<IntmaskSpec>;
#[doc = "Register `INTMASK` writer"]
pub type W = crate::W<IntmaskSpec>;
#[doc = "Field `CADIE` reader - desc CADIE"]
pub type CadieR = crate::BitReader;
#[doc = "Field `CADIE` writer - desc CADIE"]
pub type CadieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REIE` reader - desc REIE"]
pub type ReieR = crate::BitReader;
#[doc = "Field `REIE` writer - desc REIE"]
pub type ReieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDIE` reader - desc CDIE"]
pub type CdieR = crate::BitReader;
#[doc = "Field `CDIE` writer - desc CDIE"]
pub type CdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOIE` reader - desc DTOIE"]
pub type DtoieR = crate::BitReader;
#[doc = "Field `DTOIE` writer - desc DTOIE"]
pub type DtoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDRIE` reader - desc TXDRIE"]
pub type TxdrieR = crate::BitReader;
#[doc = "Field `TXDRIE` writer - desc TXDRIE"]
pub type TxdrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDRIE` reader - desc RXDRIE"]
pub type RxdrieR = crate::BitReader;
#[doc = "Field `RXDRIE` writer - desc RXDRIE"]
pub type RxdrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCRCIE` reader - desc RCRCIE"]
pub type RcrcieR = crate::BitReader;
#[doc = "Field `RCRCIE` writer - desc RCRCIE"]
pub type RcrcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRCIE` reader - desc DCRCIE"]
pub type DcrcieR = crate::BitReader;
#[doc = "Field `DCRCIE` writer - desc DCRCIE"]
pub type DcrcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOIE` reader - desc RTOIE"]
pub type RtoieR = crate::BitReader;
#[doc = "Field `RTOIE` writer - desc RTOIE"]
pub type RtoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRTOIE` reader - desc DRTOIE"]
pub type DrtoieR = crate::BitReader;
#[doc = "Field `DRTOIE` writer - desc DRTOIE"]
pub type DrtoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTOIE` reader - desc HTOIE"]
pub type HtoieR = crate::BitReader;
#[doc = "Field `HTOIE` writer - desc HTOIE"]
pub type HtoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRUNIE` reader - desc FRUNIE"]
pub type FrunieR = crate::BitReader;
#[doc = "Field `FRUNIE` writer - desc FRUNIE"]
pub type FrunieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HLEIE` reader - desc HLEIE"]
pub type HleieR = crate::BitReader;
#[doc = "Field `HLEIE` writer - desc HLEIE"]
pub type HleieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBEIE` reader - desc SBEIE"]
pub type SbeieR = crate::BitReader;
#[doc = "Field `SBEIE` writer - desc SBEIE"]
pub type SbeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACDIE` reader - desc ACDIE"]
pub type AcdieR = crate::BitReader;
#[doc = "Field `ACDIE` writer - desc ACDIE"]
pub type AcdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBEIE` reader - desc EBEIE"]
pub type EbeieR = crate::BitReader;
#[doc = "Field `EBEIE` writer - desc EBEIE"]
pub type EbeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOINTIE` reader - desc SDIOINTIE"]
pub type SdiointieR = crate::BitReader;
#[doc = "Field `SDIOINTIE` writer - desc SDIOINTIE"]
pub type SdiointieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CADIE"]
    #[inline(always)]
    pub fn cadie(&self) -> CadieR {
        CadieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc REIE"]
    #[inline(always)]
    pub fn reie(&self) -> ReieR {
        ReieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CDIE"]
    #[inline(always)]
    pub fn cdie(&self) -> CdieR {
        CdieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc DTOIE"]
    #[inline(always)]
    pub fn dtoie(&self) -> DtoieR {
        DtoieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TXDRIE"]
    #[inline(always)]
    pub fn txdrie(&self) -> TxdrieR {
        TxdrieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RXDRIE"]
    #[inline(always)]
    pub fn rxdrie(&self) -> RxdrieR {
        RxdrieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RCRCIE"]
    #[inline(always)]
    pub fn rcrcie(&self) -> RcrcieR {
        RcrcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DCRCIE"]
    #[inline(always)]
    pub fn dcrcie(&self) -> DcrcieR {
        DcrcieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RTOIE"]
    #[inline(always)]
    pub fn rtoie(&self) -> RtoieR {
        RtoieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc DRTOIE"]
    #[inline(always)]
    pub fn drtoie(&self) -> DrtoieR {
        DrtoieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HTOIE"]
    #[inline(always)]
    pub fn htoie(&self) -> HtoieR {
        HtoieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc FRUNIE"]
    #[inline(always)]
    pub fn frunie(&self) -> FrunieR {
        FrunieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc HLEIE"]
    #[inline(always)]
    pub fn hleie(&self) -> HleieR {
        HleieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc SBEIE"]
    #[inline(always)]
    pub fn sbeie(&self) -> SbeieR {
        SbeieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc ACDIE"]
    #[inline(always)]
    pub fn acdie(&self) -> AcdieR {
        AcdieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EBEIE"]
    #[inline(always)]
    pub fn ebeie(&self) -> EbeieR {
        EbeieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc SDIOINTIE"]
    #[inline(always)]
    pub fn sdiointie(&self) -> SdiointieR {
        SdiointieR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CADIE"]
    #[inline(always)]
    pub fn cadie(&mut self) -> CadieW<'_, IntmaskSpec> {
        CadieW::new(self, 0)
    }
    #[doc = "Bit 1 - desc REIE"]
    #[inline(always)]
    pub fn reie(&mut self) -> ReieW<'_, IntmaskSpec> {
        ReieW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CDIE"]
    #[inline(always)]
    pub fn cdie(&mut self) -> CdieW<'_, IntmaskSpec> {
        CdieW::new(self, 2)
    }
    #[doc = "Bit 3 - desc DTOIE"]
    #[inline(always)]
    pub fn dtoie(&mut self) -> DtoieW<'_, IntmaskSpec> {
        DtoieW::new(self, 3)
    }
    #[doc = "Bit 4 - desc TXDRIE"]
    #[inline(always)]
    pub fn txdrie(&mut self) -> TxdrieW<'_, IntmaskSpec> {
        TxdrieW::new(self, 4)
    }
    #[doc = "Bit 5 - desc RXDRIE"]
    #[inline(always)]
    pub fn rxdrie(&mut self) -> RxdrieW<'_, IntmaskSpec> {
        RxdrieW::new(self, 5)
    }
    #[doc = "Bit 6 - desc RCRCIE"]
    #[inline(always)]
    pub fn rcrcie(&mut self) -> RcrcieW<'_, IntmaskSpec> {
        RcrcieW::new(self, 6)
    }
    #[doc = "Bit 7 - desc DCRCIE"]
    #[inline(always)]
    pub fn dcrcie(&mut self) -> DcrcieW<'_, IntmaskSpec> {
        DcrcieW::new(self, 7)
    }
    #[doc = "Bit 8 - desc RTOIE"]
    #[inline(always)]
    pub fn rtoie(&mut self) -> RtoieW<'_, IntmaskSpec> {
        RtoieW::new(self, 8)
    }
    #[doc = "Bit 9 - desc DRTOIE"]
    #[inline(always)]
    pub fn drtoie(&mut self) -> DrtoieW<'_, IntmaskSpec> {
        DrtoieW::new(self, 9)
    }
    #[doc = "Bit 10 - desc HTOIE"]
    #[inline(always)]
    pub fn htoie(&mut self) -> HtoieW<'_, IntmaskSpec> {
        HtoieW::new(self, 10)
    }
    #[doc = "Bit 11 - desc FRUNIE"]
    #[inline(always)]
    pub fn frunie(&mut self) -> FrunieW<'_, IntmaskSpec> {
        FrunieW::new(self, 11)
    }
    #[doc = "Bit 12 - desc HLEIE"]
    #[inline(always)]
    pub fn hleie(&mut self) -> HleieW<'_, IntmaskSpec> {
        HleieW::new(self, 12)
    }
    #[doc = "Bit 13 - desc SBEIE"]
    #[inline(always)]
    pub fn sbeie(&mut self) -> SbeieW<'_, IntmaskSpec> {
        SbeieW::new(self, 13)
    }
    #[doc = "Bit 14 - desc ACDIE"]
    #[inline(always)]
    pub fn acdie(&mut self) -> AcdieW<'_, IntmaskSpec> {
        AcdieW::new(self, 14)
    }
    #[doc = "Bit 15 - desc EBEIE"]
    #[inline(always)]
    pub fn ebeie(&mut self) -> EbeieW<'_, IntmaskSpec> {
        EbeieW::new(self, 15)
    }
    #[doc = "Bit 16 - desc SDIOINTIE"]
    #[inline(always)]
    pub fn sdiointie(&mut self) -> SdiointieW<'_, IntmaskSpec> {
        SdiointieW::new(self, 16)
    }
}
#[doc = "desc INTMASK\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntmaskSpec;
impl crate::RegisterSpec for IntmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmask::R`](R) reader structure"]
impl crate::Readable for IntmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intmask::W`](W) writer structure"]
impl crate::Writable for IntmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTMASK to value 0"]
impl crate::Resettable for IntmaskSpec {}
