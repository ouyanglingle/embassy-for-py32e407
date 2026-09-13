#[doc = "Register `CTX_CTR2` reader"]
pub type R = crate::R<CtxCtr2Spec>;
#[doc = "Register `CTX_CTR2` writer"]
pub type W = crate::W<CtxCtr2Spec>;
#[doc = "Field `CBC_KEY2` reader - "]
pub type CbcKey2R = crate::FieldReader<u32>;
#[doc = "Field `CBC_KEY2` writer - "]
pub type CbcKey2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cbc_key2(&self) -> CbcKey2R {
        CbcKey2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cbc_key2(&mut self) -> CbcKey2W<'_, CtxCtr2Spec> {
        CbcKey2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_ctr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_ctr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxCtr2Spec;
impl crate::RegisterSpec for CtxCtr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_ctr2::R`](R) reader structure"]
impl crate::Readable for CtxCtr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_ctr2::W`](W) writer structure"]
impl crate::Writable for CtxCtr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_CTR2 to value 0"]
impl crate::Resettable for CtxCtr2Spec {}
