#[doc = "Register `STRR` reader"]
pub type R = crate::R<StrrSpec>;
#[doc = "Register `STRR` writer"]
pub type W = crate::W<StrrSpec>;
#[doc = "Field `SMP` reader - sample"]
pub type SmpR = crate::FieldReader;
#[doc = "Field `SMP` writer - sample"]
pub type SmpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - sample"]
    #[inline(always)]
    pub fn smp(&self) -> SmpR {
        SmpR::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - sample"]
    #[inline(always)]
    pub fn smp(&mut self) -> SmpW<'_, StrrSpec> {
        SmpW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`strr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`strr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StrrSpec;
impl crate::RegisterSpec for StrrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`strr::R`](R) reader structure"]
impl crate::Readable for StrrSpec {}
#[doc = "`write(|w| ..)` method takes [`strr::W`](W) writer structure"]
impl crate::Writable for StrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STRR to value 0"]
impl crate::Resettable for StrrSpec {}
