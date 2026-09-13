#[doc = "Register `CTX_KEY2` reader"]
pub type R = crate::R<CtxKey2Spec>;
#[doc = "Register `CTX_KEY2` writer"]
pub type W = crate::W<CtxKey2Spec>;
#[doc = "Field `KEY2` reader - "]
pub type Key2R = crate::FieldReader<u32>;
#[doc = "Field `KEY2` writer - "]
pub type Key2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key2(&self) -> Key2R {
        Key2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key2(&mut self) -> Key2W<'_, CtxKey2Spec> {
        Key2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxKey2Spec;
impl crate::RegisterSpec for CtxKey2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_key2::R`](R) reader structure"]
impl crate::Readable for CtxKey2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_key2::W`](W) writer structure"]
impl crate::Writable for CtxKey2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_KEY2 to value 0"]
impl crate::Resettable for CtxKey2Spec {}
