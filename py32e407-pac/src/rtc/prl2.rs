#[doc = "Register `PRL2` reader"]
pub type R = crate::R<Prl2Spec>;
#[doc = "Register `PRL2` writer"]
pub type W = crate::W<Prl2Spec>;
#[doc = "Field `PRL` reader - RTC pre-scaler reload value byte2"]
pub type PrlR = crate::FieldReader;
#[doc = "Field `PRL` writer - RTC pre-scaler reload value byte2"]
pub type PrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - RTC pre-scaler reload value byte2"]
    #[inline(always)]
    pub fn prl(&self) -> PrlR {
        PrlR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - RTC pre-scaler reload value byte2"]
    #[inline(always)]
    pub fn prl(&mut self) -> PrlW<'_, Prl2Spec> {
        PrlW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`prl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prl2Spec;
impl crate::RegisterSpec for Prl2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prl2::R`](R) reader structure"]
impl crate::Readable for Prl2Spec {}
#[doc = "`write(|w| ..)` method takes [`prl2::W`](W) writer structure"]
impl crate::Writable for Prl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRL2 to value 0"]
impl crate::Resettable for Prl2Spec {}
