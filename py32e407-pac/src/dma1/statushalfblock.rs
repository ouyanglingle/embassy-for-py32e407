#[doc = "Register `STATUSHALFBLOCK` reader"]
pub type R = crate::R<StatushalfblockSpec>;
#[doc = "Register `STATUSHALFBLOCK` writer"]
pub type W = crate::W<StatushalfblockSpec>;
#[doc = "Field `STATUS` reader - Status for IntHalfBlock Interrupt"]
pub type StatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Status for IntHalfBlock Interrupt"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 0x3f) as u8)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`statushalfblock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statushalfblock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatushalfblockSpec;
impl crate::RegisterSpec for StatushalfblockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statushalfblock::R`](R) reader structure"]
impl crate::Readable for StatushalfblockSpec {}
#[doc = "`write(|w| ..)` method takes [`statushalfblock::W`](W) writer structure"]
impl crate::Writable for StatushalfblockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUSHALFBLOCK to value 0"]
impl crate::Resettable for StatushalfblockSpec {}
