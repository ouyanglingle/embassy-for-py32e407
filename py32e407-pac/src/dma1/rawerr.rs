#[doc = "Register `RAWERR` reader"]
pub type R = crate::R<RawerrSpec>;
#[doc = "Register `RAWERR` writer"]
pub type W = crate::W<RawerrSpec>;
#[doc = "Field `RAW` reader - Raw Status for IntErr Interrupt"]
pub type RawR = crate::FieldReader;
#[doc = "Field `RAW` writer - Raw Status for IntErr Interrupt"]
pub type RawW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Raw Status for IntErr Interrupt"]
    #[inline(always)]
    pub fn raw(&self) -> RawR {
        RawR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Raw Status for IntErr Interrupt"]
    #[inline(always)]
    pub fn raw(&mut self) -> RawW<'_, RawerrSpec> {
        RawW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rawerr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawerr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawerrSpec;
impl crate::RegisterSpec for RawerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawerr::R`](R) reader structure"]
impl crate::Readable for RawerrSpec {}
#[doc = "`write(|w| ..)` method takes [`rawerr::W`](W) writer structure"]
impl crate::Writable for RawerrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAWERR to value 0"]
impl crate::Resettable for RawerrSpec {}
