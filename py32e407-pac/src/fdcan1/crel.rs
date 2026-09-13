#[doc = "Register `CREL` reader"]
pub type R = crate::R<CrelSpec>;
#[doc = "Register `CREL` writer"]
pub type W = crate::W<CrelSpec>;
#[doc = "Field `DAY` reader - -"]
pub type DayR = crate::FieldReader;
#[doc = "Field `MON` reader - -"]
pub type MonR = crate::FieldReader;
#[doc = "Field `YEAR` reader - -"]
pub type YearR = crate::FieldReader;
#[doc = "Field `SUBSTEP` reader - -"]
pub type SubstepR = crate::FieldReader;
#[doc = "Field `STEP` reader - -"]
pub type StepR = crate::FieldReader;
#[doc = "Field `REL` reader - -"]
pub type RelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - -"]
    #[inline(always)]
    pub fn day(&self) -> DayR {
        DayR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - -"]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - -"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - -"]
    #[inline(always)]
    pub fn substep(&self) -> SubstepR {
        SubstepR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - -"]
    #[inline(always)]
    pub fn step(&self) -> StepR {
        StepR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - -"]
    #[inline(always)]
    pub fn rel(&self) -> RelR {
        RelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`crel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrelSpec;
impl crate::RegisterSpec for CrelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crel::R`](R) reader structure"]
impl crate::Readable for CrelSpec {}
#[doc = "`write(|w| ..)` method takes [`crel::W`](W) writer structure"]
impl crate::Writable for CrelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CREL to value 0"]
impl crate::Resettable for CrelSpec {}
