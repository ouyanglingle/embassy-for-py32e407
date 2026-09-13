#[doc = "Register `CCMR2_OUTPUT` reader"]
pub type R = crate::R<Ccmr2OutputSpec>;
#[doc = "Register `CCMR2_OUTPUT` writer"]
pub type W = crate::W<Ccmr2OutputSpec>;
#[doc = "Field `CC3S` reader - "]
pub type Cc3sR = crate::FieldReader;
#[doc = "Field `CC3S` writer - "]
pub type Cc3sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC3FE` reader - "]
pub type Oc3feR = crate::BitReader;
#[doc = "Field `OC3FE` writer - "]
pub type Oc3feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3PE` reader - "]
pub type Oc3peR = crate::BitReader;
#[doc = "Field `OC3PE` writer - "]
pub type Oc3peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3M_L` reader - "]
pub type Oc3mLR = crate::FieldReader;
#[doc = "Field `OC3M_L` writer - "]
pub type Oc3mLW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC3CE` reader - "]
pub type Oc3ceR = crate::BitReader;
#[doc = "Field `OC3CE` writer - "]
pub type Oc3ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4S` reader - "]
pub type Cc4sR = crate::FieldReader;
#[doc = "Field `CC4S` writer - "]
pub type Cc4sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC4FE` reader - "]
pub type Oc4feR = crate::BitReader;
#[doc = "Field `OC4FE` writer - "]
pub type Oc4feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4PE` reader - "]
pub type Oc4peR = crate::BitReader;
#[doc = "Field `OC4PE` writer - "]
pub type Oc4peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M_L` reader - "]
pub type Oc4mLR = crate::FieldReader;
#[doc = "Field `OC4M_L` writer - "]
pub type Oc4mLW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC4CE` reader - "]
pub type Oc4ceR = crate::BitReader;
#[doc = "Field `OC4CE` writer - "]
pub type Oc4ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3M_H` reader - "]
pub type Oc3mHR = crate::BitReader;
#[doc = "Field `OC3M_H` writer - "]
pub type Oc3mHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M_H` reader - "]
pub type Oc4mHR = crate::BitReader;
#[doc = "Field `OC4M_H` writer - "]
pub type Oc4mHW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cc3s(&self) -> Cc3sR {
        Cc3sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn oc3fe(&self) -> Oc3feR {
        Oc3feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn oc3pe(&self) -> Oc3peR {
        Oc3peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn oc3m_l(&self) -> Oc3mLR {
        Oc3mLR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oc3ce(&self) -> Oc3ceR {
        Oc3ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cc4s(&self) -> Cc4sR {
        Cc4sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oc4fe(&self) -> Oc4feR {
        Oc4feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn oc4pe(&self) -> Oc4peR {
        Oc4peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn oc4m_l(&self) -> Oc4mLR {
        Oc4mLR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn oc4ce(&self) -> Oc4ceR {
        Oc4ceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn oc3m_h(&self) -> Oc3mHR {
        Oc3mHR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn oc4m_h(&self) -> Oc4mHR {
        Oc4mHR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cc3s(&mut self) -> Cc3sW<'_, Ccmr2OutputSpec> {
        Cc3sW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn oc3fe(&mut self) -> Oc3feW<'_, Ccmr2OutputSpec> {
        Oc3feW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn oc3pe(&mut self) -> Oc3peW<'_, Ccmr2OutputSpec> {
        Oc3peW::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn oc3m_l(&mut self) -> Oc3mLW<'_, Ccmr2OutputSpec> {
        Oc3mLW::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oc3ce(&mut self) -> Oc3ceW<'_, Ccmr2OutputSpec> {
        Oc3ceW::new(self, 7)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cc4s(&mut self) -> Cc4sW<'_, Ccmr2OutputSpec> {
        Cc4sW::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oc4fe(&mut self) -> Oc4feW<'_, Ccmr2OutputSpec> {
        Oc4feW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn oc4pe(&mut self) -> Oc4peW<'_, Ccmr2OutputSpec> {
        Oc4peW::new(self, 11)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn oc4m_l(&mut self) -> Oc4mLW<'_, Ccmr2OutputSpec> {
        Oc4mLW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn oc4ce(&mut self) -> Oc4ceW<'_, Ccmr2OutputSpec> {
        Oc4ceW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn oc3m_h(&mut self) -> Oc3mHW<'_, Ccmr2OutputSpec> {
        Oc3mHW::new(self, 16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn oc4m_h(&mut self) -> Oc4mHW<'_, Ccmr2OutputSpec> {
        Oc4mHW::new(self, 24)
    }
}
#[doc = "desc CCMR2:OUTPUT\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr2OutputSpec;
impl crate::RegisterSpec for Ccmr2OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2_output::R`](R) reader structure"]
impl crate::Readable for Ccmr2OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr2_output::W`](W) writer structure"]
impl crate::Writable for Ccmr2OutputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR2_OUTPUT to value 0"]
impl crate::Resettable for Ccmr2OutputSpec {}
