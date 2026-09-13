#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `SBK` reader - Send break"]
pub type SbkR = crate::BitReader;
#[doc = "Field `SBK` writer - Send break"]
pub type SbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWU` reader - Receiver wakeup"]
pub type RwuR = crate::BitReader;
#[doc = "Field `RE` reader - Receiver enable"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - Receiver enable"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEIE` reader - IDLE interrupt enable"]
pub type IdleieR = crate::BitReader;
#[doc = "Field `IDLEIE` writer - IDLE interrupt enable"]
pub type IdleieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNEIE` reader - RXNE interrupt enable"]
pub type RxneieR = crate::BitReader;
#[doc = "Field `RXNEIE` writer - RXNE interrupt enable"]
pub type RxneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - send complete int enable"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - send complete int enable"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEIE` reader - txe"]
pub type TxeieR = crate::BitReader;
#[doc = "Field `TXEIE` writer - txe"]
pub type TxeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIE` reader - PE interrupt enable"]
pub type PeieR = crate::BitReader;
#[doc = "Field `PEIE` writer - PE interrupt enable"]
pub type PeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - Parity selection"]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - Parity selection"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCE` reader - Parity control enable"]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - Parity control enable"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKE` reader - Wakeup method"]
pub type WakeR = crate::BitReader;
#[doc = "Field `WAKE` writer - Wakeup method"]
pub type WakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M0` reader - word length"]
pub type M0R = crate::BitReader;
#[doc = "Field `M0` writer - word length"]
pub type M0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UE` reader - uesart enable"]
pub type UeR = crate::BitReader;
#[doc = "Field `UE` writer - uesart enable"]
pub type UeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M1` reader - word length"]
pub type M1R = crate::BitReader;
#[doc = "Field `M1` writer - word length"]
pub type M1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSB` reader - msb"]
pub type MsbR = crate::BitReader;
#[doc = "Field `MSB` writer - msb"]
pub type MsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMRQ` reader - mm request"]
pub type MmrqR = crate::BitReader;
#[doc = "Field `MMRQ` writer - mm request"]
pub type MmrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDM7` reader - addm7"]
pub type Addm7R = crate::BitReader;
#[doc = "Field `ADDM7` writer - addm7"]
pub type Addm7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP` reader - swap"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - swap"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFTCFG` reader - txfifo threshoold cfg"]
pub type TxftcfgR = crate::FieldReader;
#[doc = "Field `TXFTCFG` writer - txfifo threshoold cfg"]
pub type TxftcfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXFTCFG` reader - rxfifo threshoold cfg"]
pub type RxftcfgR = crate::FieldReader;
#[doc = "Field `RXFTCFG` writer - rxfifo threshoold cfg"]
pub type RxftcfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RX_INV` reader - rxinv"]
pub type RxInvR = crate::BitReader;
#[doc = "Field `RX_INV` writer - rxinv"]
pub type RxInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_INV` reader - txinv"]
pub type TxInvR = crate::BitReader;
#[doc = "Field `TX_INV` writer - txinv"]
pub type TxInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAINV` reader - datainv"]
pub type DatainvR = crate::BitReader;
#[doc = "Field `DATAINV` writer - datainv"]
pub type DatainvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFRQ` reader - receive data flush request"]
pub type RxfrqR = crate::BitReader;
#[doc = "Field `RXFRQ` writer - receive data flush request"]
pub type RxfrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFRQ` reader - Transmit data flush request"]
pub type TxfrqR = crate::BitReader;
#[doc = "Field `TXFRQ` writer - Transmit data flush request"]
pub type TxfrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEN` reader - fifo en"]
pub type FifoenR = crate::BitReader;
#[doc = "Field `FIFOEN` writer - fifo en"]
pub type FifoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    pub fn sbk(&self) -> SbkR {
        SbkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver wakeup"]
    #[inline(always)]
    pub fn rwu(&self) -> RwuR {
        RwuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IdleieR {
        IdleieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RxneieR {
        RxneieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - send complete int enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - txe"]
    #[inline(always)]
    pub fn txeie(&self) -> TxeieR {
        TxeieR::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup method"]
    #[inline(always)]
    pub fn wake(&self) -> WakeR {
        WakeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - word length"]
    #[inline(always)]
    pub fn m0(&self) -> M0R {
        M0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - uesart enable"]
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - word length"]
    #[inline(always)]
    pub fn m1(&self) -> M1R {
        M1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - msb"]
    #[inline(always)]
    pub fn msb(&self) -> MsbR {
        MsbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - mm request"]
    #[inline(always)]
    pub fn mmrq(&self) -> MmrqR {
        MmrqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - addm7"]
    #[inline(always)]
    pub fn addm7(&self) -> Addm7R {
        Addm7R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - swap"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - txfifo threshoold cfg"]
    #[inline(always)]
    pub fn txftcfg(&self) -> TxftcfgR {
        TxftcfgR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - rxfifo threshoold cfg"]
    #[inline(always)]
    pub fn rxftcfg(&self) -> RxftcfgR {
        RxftcfgR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - rxinv"]
    #[inline(always)]
    pub fn rx_inv(&self) -> RxInvR {
        RxInvR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - txinv"]
    #[inline(always)]
    pub fn tx_inv(&self) -> TxInvR {
        TxInvR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - datainv"]
    #[inline(always)]
    pub fn datainv(&self) -> DatainvR {
        DatainvR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - receive data flush request"]
    #[inline(always)]
    pub fn rxfrq(&self) -> RxfrqR {
        RxfrqR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmit data flush request"]
    #[inline(always)]
    pub fn txfrq(&self) -> TxfrqR {
        TxfrqR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - fifo en"]
    #[inline(always)]
    pub fn fifoen(&self) -> FifoenR {
        FifoenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    pub fn sbk(&mut self) -> SbkW<'_, Cr1Spec> {
        SbkW::new(self, 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<'_, Cr1Spec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TeW<'_, Cr1Spec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&mut self) -> IdleieW<'_, Cr1Spec> {
        IdleieW::new(self, 4)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RxneieW<'_, Cr1Spec> {
        RxneieW::new(self, 5)
    }
    #[doc = "Bit 6 - send complete int enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<'_, Cr1Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - txe"]
    #[inline(always)]
    pub fn txeie(&mut self) -> TxeieW<'_, Cr1Spec> {
        TxeieW::new(self, 7)
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
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pce(&mut self) -> PceW<'_, Cr1Spec> {
        PceW::new(self, 10)
    }
    #[doc = "Bit 11 - Wakeup method"]
    #[inline(always)]
    pub fn wake(&mut self) -> WakeW<'_, Cr1Spec> {
        WakeW::new(self, 11)
    }
    #[doc = "Bit 12 - word length"]
    #[inline(always)]
    pub fn m0(&mut self) -> M0W<'_, Cr1Spec> {
        M0W::new(self, 12)
    }
    #[doc = "Bit 13 - uesart enable"]
    #[inline(always)]
    pub fn ue(&mut self) -> UeW<'_, Cr1Spec> {
        UeW::new(self, 13)
    }
    #[doc = "Bit 14 - word length"]
    #[inline(always)]
    pub fn m1(&mut self) -> M1W<'_, Cr1Spec> {
        M1W::new(self, 14)
    }
    #[doc = "Bit 15 - msb"]
    #[inline(always)]
    pub fn msb(&mut self) -> MsbW<'_, Cr1Spec> {
        MsbW::new(self, 15)
    }
    #[doc = "Bit 16 - mm request"]
    #[inline(always)]
    pub fn mmrq(&mut self) -> MmrqW<'_, Cr1Spec> {
        MmrqW::new(self, 16)
    }
    #[doc = "Bit 17 - addm7"]
    #[inline(always)]
    pub fn addm7(&mut self) -> Addm7W<'_, Cr1Spec> {
        Addm7W::new(self, 17)
    }
    #[doc = "Bit 18 - swap"]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<'_, Cr1Spec> {
        SwapW::new(self, 18)
    }
    #[doc = "Bits 19:21 - txfifo threshoold cfg"]
    #[inline(always)]
    pub fn txftcfg(&mut self) -> TxftcfgW<'_, Cr1Spec> {
        TxftcfgW::new(self, 19)
    }
    #[doc = "Bits 22:24 - rxfifo threshoold cfg"]
    #[inline(always)]
    pub fn rxftcfg(&mut self) -> RxftcfgW<'_, Cr1Spec> {
        RxftcfgW::new(self, 22)
    }
    #[doc = "Bit 25 - rxinv"]
    #[inline(always)]
    pub fn rx_inv(&mut self) -> RxInvW<'_, Cr1Spec> {
        RxInvW::new(self, 25)
    }
    #[doc = "Bit 26 - txinv"]
    #[inline(always)]
    pub fn tx_inv(&mut self) -> TxInvW<'_, Cr1Spec> {
        TxInvW::new(self, 26)
    }
    #[doc = "Bit 27 - datainv"]
    #[inline(always)]
    pub fn datainv(&mut self) -> DatainvW<'_, Cr1Spec> {
        DatainvW::new(self, 27)
    }
    #[doc = "Bit 29 - receive data flush request"]
    #[inline(always)]
    pub fn rxfrq(&mut self) -> RxfrqW<'_, Cr1Spec> {
        RxfrqW::new(self, 29)
    }
    #[doc = "Bit 30 - Transmit data flush request"]
    #[inline(always)]
    pub fn txfrq(&mut self) -> TxfrqW<'_, Cr1Spec> {
        TxfrqW::new(self, 30)
    }
    #[doc = "Bit 31 - fifo en"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FifoenW<'_, Cr1Spec> {
        FifoenW::new(self, 31)
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
