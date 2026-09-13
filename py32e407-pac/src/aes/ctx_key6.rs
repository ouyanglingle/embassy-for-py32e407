#[doc = "Register `CTX_KEY6` reader"]
pub type R = crate::R<CtxKey6Spec>;
#[doc = "Register `CTX_KEY6` writer"]
pub type W = crate::W<CtxKey6Spec>;
#[doc = "Field `KEY6` reader - "]
pub type Key6R = crate::FieldReader<u32>;
#[doc = "Field `KEY6` writer - "]
pub type Key6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key6(&self) -> Key6R {
        Key6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key6(&mut self) -> Key6W<'_, CtxKey6Spec> {
        Key6W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxKey6Spec;
impl crate::RegisterSpec for CtxKey6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_key6::R`](R) reader structure"]
impl crate::Readable for CtxKey6Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_key6::W`](W) writer structure"]
impl crate::Writable for CtxKey6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_KEY6 to value 0"]
impl crate::Resettable for CtxKey6Spec {}
