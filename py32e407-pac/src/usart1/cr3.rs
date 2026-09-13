#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `EIE` reader - error int enable"]
pub type EieR = crate::BitReader;
#[doc = "Field `EIE` writer - error int enable"]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IREN` reader - iren enable"]
pub type IrenR = crate::BitReader;
#[doc = "Field `IREN` writer - iren enable"]
pub type IrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRLP` reader - idra lowpower"]
pub type IrlpR = crate::BitReader;
#[doc = "Field `IRLP` writer - idra lowpower"]
pub type IrlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDSEL` reader - hdsel"]
pub type HdselR = crate::BitReader;
#[doc = "Field `HDSEL` writer - hdsel"]
pub type HdselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - scen nack enable"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - scen nack enable"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCEN` reader - scen enable"]
pub type ScenR = crate::BitReader;
#[doc = "Field `SCEN` writer - scen enable"]
pub type ScenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAR` reader - dma rx enable"]
pub type DmarR = crate::BitReader;
#[doc = "Field `DMAR` writer - dma rx enable"]
pub type DmarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAT` reader - dma tx enable"]
pub type DmatR = crate::BitReader;
#[doc = "Field `DMAT` writer - dma tx enable"]
pub type DmatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSE` reader - rts enable"]
pub type RtseR = crate::BitReader;
#[doc = "Field `RTSE` writer - rts enable"]
pub type RtseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSE` reader - cts enable"]
pub type CtseR = crate::BitReader;
#[doc = "Field `CTSE` writer - cts enable"]
pub type CtseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIE` reader - cts int enable"]
pub type CtsieR = crate::BitReader;
#[doc = "Field `CTSIE` writer - cts int enable"]
pub type CtsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVER8` reader - oversampling"]
pub type Over8R = crate::BitReader;
#[doc = "Field `OVER8` writer - oversampling"]
pub type Over8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABREN` reader - autobaud enable"]
pub type AbrenR = crate::BitReader;
#[doc = "Field `ABREN` writer - autobaud enable"]
pub type AbrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRMOD` reader - auto baud mod"]
pub type AbrmodR = crate::FieldReader;
#[doc = "Field `ABRMOD` writer - auto baud mod"]
pub type AbrmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXFFIE` reader - rxff int enable"]
pub type RxffieR = crate::BitReader;
#[doc = "Field `RXFFIE` writer - rxff int enable"]
pub type RxffieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFEIE` reader - txfe int enable"]
pub type TxfeieR = crate::BitReader;
#[doc = "Field `TXFEIE` writer - txfe int enable"]
pub type TxfeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFTIE` reader - rxft int enable"]
pub type RxftieR = crate::BitReader;
#[doc = "Field `RXFTIE` writer - rxft int enable"]
pub type RxftieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFTIE` reader - txft int enable"]
pub type TxftieR = crate::BitReader;
#[doc = "Field `TXFTIE` writer - txft int enable"]
pub type TxftieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDT` reader - dedt int enable"]
pub type DedtR = crate::FieldReader;
#[doc = "Field `DEDT` writer - dedt int enable"]
pub type DedtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEAT` reader - deat int enable"]
pub type DeatR = crate::FieldReader;
#[doc = "Field `DEAT` writer - deat int enable"]
pub type DeatW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RTOIE` reader - rto int enable"]
pub type RtoieR = crate::BitReader;
#[doc = "Field `RTOIE` writer - rto int enable"]
pub type RtoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOBIE` reader - eob int enable"]
pub type EobieR = crate::BitReader;
#[doc = "Field `EOBIE` writer - eob int enable"]
pub type EobieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - error int enable"]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - iren enable"]
    #[inline(always)]
    pub fn iren(&self) -> IrenR {
        IrenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - idra lowpower"]
    #[inline(always)]
    pub fn irlp(&self) -> IrlpR {
        IrlpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hdsel"]
    #[inline(always)]
    pub fn hdsel(&self) -> HdselR {
        HdselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - scen nack enable"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - scen enable"]
    #[inline(always)]
    pub fn scen(&self) -> ScenR {
        ScenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - dma rx enable"]
    #[inline(always)]
    pub fn dmar(&self) -> DmarR {
        DmarR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - dma tx enable"]
    #[inline(always)]
    pub fn dmat(&self) -> DmatR {
        DmatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - rts enable"]
    #[inline(always)]
    pub fn rtse(&self) -> RtseR {
        RtseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - cts enable"]
    #[inline(always)]
    pub fn ctse(&self) -> CtseR {
        CtseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - cts int enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CtsieR {
        CtsieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - oversampling"]
    #[inline(always)]
    pub fn over8(&self) -> Over8R {
        Over8R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - autobaud enable"]
    #[inline(always)]
    pub fn abren(&self) -> AbrenR {
        AbrenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - auto baud mod"]
    #[inline(always)]
    pub fn abrmod(&self) -> AbrmodR {
        AbrmodR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - rxff int enable"]
    #[inline(always)]
    pub fn rxffie(&self) -> RxffieR {
        RxffieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - txfe int enable"]
    #[inline(always)]
    pub fn txfeie(&self) -> TxfeieR {
        TxfeieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - rxft int enable"]
    #[inline(always)]
    pub fn rxftie(&self) -> RxftieR {
        RxftieR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - txft int enable"]
    #[inline(always)]
    pub fn txftie(&self) -> TxftieR {
        TxftieR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - dedt int enable"]
    #[inline(always)]
    pub fn dedt(&self) -> DedtR {
        DedtR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - deat int enable"]
    #[inline(always)]
    pub fn deat(&self) -> DeatR {
        DeatR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - rto int enable"]
    #[inline(always)]
    pub fn rtoie(&self) -> RtoieR {
        RtoieR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - eob int enable"]
    #[inline(always)]
    pub fn eobie(&self) -> EobieR {
        EobieR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - error int enable"]
    #[inline(always)]
    pub fn eie(&mut self) -> EieW<'_, Cr3Spec> {
        EieW::new(self, 0)
    }
    #[doc = "Bit 1 - iren enable"]
    #[inline(always)]
    pub fn iren(&mut self) -> IrenW<'_, Cr3Spec> {
        IrenW::new(self, 1)
    }
    #[doc = "Bit 2 - idra lowpower"]
    #[inline(always)]
    pub fn irlp(&mut self) -> IrlpW<'_, Cr3Spec> {
        IrlpW::new(self, 2)
    }
    #[doc = "Bit 3 - hdsel"]
    #[inline(always)]
    pub fn hdsel(&mut self) -> HdselW<'_, Cr3Spec> {
        HdselW::new(self, 3)
    }
    #[doc = "Bit 4 - scen nack enable"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<'_, Cr3Spec> {
        NackW::new(self, 4)
    }
    #[doc = "Bit 5 - scen enable"]
    #[inline(always)]
    pub fn scen(&mut self) -> ScenW<'_, Cr3Spec> {
        ScenW::new(self, 5)
    }
    #[doc = "Bit 6 - dma rx enable"]
    #[inline(always)]
    pub fn dmar(&mut self) -> DmarW<'_, Cr3Spec> {
        DmarW::new(self, 6)
    }
    #[doc = "Bit 7 - dma tx enable"]
    #[inline(always)]
    pub fn dmat(&mut self) -> DmatW<'_, Cr3Spec> {
        DmatW::new(self, 7)
    }
    #[doc = "Bit 8 - rts enable"]
    #[inline(always)]
    pub fn rtse(&mut self) -> RtseW<'_, Cr3Spec> {
        RtseW::new(self, 8)
    }
    #[doc = "Bit 9 - cts enable"]
    #[inline(always)]
    pub fn ctse(&mut self) -> CtseW<'_, Cr3Spec> {
        CtseW::new(self, 9)
    }
    #[doc = "Bit 10 - cts int enable"]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CtsieW<'_, Cr3Spec> {
        CtsieW::new(self, 10)
    }
    #[doc = "Bit 11 - oversampling"]
    #[inline(always)]
    pub fn over8(&mut self) -> Over8W<'_, Cr3Spec> {
        Over8W::new(self, 11)
    }
    #[doc = "Bit 12 - autobaud enable"]
    #[inline(always)]
    pub fn abren(&mut self) -> AbrenW<'_, Cr3Spec> {
        AbrenW::new(self, 12)
    }
    #[doc = "Bits 13:14 - auto baud mod"]
    #[inline(always)]
    pub fn abrmod(&mut self) -> AbrmodW<'_, Cr3Spec> {
        AbrmodW::new(self, 13)
    }
    #[doc = "Bit 15 - rxff int enable"]
    #[inline(always)]
    pub fn rxffie(&mut self) -> RxffieW<'_, Cr3Spec> {
        RxffieW::new(self, 15)
    }
    #[doc = "Bit 16 - txfe int enable"]
    #[inline(always)]
    pub fn txfeie(&mut self) -> TxfeieW<'_, Cr3Spec> {
        TxfeieW::new(self, 16)
    }
    #[doc = "Bit 17 - rxft int enable"]
    #[inline(always)]
    pub fn rxftie(&mut self) -> RxftieW<'_, Cr3Spec> {
        RxftieW::new(self, 17)
    }
    #[doc = "Bit 18 - txft int enable"]
    #[inline(always)]
    pub fn txftie(&mut self) -> TxftieW<'_, Cr3Spec> {
        TxftieW::new(self, 18)
    }
    #[doc = "Bits 19:23 - dedt int enable"]
    #[inline(always)]
    pub fn dedt(&mut self) -> DedtW<'_, Cr3Spec> {
        DedtW::new(self, 19)
    }
    #[doc = "Bits 24:28 - deat int enable"]
    #[inline(always)]
    pub fn deat(&mut self) -> DeatW<'_, Cr3Spec> {
        DeatW::new(self, 24)
    }
    #[doc = "Bit 29 - rto int enable"]
    #[inline(always)]
    pub fn rtoie(&mut self) -> RtoieW<'_, Cr3Spec> {
        RtoieW::new(self, 29)
    }
    #[doc = "Bit 30 - eob int enable"]
    #[inline(always)]
    pub fn eobie(&mut self) -> EobieW<'_, Cr3Spec> {
        EobieW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for Cr3Spec {}
