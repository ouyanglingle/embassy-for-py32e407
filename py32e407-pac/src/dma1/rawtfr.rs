#[doc = "Register `RAWTFR` reader"]
pub type R = crate::R<RawtfrSpec>;
#[doc = "Register `RAWTFR` writer"]
pub type W = crate::W<RawtfrSpec>;
#[doc = "Field `RAW` reader - Raw Status for IntTfr Interrupt"]
pub type RawR = crate::FieldReader;
#[doc = "Field `RAW` writer - Raw Status for IntTfr Interrupt"]
pub type RawW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Raw Status for IntTfr Interrupt"]
    #[inline(always)]
    pub fn raw(&self) -> RawR {
        RawR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Raw Status for IntTfr Interrupt"]
    #[inline(always)]
    pub fn raw(&mut self) -> RawW<'_, RawtfrSpec> {
        RawW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rawtfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawtfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawtfrSpec;
impl crate::RegisterSpec for RawtfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawtfr::R`](R) reader structure"]
impl crate::Readable for RawtfrSpec {}
#[doc = "`write(|w| ..)` method takes [`rawtfr::W`](W) writer structure"]
impl crate::Writable for RawtfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAWTFR to value 0"]
impl crate::Resettable for RawtfrSpec {}
