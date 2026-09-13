#[doc = "Register `OR` reader"]
pub type R = crate::R<OrSpec>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<OrSpec>;
#[doc = "Field `IN1_L` reader - LPTimer input 1 remap extension"]
pub type In1LR = crate::BitReader;
#[doc = "Field `IN1_L` writer - LPTimer input 1 remap extension"]
pub type In1LW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN2_L` reader - LPTimer input 1 remap extension"]
pub type In2LR = crate::BitReader;
#[doc = "Field `IN2_L` writer - LPTimer input 1 remap extension"]
pub type In2LW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN1_H` reader - LPTimer input 3 remap extension"]
pub type In1HR = crate::FieldReader;
#[doc = "Field `IN1_H` writer - LPTimer input 3 remap extension"]
pub type In1HW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IN2_H` reader - LPTimer input 2 remap extension"]
pub type In2HR = crate::FieldReader;
#[doc = "Field `IN2_H` writer - LPTimer input 2 remap extension"]
pub type In2HW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - LPTimer input 1 remap extension"]
    #[inline(always)]
    pub fn in1_l(&self) -> In1LR {
        In1LR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPTimer input 1 remap extension"]
    #[inline(always)]
    pub fn in2_l(&self) -> In2LR {
        In2LR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - LPTimer input 3 remap extension"]
    #[inline(always)]
    pub fn in1_h(&self) -> In1HR {
        In1HR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - LPTimer input 2 remap extension"]
    #[inline(always)]
    pub fn in2_h(&self) -> In2HR {
        In2HR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LPTimer input 1 remap extension"]
    #[inline(always)]
    pub fn in1_l(&mut self) -> In1LW<'_, OrSpec> {
        In1LW::new(self, 0)
    }
    #[doc = "Bit 1 - LPTimer input 1 remap extension"]
    #[inline(always)]
    pub fn in2_l(&mut self) -> In2LW<'_, OrSpec> {
        In2LW::new(self, 1)
    }
    #[doc = "Bits 2:3 - LPTimer input 3 remap extension"]
    #[inline(always)]
    pub fn in1_h(&mut self) -> In1HW<'_, OrSpec> {
        In1HW::new(self, 2)
    }
    #[doc = "Bits 4:5 - LPTimer input 2 remap extension"]
    #[inline(always)]
    pub fn in2_h(&mut self) -> In2HW<'_, OrSpec> {
        In2HW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OrSpec;
impl crate::RegisterSpec for OrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for OrSpec {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for OrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OrSpec {}
