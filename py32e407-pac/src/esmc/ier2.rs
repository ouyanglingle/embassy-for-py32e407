#[doc = "Register `IER2` reader"]
pub type R = crate::R<Ier2Spec>;
#[doc = "Register `IER2` writer"]
pub type W = crate::W<Ier2Spec>;
#[doc = "Field `TXFIFOEIE` reader - tx fifo empty interrupt enable"]
pub type TxfifoeieR = crate::BitReader;
#[doc = "Field `TXFIFOEIE` writer - tx fifo empty interrupt enable"]
pub type TxfifoeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOHEIE` reader - tx fifo half full interrupt enable"]
pub type TxfifoheieR = crate::BitReader;
#[doc = "Field `TXFIFOHEIE` writer - tx fifo half full interrupt enable"]
pub type TxfifoheieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOFIE` reader - tx fifo full interrupt enable"]
pub type TxfifofieR = crate::BitReader;
#[doc = "Field `TXFIFOFIE` writer - tx fifo full interrupt enable"]
pub type TxfifofieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOOVIE` reader - tx fifo overflow interrupt enable"]
pub type TxfifoovieR = crate::BitReader;
#[doc = "Field `TXFIFOOVIE` writer - tx fifo overflow interrupt enable"]
pub type TxfifoovieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXWORDOVIE` reader - txfifo word interrupt enable"]
pub type TxwordovieR = crate::BitReader;
#[doc = "Field `TXWORDOVIE` writer - txfifo word interrupt enable"]
pub type TxwordovieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - tx fifo empty interrupt enable"]
    #[inline(always)]
    pub fn txfifoeie(&self) -> TxfifoeieR {
        TxfifoeieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tx fifo half full interrupt enable"]
    #[inline(always)]
    pub fn txfifoheie(&self) -> TxfifoheieR {
        TxfifoheieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tx fifo full interrupt enable"]
    #[inline(always)]
    pub fn txfifofie(&self) -> TxfifofieR {
        TxfifofieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tx fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn txfifoovie(&self) -> TxfifoovieR {
        TxfifoovieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - txfifo word interrupt enable"]
    #[inline(always)]
    pub fn txwordovie(&self) -> TxwordovieR {
        TxwordovieR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - tx fifo empty interrupt enable"]
    #[inline(always)]
    pub fn txfifoeie(&mut self) -> TxfifoeieW<'_, Ier2Spec> {
        TxfifoeieW::new(self, 1)
    }
    #[doc = "Bit 2 - tx fifo half full interrupt enable"]
    #[inline(always)]
    pub fn txfifoheie(&mut self) -> TxfifoheieW<'_, Ier2Spec> {
        TxfifoheieW::new(self, 2)
    }
    #[doc = "Bit 3 - tx fifo full interrupt enable"]
    #[inline(always)]
    pub fn txfifofie(&mut self) -> TxfifofieW<'_, Ier2Spec> {
        TxfifofieW::new(self, 3)
    }
    #[doc = "Bit 4 - tx fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn txfifoovie(&mut self) -> TxfifoovieW<'_, Ier2Spec> {
        TxfifoovieW::new(self, 4)
    }
    #[doc = "Bit 5 - txfifo word interrupt enable"]
    #[inline(always)]
    pub fn txwordovie(&mut self) -> TxwordovieW<'_, Ier2Spec> {
        TxwordovieW::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ier2Spec;
impl crate::RegisterSpec for Ier2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ier2::R`](R) reader structure"]
impl crate::Readable for Ier2Spec {}
#[doc = "`write(|w| ..)` method takes [`ier2::W`](W) writer structure"]
impl crate::Writable for Ier2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER2 to value 0"]
impl crate::Resettable for Ier2Spec {}
