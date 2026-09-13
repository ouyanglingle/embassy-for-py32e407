#[doc = "Register `CCMR3` reader"]
pub type R = crate::R<Ccmr3Spec>;
#[doc = "Register `CCMR3` writer"]
pub type W = crate::W<Ccmr3Spec>;
#[doc = "Field `OC5FE` reader - "]
pub type Oc5feR = crate::BitReader;
#[doc = "Field `OC5FE` writer - "]
pub type Oc5feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5PE` reader - "]
pub type Oc5peR = crate::BitReader;
#[doc = "Field `OC5PE` writer - "]
pub type Oc5peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5M_L` reader - "]
pub type Oc5mLR = crate::FieldReader;
#[doc = "Field `OC5M_L` writer - "]
pub type Oc5mLW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC5CE` reader - "]
pub type Oc5ceR = crate::BitReader;
#[doc = "Field `OC5CE` writer - "]
pub type Oc5ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6FE` reader - "]
pub type Oc6feR = crate::BitReader;
#[doc = "Field `OC6FE` writer - "]
pub type Oc6feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6PE` reader - "]
pub type Oc6peR = crate::BitReader;
#[doc = "Field `OC6PE` writer - "]
pub type Oc6peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6M_L` reader - "]
pub type Oc6mLR = crate::FieldReader;
#[doc = "Field `OC6M_L` writer - "]
pub type Oc6mLW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC6CE` reader - "]
pub type Oc6ceR = crate::BitReader;
#[doc = "Field `OC6CE` writer - "]
pub type Oc6ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5M_H` reader - "]
pub type Oc5mHR = crate::BitReader;
#[doc = "Field `OC5M_H` writer - "]
pub type Oc5mHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6M_H` reader - "]
pub type Oc6mHR = crate::BitReader;
#[doc = "Field `OC6M_H` writer - "]
pub type Oc6mHW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn oc5fe(&self) -> Oc5feR {
        Oc5feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn oc5pe(&self) -> Oc5peR {
        Oc5peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn oc5m_l(&self) -> Oc5mLR {
        Oc5mLR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oc5ce(&self) -> Oc5ceR {
        Oc5ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oc6fe(&self) -> Oc6feR {
        Oc6feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn oc6pe(&self) -> Oc6peR {
        Oc6peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn oc6m_l(&self) -> Oc6mLR {
        Oc6mLR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn oc6ce(&self) -> Oc6ceR {
        Oc6ceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn oc5m_h(&self) -> Oc5mHR {
        Oc5mHR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn oc6m_h(&self) -> Oc6mHR {
        Oc6mHR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn oc5fe(&mut self) -> Oc5feW<'_, Ccmr3Spec> {
        Oc5feW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn oc5pe(&mut self) -> Oc5peW<'_, Ccmr3Spec> {
        Oc5peW::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn oc5m_l(&mut self) -> Oc5mLW<'_, Ccmr3Spec> {
        Oc5mLW::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oc5ce(&mut self) -> Oc5ceW<'_, Ccmr3Spec> {
        Oc5ceW::new(self, 7)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oc6fe(&mut self) -> Oc6feW<'_, Ccmr3Spec> {
        Oc6feW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn oc6pe(&mut self) -> Oc6peW<'_, Ccmr3Spec> {
        Oc6peW::new(self, 11)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn oc6m_l(&mut self) -> Oc6mLW<'_, Ccmr3Spec> {
        Oc6mLW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn oc6ce(&mut self) -> Oc6ceW<'_, Ccmr3Spec> {
        Oc6ceW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn oc5m_h(&mut self) -> Oc5mHW<'_, Ccmr3Spec> {
        Oc5mHW::new(self, 16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn oc6m_h(&mut self) -> Oc6mHW<'_, Ccmr3Spec> {
        Oc6mHW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr3Spec;
impl crate::RegisterSpec for Ccmr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr3::R`](R) reader structure"]
impl crate::Readable for Ccmr3Spec {}
#[doc = "`write(|w| ..)` method takes [`ccmr3::W`](W) writer structure"]
impl crate::Writable for Ccmr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR3 to value 0"]
impl crate::Resettable for Ccmr3Spec {}
