#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `CPHA` reader - clock phase"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - clock phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - clock polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - clock polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_CLR` reader - txfifo clear"]
pub type TxfifoClrR = crate::BitReader;
#[doc = "Field `TXFIFO_CLR` writer - txfifo clear"]
pub type TxfifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_CLR` reader - rxfifo clear"]
pub type RxfifoClrR = crate::BitReader;
#[doc = "Field `RXFIFO_CLR` writer - rxfifo clear"]
pub type RxfifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XSPI_DUAL_LAT_OFF` reader - xspi dual latency off enable"]
pub type XspiDualLatOffR = crate::BitReader;
#[doc = "Field `XSPI_DUAL_LAT_OFF` writer - xspi dual latency off enable"]
pub type XspiDualLatOffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - txfifo clear"]
    #[inline(always)]
    pub fn txfifo_clr(&self) -> TxfifoClrR {
        TxfifoClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - rxfifo clear"]
    #[inline(always)]
    pub fn rxfifo_clr(&self) -> RxfifoClrR {
        RxfifoClrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - xspi dual latency off enable"]
    #[inline(always)]
    pub fn xspi_dual_lat_off(&self) -> XspiDualLatOffR {
        XspiDualLatOffR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clock phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, Cr2Spec> {
        CphaW::new(self, 0)
    }
    #[doc = "Bit 1 - clock polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, Cr2Spec> {
        CpolW::new(self, 1)
    }
    #[doc = "Bit 3 - txfifo clear"]
    #[inline(always)]
    pub fn txfifo_clr(&mut self) -> TxfifoClrW<'_, Cr2Spec> {
        TxfifoClrW::new(self, 3)
    }
    #[doc = "Bit 4 - rxfifo clear"]
    #[inline(always)]
    pub fn rxfifo_clr(&mut self) -> RxfifoClrW<'_, Cr2Spec> {
        RxfifoClrW::new(self, 4)
    }
    #[doc = "Bit 7 - xspi dual latency off enable"]
    #[inline(always)]
    pub fn xspi_dual_lat_off(&mut self) -> XspiDualLatOffW<'_, Cr2Spec> {
        XspiDualLatOffW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
