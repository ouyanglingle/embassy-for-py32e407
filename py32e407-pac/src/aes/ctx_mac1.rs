#[doc = "Register `CTX_MAC1` reader"]
pub type R = crate::R<CtxMac1Spec>;
#[doc = "Register `CTX_MAC1` writer"]
pub type W = crate::W<CtxMac1Spec>;
#[doc = "Field `MAC1` reader - "]
pub type Mac1R = crate::FieldReader<u32>;
#[doc = "Field `MAC1` writer - "]
pub type Mac1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mac1(&self) -> Mac1R {
        Mac1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mac1(&mut self) -> Mac1W<'_, CtxMac1Spec> {
        Mac1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_mac1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_mac1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxMac1Spec;
impl crate::RegisterSpec for CtxMac1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_mac1::R`](R) reader structure"]
impl crate::Readable for CtxMac1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_mac1::W`](W) writer structure"]
impl crate::Writable for CtxMac1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_MAC1 to value 0"]
impl crate::Resettable for CtxMac1Spec {}
