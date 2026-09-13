#[doc = "Register `CTX_KEY7` reader"]
pub type R = crate::R<CtxKey7Spec>;
#[doc = "Register `CTX_KEY7` writer"]
pub type W = crate::W<CtxKey7Spec>;
#[doc = "Field `KEY7` reader - "]
pub type Key7R = crate::FieldReader<u32>;
#[doc = "Field `KEY7` writer - "]
pub type Key7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key7(&self) -> Key7R {
        Key7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key7(&mut self) -> Key7W<'_, CtxKey7Spec> {
        Key7W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxKey7Spec;
impl crate::RegisterSpec for CtxKey7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_key7::R`](R) reader structure"]
impl crate::Readable for CtxKey7Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_key7::W`](W) writer structure"]
impl crate::Writable for CtxKey7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_KEY7 to value 0"]
impl crate::Resettable for CtxKey7Spec {}
