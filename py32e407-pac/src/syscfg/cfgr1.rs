#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<Cfgr1Spec>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<Cfgr1Spec>;
#[doc = "Field `MEM_MODE` reader - memory mapping mode"]
pub type MemModeR = crate::FieldReader;
#[doc = "Field `MEM_MODE` writer - memory mapping mode"]
pub type MemModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ETH_PHYSEL` reader - Ethernet PHY Interface selection"]
pub type EthPhyselR = crate::BitReader;
#[doc = "Field `ETH_PHYSEL` writer - Ethernet PHY Interface selection"]
pub type EthPhyselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM2ITR1_REMAP` reader - timer2 inner trigger input 1 mapping"]
pub type Tim2itr1RemapR = crate::FieldReader;
#[doc = "Field `TIM2ITR1_REMAP` writer - timer2 inner trigger input 1 mapping"]
pub type Tim2itr1RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PTP_PPS_REMAP` reader - ETH PTP PPS mapping"]
pub type PtpPpsRemapR = crate::BitReader;
#[doc = "Field `PTP_PPS_REMAP` writer - ETH PTP PPS mapping"]
pub type PtpPpsRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTC_SOF_SEL` reader - selection of usb sof to ctc"]
pub type CtcSofSelR = crate::BitReader;
#[doc = "Field `CTC_SOF_SEL` writer - selection of usb sof to ctc"]
pub type CtcSofSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_FMP` reader - I2C1 Fast-mode Plus driving capability activation"]
pub type I2c1FmpR = crate::BitReader;
#[doc = "Field `I2C1_FMP` writer - I2C1 Fast-mode Plus driving capability activation"]
pub type I2c1FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_FMP` reader - I2C2 Fast-mode Plus driving capability activation"]
pub type I2c2FmpR = crate::BitReader;
#[doc = "Field `I2C2_FMP` writer - I2C2 Fast-mode Plus driving capability activation"]
pub type I2c2FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3_FMP` reader - I2C3 Fast-mode Plus driving capability activation"]
pub type I2c3FmpR = crate::BitReader;
#[doc = "Field `I2C3_FMP` writer - I2C3 Fast-mode Plus driving capability activation"]
pub type I2c3FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4_FMP` reader - I2C4 Fast-mode Plus driving capability activation"]
pub type I2c4FmpR = crate::BitReader;
#[doc = "Field `I2C4_FMP` writer - I2C4 Fast-mode Plus driving capability activation"]
pub type I2c4FmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_EIIC` reader - I2C analog filter enable"]
pub type I2cEiicR = crate::FieldReader<u16>;
#[doc = "Field `I2C_EIIC` writer - I2C analog filter enable"]
pub type I2cEiicW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - memory mapping mode"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MemModeR {
        MemModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Ethernet PHY Interface selection"]
    #[inline(always)]
    pub fn eth_physel(&self) -> EthPhyselR {
        EthPhyselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - timer2 inner trigger input 1 mapping"]
    #[inline(always)]
    pub fn tim2itr1_remap(&self) -> Tim2itr1RemapR {
        Tim2itr1RemapR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - ETH PTP PPS mapping"]
    #[inline(always)]
    pub fn ptp_pps_remap(&self) -> PtpPpsRemapR {
        PtpPpsRemapR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - selection of usb sof to ctc"]
    #[inline(always)]
    pub fn ctc_sof_sel(&self) -> CtcSofSelR {
        CtcSofSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C1 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2c1FmpR {
        I2c1FmpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C2 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2c2FmpR {
        I2c2FmpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C3 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2c3FmpR {
        I2c3FmpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C4 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c4_fmp(&self) -> I2c4FmpR {
        I2c4FmpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:31 - I2C analog filter enable"]
    #[inline(always)]
    pub fn i2c_eiic(&self) -> I2cEiicR {
        I2cEiicR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - memory mapping mode"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MemModeW<'_, Cfgr1Spec> {
        MemModeW::new(self, 0)
    }
    #[doc = "Bit 2 - Ethernet PHY Interface selection"]
    #[inline(always)]
    pub fn eth_physel(&mut self) -> EthPhyselW<'_, Cfgr1Spec> {
        EthPhyselW::new(self, 2)
    }
    #[doc = "Bits 3:4 - timer2 inner trigger input 1 mapping"]
    #[inline(always)]
    pub fn tim2itr1_remap(&mut self) -> Tim2itr1RemapW<'_, Cfgr1Spec> {
        Tim2itr1RemapW::new(self, 3)
    }
    #[doc = "Bit 5 - ETH PTP PPS mapping"]
    #[inline(always)]
    pub fn ptp_pps_remap(&mut self) -> PtpPpsRemapW<'_, Cfgr1Spec> {
        PtpPpsRemapW::new(self, 5)
    }
    #[doc = "Bit 6 - selection of usb sof to ctc"]
    #[inline(always)]
    pub fn ctc_sof_sel(&mut self) -> CtcSofSelW<'_, Cfgr1Spec> {
        CtcSofSelW::new(self, 6)
    }
    #[doc = "Bit 7 - I2C1 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2c1FmpW<'_, Cfgr1Spec> {
        I2c1FmpW::new(self, 7)
    }
    #[doc = "Bit 8 - I2C2 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2c2FmpW<'_, Cfgr1Spec> {
        I2c2FmpW::new(self, 8)
    }
    #[doc = "Bit 9 - I2C3 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2c3FmpW<'_, Cfgr1Spec> {
        I2c3FmpW::new(self, 9)
    }
    #[doc = "Bit 10 - I2C4 Fast-mode Plus driving capability activation"]
    #[inline(always)]
    pub fn i2c4_fmp(&mut self) -> I2c4FmpW<'_, Cfgr1Spec> {
        I2c4FmpW::new(self, 10)
    }
    #[doc = "Bits 16:31 - I2C analog filter enable"]
    #[inline(always)]
    pub fn i2c_eiic(&mut self) -> I2cEiicW<'_, Cfgr1Spec> {
        I2cEiicW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr1Spec;
impl crate::RegisterSpec for Cfgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for Cfgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for Cfgr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for Cfgr1Spec {}
