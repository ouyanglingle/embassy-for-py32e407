#[doc = "Register `CTX_IV3` reader"]
pub type R = crate::R<CtxIv3Spec>;
#[doc = "Register `CTX_IV3` writer"]
pub type W = crate::W<CtxIv3Spec>;
#[doc = "Field `IV3` reader - "]
pub type Iv3R = crate::FieldReader<u32>;
#[doc = "Field `IV3` writer - "]
pub type Iv3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn iv3(&self) -> Iv3R {
        Iv3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn iv3(&mut self) -> Iv3W<'_, CtxIv3Spec> {
        Iv3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_iv3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_iv3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxIv3Spec;
impl crate::RegisterSpec for CtxIv3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_iv3::R`](R) reader structure"]
impl crate::Readable for CtxIv3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_iv3::W`](W) writer structure"]
impl crate::Writable for CtxIv3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_IV3 to value 0"]
impl crate::Resettable for CtxIv3Spec {}
