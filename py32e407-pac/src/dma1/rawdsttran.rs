#[doc = "Register `RAWDSTTRAN` reader"]
pub type R = crate::R<RawdsttranSpec>;
#[doc = "Register `RAWDSTTRAN` writer"]
pub type W = crate::W<RawdsttranSpec>;
#[doc = "Field `RAW` reader - Raw Status for IntDstTran Interrupt"]
pub type RawR = crate::FieldReader;
#[doc = "Field `RAW` writer - Raw Status for IntDstTran Interrupt"]
pub type RawW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Raw Status for IntDstTran Interrupt"]
    #[inline(always)]
    pub fn raw(&self) -> RawR {
        RawR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Raw Status for IntDstTran Interrupt"]
    #[inline(always)]
    pub fn raw(&mut self) -> RawW<'_, RawdsttranSpec> {
        RawW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rawdsttran::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawdsttran::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawdsttranSpec;
impl crate::RegisterSpec for RawdsttranSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawdsttran::R`](R) reader structure"]
impl crate::Readable for RawdsttranSpec {}
#[doc = "`write(|w| ..)` method takes [`rawdsttran::W`](W) writer structure"]
impl crate::Writable for RawdsttranSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAWDSTTRAN to value 0"]
impl crate::Resettable for RawdsttranSpec {}
