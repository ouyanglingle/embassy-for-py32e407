#[doc = "Register `SKR` reader"]
pub type R = crate::R<SkrSpec>;
#[doc = "Register `SKR` writer"]
pub type W = crate::W<SkrSpec>;
#[doc = "Field `KEY` reader - protective key of SYSCFG_SCSR"]
pub type KeyR = crate::FieldReader;
#[doc = "Field `KEY` writer - protective key of SYSCFG_SCSR"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - protective key of SYSCFG_SCSR"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - protective key of SYSCFG_SCSR"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, SkrSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`skr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`skr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SkrSpec;
impl crate::RegisterSpec for SkrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`skr::R`](R) reader structure"]
impl crate::Readable for SkrSpec {}
#[doc = "`write(|w| ..)` method takes [`skr::W`](W) writer structure"]
impl crate::Writable for SkrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SKR to value 0"]
impl crate::Resettable for SkrSpec {}
