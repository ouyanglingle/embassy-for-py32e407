#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `RBG0_EN` reader - "]
pub type Rbg0EnR = crate::BitReader;
#[doc = "Field `RBG0_EN` writer - "]
pub type Rbg0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBG1_EN` reader - "]
pub type Rbg1EnR = crate::BitReader;
#[doc = "Field `RBG1_EN` writer - "]
pub type Rbg1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - "]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - "]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbg0_en(&self) -> Rbg0EnR {
        Rbg0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rbg1_en(&self) -> Rbg1EnR {
        Rbg1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbg0_en(&mut self) -> Rbg0EnW<'_, CrSpec> {
        Rbg0EnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rbg1_en(&mut self) -> Rbg1EnW<'_, CrSpec> {
        Rbg1EnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, CrSpec> {
        IeW::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
