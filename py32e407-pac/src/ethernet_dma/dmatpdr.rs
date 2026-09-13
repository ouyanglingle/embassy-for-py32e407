#[doc = "Register `DMATPDR` reader"]
pub type R = crate::R<DmatpdrSpec>;
#[doc = "Register `DMATPDR` writer"]
pub type W = crate::W<DmatpdrSpec>;
#[doc = "Field `TPD` reader - Transmit poll demand"]
pub type TpdR = crate::FieldReader<u32>;
#[doc = "Field `TPD` writer - Transmit poll demand"]
pub type TpdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    pub fn tpd(&self) -> TpdR {
        TpdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    pub fn tpd(&mut self) -> TpdW<'_, DmatpdrSpec> {
        TpdW::new(self, 0)
    }
}
#[doc = "Ethernet DMA transmit poll demand register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatpdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatpdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatpdrSpec;
impl crate::RegisterSpec for DmatpdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatpdr::R`](R) reader structure"]
impl crate::Readable for DmatpdrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatpdr::W`](W) writer structure"]
impl crate::Writable for DmatpdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMATPDR to value 0"]
impl crate::Resettable for DmatpdrSpec {}
