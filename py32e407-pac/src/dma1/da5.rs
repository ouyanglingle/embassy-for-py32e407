#[doc = "Register `DA5` reader"]
pub type R = crate::R<Da5Spec>;
#[doc = "Register `DA5` writer"]
pub type W = crate::W<Da5Spec>;
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
    pub fn da(&mut self) -> DaW<'_, Da5Spec> {
        DaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`da5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`da5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Da5Spec;
impl crate::RegisterSpec for Da5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`da5::R`](R) reader structure"]
impl crate::Readable for Da5Spec {}
#[doc = "`write(|w| ..)` method takes [`da5::W`](W) writer structure"]
impl crate::Writable for Da5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DA5 to value 0"]
impl crate::Resettable for Da5Spec {}
