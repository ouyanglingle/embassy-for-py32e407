#[doc = "Register `PTPTSHUR` reader"]
pub type R = crate::R<PtptshurSpec>;
#[doc = "Register `PTPTSHUR` writer"]
pub type W = crate::W<PtptshurSpec>;
#[doc = "Field `TSUS` reader - Time stamp update second"]
pub type TsusR = crate::FieldReader<u32>;
#[doc = "Field `TSUS` writer - Time stamp update second"]
pub type TsusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Time stamp update second"]
    #[inline(always)]
    pub fn tsus(&self) -> TsusR {
        TsusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time stamp update second"]
    #[inline(always)]
    pub fn tsus(&mut self) -> TsusW<'_, PtptshurSpec> {
        TsusW::new(self, 0)
    }
}
#[doc = "Ethernet PTP time stamp high update register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptshur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptshur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtptshurSpec;
impl crate::RegisterSpec for PtptshurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptshur::R`](R) reader structure"]
impl crate::Readable for PtptshurSpec {}
#[doc = "`write(|w| ..)` method takes [`ptptshur::W`](W) writer structure"]
impl crate::Writable for PtptshurSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PTPTSHUR to value 0"]
impl crate::Resettable for PtptshurSpec {}
