#[doc = "Register `CTX_KEY1` reader"]
pub type R = crate::R<CtxKey1Spec>;
#[doc = "Register `CTX_KEY1` writer"]
pub type W = crate::W<CtxKey1Spec>;
#[doc = "Field `KEY1` reader - "]
pub type Key1R = crate::FieldReader<u32>;
#[doc = "Field `KEY1` writer - "]
pub type Key1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key1(&self) -> Key1R {
        Key1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key1(&mut self) -> Key1W<'_, CtxKey1Spec> {
        Key1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxKey1Spec;
impl crate::RegisterSpec for CtxKey1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_key1::R`](R) reader structure"]
impl crate::Readable for CtxKey1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_key1::W`](W) writer structure"]
impl crate::Writable for CtxKey1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_KEY1 to value 0"]
impl crate::Resettable for CtxKey1Spec {}
