#[doc = "Register `RAWHALFBLOCK` reader"]
pub type R = crate::R<RawhalfblockSpec>;
#[doc = "Register `RAWHALFBLOCK` writer"]
pub type W = crate::W<RawhalfblockSpec>;
#[doc = "Field `RAW` reader - Raw Status for IntHalfBlock Interrupt"]
pub type RawR = crate::FieldReader;
#[doc = "Field `RAW` writer - Raw Status for IntHalfBlock Interrupt"]
pub type RawW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Raw Status for IntHalfBlock Interrupt"]
    #[inline(always)]
    pub fn raw(&self) -> RawR {
        RawR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Raw Status for IntHalfBlock Interrupt"]
    #[inline(always)]
    pub fn raw(&mut self) -> RawW<'_, RawhalfblockSpec> {
        RawW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rawhalfblock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawhalfblock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawhalfblockSpec;
impl crate::RegisterSpec for RawhalfblockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawhalfblock::R`](R) reader structure"]
impl crate::Readable for RawhalfblockSpec {}
#[doc = "`write(|w| ..)` method takes [`rawhalfblock::W`](W) writer structure"]
impl crate::Writable for RawhalfblockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAWHALFBLOCK to value 0"]
impl crate::Resettable for RawhalfblockSpec {}
