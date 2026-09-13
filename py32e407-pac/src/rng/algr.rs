#[doc = "Register `ALGR` reader"]
pub type R = crate::R<AlgrSpec>;
#[doc = "Register `ALGR` writer"]
pub type W = crate::W<AlgrSpec>;
#[doc = "Field `RNG_ALG` reader - "]
pub type RngAlgR = crate::FieldReader<u32>;
#[doc = "Field `RNG_ALG` writer - "]
pub type RngAlgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rng_alg(&self) -> RngAlgR {
        RngAlgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rng_alg(&mut self) -> RngAlgW<'_, AlgrSpec> {
        RngAlgW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`algr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`algr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlgrSpec;
impl crate::RegisterSpec for AlgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`algr::R`](R) reader structure"]
impl crate::Readable for AlgrSpec {}
#[doc = "`write(|w| ..)` method takes [`algr::W`](W) writer structure"]
impl crate::Writable for AlgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALGR to value 0"]
impl crate::Resettable for AlgrSpec {}
