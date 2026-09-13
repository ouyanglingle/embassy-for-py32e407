#[doc = "Register `CCR5` reader"]
pub type R = crate::R<Ccr5Spec>;
#[doc = "Register `CCR5` writer"]
pub type W = crate::W<Ccr5Spec>;
#[doc = "Field `CCR5` reader - "]
pub type Ccr5R = crate::FieldReader<u32>;
#[doc = "Field `CCR5` writer - "]
pub type Ccr5W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `GC5C1` reader - "]
pub type Gc5c1R = crate::BitReader;
#[doc = "Field `GC5C1` writer - "]
pub type Gc5c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC5C2` reader - "]
pub type Gc5c2R = crate::BitReader;
#[doc = "Field `GC5C2` writer - "]
pub type Gc5c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC5C3` reader - "]
pub type Gc5c3R = crate::BitReader;
#[doc = "Field `GC5C3` writer - "]
pub type Gc5c3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn ccr5(&self) -> Ccr5R {
        Ccr5R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gc5c1(&self) -> Gc5c1R {
        Gc5c1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gc5c2(&self) -> Gc5c2R {
        Gc5c2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gc5c3(&self) -> Gc5c3R {
        Gc5c3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn ccr5(&mut self) -> Ccr5W<'_, Ccr5Spec> {
        Ccr5W::new(self, 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gc5c1(&mut self) -> Gc5c1W<'_, Ccr5Spec> {
        Gc5c1W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gc5c2(&mut self) -> Gc5c2W<'_, Ccr5Spec> {
        Gc5c2W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gc5c3(&mut self) -> Gc5c3W<'_, Ccr5Spec> {
        Gc5c3W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr5Spec;
impl crate::RegisterSpec for Ccr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr5::R`](R) reader structure"]
impl crate::Readable for Ccr5Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr5::W`](W) writer structure"]
impl crate::Writable for Ccr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR5 to value 0"]
impl crate::Resettable for Ccr5Spec {}
