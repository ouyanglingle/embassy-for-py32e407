#[doc = "Register `ENGFIFO_STATUS` reader"]
pub type R = crate::R<EngfifoStatusSpec>;
#[doc = "Register `ENGFIFO_STATUS` writer"]
pub type W = crate::W<EngfifoStatusSpec>;
#[doc = "Field `ENG_EMPTY` reader - "]
pub type EngEmptyR = crate::BitReader;
#[doc = "Field `ENG_FULL` reader - "]
pub type EngFullR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn eng_empty(&self) -> EngEmptyR {
        EngEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn eng_full(&self) -> EngFullR {
        EngFullR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`engfifo_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`engfifo_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EngfifoStatusSpec;
impl crate::RegisterSpec for EngfifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`engfifo_status::R`](R) reader structure"]
impl crate::Readable for EngfifoStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`engfifo_status::W`](W) writer structure"]
impl crate::Writable for EngfifoStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENGFIFO_STATUS to value 0"]
impl crate::Resettable for EngfifoStatusSpec {}
