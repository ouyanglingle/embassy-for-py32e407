#[doc = "Register `MACFCR` reader"]
pub type R = crate::R<MacfcrSpec>;
#[doc = "Register `MACFCR` writer"]
pub type W = crate::W<MacfcrSpec>;
#[doc = "Field `FCB_BPA` reader - Flow control busy/back pressure activate"]
pub type FcbBpaR = crate::BitReader;
#[doc = "Field `FCB_BPA` writer - Flow control busy/back pressure activate"]
pub type FcbBpaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFCE` reader - Transmit flow control enable"]
pub type TfceR = crate::BitReader;
#[doc = "Field `TFCE` writer - Transmit flow control enable"]
pub type TfceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFCE` reader - Receive flow control enable"]
pub type RfceR = crate::BitReader;
#[doc = "Field `RFCE` writer - Receive flow control enable"]
pub type RfceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPFD` reader - Unicast pause frame detect"]
pub type UpfdR = crate::BitReader;
#[doc = "Field `UPFD` writer - Unicast pause frame detect"]
pub type UpfdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLT` reader - Pause low threshold"]
pub type PltR = crate::FieldReader;
#[doc = "Field `PLT` writer - Pause low threshold"]
pub type PltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ZQPD` reader - Zero-quanta pause disable"]
pub type ZqpdR = crate::BitReader;
#[doc = "Field `ZQPD` writer - Zero-quanta pause disable"]
pub type ZqpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT` reader - Pass control frames"]
pub type PtR = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - Pass control frames"]
pub type PtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FcbBpaR {
        FcbBpaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit flow control enable"]
    #[inline(always)]
    pub fn tfce(&self) -> TfceR {
        TfceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive flow control enable"]
    #[inline(always)]
    pub fn rfce(&self) -> RfceR {
        RfceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unicast pause frame detect"]
    #[inline(always)]
    pub fn upfd(&self) -> UpfdR {
        UpfdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PltR {
        PltR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Zero-quanta pause disable"]
    #[inline(always)]
    pub fn zqpd(&self) -> ZqpdR {
        ZqpdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pass control frames"]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    pub fn fcb_bpa(&mut self) -> FcbBpaW<'_, MacfcrSpec> {
        FcbBpaW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit flow control enable"]
    #[inline(always)]
    pub fn tfce(&mut self) -> TfceW<'_, MacfcrSpec> {
        TfceW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive flow control enable"]
    #[inline(always)]
    pub fn rfce(&mut self) -> RfceW<'_, MacfcrSpec> {
        RfceW::new(self, 2)
    }
    #[doc = "Bit 3 - Unicast pause frame detect"]
    #[inline(always)]
    pub fn upfd(&mut self) -> UpfdW<'_, MacfcrSpec> {
        UpfdW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    pub fn plt(&mut self) -> PltW<'_, MacfcrSpec> {
        PltW::new(self, 4)
    }
    #[doc = "Bit 7 - Zero-quanta pause disable"]
    #[inline(always)]
    pub fn zqpd(&mut self) -> ZqpdW<'_, MacfcrSpec> {
        ZqpdW::new(self, 7)
    }
    #[doc = "Bits 16:31 - Pass control frames"]
    #[inline(always)]
    pub fn pt(&mut self) -> PtW<'_, MacfcrSpec> {
        PtW::new(self, 16)
    }
}
#[doc = "Ethernet MAC flow control register (ETH_MACFCR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacfcrSpec;
impl crate::RegisterSpec for MacfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macfcr::R`](R) reader structure"]
impl crate::Readable for MacfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`macfcr::W`](W) writer structure"]
impl crate::Writable for MacfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACFCR to value 0"]
impl crate::Resettable for MacfcrSpec {}
