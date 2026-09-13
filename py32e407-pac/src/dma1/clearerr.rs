#[doc = "Register `CLEARERR` reader"]
pub type R = crate::R<ClearerrSpec>;
#[doc = "Register `CLEARERR` writer"]
pub type W = crate::W<ClearerrSpec>;
#[doc = "Field `CLEAR` reader - Clear for IntErr Interrupt"]
pub type ClearR = crate::FieldReader;
#[doc = "Field `CLEAR` writer - Clear for IntErr Interrupt"]
pub type ClearW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Clear for IntErr Interrupt"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clear for IntErr Interrupt"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<'_, ClearerrSpec> {
        ClearW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clearerr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearerr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearerrSpec;
impl crate::RegisterSpec for ClearerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clearerr::R`](R) reader structure"]
impl crate::Readable for ClearerrSpec {}
#[doc = "`write(|w| ..)` method takes [`clearerr::W`](W) writer structure"]
impl crate::Writable for ClearerrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLEARERR to value 0"]
impl crate::Resettable for ClearerrSpec {}
