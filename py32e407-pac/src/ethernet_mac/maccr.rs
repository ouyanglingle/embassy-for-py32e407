#[doc = "Register `MACCR` reader"]
pub type R = crate::R<MaccrSpec>;
#[doc = "Register `MACCR` writer"]
pub type W = crate::W<MaccrSpec>;
#[doc = "Field `RE` reader - Receiver enable"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - Receiver enable"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC` reader - Deferral check"]
pub type DcR = crate::BitReader;
#[doc = "Field `DC` writer - Deferral check"]
pub type DcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL` reader - Back-off limit"]
pub type BlR = crate::FieldReader;
#[doc = "Field `BL` writer - Back-off limit"]
pub type BlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `APCS` reader - Automatic pad/CRC stripping"]
pub type ApcsR = crate::BitReader;
#[doc = "Field `APCS` writer - Automatic pad/CRC stripping"]
pub type ApcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - Retry disable"]
pub type RdR = crate::BitReader;
#[doc = "Field `RD` writer - Retry disable"]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPCO` reader - IPv4 checksum offload"]
pub type IpcoR = crate::BitReader;
#[doc = "Field `IPCO` writer - IPv4 checksum offload"]
pub type IpcoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM` reader - Duplex mode"]
pub type DmR = crate::BitReader;
#[doc = "Field `DM` writer - Duplex mode"]
pub type DmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LM` reader - Loopback mode"]
pub type LmR = crate::BitReader;
#[doc = "Field `LM` writer - Loopback mode"]
pub type LmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROD` reader - Receive own disable"]
pub type RodR = crate::BitReader;
#[doc = "Field `ROD` writer - Receive own disable"]
pub type RodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FES` reader - Fast Ethernet speed"]
pub type FesR = crate::BitReader;
#[doc = "Field `FES` writer - Fast Ethernet speed"]
pub type FesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSD` reader - Carrier sense disable"]
pub type CsdR = crate::BitReader;
#[doc = "Field `CSD` writer - Carrier sense disable"]
pub type CsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFG` reader - Interframe gap"]
pub type IfgR = crate::FieldReader;
#[doc = "Field `IFG` writer - Interframe gap"]
pub type IfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JD` reader - Jabber disable"]
pub type JdR = crate::BitReader;
#[doc = "Field `JD` writer - Jabber disable"]
pub type JdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD` reader - Watchdog disable"]
pub type WdR = crate::BitReader;
#[doc = "Field `WD` writer - Watchdog disable"]
pub type WdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
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
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    pub fn dc(&self) -> DcR {
        DcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    pub fn bl(&self) -> BlR {
        BlR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    pub fn apcs(&self) -> ApcsR {
        ApcsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    pub fn ipco(&self) -> IpcoR {
        IpcoR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    pub fn dm(&self) -> DmR {
        DmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    pub fn lm(&self) -> LmR {
        LmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive own disable"]
    #[inline(always)]
    pub fn rod(&self) -> RodR {
        RodR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Ethernet speed"]
    #[inline(always)]
    pub fn fes(&self) -> FesR {
        FesR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    pub fn csd(&self) -> CsdR {
        CsdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    pub fn ifg(&self) -> IfgR {
        IfgR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    pub fn jd(&self) -> JdR {
        JdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wd(&self) -> WdR {
        WdR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<'_, MaccrSpec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TeW<'_, MaccrSpec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    pub fn dc(&mut self) -> DcW<'_, MaccrSpec> {
        DcW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    pub fn bl(&mut self) -> BlW<'_, MaccrSpec> {
        BlW::new(self, 5)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    pub fn apcs(&mut self) -> ApcsW<'_, MaccrSpec> {
        ApcsW::new(self, 7)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    pub fn rd(&mut self) -> RdW<'_, MaccrSpec> {
        RdW::new(self, 9)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    pub fn ipco(&mut self) -> IpcoW<'_, MaccrSpec> {
        IpcoW::new(self, 10)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    pub fn dm(&mut self) -> DmW<'_, MaccrSpec> {
        DmW::new(self, 11)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    pub fn lm(&mut self) -> LmW<'_, MaccrSpec> {
        LmW::new(self, 12)
    }
    #[doc = "Bit 13 - Receive own disable"]
    #[inline(always)]
    pub fn rod(&mut self) -> RodW<'_, MaccrSpec> {
        RodW::new(self, 13)
    }
    #[doc = "Bit 14 - Fast Ethernet speed"]
    #[inline(always)]
    pub fn fes(&mut self) -> FesW<'_, MaccrSpec> {
        FesW::new(self, 14)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    pub fn csd(&mut self) -> CsdW<'_, MaccrSpec> {
        CsdW::new(self, 16)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    pub fn ifg(&mut self) -> IfgW<'_, MaccrSpec> {
        IfgW::new(self, 17)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    pub fn jd(&mut self) -> JdW<'_, MaccrSpec> {
        JdW::new(self, 22)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wd(&mut self) -> WdW<'_, MaccrSpec> {
        WdW::new(self, 23)
    }
}
#[doc = "Ethernet MAC configuration register (ETH_MACCR)\n\nYou can [`read`](crate::Reg::read) this register and get [`maccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaccrSpec;
impl crate::RegisterSpec for MaccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maccr::R`](R) reader structure"]
impl crate::Readable for MaccrSpec {}
#[doc = "`write(|w| ..)` method takes [`maccr::W`](W) writer structure"]
impl crate::Writable for MaccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACCR to value 0"]
impl crate::Resettable for MaccrSpec {}
