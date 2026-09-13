#[doc = "Register `IFR2` reader"]
pub type R = crate::R<Ifr2Spec>;
#[doc = "Register `IFR2` writer"]
pub type W = crate::W<Ifr2Spec>;
#[doc = "Field `XIPRDINSTRSETIF` reader - xip read instr set interrupt flag"]
pub type XiprdinstrsetifR = crate::BitReader;
#[doc = "Field `TXFIFOEIF` reader - tx fifo empty interrupt flag"]
pub type TxfifoeifR = crate::BitReader;
#[doc = "Field `TXFIFOHEIF` reader - tx fifo half empty interrupt flag"]
pub type TxfifoheifR = crate::BitReader;
#[doc = "Field `TXFIFOFIF` reader - tx fifo full interrupt flag"]
pub type TxfifofifR = crate::BitReader;
#[doc = "Field `TXFIFOOVIF` reader - tx fifo overflow interrupt flag"]
pub type TxfifoovifR = crate::BitReader;
#[doc = "Field `TXWORDOVIF` reader - txfifo word interrupt flag"]
pub type TxwordovifR = crate::BitReader;
#[doc = "Field `TXWORDOVIF` writer - txfifo word interrupt flag"]
pub type TxwordovifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIPWEINSTRSETIF` reader - xip write instr set interrupt flag"]
pub type XipweinstrsetifR = crate::BitReader;
#[doc = "Field `RXTOUTIF` reader - rx timeout interrupt flag"]
pub type RxtoutifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - xip read instr set interrupt flag"]
    #[inline(always)]
    pub fn xiprdinstrsetif(&self) -> XiprdinstrsetifR {
        XiprdinstrsetifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tx fifo empty interrupt flag"]
    #[inline(always)]
    pub fn txfifoeif(&self) -> TxfifoeifR {
        TxfifoeifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tx fifo half empty interrupt flag"]
    #[inline(always)]
    pub fn txfifoheif(&self) -> TxfifoheifR {
        TxfifoheifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tx fifo full interrupt flag"]
    #[inline(always)]
    pub fn txfifofif(&self) -> TxfifofifR {
        TxfifofifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tx fifo overflow interrupt flag"]
    #[inline(always)]
    pub fn txfifoovif(&self) -> TxfifoovifR {
        TxfifoovifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - txfifo word interrupt flag"]
    #[inline(always)]
    pub fn txwordovif(&self) -> TxwordovifR {
        TxwordovifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - xip write instr set interrupt flag"]
    #[inline(always)]
    pub fn xipweinstrsetif(&self) -> XipweinstrsetifR {
        XipweinstrsetifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - rx timeout interrupt flag"]
    #[inline(always)]
    pub fn rxtoutif(&self) -> RxtoutifR {
        RxtoutifR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - txfifo word interrupt flag"]
    #[inline(always)]
    pub fn txwordovif(&mut self) -> TxwordovifW<'_, Ifr2Spec> {
        TxwordovifW::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ifr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ifr2Spec;
impl crate::RegisterSpec for Ifr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ifr2::R`](R) reader structure"]
impl crate::Readable for Ifr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ifr2::W`](W) writer structure"]
impl crate::Writable for Ifr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFR2 to value 0"]
impl crate::Resettable for Ifr2Spec {}
