#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `RXNE` reader - Receive register not empty"]
pub type RxneR = crate::BitReader;
#[doc = "Field `ORE` reader - Overrun error bit"]
pub type OreR = crate::BitReader;
#[doc = "Field `ORE` writer - Overrun error bit"]
pub type OreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` reader - Parity Error bit"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - Parity Error bit"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FE` reader - Framing Error bit"]
pub type FeR = crate::BitReader;
#[doc = "Field `FE` writer - Framing Error bit"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRI` reader - Break Interrupt bit"]
pub type BriR = crate::BitReader;
#[doc = "Field `BRI` writer - Break Interrupt bit"]
pub type BriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDRE` reader - Transmit Holding Register Empty bit"]
pub type TdreR = crate::BitReader;
#[doc = "Field `TXE` reader - Transmitter Empty bit"]
pub type TxeR = crate::BitReader;
#[doc = "Field `ADDR_RCVD` reader - Address Received Bit"]
pub type AddrRcvdR = crate::BitReader;
#[doc = "Field `ADDR_RCVD` writer - Address Received Bit"]
pub type AddrRcvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - UART Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY_ERR` reader - Busy Detect Error"]
pub type BusyErrR = crate::BitReader;
#[doc = "Field `BUSY_ERR` writer - Busy Detect Error"]
pub type BusyErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive register not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun error bit"]
    #[inline(always)]
    pub fn ore(&self) -> OreR {
        OreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error bit"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing Error bit"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt bit"]
    #[inline(always)]
    pub fn bri(&self) -> BriR {
        BriR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Holding Register Empty bit"]
    #[inline(always)]
    pub fn tdre(&self) -> TdreR {
        TdreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty bit"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Address Received Bit"]
    #[inline(always)]
    pub fn addr_rcvd(&self) -> AddrRcvdR {
        AddrRcvdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Busy Detect Error"]
    #[inline(always)]
    pub fn busy_err(&self) -> BusyErrR {
        BusyErrR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Overrun error bit"]
    #[inline(always)]
    pub fn ore(&mut self) -> OreW<'_, SrSpec> {
        OreW::new(self, 1)
    }
    #[doc = "Bit 2 - Parity Error bit"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, SrSpec> {
        PeW::new(self, 2)
    }
    #[doc = "Bit 3 - Framing Error bit"]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<'_, SrSpec> {
        FeW::new(self, 3)
    }
    #[doc = "Bit 4 - Break Interrupt bit"]
    #[inline(always)]
    pub fn bri(&mut self) -> BriW<'_, SrSpec> {
        BriW::new(self, 4)
    }
    #[doc = "Bit 8 - Address Received Bit"]
    #[inline(always)]
    pub fn addr_rcvd(&mut self) -> AddrRcvdW<'_, SrSpec> {
        AddrRcvdW::new(self, 8)
    }
    #[doc = "Bit 10 - Busy Detect Error"]
    #[inline(always)]
    pub fn busy_err(&mut self) -> BusyErrW<'_, SrSpec> {
        BusyErrW::new(self, 10)
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
