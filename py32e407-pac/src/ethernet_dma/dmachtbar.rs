#[doc = "Register `DMACHTBAR` reader"]
pub type R = crate::R<DmachtbarSpec>;
#[doc = "Register `DMACHTBAR` writer"]
pub type W = crate::W<DmachtbarSpec>;
#[doc = "Field `HTBAP` reader - Host transmit buffer address pointer"]
pub type HtbapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host transmit buffer address pointer"]
    #[inline(always)]
    pub fn htbap(&self) -> HtbapR {
        HtbapR::new(self.bits)
    }
}
impl W {}
#[doc = "Ethernet DMA current host transmit buffer address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmachtbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmachtbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmachtbarSpec;
impl crate::RegisterSpec for DmachtbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmachtbar::R`](R) reader structure"]
impl crate::Readable for DmachtbarSpec {}
#[doc = "`write(|w| ..)` method takes [`dmachtbar::W`](W) writer structure"]
impl crate::Writable for DmachtbarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACHTBAR to value 0"]
impl crate::Resettable for DmachtbarSpec {}
