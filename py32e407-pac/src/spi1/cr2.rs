#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `RXDMAEN` reader - Rx buffer DMA enable"]
pub type RxdmaenR = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - Rx buffer DMA enable"]
pub type RxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAEN` reader - Tx buffer DMA enable"]
pub type TxdmaenR = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - Tx buffer DMA enable"]
pub type TxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSOE` reader - SS Output Enable"]
pub type SsoeR = crate::BitReader;
#[doc = "Field `SSOE` writer - SS Output Enable"]
pub type SsoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRTXFIFO` reader - Clear FIFO"]
pub type ClrtxfifoR = crate::BitReader;
#[doc = "Field `CLRTXFIFO` writer - Clear FIFO"]
pub type ClrtxfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error Interrupt Enable"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error Interrupt Enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNEIE` reader - RX buffer Not Empty Interrupt Enable"]
pub type RxneieR = crate::BitReader;
#[doc = "Field `RXNEIE` writer - RX buffer Not Empty Interrupt Enable"]
pub type RxneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEIE` reader - Tx buffer Empty Interrupt Enable"]
pub type TxeieR = crate::BitReader;
#[doc = "Field `TXEIE` writer - Tx buffer Empty Interrupt Enable"]
pub type TxeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRXTH` reader - "]
pub type FrxthR = crate::BitReader;
#[doc = "Field `FRXTH` writer - "]
pub type FrxthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRF` reader - "]
pub type FrfR = crate::BitReader;
#[doc = "Field `FRF` writer - "]
pub type FrfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TxdmaenR {
        TxdmaenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS Output Enable"]
    #[inline(always)]
    pub fn ssoe(&self) -> SsoeR {
        SsoeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear FIFO"]
    #[inline(always)]
    pub fn clrtxfifo(&self) -> ClrtxfifoR {
        ClrtxfifoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX buffer Not Empty Interrupt Enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RxneieR {
        RxneieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TxeieR {
        TxeieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn frxth(&self) -> FrxthR {
        FrxthR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RxdmaenW<'_, Cr2Spec> {
        RxdmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TxdmaenW<'_, Cr2Spec> {
        TxdmaenW::new(self, 1)
    }
    #[doc = "Bit 2 - SS Output Enable"]
    #[inline(always)]
    pub fn ssoe(&mut self) -> SsoeW<'_, Cr2Spec> {
        SsoeW::new(self, 2)
    }
    #[doc = "Bit 4 - Clear FIFO"]
    #[inline(always)]
    pub fn clrtxfifo(&mut self) -> ClrtxfifoW<'_, Cr2Spec> {
        ClrtxfifoW::new(self, 4)
    }
    #[doc = "Bit 5 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, Cr2Spec> {
        ErrieW::new(self, 5)
    }
    #[doc = "Bit 6 - RX buffer Not Empty Interrupt Enable"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RxneieW<'_, Cr2Spec> {
        RxneieW::new(self, 6)
    }
    #[doc = "Bit 7 - Tx buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn txeie(&mut self) -> TxeieW<'_, Cr2Spec> {
        TxeieW::new(self, 7)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn frxth(&mut self) -> FrxthW<'_, Cr2Spec> {
        FrxthW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn frf(&mut self) -> FrfW<'_, Cr2Spec> {
        FrfW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
