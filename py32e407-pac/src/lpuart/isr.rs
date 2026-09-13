#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `PE` reader - Parity error"]
pub type PeR = crate::BitReader;
#[doc = "Field `FE` reader - Frame error flag"]
pub type FeR = crate::BitReader;
#[doc = "Field `NE` reader - START bit noise flag"]
pub type NeR = crate::BitReader;
#[doc = "Field `ORE` reader - Overrun error flag"]
pub type OreR = crate::BitReader;
#[doc = "Field `IDLE` reader - idle flag"]
pub type IdleR = crate::BitReader;
#[doc = "Field `RXFNE` reader - RX register not empty"]
pub type RxfneR = crate::BitReader;
#[doc = "Field `TC` reader - Transmission completion flag"]
pub type TcR = crate::BitReader;
#[doc = "Field `TXFNF` reader - TX register not full"]
pub type TxfnfR = crate::BitReader;
#[doc = "Field `CTSIF` reader - CTS interrupt flag"]
pub type CtsifR = crate::BitReader;
#[doc = "Field `CTS` reader - CTS flag"]
pub type CtsR = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy flag"]
pub type BusyR = crate::BitReader;
#[doc = "Field `CMF` reader - Address match flag"]
pub type CmfR = crate::BitReader;
#[doc = "Field `SBKF` reader - Send abort flag"]
pub type SbkfR = crate::BitReader;
#[doc = "Field `RWU` reader - Receive Mute mode wake-up flag"]
pub type RwuR = crate::BitReader;
#[doc = "Field `WUF` reader - Wake flag from low-power mode"]
pub type WufR = crate::BitReader;
#[doc = "Field `TEACK` reader - Transport enable acknowledgement flag"]
pub type TeackR = crate::BitReader;
#[doc = "Field `REACK` reader - Receive the enable acknowledgment flag"]
pub type ReackR = crate::BitReader;
#[doc = "Field `TXFE` reader - TXFIFO empty"]
pub type TxfeR = crate::BitReader;
#[doc = "Field `RXFF` reader - RXFIFO full logo"]
pub type RxffR = crate::BitReader;
#[doc = "Field `RXFT` reader - RCFIFO threshold flag"]
pub type RxftR = crate::BitReader;
#[doc = "Field `TXFT` reader - TXFIFO threshold flag"]
pub type TxftR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Parity error"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame error flag"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - START bit noise flag"]
    #[inline(always)]
    pub fn ne(&self) -> NeR {
        NeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error flag"]
    #[inline(always)]
    pub fn ore(&self) -> OreR {
        OreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - idle flag"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX register not empty"]
    #[inline(always)]
    pub fn rxfne(&self) -> RxfneR {
        RxfneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission completion flag"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX register not full"]
    #[inline(always)]
    pub fn txfnf(&self) -> TxfnfR {
        TxfnfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS interrupt flag"]
    #[inline(always)]
    pub fn ctsif(&self) -> CtsifR {
        CtsifR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS flag"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy flag"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Address match flag"]
    #[inline(always)]
    pub fn cmf(&self) -> CmfR {
        CmfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Send abort flag"]
    #[inline(always)]
    pub fn sbkf(&self) -> SbkfR {
        SbkfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive Mute mode wake-up flag"]
    #[inline(always)]
    pub fn rwu(&self) -> RwuR {
        RwuR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wake flag from low-power mode"]
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transport enable acknowledgement flag"]
    #[inline(always)]
    pub fn teack(&self) -> TeackR {
        TeackR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive the enable acknowledgment flag"]
    #[inline(always)]
    pub fn reack(&self) -> ReackR {
        ReackR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TXFIFO empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RXFIFO full logo"]
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - RCFIFO threshold flag"]
    #[inline(always)]
    pub fn rxft(&self) -> RxftR {
        RxftR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TXFIFO threshold flag"]
    #[inline(always)]
    pub fn txft(&self) -> TxftR {
        TxftR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
