#[doc = "Register `CTX_IV2` reader"]
pub type R = crate::R<CtxIv2Spec>;
#[doc = "Register `CTX_IV2` writer"]
pub type W = crate::W<CtxIv2Spec>;
#[doc = "Field `IV2` reader - "]
pub type Iv2R = crate::FieldReader<u32>;
#[doc = "Field `IV2` writer - "]
pub type Iv2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn iv2(&self) -> Iv2R {
        Iv2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn iv2(&mut self) -> Iv2W<'_, CtxIv2Spec> {
        Iv2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_iv2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_iv2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxIv2Spec;
impl crate::RegisterSpec for CtxIv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_iv2::R`](R) reader structure"]
impl crate::Readable for CtxIv2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_iv2::W`](W) writer structure"]
impl crate::Writable for CtxIv2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_IV2 to value 0"]
impl crate::Resettable for CtxIv2Spec {}
