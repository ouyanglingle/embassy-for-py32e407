#[doc = "Register `CTX_CTR0` reader"]
pub type R = crate::R<CtxCtr0Spec>;
#[doc = "Register `CTX_CTR0` writer"]
pub type W = crate::W<CtxCtr0Spec>;
#[doc = "Field `CBC_KEY0` reader - "]
pub type CbcKey0R = crate::FieldReader<u32>;
#[doc = "Field `CBC_KEY0` writer - "]
pub type CbcKey0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cbc_key0(&self) -> CbcKey0R {
        CbcKey0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cbc_key0(&mut self) -> CbcKey0W<'_, CtxCtr0Spec> {
        CbcKey0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_ctr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_ctr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxCtr0Spec;
impl crate::RegisterSpec for CtxCtr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_ctr0::R`](R) reader structure"]
impl crate::Readable for CtxCtr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_ctr0::W`](W) writer structure"]
impl crate::Writable for CtxCtr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_CTR0 to value 0"]
impl crate::Resettable for CtxCtr0Spec {}
