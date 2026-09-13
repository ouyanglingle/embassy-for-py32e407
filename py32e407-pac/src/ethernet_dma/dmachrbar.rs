#[doc = "Register `DMACHRBAR` reader"]
pub type R = crate::R<DmachrbarSpec>;
#[doc = "Register `DMACHRBAR` writer"]
pub type W = crate::W<DmachrbarSpec>;
#[doc = "Field `HRBAP` reader - Host receive buffer address pointer"]
pub type HrbapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host receive buffer address pointer"]
    #[inline(always)]
    pub fn hrbap(&self) -> HrbapR {
        HrbapR::new(self.bits)
    }
}
impl W {}
#[doc = "Ethernet DMA current host receive buffer address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmachrbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmachrbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmachrbarSpec;
impl crate::RegisterSpec for DmachrbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmachrbar::R`](R) reader structure"]
impl crate::Readable for DmachrbarSpec {}
#[doc = "`write(|w| ..)` method takes [`dmachrbar::W`](W) writer structure"]
impl crate::Writable for DmachrbarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACHRBAR to value 0"]
impl crate::Resettable for DmachrbarSpec {}
