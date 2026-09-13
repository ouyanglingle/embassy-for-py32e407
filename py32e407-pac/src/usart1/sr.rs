#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `PE` reader - Parity error"]
pub type PeR = crate::BitReader;
#[doc = "Field `FE` reader - Framing error"]
pub type FeR = crate::BitReader;
#[doc = "Field `NE` reader - Noise error flag"]
pub type NeR = crate::BitReader;
#[doc = "Field `ORE` reader - overrun error"]
pub type OreR = crate::BitReader;
#[doc = "Field `IDLE` reader - idle"]
pub type IdleR = crate::BitReader;
#[doc = "Field `RXNE_RXFNF` reader - Read data register not empty"]
pub type RxneRxfnfR = crate::BitReader;
#[doc = "Field `TC` reader - Transmission complete"]
pub type TcR = crate::BitReader;
#[doc = "Field `TXE_TXFNF` reader - Transmit data register empty"]
pub type TxeTxfnfR = crate::BitReader;
#[doc = "Field `LBD` reader - LIN break detection flag"]
pub type LbdR = crate::BitReader;
#[doc = "Field `LBD` writer - LIN break detection flag"]
pub type LbdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS` reader - cts flag"]
pub type CtsR = crate::BitReader;
#[doc = "Field `CTS` writer - cts flag"]
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRF` reader - auto baud rate flag"]
pub type AbrfR = crate::BitReader;
#[doc = "Field `ABRE` reader - auto baud rateerror"]
pub type AbreR = crate::BitReader;
#[doc = "Field `ABRQ` reader - auto baud rate req"]
pub type AbrqR = crate::BitReader;
#[doc = "Field `ABRRQ` reader - ABRRQ"]
pub type AbrrqR = crate::BitReader;
#[doc = "Field `ABRRQ` writer - ABRRQ"]
pub type AbrrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFT` reader - txfifo threshold flag"]
pub type TxftR = crate::BitReader;
#[doc = "Field `RXFT` reader - rxfifo threshoold flag"]
pub type RxftR = crate::BitReader;
#[doc = "Field `RXFF` reader - rxfifo full"]
pub type RxffR = crate::BitReader;
#[doc = "Field `TXFE` reader - txfifo empty"]
pub type TxfeR = crate::BitReader;
#[doc = "Field `EOBF` reader - end of block flag"]
pub type EobfR = crate::BitReader;
#[doc = "Field `EOBF` writer - end of block flag"]
pub type EobfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOF` reader - rto flag"]
pub type RtofR = crate::BitReader;
#[doc = "Field `RTOF` writer - rto flag"]
pub type RtofW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Parity error"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise error flag"]
    #[inline(always)]
    pub fn ne(&self) -> NeR {
        NeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - overrun error"]
    #[inline(always)]
    pub fn ore(&self) -> OreR {
        OreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - idle"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data register not empty"]
    #[inline(always)]
    pub fn rxne_rxfnf(&self) -> RxneRxfnfR {
        RxneRxfnfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data register empty"]
    #[inline(always)]
    pub fn txe_txfnf(&self) -> TxeTxfnfR {
        TxeTxfnfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    pub fn lbd(&self) -> LbdR {
        LbdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - cts flag"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - auto baud rate flag"]
    #[inline(always)]
    pub fn abrf(&self) -> AbrfR {
        AbrfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - auto baud rateerror"]
    #[inline(always)]
    pub fn abre(&self) -> AbreR {
        AbreR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - auto baud rate req"]
    #[inline(always)]
    pub fn abrq(&self) -> AbrqR {
        AbrqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 12 - ABRRQ"]
    #[inline(always)]
    pub fn abrrq(&self) -> AbrrqR {
        AbrrqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - txfifo threshold flag"]
    #[inline(always)]
    pub fn txft(&self) -> TxftR {
        TxftR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - rxfifo threshoold flag"]
    #[inline(always)]
    pub fn rxft(&self) -> RxftR {
        RxftR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - rxfifo full"]
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - txfifo empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - end of block flag"]
    #[inline(always)]
    pub fn eobf(&self) -> EobfR {
        EobfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - rto flag"]
    #[inline(always)]
    pub fn rtof(&self) -> RtofR {
        RtofR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    pub fn lbd(&mut self) -> LbdW<'_, SrSpec> {
        LbdW::new(self, 8)
    }
    #[doc = "Bit 9 - cts flag"]
    #[inline(always)]
    pub fn cts(&mut self) -> CtsW<'_, SrSpec> {
        CtsW::new(self, 9)
    }
    #[doc = "Bit 12 - ABRRQ"]
    #[inline(always)]
    pub fn abrrq(&mut self) -> AbrrqW<'_, SrSpec> {
        AbrrqW::new(self, 12)
    }
    #[doc = "Bit 18 - end of block flag"]
    #[inline(always)]
    pub fn eobf(&mut self) -> EobfW<'_, SrSpec> {
        EobfW::new(self, 18)
    }
    #[doc = "Bit 19 - rto flag"]
    #[inline(always)]
    pub fn rtof(&mut self) -> RtofW<'_, SrSpec> {
        RtofW::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
