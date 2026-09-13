#[doc = "Register `DIFSEL` reader"]
pub type R = crate::R<DifselSpec>;
#[doc = "Register `DIFSEL` writer"]
pub type W = crate::W<DifselSpec>;
#[doc = "Field `DIFSEL` reader - "]
pub type DifselR = crate::FieldReader<u32>;
#[doc = "Field `DIFSEL` writer - "]
pub type DifselW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn difsel(&self) -> DifselR {
        DifselR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn difsel(&mut self) -> DifselW<'_, DifselSpec> {
        DifselW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`difsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DifselSpec;
impl crate::RegisterSpec for DifselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`difsel::R`](R) reader structure"]
impl crate::Readable for DifselSpec {}
#[doc = "`write(|w| ..)` method takes [`difsel::W`](W) writer structure"]
impl crate::Writable for DifselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIFSEL to value 0"]
impl crate::Resettable for DifselSpec {}
