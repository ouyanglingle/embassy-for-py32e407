#[doc = "Register `DMASR` reader"]
pub type R = crate::R<DmasrSpec>;
#[doc = "Register `DMASR` writer"]
pub type W = crate::W<DmasrSpec>;
#[doc = "Field `TS` reader - Transmit status"]
pub type TsR = crate::BitReader;
#[doc = "Field `TS` writer - Transmit status"]
pub type TsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPSS` reader - Transmit process stopped status"]
pub type TpssR = crate::BitReader;
#[doc = "Field `TPSS` writer - Transmit process stopped status"]
pub type TpssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBUS` reader - Transmit buffer unavailable status"]
pub type TbusR = crate::BitReader;
#[doc = "Field `TBUS` writer - Transmit buffer unavailable status"]
pub type TbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TJTS` reader - Transmit jabber timeout status"]
pub type TjtsR = crate::BitReader;
#[doc = "Field `TJTS` writer - Transmit jabber timeout status"]
pub type TjtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROS` reader - Receive overflow status"]
pub type RosR = crate::BitReader;
#[doc = "Field `ROS` writer - Receive overflow status"]
pub type RosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUS` reader - Transmit underflow status"]
pub type TusR = crate::BitReader;
#[doc = "Field `TUS` writer - Transmit underflow status"]
pub type TusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS` reader - Receive status"]
pub type RsR = crate::BitReader;
#[doc = "Field `RS` writer - Receive status"]
pub type RsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBUS` reader - Receive buffer unavailable status"]
pub type RbusR = crate::BitReader;
#[doc = "Field `RBUS` writer - Receive buffer unavailable status"]
pub type RbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPSS` reader - Receive process stopped status"]
pub type RpssR = crate::BitReader;
#[doc = "Field `RPSS` writer - Receive process stopped status"]
pub type RpssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWTS` reader - Receive watchdog timeout status"]
pub type PwtsR = crate::BitReader;
#[doc = "Field `PWTS` writer - Receive watchdog timeout status"]
pub type PwtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETS` reader - Early transmit status"]
pub type EtsR = crate::BitReader;
#[doc = "Field `ETS` writer - Early transmit status"]
pub type EtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBES` reader - Fatal bus error status"]
pub type FbesR = crate::BitReader;
#[doc = "Field `FBES` writer - Fatal bus error status"]
pub type FbesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERS` reader - Early receive status"]
pub type ErsR = crate::BitReader;
#[doc = "Field `ERS` writer - Early receive status"]
pub type ErsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal interrupt summary"]
pub type AisR = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal interrupt summary"]
pub type AisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIS` reader - Normal interrupt summary"]
pub type NisR = crate::BitReader;
#[doc = "Field `NIS` writer - Normal interrupt summary"]
pub type NisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPS` reader - Receive process state"]
pub type RpsR = crate::FieldReader;
#[doc = "Field `TPS` reader - Transmit process state"]
pub type TpsR = crate::FieldReader;
#[doc = "Field `EBS` reader - Error bits status"]
pub type EbsR = crate::FieldReader;
#[doc = "Field `MMCS` reader - MMC status"]
pub type MmcsR = crate::BitReader;
#[doc = "Field `PMTS` reader - PMT status"]
pub type PmtsR = crate::BitReader;
#[doc = "Field `TSTS` reader - Time stamp trigger status"]
pub type TstsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit status"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit process stopped status"]
    #[inline(always)]
    pub fn tpss(&self) -> TpssR {
        TpssR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable status"]
    #[inline(always)]
    pub fn tbus(&self) -> TbusR {
        TbusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout status"]
    #[inline(always)]
    pub fn tjts(&self) -> TjtsR {
        TjtsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive overflow status"]
    #[inline(always)]
    pub fn ros(&self) -> RosR {
        RosR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit underflow status"]
    #[inline(always)]
    pub fn tus(&self) -> TusR {
        TusR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive status"]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable status"]
    #[inline(always)]
    pub fn rbus(&self) -> RbusR {
        RbusR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive process stopped status"]
    #[inline(always)]
    pub fn rpss(&self) -> RpssR {
        RpssR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive watchdog timeout status"]
    #[inline(always)]
    pub fn pwts(&self) -> PwtsR {
        PwtsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit status"]
    #[inline(always)]
    pub fn ets(&self) -> EtsR {
        EtsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error status"]
    #[inline(always)]
    pub fn fbes(&self) -> FbesR {
        FbesR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early receive status"]
    #[inline(always)]
    pub fn ers(&self) -> ErsR {
        ErsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    pub fn ais(&self) -> AisR {
        AisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    pub fn nis(&self) -> NisR {
        NisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Receive process state"]
    #[inline(always)]
    pub fn rps(&self) -> RpsR {
        RpsR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit process state"]
    #[inline(always)]
    pub fn tps(&self) -> TpsR {
        TpsR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error bits status"]
    #[inline(always)]
    pub fn ebs(&self) -> EbsR {
        EbsR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - MMC status"]
    #[inline(always)]
    pub fn mmcs(&self) -> MmcsR {
        MmcsR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PMT status"]
    #[inline(always)]
    pub fn pmts(&self) -> PmtsR {
        PmtsR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Time stamp trigger status"]
    #[inline(always)]
    pub fn tsts(&self) -> TstsR {
        TstsR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit status"]
    #[inline(always)]
    pub fn ts(&mut self) -> TsW<'_, DmasrSpec> {
        TsW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit process stopped status"]
    #[inline(always)]
    pub fn tpss(&mut self) -> TpssW<'_, DmasrSpec> {
        TpssW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable status"]
    #[inline(always)]
    pub fn tbus(&mut self) -> TbusW<'_, DmasrSpec> {
        TbusW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit jabber timeout status"]
    #[inline(always)]
    pub fn tjts(&mut self) -> TjtsW<'_, DmasrSpec> {
        TjtsW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive overflow status"]
    #[inline(always)]
    pub fn ros(&mut self) -> RosW<'_, DmasrSpec> {
        RosW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit underflow status"]
    #[inline(always)]
    pub fn tus(&mut self) -> TusW<'_, DmasrSpec> {
        TusW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive status"]
    #[inline(always)]
    pub fn rs(&mut self) -> RsW<'_, DmasrSpec> {
        RsW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive buffer unavailable status"]
    #[inline(always)]
    pub fn rbus(&mut self) -> RbusW<'_, DmasrSpec> {
        RbusW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive process stopped status"]
    #[inline(always)]
    pub fn rpss(&mut self) -> RpssW<'_, DmasrSpec> {
        RpssW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive watchdog timeout status"]
    #[inline(always)]
    pub fn pwts(&mut self) -> PwtsW<'_, DmasrSpec> {
        PwtsW::new(self, 9)
    }
    #[doc = "Bit 10 - Early transmit status"]
    #[inline(always)]
    pub fn ets(&mut self) -> EtsW<'_, DmasrSpec> {
        EtsW::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal bus error status"]
    #[inline(always)]
    pub fn fbes(&mut self) -> FbesW<'_, DmasrSpec> {
        FbesW::new(self, 13)
    }
    #[doc = "Bit 14 - Early receive status"]
    #[inline(always)]
    pub fn ers(&mut self) -> ErsW<'_, DmasrSpec> {
        ErsW::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    pub fn ais(&mut self) -> AisW<'_, DmasrSpec> {
        AisW::new(self, 15)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    pub fn nis(&mut self) -> NisW<'_, DmasrSpec> {
        NisW::new(self, 16)
    }
}
#[doc = "Ethernet DMA status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmasrSpec;
impl crate::RegisterSpec for DmasrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasr::R`](R) reader structure"]
impl crate::Readable for DmasrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmasr::W`](W) writer structure"]
impl crate::Writable for DmasrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMASR to value 0"]
impl crate::Resettable for DmasrSpec {}
