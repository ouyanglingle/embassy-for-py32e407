#[doc = "Register `CCMR1_OUTPUT` reader"]
pub type R = crate::R<Ccmr1OutputSpec>;
#[doc = "Register `CCMR1_OUTPUT` writer"]
pub type W = crate::W<Ccmr1OutputSpec>;
#[doc = "Field `CC1S` reader - "]
pub type Cc1sR = crate::FieldReader;
#[doc = "Field `CC1S` writer - "]
pub type Cc1sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC1FE` reader - "]
pub type Oc1feR = crate::BitReader;
#[doc = "Field `OC1FE` writer - "]
pub type Oc1feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1PE` reader - "]
pub type Oc1peR = crate::BitReader;
#[doc = "Field `OC1PE` writer - "]
pub type Oc1peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M_L` reader - "]
pub type Oc1mLR = crate::FieldReader;
#[doc = "Field `OC1M_L` writer - "]
pub type Oc1mLW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC1CE` reader - "]
pub type Oc1ceR = crate::BitReader;
#[doc = "Field `OC1CE` writer - "]
pub type Oc1ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2S` reader - "]
pub type Cc2sR = crate::FieldReader;
#[doc = "Field `CC2S` writer - "]
pub type Cc2sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC2FE` reader - "]
pub type Oc2feR = crate::BitReader;
#[doc = "Field `OC2FE` writer - "]
pub type Oc2feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2PE` reader - "]
pub type Oc2peR = crate::BitReader;
#[doc = "Field `OC2PE` writer - "]
pub type Oc2peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2M_L` reader - "]
pub type Oc2mLR = crate::FieldReader;
#[doc = "Field `OC2M_L` writer - "]
pub type Oc2mLW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC2CE` reader - "]
pub type Oc2ceR = crate::BitReader;
#[doc = "Field `OC2CE` writer - "]
pub type Oc2ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M_H` reader - "]
pub type Oc1mHR = crate::BitReader;
#[doc = "Field `OC1M_H` writer - "]
pub type Oc1mHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2M_H` reader - "]
pub type Oc2mHR = crate::BitReader;
#[doc = "Field `OC2M_H` writer - "]
pub type Oc2mHW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cc1s(&self) -> Cc1sR {
        Cc1sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn oc1fe(&self) -> Oc1feR {
        Oc1feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn oc1pe(&self) -> Oc1peR {
        Oc1peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn oc1m_l(&self) -> Oc1mLR {
        Oc1mLR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oc1ce(&self) -> Oc1ceR {
        Oc1ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cc2s(&self) -> Cc2sR {
        Cc2sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oc2fe(&self) -> Oc2feR {
        Oc2feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn oc2pe(&self) -> Oc2peR {
        Oc2peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn oc2m_l(&self) -> Oc2mLR {
        Oc2mLR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn oc2ce(&self) -> Oc2ceR {
        Oc2ceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn oc1m_h(&self) -> Oc1mHR {
        Oc1mHR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn oc2m_h(&self) -> Oc2mHR {
        Oc2mHR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cc1s(&mut self) -> Cc1sW<'_, Ccmr1OutputSpec> {
        Cc1sW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn oc1fe(&mut self) -> Oc1feW<'_, Ccmr1OutputSpec> {
        Oc1feW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn oc1pe(&mut self) -> Oc1peW<'_, Ccmr1OutputSpec> {
        Oc1peW::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn oc1m_l(&mut self) -> Oc1mLW<'_, Ccmr1OutputSpec> {
        Oc1mLW::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oc1ce(&mut self) -> Oc1ceW<'_, Ccmr1OutputSpec> {
        Oc1ceW::new(self, 7)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cc2s(&mut self) -> Cc2sW<'_, Ccmr1OutputSpec> {
        Cc2sW::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oc2fe(&mut self) -> Oc2feW<'_, Ccmr1OutputSpec> {
        Oc2feW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn oc2pe(&mut self) -> Oc2peW<'_, Ccmr1OutputSpec> {
        Oc2peW::new(self, 11)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn oc2m_l(&mut self) -> Oc2mLW<'_, Ccmr1OutputSpec> {
        Oc2mLW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn oc2ce(&mut self) -> Oc2ceW<'_, Ccmr1OutputSpec> {
        Oc2ceW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn oc1m_h(&mut self) -> Oc1mHW<'_, Ccmr1OutputSpec> {
        Oc1mHW::new(self, 16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn oc2m_h(&mut self) -> Oc2mHW<'_, Ccmr1OutputSpec> {
        Oc2mHW::new(self, 24)
    }
}
#[doc = "desc CCMR1:OUTPUT\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr1OutputSpec;
impl crate::RegisterSpec for Ccmr1OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_output::R`](R) reader structure"]
impl crate::Readable for Ccmr1OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_output::W`](W) writer structure"]
impl crate::Writable for Ccmr1OutputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR1_OUTPUT to value 0"]
impl crate::Resettable for Ccmr1OutputSpec {}
