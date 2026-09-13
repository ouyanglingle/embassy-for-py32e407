#[doc = "Register `INGFIFIF_STATUS` reader"]
pub type R = crate::R<IngfififStatusSpec>;
#[doc = "Register `INGFIFIF_STATUS` writer"]
pub type W = crate::W<IngfififStatusSpec>;
#[doc = "Field `ING_EMPTY` reader - "]
pub type IngEmptyR = crate::BitReader;
#[doc = "Field `ING_FULL` reader - "]
pub type IngFullR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ing_empty(&self) -> IngEmptyR {
        IngEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ing_full(&self) -> IngFullR {
        IngFullR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ingfifif_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ingfifif_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IngfififStatusSpec;
impl crate::RegisterSpec for IngfififStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ingfifif_status::R`](R) reader structure"]
impl crate::Readable for IngfififStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ingfifif_status::W`](W) writer structure"]
impl crate::Writable for IngfififStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INGFIFIF_STATUS to value 0"]
impl crate::Resettable for IngfififStatusSpec {}
