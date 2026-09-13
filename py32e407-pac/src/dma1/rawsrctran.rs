#[doc = "Register `RAWSRCTRAN` reader"]
pub type R = crate::R<RawsrctranSpec>;
#[doc = "Register `RAWSRCTRAN` writer"]
pub type W = crate::W<RawsrctranSpec>;
#[doc = "Field `RAW` reader - Raw Status for IntSrcTran Interrupt"]
pub type RawR = crate::FieldReader;
#[doc = "Field `RAW` writer - Raw Status for IntSrcTran Interrupt"]
pub type RawW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Raw Status for IntSrcTran Interrupt"]
    #[inline(always)]
    pub fn raw(&self) -> RawR {
        RawR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Raw Status for IntSrcTran Interrupt"]
    #[inline(always)]
    pub fn raw(&mut self) -> RawW<'_, RawsrctranSpec> {
        RawW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rawsrctran::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawsrctran::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawsrctranSpec;
impl crate::RegisterSpec for RawsrctranSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawsrctran::R`](R) reader structure"]
impl crate::Readable for RawsrctranSpec {}
#[doc = "`write(|w| ..)` method takes [`rawsrctran::W`](W) writer structure"]
impl crate::Writable for RawsrctranSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAWSRCTRAN to value 0"]
impl crate::Resettable for RawsrctranSpec {}
