#[doc = "Register `RAWBLOCK` reader"]
pub type R = crate::R<RawblockSpec>;
#[doc = "Register `RAWBLOCK` writer"]
pub type W = crate::W<RawblockSpec>;
#[doc = "Field `RAW` reader - Raw Status for IntBlock Interrupt"]
pub type RawR = crate::FieldReader;
#[doc = "Field `RAW` writer - Raw Status for IntBlock Interrupt"]
pub type RawW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Raw Status for IntBlock Interrupt"]
    #[inline(always)]
    pub fn raw(&self) -> RawR {
        RawR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Raw Status for IntBlock Interrupt"]
    #[inline(always)]
    pub fn raw(&mut self) -> RawW<'_, RawblockSpec> {
        RawW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rawblock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawblock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawblockSpec;
impl crate::RegisterSpec for RawblockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawblock::R`](R) reader structure"]
impl crate::Readable for RawblockSpec {}
#[doc = "`write(|w| ..)` method takes [`rawblock::W`](W) writer structure"]
impl crate::Writable for RawblockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAWBLOCK to value 0"]
impl crate::Resettable for RawblockSpec {}
