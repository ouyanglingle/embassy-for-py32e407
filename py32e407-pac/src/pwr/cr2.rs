#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `PVDE` reader - "]
pub type PvdeR = crate::BitReader;
#[doc = "Field `PVDE` writer - "]
pub type PvdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - "]
pub type PlsR = crate::FieldReader;
#[doc = "Field `PLS` writer - "]
pub type PlsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLTEN` reader - "]
pub type FltenR = crate::BitReader;
#[doc = "Field `FLTEN` writer - "]
pub type FltenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT_CTRL` reader - "]
pub type FltCtrlR = crate::FieldReader;
#[doc = "Field `FLT_CTRL` writer - "]
pub type FltCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pvde(&self) -> PvdeR {
        PvdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn pls(&self) -> PlsR {
        PlsR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn flten(&self) -> FltenR {
        FltenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn flt_ctrl(&self) -> FltCtrlR {
        FltCtrlR::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pvde(&mut self) -> PvdeW<'_, Cr2Spec> {
        PvdeW::new(self, 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn pls(&mut self) -> PlsW<'_, Cr2Spec> {
        PlsW::new(self, 1)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn flten(&mut self) -> FltenW<'_, Cr2Spec> {
        FltenW::new(self, 8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn flt_ctrl(&mut self) -> FltCtrlW<'_, Cr2Spec> {
        FltCtrlW::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
