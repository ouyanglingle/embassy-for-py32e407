#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `PE` reader - desc PE"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - desc PE"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBUS` reader - desc SMBUS"]
pub type SmbusR = crate::BitReader;
#[doc = "Field `SMBUS` writer - desc SMBUS"]
pub type SmbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN` reader - WUPEN"]
pub type WupenR = crate::BitReader;
#[doc = "Field `WUPEN` writer - WUPEN"]
pub type WupenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBTYPE` reader - desc SMBTYPE"]
pub type SmbtypeR = crate::BitReader;
#[doc = "Field `SMBTYPE` writer - desc SMBTYPE"]
pub type SmbtypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENARP` reader - desc ENARP"]
pub type EnarpR = crate::BitReader;
#[doc = "Field `ENARP` writer - desc ENARP"]
pub type EnarpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENPEC` reader - desc ENPEC"]
pub type EnpecR = crate::BitReader;
#[doc = "Field `ENPEC` writer - desc ENPEC"]
pub type EnpecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENGC` reader - desc ENGC"]
pub type EngcR = crate::BitReader;
#[doc = "Field `ENGC` writer - desc ENGC"]
pub type EngcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOSTRETCH` reader - desc NOSTRETCH"]
pub type NostretchR = crate::BitReader;
#[doc = "Field `NOSTRETCH` writer - desc NOSTRETCH"]
pub type NostretchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - desc START"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - desc START"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - desc STOP"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - desc STOP"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - desc ACK"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - desc ACK"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS` reader - desc POS"]
pub type PosR = crate::BitReader;
#[doc = "Field `POS` writer - desc POS"]
pub type PosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEC` reader - desc PEC"]
pub type PecR = crate::BitReader;
#[doc = "Field `PEC` writer - desc PEC"]
pub type PecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERT` reader - desc ALERT"]
pub type AlertR = crate::BitReader;
#[doc = "Field `ALERT` writer - desc ALERT"]
pub type AlertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` reader - desc SWRST"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - desc SWRST"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUP_DIV` reader - WKUP_DIV"]
pub type WkupDivR = crate::FieldReader;
#[doc = "Field `WKUP_DIV` writer - WKUP_DIV"]
pub type WkupDivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WKUP_CNT` reader - WKUP_CNT"]
pub type WkupCntR = crate::FieldReader;
#[doc = "Field `WKUP_CNT` writer - WKUP_CNT"]
pub type WkupCntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc PE"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SMBUS"]
    #[inline(always)]
    pub fn smbus(&self) -> SmbusR {
        SmbusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WUPEN"]
    #[inline(always)]
    pub fn wupen(&self) -> WupenR {
        WupenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SMBTYPE"]
    #[inline(always)]
    pub fn smbtype(&self) -> SmbtypeR {
        SmbtypeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc ENARP"]
    #[inline(always)]
    pub fn enarp(&self) -> EnarpR {
        EnarpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ENPEC"]
    #[inline(always)]
    pub fn enpec(&self) -> EnpecR {
        EnpecR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ENGC"]
    #[inline(always)]
    pub fn engc(&self) -> EngcR {
        EngcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc NOSTRETCH"]
    #[inline(always)]
    pub fn nostretch(&self) -> NostretchR {
        NostretchR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ACK"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc POS"]
    #[inline(always)]
    pub fn pos(&self) -> PosR {
        PosR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PEC"]
    #[inline(always)]
    pub fn pec(&self) -> PecR {
        PecR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc ALERT"]
    #[inline(always)]
    pub fn alert(&self) -> AlertR {
        AlertR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - desc SWRST"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - WKUP_DIV"]
    #[inline(always)]
    pub fn wkup_div(&self) -> WkupDivR {
        WkupDivR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - WKUP_CNT"]
    #[inline(always)]
    pub fn wkup_cnt(&self) -> WkupCntR {
        WkupCntR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc PE"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, Cr1Spec> {
        PeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SMBUS"]
    #[inline(always)]
    pub fn smbus(&mut self) -> SmbusW<'_, Cr1Spec> {
        SmbusW::new(self, 1)
    }
    #[doc = "Bit 2 - WUPEN"]
    #[inline(always)]
    pub fn wupen(&mut self) -> WupenW<'_, Cr1Spec> {
        WupenW::new(self, 2)
    }
    #[doc = "Bit 3 - desc SMBTYPE"]
    #[inline(always)]
    pub fn smbtype(&mut self) -> SmbtypeW<'_, Cr1Spec> {
        SmbtypeW::new(self, 3)
    }
    #[doc = "Bit 4 - desc ENARP"]
    #[inline(always)]
    pub fn enarp(&mut self) -> EnarpW<'_, Cr1Spec> {
        EnarpW::new(self, 4)
    }
    #[doc = "Bit 5 - desc ENPEC"]
    #[inline(always)]
    pub fn enpec(&mut self) -> EnpecW<'_, Cr1Spec> {
        EnpecW::new(self, 5)
    }
    #[doc = "Bit 6 - desc ENGC"]
    #[inline(always)]
    pub fn engc(&mut self) -> EngcW<'_, Cr1Spec> {
        EngcW::new(self, 6)
    }
    #[doc = "Bit 7 - desc NOSTRETCH"]
    #[inline(always)]
    pub fn nostretch(&mut self) -> NostretchW<'_, Cr1Spec> {
        NostretchW::new(self, 7)
    }
    #[doc = "Bit 8 - desc START"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, Cr1Spec> {
        StartW::new(self, 8)
    }
    #[doc = "Bit 9 - desc STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, Cr1Spec> {
        StopW::new(self, 9)
    }
    #[doc = "Bit 10 - desc ACK"]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<'_, Cr1Spec> {
        AckW::new(self, 10)
    }
    #[doc = "Bit 11 - desc POS"]
    #[inline(always)]
    pub fn pos(&mut self) -> PosW<'_, Cr1Spec> {
        PosW::new(self, 11)
    }
    #[doc = "Bit 12 - desc PEC"]
    #[inline(always)]
    pub fn pec(&mut self) -> PecW<'_, Cr1Spec> {
        PecW::new(self, 12)
    }
    #[doc = "Bit 13 - desc ALERT"]
    #[inline(always)]
    pub fn alert(&mut self) -> AlertW<'_, Cr1Spec> {
        AlertW::new(self, 13)
    }
    #[doc = "Bit 15 - desc SWRST"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, Cr1Spec> {
        SwrstW::new(self, 15)
    }
    #[doc = "Bits 16:17 - WKUP_DIV"]
    #[inline(always)]
    pub fn wkup_div(&mut self) -> WkupDivW<'_, Cr1Spec> {
        WkupDivW::new(self, 16)
    }
    #[doc = "Bits 18:19 - WKUP_CNT"]
    #[inline(always)]
    pub fn wkup_cnt(&mut self) -> WkupCntW<'_, Cr1Spec> {
        WkupCntW::new(self, 18)
    }
}
#[doc = "desc CR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
