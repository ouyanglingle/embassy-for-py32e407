#[doc = "Register `CLEARTFR` reader"]
pub type R = crate::R<CleartfrSpec>;
#[doc = "Register `CLEARTFR` writer"]
pub type W = crate::W<CleartfrSpec>;
#[doc = "Field `CLEAR` reader - Clear for IntTfr Interrupt"]
pub type ClearR = crate::FieldReader;
#[doc = "Field `CLEAR` writer - Clear for IntTfr Interrupt"]
pub type ClearW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Clear for IntTfr Interrupt"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clear for IntTfr Interrupt"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<'_, CleartfrSpec> {
        ClearW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cleartfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cleartfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CleartfrSpec;
impl crate::RegisterSpec for CleartfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cleartfr::R`](R) reader structure"]
impl crate::Readable for CleartfrSpec {}
#[doc = "`write(|w| ..)` method takes [`cleartfr::W`](W) writer structure"]
impl crate::Writable for CleartfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLEARTFR to value 0"]
impl crate::Resettable for CleartfrSpec {}
