#[doc = "Register `DA3` reader"]
pub type R = crate::R<Da3Spec>;
#[doc = "Register `DA3` writer"]
pub type W = crate::W<Da3Spec>;
#[doc = "Field `DA` reader - Destination Address Register for Channel0"]
pub type DaR = crate::FieldReader<u32>;
#[doc = "Field `DA` writer - Destination Address Register for Channel0"]
pub type DaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination Address Register for Channel0"]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Address Register for Channel0"]
    #[inline(always)]
    pub fn da(&mut self) -> DaW<'_, Da3Spec> {
        DaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`da3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`da3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Da3Spec;
impl crate::RegisterSpec for Da3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`da3::R`](R) reader structure"]
impl crate::Readable for Da3Spec {}
#[doc = "`write(|w| ..)` method takes [`da3::W`](W) writer structure"]
impl crate::Writable for Da3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DA3 to value 0"]
impl crate::Resettable for Da3Spec {}
