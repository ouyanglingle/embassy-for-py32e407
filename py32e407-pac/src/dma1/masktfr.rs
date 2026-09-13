#[doc = "Register `MASKTFR` reader"]
pub type R = crate::R<MasktfrSpec>;
#[doc = "Register `MASKTFR` writer"]
pub type W = crate::W<MasktfrSpec>;
#[doc = "Field `INT_MASK` reader - Mask for IntTfr Interrupt"]
pub type IntMaskR = crate::FieldReader;
#[doc = "Field `INT_MASK` writer - Mask for IntTfr Interrupt"]
pub type IntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `INT_MASK_WE` reader - Interrupt Mask Write Enable"]
pub type IntMaskWeR = crate::FieldReader;
#[doc = "Field `INT_MASK_WE` writer - Interrupt Mask Write Enable"]
pub type IntMaskWeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Mask for IntTfr Interrupt"]
    #[inline(always)]
    pub fn int_mask(&self) -> IntMaskR {
        IntMaskR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Interrupt Mask Write Enable"]
    #[inline(always)]
    pub fn int_mask_we(&self) -> IntMaskWeR {
        IntMaskWeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Mask for IntTfr Interrupt"]
    #[inline(always)]
    pub fn int_mask(&mut self) -> IntMaskW<'_, MasktfrSpec> {
        IntMaskW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Interrupt Mask Write Enable"]
    #[inline(always)]
    pub fn int_mask_we(&mut self) -> IntMaskWeW<'_, MasktfrSpec> {
        IntMaskWeW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`masktfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masktfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MasktfrSpec;
impl crate::RegisterSpec for MasktfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`masktfr::R`](R) reader structure"]
impl crate::Readable for MasktfrSpec {}
#[doc = "`write(|w| ..)` method takes [`masktfr::W`](W) writer structure"]
impl crate::Writable for MasktfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASKTFR to value 0"]
impl crate::Resettable for MasktfrSpec {}
