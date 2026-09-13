#[doc = "Register `STATUSBLOCK` reader"]
pub type R = crate::R<StatusblockSpec>;
#[doc = "Register `STATUSBLOCK` writer"]
pub type W = crate::W<StatusblockSpec>;
#[doc = "Field `STATUS` reader - Status for IntBlock Interrupt"]
pub type StatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Status for IntBlock Interrupt"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 0x3f) as u8)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`statusblock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusblock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusblockSpec;
impl crate::RegisterSpec for StatusblockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusblock::R`](R) reader structure"]
impl crate::Readable for StatusblockSpec {}
#[doc = "`write(|w| ..)` method takes [`statusblock::W`](W) writer structure"]
impl crate::Writable for StatusblockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUSBLOCK to value 0"]
impl crate::Resettable for StatusblockSpec {}
