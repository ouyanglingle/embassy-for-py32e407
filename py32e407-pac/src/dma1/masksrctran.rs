#[doc = "Register `MASKSRCTRAN` reader"]
pub type R = crate::R<MasksrctranSpec>;
#[doc = "Register `MASKSRCTRAN` writer"]
pub type W = crate::W<MasksrctranSpec>;
#[doc = "Field `INT_MASK` reader - Mask for IntSrcTran Interrupt"]
pub type IntMaskR = crate::FieldReader;
#[doc = "Field `INT_MASK` writer - Mask for IntSrcTran Interrupt"]
pub type IntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `INT_MASK_WE` reader - Interrupt Mask Write Enable"]
pub type IntMaskWeR = crate::FieldReader;
#[doc = "Field `INT_MASK_WE` writer - Interrupt Mask Write Enable"]
pub type IntMaskWeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Mask for IntSrcTran Interrupt"]
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
    #[doc = "Bits 0:5 - Mask for IntSrcTran Interrupt"]
    #[inline(always)]
    pub fn int_mask(&mut self) -> IntMaskW<'_, MasksrctranSpec> {
        IntMaskW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Interrupt Mask Write Enable"]
    #[inline(always)]
    pub fn int_mask_we(&mut self) -> IntMaskWeW<'_, MasksrctranSpec> {
        IntMaskWeW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`masksrctran::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masksrctran::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MasksrctranSpec;
impl crate::RegisterSpec for MasksrctranSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`masksrctran::R`](R) reader structure"]
impl crate::Readable for MasksrctranSpec {}
#[doc = "`write(|w| ..)` method takes [`masksrctran::W`](W) writer structure"]
impl crate::Writable for MasksrctranSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASKSRCTRAN to value 0"]
impl crate::Resettable for MasksrctranSpec {}
