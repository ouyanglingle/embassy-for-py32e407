#[doc = "Register `SR1` reader"]
pub type R = crate::R<Sr1Spec>;
#[doc = "Register `SR1` writer"]
pub type W = crate::W<Sr1Spec>;
#[doc = "Field `RWDSI_ND` reader - rwdsi not detect"]
pub type RwdsiNdR = crate::BitReader;
#[doc = "Field `XSPI_DUAL_LATENCY` reader - xspi dual latency flag"]
pub type XspiDualLatencyR = crate::BitReader;
#[doc = "Field `XSPI_DUAL_LATENCY` writer - xspi dual latency flag"]
pub type XspiDualLatencyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - rwdsi not detect"]
    #[inline(always)]
    pub fn rwdsi_nd(&self) -> RwdsiNdR {
        RwdsiNdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - xspi dual latency flag"]
    #[inline(always)]
    pub fn xspi_dual_latency(&self) -> XspiDualLatencyR {
        XspiDualLatencyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - xspi dual latency flag"]
    #[inline(always)]
    pub fn xspi_dual_latency(&mut self) -> XspiDualLatencyW<'_, Sr1Spec> {
        XspiDualLatencyW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr1Spec;
impl crate::RegisterSpec for Sr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for Sr1Spec {}
#[doc = "`write(|w| ..)` method takes [`sr1::W`](W) writer structure"]
impl crate::Writable for Sr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for Sr1Spec {}
