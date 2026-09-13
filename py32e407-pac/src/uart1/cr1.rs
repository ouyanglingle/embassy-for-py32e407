#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `M` reader - Data Length Select"]
pub type MR = crate::FieldReader;
#[doc = "Field `M` writer - Data Length Select"]
pub type MW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STOP` reader - Number of stop bits"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Number of stop bits"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCE` reader - Parity Enable"]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - Parity Enable"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - Even Parity Select"]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - Even Parity Select"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP` reader - Stick Parity"]
pub type SpR = crate::BitReader;
#[doc = "Field `SP` writer - Stick Parity"]
pub type SpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBK` reader - Send Break"]
pub type SbkR = crate::BitReader;
#[doc = "Field `SBK` writer - Send Break"]
pub type SbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP` reader - TX/RX pin swap"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - TX/RX pin swap"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSBFIRST` reader - MSB first mode"]
pub type MsbfirstR = crate::BitReader;
#[doc = "Field `MSBFIRST` writer - MSB first mode"]
pub type MsbfirstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Data Length Select"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Number of stop bits"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Even Parity Select"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stick Parity"]
    #[inline(always)]
    pub fn sp(&self) -> SpR {
        SpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Send Break"]
    #[inline(always)]
    pub fn sbk(&self) -> SbkR {
        SbkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - TX/RX pin swap"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MSB first mode"]
    #[inline(always)]
    pub fn msbfirst(&self) -> MsbfirstR {
        MsbfirstR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data Length Select"]
    #[inline(always)]
    pub fn m(&mut self) -> MW<'_, Cr1Spec> {
        MW::new(self, 0)
    }
    #[doc = "Bit 2 - Number of stop bits"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, Cr1Spec> {
        StopW::new(self, 2)
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    pub fn pce(&mut self) -> PceW<'_, Cr1Spec> {
        PceW::new(self, 3)
    }
    #[doc = "Bit 4 - Even Parity Select"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, Cr1Spec> {
        PsW::new(self, 4)
    }
    #[doc = "Bit 5 - Stick Parity"]
    #[inline(always)]
    pub fn sp(&mut self) -> SpW<'_, Cr1Spec> {
        SpW::new(self, 5)
    }
    #[doc = "Bit 6 - Send Break"]
    #[inline(always)]
    pub fn sbk(&mut self) -> SbkW<'_, Cr1Spec> {
        SbkW::new(self, 6)
    }
    #[doc = "Bit 8 - TX/RX pin swap"]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<'_, Cr1Spec> {
        SwapW::new(self, 8)
    }
    #[doc = "Bit 9 - MSB first mode"]
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MsbfirstW<'_, Cr1Spec> {
        MsbfirstW::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
