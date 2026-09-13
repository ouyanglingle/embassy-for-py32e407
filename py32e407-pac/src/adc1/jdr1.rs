#[doc = "Register `JDR1` reader"]
pub type R = crate::R<Jdr1Spec>;
#[doc = "Register `JDR1` writer"]
pub type W = crate::W<Jdr1Spec>;
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
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jdr1Spec;
impl crate::RegisterSpec for Jdr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr1::R`](R) reader structure"]
impl crate::Readable for Jdr1Spec {}
#[doc = "`write(|w| ..)` method takes [`jdr1::W`](W) writer structure"]
impl crate::Writable for Jdr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JDR1 to value 0"]
impl crate::Resettable for Jdr1Spec {}
