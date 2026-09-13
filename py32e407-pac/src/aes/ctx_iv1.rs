#[doc = "Register `CTX_IV1` reader"]
pub type R = crate::R<CtxIv1Spec>;
#[doc = "Register `CTX_IV1` writer"]
pub type W = crate::W<CtxIv1Spec>;
#[doc = "Field `IV1` reader - "]
pub type Iv1R = crate::FieldReader<u32>;
#[doc = "Field `IV1` writer - "]
pub type Iv1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn iv1(&self) -> Iv1R {
        Iv1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn iv1(&mut self) -> Iv1W<'_, CtxIv1Spec> {
        Iv1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_iv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_iv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxIv1Spec;
impl crate::RegisterSpec for CtxIv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_iv1::R`](R) reader structure"]
impl crate::Readable for CtxIv1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_iv1::W`](W) writer structure"]
impl crate::Writable for CtxIv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_IV1 to value 0"]
impl crate::Resettable for CtxIv1Spec {}
