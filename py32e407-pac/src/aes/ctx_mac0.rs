#[doc = "Register `CTX_MAC0` reader"]
pub type R = crate::R<CtxMac0Spec>;
#[doc = "Register `CTX_MAC0` writer"]
pub type W = crate::W<CtxMac0Spec>;
#[doc = "Field `MAC0` reader - "]
pub type Mac0R = crate::FieldReader<u32>;
#[doc = "Field `MAC0` writer - "]
pub type Mac0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mac0(&self) -> Mac0R {
        Mac0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mac0(&mut self) -> Mac0W<'_, CtxMac0Spec> {
        Mac0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_mac0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_mac0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxMac0Spec;
impl crate::RegisterSpec for CtxMac0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_mac0::R`](R) reader structure"]
impl crate::Readable for CtxMac0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_mac0::W`](W) writer structure"]
impl crate::Writable for CtxMac0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_MAC0 to value 0"]
impl crate::Resettable for CtxMac0Spec {}
