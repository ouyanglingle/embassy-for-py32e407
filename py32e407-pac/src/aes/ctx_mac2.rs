#[doc = "Register `CTX_MAC2` reader"]
pub type R = crate::R<CtxMac2Spec>;
#[doc = "Register `CTX_MAC2` writer"]
pub type W = crate::W<CtxMac2Spec>;
#[doc = "Field `MAC2` reader - "]
pub type Mac2R = crate::FieldReader<u32>;
#[doc = "Field `MAC2` writer - "]
pub type Mac2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mac2(&self) -> Mac2R {
        Mac2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mac2(&mut self) -> Mac2W<'_, CtxMac2Spec> {
        Mac2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_mac2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_mac2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxMac2Spec;
impl crate::RegisterSpec for CtxMac2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_mac2::R`](R) reader structure"]
impl crate::Readable for CtxMac2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_mac2::W`](W) writer structure"]
impl crate::Writable for CtxMac2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_MAC2 to value 0"]
impl crate::Resettable for CtxMac2Spec {}
