#[doc = "Register `CTX_KEY4` reader"]
pub type R = crate::R<CtxKey4Spec>;
#[doc = "Register `CTX_KEY4` writer"]
pub type W = crate::W<CtxKey4Spec>;
#[doc = "Field `KEY4` reader - "]
pub type Key4R = crate::FieldReader<u32>;
#[doc = "Field `KEY4` writer - "]
pub type Key4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key4(&self) -> Key4R {
        Key4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key4(&mut self) -> Key4W<'_, CtxKey4Spec> {
        Key4W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxKey4Spec;
impl crate::RegisterSpec for CtxKey4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_key4::R`](R) reader structure"]
impl crate::Readable for CtxKey4Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_key4::W`](W) writer structure"]
impl crate::Writable for CtxKey4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_KEY4 to value 0"]
impl crate::Resettable for CtxKey4Spec {}
