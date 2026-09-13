#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `ADDM7` reader - 7bit/4bit address detection configuration"]
pub type Addm7R = crate::BitReader;
#[doc = "Field `ADDM7` writer - 7bit/4bit address detection configuration"]
pub type Addm7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Stop bit configuration"]
pub type StopR = crate::FieldReader;
#[doc = "Field `STOP` writer - Stop bit configuration"]
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWAP` reader - TX/RX pin interchangeable"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - TX/RX pin interchangeable"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINV` reader - The RX pin is active level inverting"]
pub type RxinvR = crate::BitReader;
#[doc = "Field `RXINV` writer - The RX pin is active level inverting"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINV` reader - TX pin active level inverted"]
pub type TxinvR = crate::BitReader;
#[doc = "Field `TXINV` writer - TX pin active level inverted"]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAINV` reader - Inverse binary data processing"]
pub type DatainvR = crate::BitReader;
#[doc = "Field `DATAINV` writer - Inverse binary data processing"]
pub type DatainvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSBFIRST` reader - MSB first"]
pub type MsbfirstR = crate::BitReader;
#[doc = "Field `MSBFIRST` writer - MSB first"]
pub type MsbfirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOE_ALWAYS_ON` reader - "]
pub type TxoeAlwaysOnR = crate::BitReader;
#[doc = "Field `TXOE_ALWAYS_ON` writer - "]
pub type TxoeAlwaysOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD` reader - LPUART address"]
pub type AddR = crate::FieldReader;
#[doc = "Field `ADD` writer - LPUART address"]
pub type AddW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 4 - 7bit/4bit address detection configuration"]
    #[inline(always)]
    pub fn addm7(&self) -> Addm7R {
        Addm7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Stop bit configuration"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - TX/RX pin interchangeable"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The RX pin is active level inverting"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inverted"]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Inverse binary data processing"]
    #[inline(always)]
    pub fn datainv(&self) -> DatainvR {
        DatainvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MSB first"]
    #[inline(always)]
    pub fn msbfirst(&self) -> MsbfirstR {
        MsbfirstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn txoe_always_on(&self) -> TxoeAlwaysOnR {
        TxoeAlwaysOnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:31 - LPUART address"]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 7bit/4bit address detection configuration"]
    #[inline(always)]
    pub fn addm7(&mut self) -> Addm7W<'_, Cr2Spec> {
        Addm7W::new(self, 4)
    }
    #[doc = "Bits 12:13 - Stop bit configuration"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, Cr2Spec> {
        StopW::new(self, 12)
    }
    #[doc = "Bit 15 - TX/RX pin interchangeable"]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<'_, Cr2Spec> {
        SwapW::new(self, 15)
    }
    #[doc = "Bit 16 - The RX pin is active level inverting"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RxinvW<'_, Cr2Spec> {
        RxinvW::new(self, 16)
    }
    #[doc = "Bit 17 - TX pin active level inverted"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TxinvW<'_, Cr2Spec> {
        TxinvW::new(self, 17)
    }
    #[doc = "Bit 18 - Inverse binary data processing"]
    #[inline(always)]
    pub fn datainv(&mut self) -> DatainvW<'_, Cr2Spec> {
        DatainvW::new(self, 18)
    }
    #[doc = "Bit 19 - MSB first"]
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MsbfirstW<'_, Cr2Spec> {
        MsbfirstW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn txoe_always_on(&mut self) -> TxoeAlwaysOnW<'_, Cr2Spec> {
        TxoeAlwaysOnW::new(self, 20)
    }
    #[doc = "Bits 24:31 - LPUART address"]
    #[inline(always)]
    pub fn add(&mut self) -> AddW<'_, Cr2Spec> {
        AddW::new(self, 24)
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
