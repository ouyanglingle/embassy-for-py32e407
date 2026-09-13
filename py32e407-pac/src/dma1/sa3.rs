#[doc = "Register `SA3` reader"]
pub type R = crate::R<Sa3Spec>;
#[doc = "Register `SA3` writer"]
pub type W = crate::W<Sa3Spec>;
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
    pub fn sa(&mut self) -> SaW<'_, Sa3Spec> {
        SaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sa3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sa3Spec;
impl crate::RegisterSpec for Sa3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa3::R`](R) reader structure"]
impl crate::Readable for Sa3Spec {}
#[doc = "`write(|w| ..)` method takes [`sa3::W`](W) writer structure"]
impl crate::Writable for Sa3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SA3 to value 0"]
impl crate::Resettable for Sa3Spec {}
