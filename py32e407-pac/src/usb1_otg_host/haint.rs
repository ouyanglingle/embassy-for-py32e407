#[doc = "Register `HAINT` reader"]
pub type R = crate::R<HaintSpec>;
#[doc = "Register `HAINT` writer"]
pub type W = crate::W<HaintSpec>;
#[doc = "Field `HAINT` reader - Channel interrupts"]
pub type HaintR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Channel interrupts"]
    #[inline(always)]
    pub fn haint(&self) -> HaintR {
        HaintR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "OTG_FS Host all channels interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`haint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HaintSpec;
impl crate::RegisterSpec for HaintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haint::R`](R) reader structure"]
impl crate::Readable for HaintSpec {}
#[doc = "`write(|w| ..)` method takes [`haint::W`](W) writer structure"]
impl crate::Writable for HaintSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HAINT to value 0"]
impl crate::Resettable for HaintSpec {}
