#[doc = "Register `JDR2` reader"]
pub type R = crate::R<Jdr2Spec>;
#[doc = "Register `JDR2` writer"]
pub type W = crate::W<Jdr2Spec>;
#[doc = "Field `JDATA` reader - "]
pub type JdataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn jdata(&self) -> JdataR {
        JdataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jdr2Spec;
impl crate::RegisterSpec for Jdr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr2::R`](R) reader structure"]
impl crate::Readable for Jdr2Spec {}
#[doc = "`write(|w| ..)` method takes [`jdr2::W`](W) writer structure"]
impl crate::Writable for Jdr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JDR2 to value 0"]
impl crate::Resettable for Jdr2Spec {}
