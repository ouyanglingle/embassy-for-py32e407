#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `UE` reader - LPUART enabled"]
pub type UeR = crate::BitReader;
#[doc = "Field `UE` writer - LPUART enabled"]
pub type UeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UESM` reader - LPUART enabled in Stop mode"]
pub type UesmR = crate::BitReader;
#[doc = "Field `UESM` writer - LPUART enabled in Stop mode"]
pub type UesmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE` reader - Receive enable"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - Receive enable"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transfer enable"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - Transfer enable"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEIE` reader - IDLE interrupt enable"]
pub type IdleieR = crate::BitReader;
#[doc = "Field `IDLEIE` writer - IDLE interrupt enable"]
pub type IdleieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFNEIE` reader - RXFIFO non-null interrupt enable"]
pub type RxfneieR = crate::BitReader;
#[doc = "Field `RXFNEIE` writer - RXFIFO non-null interrupt enable"]
pub type RxfneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - End of Transfer interrupt enabled"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - End of Transfer interrupt enabled"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNFIE` reader - TXFIFO is not full interrupt enabled"]
pub type TxfnfieR = crate::BitReader;
#[doc = "Field `TXFNFIE` writer - TXFIFO is not full interrupt enabled"]
pub type TxfnfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIE` reader - PE interrupt enable"]
pub type PeieR = crate::BitReader;
#[doc = "Field `PEIE` writer - PE interrupt enable"]
pub type PeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - Parity selection"]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - Parity selection"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCE` reader - Parity control"]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - Parity control"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKE` reader - Receive wake-up mode"]
pub type WakeR = crate::BitReader;
#[doc = "Field `WAKE` writer - Receive wake-up mode"]
pub type WakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M0` reader - Configure the word length in combination with M1"]
pub type M0R = crate::BitReader;
#[doc = "Field `M0` writer - Configure the word length in combination with M1"]
pub type M0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MME` reader - Silent mode enabled"]
pub type MmeR = crate::BitReader;
#[doc = "Field `MME` writer - Silent mode enabled"]
pub type MmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMIE` reader - Character match interrupt enabled"]
pub type CmieR = crate::BitReader;
#[doc = "Field `CMIE` writer - Character match interrupt enabled"]
pub type CmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDT` reader - The time between the transmit frame stop bit and the invalid DE (driver enable) signal"]
pub type DedtR = crate::FieldReader;
#[doc = "Field `DEDT` writer - The time between the transmit frame stop bit and the invalid DE (driver enable) signal"]
pub type DedtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEAT` reader - The time between the DE (driver enable) signal validity and the start bit"]
pub type DeatR = crate::FieldReader;
#[doc = "Field `DEAT` writer - The time between the DE (driver enable) signal validity and the start bit"]
pub type DeatW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `M1` reader - The value of {M1,M0} the configurate length"]
pub type M1R = crate::BitReader;
#[doc = "Field `M1` writer - The value of {M1,M0} the configurate length"]
pub type M1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEN` reader - FIFO mode enabled"]
pub type FifoenR = crate::BitReader;
#[doc = "Field `FIFOEN` writer - FIFO mode enabled"]
pub type FifoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFEIE` reader - TXFIFO null interrupt enabled"]
pub type TxfeieR = crate::BitReader;
#[doc = "Field `TXFEIE` writer - TXFIFO null interrupt enabled"]
pub type TxfeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFIE` reader - RXFIFO full interrupt enabled"]
pub type RxffieR = crate::BitReader;
#[doc = "Field `RXFFIE` writer - RXFIFO full interrupt enabled"]
pub type RxffieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPUART enabled"]
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPUART enabled in Stop mode"]
    #[inline(always)]
    pub fn uesm(&self) -> UesmR {
        UesmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive enable"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IdleieR {
        IdleieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXFIFO non-null interrupt enable"]
    #[inline(always)]
    pub fn rxfneie(&self) -> RxfneieR {
        RxfneieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of Transfer interrupt enabled"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFIFO is not full interrupt enabled"]
    #[inline(always)]
    pub fn txfnfie(&self) -> TxfnfieR {
        TxfnfieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    pub fn peie(&self) -> PeieR {
        PeieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity control"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive wake-up mode"]
    #[inline(always)]
    pub fn wake(&self) -> WakeR {
        WakeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configure the word length in combination with M1"]
    #[inline(always)]
    pub fn m0(&self) -> M0R {
        M0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Silent mode enabled"]
    #[inline(always)]
    pub fn mme(&self) -> MmeR {
        MmeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Character match interrupt enabled"]
    #[inline(always)]
    pub fn cmie(&self) -> CmieR {
        CmieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:20 - The time between the transmit frame stop bit and the invalid DE (driver enable) signal"]
    #[inline(always)]
    pub fn dedt(&self) -> DedtR {
        DedtR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - The time between the DE (driver enable) signal validity and the start bit"]
    #[inline(always)]
    pub fn deat(&self) -> DeatR {
        DeatR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - The value of {M1,M0} the configurate length"]
    #[inline(always)]
    pub fn m1(&self) -> M1R {
        M1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FIFO mode enabled"]
    #[inline(always)]
    pub fn fifoen(&self) -> FifoenR {
        FifoenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TXFIFO null interrupt enabled"]
    #[inline(always)]
    pub fn txfeie(&self) -> TxfeieR {
        TxfeieR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RXFIFO full interrupt enabled"]
    #[inline(always)]
    pub fn rxffie(&self) -> RxffieR {
        RxffieR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPUART enabled"]
    #[inline(always)]
    pub fn ue(&mut self) -> UeW<'_, Cr1Spec> {
        UeW::new(self, 0)
    }
    #[doc = "Bit 1 - LPUART enabled in Stop mode"]
    #[inline(always)]
    pub fn uesm(&mut self) -> UesmW<'_, Cr1Spec> {
        UesmW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive enable"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<'_, Cr1Spec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Transfer enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TeW<'_, Cr1Spec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&mut self) -> IdleieW<'_, Cr1Spec> {
        IdleieW::new(self, 4)
    }
    #[doc = "Bit 5 - RXFIFO non-null interrupt enable"]
    #[inline(always)]
    pub fn rxfneie(&mut self) -> RxfneieW<'_, Cr1Spec> {
        RxfneieW::new(self, 5)
    }
    #[doc = "Bit 6 - End of Transfer interrupt enabled"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<'_, Cr1Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - TXFIFO is not full interrupt enabled"]
    #[inline(always)]
    pub fn txfnfie(&mut self) -> TxfnfieW<'_, Cr1Spec> {
        TxfnfieW::new(self, 7)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    pub fn peie(&mut self) -> PeieW<'_, Cr1Spec> {
        PeieW::new(self, 8)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, Cr1Spec> {
        PsW::new(self, 9)
    }
    #[doc = "Bit 10 - Parity control"]
    #[inline(always)]
    pub fn pce(&mut self) -> PceW<'_, Cr1Spec> {
        PceW::new(self, 10)
    }
    #[doc = "Bit 11 - Receive wake-up mode"]
    #[inline(always)]
    pub fn wake(&mut self) -> WakeW<'_, Cr1Spec> {
        WakeW::new(self, 11)
    }
    #[doc = "Bit 12 - Configure the word length in combination with M1"]
    #[inline(always)]
    pub fn m0(&mut self) -> M0W<'_, Cr1Spec> {
        M0W::new(self, 12)
    }
    #[doc = "Bit 13 - Silent mode enabled"]
    #[inline(always)]
    pub fn mme(&mut self) -> MmeW<'_, Cr1Spec> {
        MmeW::new(self, 13)
    }
    #[doc = "Bit 14 - Character match interrupt enabled"]
    #[inline(always)]
    pub fn cmie(&mut self) -> CmieW<'_, Cr1Spec> {
        CmieW::new(self, 14)
    }
    #[doc = "Bits 16:20 - The time between the transmit frame stop bit and the invalid DE (driver enable) signal"]
    #[inline(always)]
    pub fn dedt(&mut self) -> DedtW<'_, Cr1Spec> {
        DedtW::new(self, 16)
    }
    #[doc = "Bits 21:25 - The time between the DE (driver enable) signal validity and the start bit"]
    #[inline(always)]
    pub fn deat(&mut self) -> DeatW<'_, Cr1Spec> {
        DeatW::new(self, 21)
    }
    #[doc = "Bit 28 - The value of {M1,M0} the configurate length"]
    #[inline(always)]
    pub fn m1(&mut self) -> M1W<'_, Cr1Spec> {
        M1W::new(self, 28)
    }
    #[doc = "Bit 29 - FIFO mode enabled"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FifoenW<'_, Cr1Spec> {
        FifoenW::new(self, 29)
    }
    #[doc = "Bit 30 - TXFIFO null interrupt enabled"]
    #[inline(always)]
    pub fn txfeie(&mut self) -> TxfeieW<'_, Cr1Spec> {
        TxfeieW::new(self, 30)
    }
    #[doc = "Bit 31 - RXFIFO full interrupt enabled"]
    #[inline(always)]
    pub fn rxffie(&mut self) -> RxffieW<'_, Cr1Spec> {
        RxffieW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
