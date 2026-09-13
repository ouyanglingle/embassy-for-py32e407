#[doc = "Register `DTXFSTS2` reader"]
pub type R = crate::R<Dtxfsts2Spec>;
#[doc = "Register `DTXFSTS2` writer"]
pub type W = crate::W<Dtxfsts2Spec>;
#[doc = "Field `INEPTFSAV` reader - IN endpoint TxFIFO space available"]
pub type IneptfsavR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space available"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> IneptfsavR {
        IneptfsavR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtxfsts2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dtxfsts2Spec;
impl crate::RegisterSpec for Dtxfsts2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts2::R`](R) reader structure"]
impl crate::Readable for Dtxfsts2Spec {}
#[doc = "`write(|w| ..)` method takes [`dtxfsts2::W`](W) writer structure"]
impl crate::Writable for Dtxfsts2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTXFSTS2 to value 0"]
impl crate::Resettable for Dtxfsts2Spec {}
