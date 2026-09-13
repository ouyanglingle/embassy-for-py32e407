#[doc = "Register `CTX_KEY3` reader"]
pub type R = crate::R<CtxKey3Spec>;
#[doc = "Register `CTX_KEY3` writer"]
pub type W = crate::W<CtxKey3Spec>;
#[doc = "Field `KEY3` reader - "]
pub type Key3R = crate::FieldReader<u32>;
#[doc = "Field `KEY3` writer - "]
pub type Key3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key3(&self) -> Key3R {
        Key3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key3(&mut self) -> Key3W<'_, CtxKey3Spec> {
        Key3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxKey3Spec;
impl crate::RegisterSpec for CtxKey3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_key3::R`](R) reader structure"]
impl crate::Readable for CtxKey3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_key3::W`](W) writer structure"]
impl crate::Writable for CtxKey3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_KEY3 to value 0"]
impl crate::Resettable for CtxKey3Spec {}
