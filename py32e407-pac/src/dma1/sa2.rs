#[doc = "Register `SA2` reader"]
pub type R = crate::R<Sa2Spec>;
#[doc = "Register `SA2` writer"]
pub type W = crate::W<Sa2Spec>;
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
    pub fn sa(&mut self) -> SaW<'_, Sa2Spec> {
        SaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sa2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sa2Spec;
impl crate::RegisterSpec for Sa2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa2::R`](R) reader structure"]
impl crate::Readable for Sa2Spec {}
#[doc = "`write(|w| ..)` method takes [`sa2::W`](W) writer structure"]
impl crate::Writable for Sa2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SA2 to value 0"]
impl crate::Resettable for Sa2Spec {}
