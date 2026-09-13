#[doc = "Register `SA4` reader"]
pub type R = crate::R<Sa4Spec>;
#[doc = "Register `SA4` writer"]
pub type W = crate::W<Sa4Spec>;
#[doc = "Field `SA` reader - Source Address for Channel 0"]
pub type SaR = crate::FieldReader<u32>;
#[doc = "Field `SA` writer - Source Address for Channel 0"]
pub type SaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Address for Channel 0"]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Address for Channel 0"]
    #[inline(always)]
    pub fn sa(&mut self) -> SaW<'_, Sa4Spec> {
        SaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sa4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sa4Spec;
impl crate::RegisterSpec for Sa4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa4::R`](R) reader structure"]
impl crate::Readable for Sa4Spec {}
#[doc = "`write(|w| ..)` method takes [`sa4::W`](W) writer structure"]
impl crate::Writable for Sa4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SA4 to value 0"]
impl crate::Resettable for Sa4Spec {}
