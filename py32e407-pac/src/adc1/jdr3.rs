#[doc = "Register `JDR3` reader"]
pub type R = crate::R<Jdr3Spec>;
#[doc = "Register `JDR3` writer"]
pub type W = crate::W<Jdr3Spec>;
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
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jdr3Spec;
impl crate::RegisterSpec for Jdr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr3::R`](R) reader structure"]
impl crate::Readable for Jdr3Spec {}
#[doc = "`write(|w| ..)` method takes [`jdr3::W`](W) writer structure"]
impl crate::Writable for Jdr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JDR3 to value 0"]
impl crate::Resettable for Jdr3Spec {}
