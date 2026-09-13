#[doc = "Register `CTX_KEY0` reader"]
pub type R = crate::R<CtxKey0Spec>;
#[doc = "Register `CTX_KEY0` writer"]
pub type W = crate::W<CtxKey0Spec>;
#[doc = "Field `KEY0` reader - "]
pub type Key0R = crate::FieldReader<u32>;
#[doc = "Field `KEY0` writer - "]
pub type Key0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key0(&self) -> Key0R {
        Key0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key0(&mut self) -> Key0W<'_, CtxKey0Spec> {
        Key0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxKey0Spec;
impl crate::RegisterSpec for CtxKey0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_key0::R`](R) reader structure"]
impl crate::Readable for CtxKey0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_key0::W`](W) writer structure"]
impl crate::Writable for CtxKey0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_KEY0 to value 0"]
impl crate::Resettable for CtxKey0Spec {}
