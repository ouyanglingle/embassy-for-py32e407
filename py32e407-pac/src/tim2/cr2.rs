#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `CCDS` reader - "]
pub type CcdsR = crate::BitReader;
#[doc = "Field `CCDS` writer - "]
pub type CcdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMS_L` reader - "]
pub type MmsLR = crate::FieldReader;
#[doc = "Field `TI1S` reader - "]
pub type Ti1sR = crate::BitReader;
#[doc = "Field `TI1S` writer - "]
pub type Ti1sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMS_H` reader - "]
pub type MmsHR = crate::BitReader;
#[doc = "Field `MMS_H` writer - "]
pub type MmsHW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ccds(&self) -> CcdsR {
        CcdsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn mms_l(&self) -> MmsLR {
        MmsLR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ti1s(&self) -> Ti1sR {
        Ti1sR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn mms_h(&self) -> MmsHR {
        MmsHR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ccds(&mut self) -> CcdsW<'_, Cr2Spec> {
        CcdsW::new(self, 3)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ti1s(&mut self) -> Ti1sW<'_, Cr2Spec> {
        Ti1sW::new(self, 7)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn mms_h(&mut self) -> MmsHW<'_, Cr2Spec> {
        MmsHW::new(self, 25)
    }
}
#[doc = "desc CR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
