#[doc = "Register `CLEARDSTTRAN` reader"]
pub type R = crate::R<CleardsttranSpec>;
#[doc = "Register `CLEARDSTTRAN` writer"]
pub type W = crate::W<CleardsttranSpec>;
#[doc = "Field `CLEAR` reader - Clear for IntDstTran Interrupt"]
pub type ClearR = crate::FieldReader;
#[doc = "Field `CLEAR` writer - Clear for IntDstTran Interrupt"]
pub type ClearW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Clear for IntDstTran Interrupt"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clear for IntDstTran Interrupt"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<'_, CleardsttranSpec> {
        ClearW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cleardsttran::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cleardsttran::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CleardsttranSpec;
impl crate::RegisterSpec for CleardsttranSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cleardsttran::R`](R) reader structure"]
impl crate::Readable for CleardsttranSpec {}
#[doc = "`write(|w| ..)` method takes [`cleardsttran::W`](W) writer structure"]
impl crate::Writable for CleardsttranSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLEARDSTTRAN to value 0"]
impl crate::Resettable for CleardsttranSpec {}
