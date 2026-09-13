#[doc = "Register `PRL0` reader"]
pub type R = crate::R<Prl0Spec>;
#[doc = "Register `PRL0` writer"]
pub type W = crate::W<Prl0Spec>;
#[doc = "Field `PRL` reader - RTC pre-scaler reload value byte0"]
pub type PrlR = crate::FieldReader;
#[doc = "Field `PRL` writer - RTC pre-scaler reload value byte0"]
pub type PrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RTC pre-scaler reload value byte0"]
    #[inline(always)]
    pub fn prl(&self) -> PrlR {
        PrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC pre-scaler reload value byte0"]
    #[inline(always)]
    pub fn prl(&mut self) -> PrlW<'_, Prl0Spec> {
        PrlW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`prl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prl0Spec;
impl crate::RegisterSpec for Prl0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prl0::R`](R) reader structure"]
impl crate::Readable for Prl0Spec {}
#[doc = "`write(|w| ..)` method takes [`prl0::W`](W) writer structure"]
impl crate::Writable for Prl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRL0 to value 0"]
impl crate::Resettable for Prl0Spec {}
