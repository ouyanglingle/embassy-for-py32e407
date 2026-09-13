#[doc = "Register `CTX_CBC_KEY3` reader"]
pub type R = crate::R<CtxCbcKey3Spec>;
#[doc = "Register `CTX_CBC_KEY3` writer"]
pub type W = crate::W<CtxCbcKey3Spec>;
#[doc = "Field `CBC_KEY3` reader - "]
pub type CbcKey3R = crate::FieldReader<u32>;
#[doc = "Field `CBC_KEY3` writer - "]
pub type CbcKey3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cbc_key3(&self) -> CbcKey3R {
        CbcKey3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cbc_key3(&mut self) -> CbcKey3W<'_, CtxCbcKey3Spec> {
        CbcKey3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_cbc_key3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_cbc_key3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxCbcKey3Spec;
impl crate::RegisterSpec for CtxCbcKey3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_cbc_key3::R`](R) reader structure"]
impl crate::Readable for CtxCbcKey3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_cbc_key3::W`](W) writer structure"]
impl crate::Writable for CtxCbcKey3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_CBC_KEY3 to value 0"]
impl crate::Resettable for CtxCbcKey3Spec {}
