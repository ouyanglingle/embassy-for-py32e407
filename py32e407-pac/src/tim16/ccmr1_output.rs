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
#[doc = "Field `OC1M_H` reader - "]
pub type Oc1mHR = crate::BitReader;
#[doc = "Field `OC1M_H` writer - "]
pub type Oc1mHW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn oc1m_h(&self) -> Oc1mHR {
        Oc1mHR::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn oc1m_h(&mut self) -> Oc1mHW<'_, Ccmr1OutputSpec> {
        Oc1mHW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
