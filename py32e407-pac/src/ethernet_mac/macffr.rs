#[doc = "Register `MACFFR` reader"]
pub type R = crate::R<MacffrSpec>;
#[doc = "Register `MACFFR` writer"]
pub type W = crate::W<MacffrSpec>;
#[doc = "Field `PM` reader - Promiscuous mode"]
pub type PmR = crate::BitReader;
#[doc = "Field `PM` writer - Promiscuous mode"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HU` reader - Hash unicast"]
pub type HuR = crate::BitReader;
#[doc = "Field `HU` writer - Hash unicast"]
pub type HuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HM` reader - Hash multicast"]
pub type HmR = crate::BitReader;
#[doc = "Field `HM` writer - Hash multicast"]
pub type HmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAIF` reader - Destination address inverse filtering"]
pub type DaifR = crate::BitReader;
#[doc = "Field `DAIF` writer - Destination address inverse filtering"]
pub type DaifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAM` reader - Pass all multicast"]
pub type PamR = crate::BitReader;
#[doc = "Field `PAM` writer - Pass all multicast"]
pub type PamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFD` reader - Broadcast frames disable"]
pub type BfdR = crate::BitReader;
#[doc = "Field `BFD` writer - Broadcast frames disable"]
pub type BfdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCF` reader - Pass control frames"]
pub type PcfR = crate::FieldReader;
#[doc = "Field `PCF` writer - Pass control frames"]
pub type PcfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAIF` reader - Source address inverse filtering"]
pub type SaifR = crate::BitReader;
#[doc = "Field `SAIF` writer - Source address inverse filtering"]
pub type SaifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAF` reader - Source address filter"]
pub type SafR = crate::BitReader;
#[doc = "Field `SAF` writer - Source address filter"]
pub type SafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPF` reader - Hash or perfect filter"]
pub type HpfR = crate::BitReader;
#[doc = "Field `HPF` writer - Hash or perfect filter"]
pub type HpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTFE` reader - VTFE"]
pub type VtfeR = crate::BitReader;
#[doc = "Field `VTFE` writer - VTFE"]
pub type VtfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RA` reader - Receive all"]
pub type RaR = crate::BitReader;
#[doc = "Field `RA` writer - Receive all"]
pub type RaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    pub fn hu(&self) -> HuR {
        HuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    pub fn hm(&self) -> HmR {
        HmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline(always)]
    pub fn daif(&self) -> DaifR {
        DaifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass all multicast"]
    #[inline(always)]
    pub fn pam(&self) -> PamR {
        PamR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Broadcast frames disable"]
    #[inline(always)]
    pub fn bfd(&self) -> BfdR {
        BfdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    pub fn pcf(&self) -> PcfR {
        PcfR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline(always)]
    pub fn saif(&self) -> SaifR {
        SaifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline(always)]
    pub fn saf(&self) -> SafR {
        SafR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline(always)]
    pub fn hpf(&self) -> HpfR {
        HpfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - VTFE"]
    #[inline(always)]
    pub fn vtfe(&self) -> VtfeR {
        VtfeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<'_, MacffrSpec> {
        PmW::new(self, 0)
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    pub fn hu(&mut self) -> HuW<'_, MacffrSpec> {
        HuW::new(self, 1)
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    pub fn hm(&mut self) -> HmW<'_, MacffrSpec> {
        HmW::new(self, 2)
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline(always)]
    pub fn daif(&mut self) -> DaifW<'_, MacffrSpec> {
        DaifW::new(self, 3)
    }
    #[doc = "Bit 4 - Pass all multicast"]
    #[inline(always)]
    pub fn pam(&mut self) -> PamW<'_, MacffrSpec> {
        PamW::new(self, 4)
    }
    #[doc = "Bit 5 - Broadcast frames disable"]
    #[inline(always)]
    pub fn bfd(&mut self) -> BfdW<'_, MacffrSpec> {
        BfdW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    pub fn pcf(&mut self) -> PcfW<'_, MacffrSpec> {
        PcfW::new(self, 6)
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline(always)]
    pub fn saif(&mut self) -> SaifW<'_, MacffrSpec> {
        SaifW::new(self, 8)
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline(always)]
    pub fn saf(&mut self) -> SafW<'_, MacffrSpec> {
        SafW::new(self, 9)
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline(always)]
    pub fn hpf(&mut self) -> HpfW<'_, MacffrSpec> {
        HpfW::new(self, 10)
    }
    #[doc = "Bit 16 - VTFE"]
    #[inline(always)]
    pub fn vtfe(&mut self) -> VtfeW<'_, MacffrSpec> {
        VtfeW::new(self, 16)
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    pub fn ra(&mut self) -> RaW<'_, MacffrSpec> {
        RaW::new(self, 31)
    }
}
#[doc = "Ethernet MAC frame filter register (ETH_MACCFFR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macffr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macffr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacffrSpec;
impl crate::RegisterSpec for MacffrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macffr::R`](R) reader structure"]
impl crate::Readable for MacffrSpec {}
#[doc = "`write(|w| ..)` method takes [`macffr::W`](W) writer structure"]
impl crate::Writable for MacffrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACFFR to value 0"]
impl crate::Resettable for MacffrSpec {}
