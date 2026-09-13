#[doc = "Register `CALFACT` reader"]
pub type R = crate::R<CalfactSpec>;
#[doc = "Register `CALFACT` writer"]
pub type W = crate::W<CalfactSpec>;
#[doc = "Field `OFFSUC` reader - "]
pub type OffsucR = crate::BitReader;
#[doc = "Field `OFFSUC` writer - "]
pub type OffsucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPSUC` reader - "]
pub type CapsucR = crate::BitReader;
#[doc = "Field `CAPSUC` writer - "]
pub type CapsucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALON` reader - "]
pub type CalonR = crate::BitReader;
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn offsuc(&self) -> OffsucR {
        OffsucR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn capsuc(&self) -> CapsucR {
        CapsucR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn calon(&self) -> CalonR {
        CalonR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn offsuc(&mut self) -> OffsucW<'_, CalfactSpec> {
        OffsucW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn capsuc(&mut self) -> CapsucW<'_, CalfactSpec> {
        CapsucW::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`calfact::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalfactSpec;
impl crate::RegisterSpec for CalfactSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calfact::R`](R) reader structure"]
impl crate::Readable for CalfactSpec {}
#[doc = "`write(|w| ..)` method takes [`calfact::W`](W) writer structure"]
impl crate::Writable for CalfactSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALFACT to value 0"]
impl crate::Resettable for CalfactSpec {}
