#[doc = "Register `MACPMTCSR` reader"]
pub type R = crate::R<MacpmtcsrSpec>;
#[doc = "Register `MACPMTCSR` writer"]
pub type W = crate::W<MacpmtcsrSpec>;
#[doc = "Field `PD` reader - Power down"]
pub type PdR = crate::BitReader;
#[doc = "Field `PD` writer - Power down"]
pub type PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPE` reader - Magic Packet enable"]
pub type MpeR = crate::BitReader;
#[doc = "Field `MPE` writer - Magic Packet enable"]
pub type MpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFE` reader - Wakeup frame enable"]
pub type WfeR = crate::BitReader;
#[doc = "Field `WFE` writer - Wakeup frame enable"]
pub type WfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPR` reader - Magic packet received"]
pub type MprR = crate::BitReader;
#[doc = "Field `MPR` writer - Magic packet received"]
pub type MprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFR` reader - Wakeup frame received"]
pub type WfrR = crate::BitReader;
#[doc = "Field `WFR` writer - Wakeup frame received"]
pub type WfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GU` reader - Global unicast"]
pub type GuR = crate::BitReader;
#[doc = "Field `GU` writer - Global unicast"]
pub type GuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFFRPR` reader - Wakeup frame filter register pointer reset"]
pub type WffrprR = crate::BitReader;
#[doc = "Field `WFFRPR` writer - Wakeup frame filter register pointer reset"]
pub type WffrprW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet enable"]
    #[inline(always)]
    pub fn mpe(&self) -> MpeR {
        MpeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    pub fn wfe(&self) -> WfeR {
        WfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    pub fn mpr(&self) -> MprR {
        MprR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    pub fn wfr(&self) -> WfrR {
        WfrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn gu(&self) -> GuR {
        GuR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn wffrpr(&self) -> WffrprR {
        WffrprR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pd(&mut self) -> PdW<'_, MacpmtcsrSpec> {
        PdW::new(self, 0)
    }
    #[doc = "Bit 1 - Magic Packet enable"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MpeW<'_, MacpmtcsrSpec> {
        MpeW::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    pub fn wfe(&mut self) -> WfeW<'_, MacpmtcsrSpec> {
        WfeW::new(self, 2)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    pub fn mpr(&mut self) -> MprW<'_, MacpmtcsrSpec> {
        MprW::new(self, 5)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    pub fn wfr(&mut self) -> WfrW<'_, MacpmtcsrSpec> {
        WfrW::new(self, 6)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn gu(&mut self) -> GuW<'_, MacpmtcsrSpec> {
        GuW::new(self, 9)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn wffrpr(&mut self) -> WffrprW<'_, MacpmtcsrSpec> {
        WffrprW::new(self, 31)
    }
}
#[doc = "Ethernet MAC PMT control and status register (ETH_MACPMTCSR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macpmtcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpmtcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacpmtcsrSpec;
impl crate::RegisterSpec for MacpmtcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macpmtcsr::R`](R) reader structure"]
impl crate::Readable for MacpmtcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`macpmtcsr::W`](W) writer structure"]
impl crate::Writable for MacpmtcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACPMTCSR to value 0"]
impl crate::Resettable for MacpmtcsrSpec {}
