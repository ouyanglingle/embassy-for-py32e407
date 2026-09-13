#[doc = "Register `CTX_MAC3` reader"]
pub type R = crate::R<CtxMac3Spec>;
#[doc = "Register `CTX_MAC3` writer"]
pub type W = crate::W<CtxMac3Spec>;
#[doc = "Field `MAC3` reader - "]
pub type Mac3R = crate::FieldReader<u32>;
#[doc = "Field `MAC3` writer - "]
pub type Mac3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mac3(&self) -> Mac3R {
        Mac3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mac3(&mut self) -> Mac3W<'_, CtxMac3Spec> {
        Mac3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_mac3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_mac3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxMac3Spec;
impl crate::RegisterSpec for CtxMac3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_mac3::R`](R) reader structure"]
impl crate::Readable for CtxMac3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_mac3::W`](W) writer structure"]
impl crate::Writable for CtxMac3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_MAC3 to value 0"]
impl crate::Resettable for CtxMac3Spec {}
