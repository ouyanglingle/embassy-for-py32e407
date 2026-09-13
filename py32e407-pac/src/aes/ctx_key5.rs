#[doc = "Register `CTX_KEY5` reader"]
pub type R = crate::R<CtxKey5Spec>;
#[doc = "Register `CTX_KEY5` writer"]
pub type W = crate::W<CtxKey5Spec>;
#[doc = "Field `KEY5` reader - "]
pub type Key5R = crate::FieldReader<u32>;
#[doc = "Field `KEY5` writer - "]
pub type Key5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key5(&self) -> Key5R {
        Key5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key5(&mut self) -> Key5W<'_, CtxKey5Spec> {
        Key5W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxKey5Spec;
impl crate::RegisterSpec for CtxKey5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_key5::R`](R) reader structure"]
impl crate::Readable for CtxKey5Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_key5::W`](W) writer structure"]
impl crate::Writable for CtxKey5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_KEY5 to value 0"]
impl crate::Resettable for CtxKey5Spec {}
