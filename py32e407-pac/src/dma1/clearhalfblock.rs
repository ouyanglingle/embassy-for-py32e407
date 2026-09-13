#[doc = "Register `CLEARHALFBLOCK` reader"]
pub type R = crate::R<ClearhalfblockSpec>;
#[doc = "Register `CLEARHALFBLOCK` writer"]
pub type W = crate::W<ClearhalfblockSpec>;
#[doc = "Field `CLEAR` reader - Clear for IntHalfBlock Interrupt"]
pub type ClearR = crate::FieldReader;
#[doc = "Field `CLEAR` writer - Clear for IntHalfBlock Interrupt"]
pub type ClearW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Clear for IntHalfBlock Interrupt"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clear for IntHalfBlock Interrupt"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<'_, ClearhalfblockSpec> {
        ClearW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clearhalfblock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearhalfblock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearhalfblockSpec;
impl crate::RegisterSpec for ClearhalfblockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clearhalfblock::R`](R) reader structure"]
impl crate::Readable for ClearhalfblockSpec {}
#[doc = "`write(|w| ..)` method takes [`clearhalfblock::W`](W) writer structure"]
impl crate::Writable for ClearhalfblockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLEARHALFBLOCK to value 0"]
impl crate::Resettable for ClearhalfblockSpec {}
