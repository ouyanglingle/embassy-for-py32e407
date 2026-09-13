#[doc = "Register `CLEARBLOCK` reader"]
pub type R = crate::R<ClearblockSpec>;
#[doc = "Register `CLEARBLOCK` writer"]
pub type W = crate::W<ClearblockSpec>;
#[doc = "Field `CLEAR` reader - Clear for IntBlock Interrupt"]
pub type ClearR = crate::FieldReader;
#[doc = "Field `CLEAR` writer - Clear for IntBlock Interrupt"]
pub type ClearW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Clear for IntBlock Interrupt"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clear for IntBlock Interrupt"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<'_, ClearblockSpec> {
        ClearW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clearblock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearblock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearblockSpec;
impl crate::RegisterSpec for ClearblockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clearblock::R`](R) reader structure"]
impl crate::Readable for ClearblockSpec {}
#[doc = "`write(|w| ..)` method takes [`clearblock::W`](W) writer structure"]
impl crate::Writable for ClearblockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLEARBLOCK to value 0"]
impl crate::Resettable for ClearblockSpec {}
