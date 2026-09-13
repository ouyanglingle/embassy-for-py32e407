#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `CCPC` reader - "]
pub type CcpcR = crate::BitReader;
#[doc = "Field `CCPC` writer - "]
pub type CcpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCUS` reader - "]
pub type CcusR = crate::BitReader;
#[doc = "Field `CCUS` writer - "]
pub type CcusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCDS` reader - "]
pub type CcdsR = crate::BitReader;
#[doc = "Field `CCDS` writer - "]
pub type CcdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMS_L` reader - "]
pub type MmsLR = crate::FieldReader;
#[doc = "Field `MMS_L` writer - "]
pub type MmsLW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TI1S` reader - "]
pub type Ti1sR = crate::BitReader;
#[doc = "Field `TI1S` writer - "]
pub type Ti1sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1` reader - "]
pub type Ois1R = crate::BitReader;
#[doc = "Field `OIS1` writer - "]
pub type Ois1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1N` reader - "]
pub type Ois1nR = crate::BitReader;
#[doc = "Field `OIS1N` writer - "]
pub type Ois1nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2` reader - "]
pub type Ois2R = crate::BitReader;
#[doc = "Field `OIS2` writer - "]
pub type Ois2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2N` reader - "]
pub type Ois2nR = crate::BitReader;
#[doc = "Field `OIS2N` writer - "]
pub type Ois2nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3` reader - "]
pub type Ois3R = crate::BitReader;
#[doc = "Field `OIS3` writer - "]
pub type Ois3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3N` reader - "]
pub type Ois3nR = crate::BitReader;
#[doc = "Field `OIS3N` writer - "]
pub type Ois3nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS4` reader - "]
pub type Ois4R = crate::BitReader;
#[doc = "Field `OIS4` writer - "]
pub type Ois4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS4N` reader - "]
pub type Ois4nR = crate::BitReader;
#[doc = "Field `OIS4N` writer - "]
pub type Ois4nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS5` reader - "]
pub type Ois5R = crate::BitReader;
#[doc = "Field `OIS5` writer - "]
pub type Ois5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS6` reader - "]
pub type Ois6R = crate::BitReader;
#[doc = "Field `OIS6` writer - "]
pub type Ois6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMS2` reader - "]
pub type Mms2R = crate::FieldReader;
#[doc = "Field `MMS2` writer - "]
pub type Mms2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MMS_H` reader - "]
pub type MmsHR = crate::BitReader;
#[doc = "Field `MMS_H` writer - "]
pub type MmsHW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ccpc(&self) -> CcpcR {
        CcpcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ccus(&self) -> CcusR {
        CcusR::new(((self.bits >> 2) & 1) != 0)
    }
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ois1(&self) -> Ois1R {
        Ois1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ois1n(&self) -> Ois1nR {
        Ois1nR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ois2(&self) -> Ois2R {
        Ois2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ois2n(&self) -> Ois2nR {
        Ois2nR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ois3(&self) -> Ois3R {
        Ois3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ois3n(&self) -> Ois3nR {
        Ois3nR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ois4(&self) -> Ois4R {
        Ois4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ois4n(&self) -> Ois4nR {
        Ois4nR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ois5(&self) -> Ois5R {
        Ois5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ois6(&self) -> Ois6R {
        Ois6R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn mms2(&self) -> Mms2R {
        Mms2R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn mms_h(&self) -> MmsHR {
        MmsHR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ccpc(&mut self) -> CcpcW<'_, Cr2Spec> {
        CcpcW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ccus(&mut self) -> CcusW<'_, Cr2Spec> {
        CcusW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ccds(&mut self) -> CcdsW<'_, Cr2Spec> {
        CcdsW::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn mms_l(&mut self) -> MmsLW<'_, Cr2Spec> {
        MmsLW::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ti1s(&mut self) -> Ti1sW<'_, Cr2Spec> {
        Ti1sW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ois1(&mut self) -> Ois1W<'_, Cr2Spec> {
        Ois1W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ois1n(&mut self) -> Ois1nW<'_, Cr2Spec> {
        Ois1nW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ois2(&mut self) -> Ois2W<'_, Cr2Spec> {
        Ois2W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ois2n(&mut self) -> Ois2nW<'_, Cr2Spec> {
        Ois2nW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ois3(&mut self) -> Ois3W<'_, Cr2Spec> {
        Ois3W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ois3n(&mut self) -> Ois3nW<'_, Cr2Spec> {
        Ois3nW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ois4(&mut self) -> Ois4W<'_, Cr2Spec> {
        Ois4W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ois4n(&mut self) -> Ois4nW<'_, Cr2Spec> {
        Ois4nW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ois5(&mut self) -> Ois5W<'_, Cr2Spec> {
        Ois5W::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ois6(&mut self) -> Ois6W<'_, Cr2Spec> {
        Ois6W::new(self, 18)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn mms2(&mut self) -> Mms2W<'_, Cr2Spec> {
        Mms2W::new(self, 20)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn mms_h(&mut self) -> MmsHW<'_, Cr2Spec> {
        MmsHW::new(self, 25)
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
