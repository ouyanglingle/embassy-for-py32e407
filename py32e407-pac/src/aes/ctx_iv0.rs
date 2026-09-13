#[doc = "Register `CTX_IV0` reader"]
pub type R = crate::R<CtxIv0Spec>;
#[doc = "Register `CTX_IV0` writer"]
pub type W = crate::W<CtxIv0Spec>;
#[doc = "Field `IV0` reader - "]
pub type Iv0R = crate::FieldReader<u32>;
#[doc = "Field `IV0` writer - "]
pub type Iv0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn iv0(&self) -> Iv0R {
        Iv0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn iv0(&mut self) -> Iv0W<'_, CtxIv0Spec> {
        Iv0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_iv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_iv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxIv0Spec;
impl crate::RegisterSpec for CtxIv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_iv0::R`](R) reader structure"]
impl crate::Readable for CtxIv0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_iv0::W`](W) writer structure"]
impl crate::Writable for CtxIv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_IV0 to value 0"]
impl crate::Resettable for CtxIv0Spec {}
