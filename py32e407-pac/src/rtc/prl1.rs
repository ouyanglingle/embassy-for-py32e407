#[doc = "Register `PRL1` reader"]
pub type R = crate::R<Prl1Spec>;
#[doc = "Register `PRL1` writer"]
pub type W = crate::W<Prl1Spec>;
#[doc = "Field `PRL` reader - RTC pre-scaler reload value byte1"]
pub type PrlR = crate::FieldReader;
#[doc = "Field `PRL` writer - RTC pre-scaler reload value byte1"]
pub type PrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RTC pre-scaler reload value byte1"]
    #[inline(always)]
    pub fn prl(&self) -> PrlR {
        PrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC pre-scaler reload value byte1"]
    #[inline(always)]
    pub fn prl(&mut self) -> PrlW<'_, Prl1Spec> {
        PrlW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`prl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prl1Spec;
impl crate::RegisterSpec for Prl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prl1::R`](R) reader structure"]
impl crate::Readable for Prl1Spec {}
#[doc = "`write(|w| ..)` method takes [`prl1::W`](W) writer structure"]
impl crate::Writable for Prl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRL1 to value 0"]
impl crate::Resettable for Prl1Spec {}
