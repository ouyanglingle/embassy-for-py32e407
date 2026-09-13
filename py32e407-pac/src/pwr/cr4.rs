#[doc = "Register `CR4` reader"]
pub type R = crate::R<Cr4Spec>;
#[doc = "Register `CR4` writer"]
pub type W = crate::W<Cr4Spec>;
#[doc = "Field `WP1` reader - "]
pub type Wp1R = crate::BitReader;
#[doc = "Field `WP1` writer - "]
pub type Wp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP2` reader - "]
pub type Wp2R = crate::BitReader;
#[doc = "Field `WP2` writer - "]
pub type Wp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP3` reader - "]
pub type Wp3R = crate::BitReader;
#[doc = "Field `WP3` writer - "]
pub type Wp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP4` reader - "]
pub type Wp4R = crate::BitReader;
#[doc = "Field `WP4` writer - "]
pub type Wp4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP5` reader - "]
pub type Wp5R = crate::BitReader;
#[doc = "Field `WP5` writer - "]
pub type Wp5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wp1(&self) -> Wp1R {
        Wp1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wp2(&self) -> Wp2R {
        Wp2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wp3(&self) -> Wp3R {
        Wp3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wp4(&self) -> Wp4R {
        Wp4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wp5(&self) -> Wp5R {
        Wp5R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wp1(&mut self) -> Wp1W<'_, Cr4Spec> {
        Wp1W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wp2(&mut self) -> Wp2W<'_, Cr4Spec> {
        Wp2W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wp3(&mut self) -> Wp3W<'_, Cr4Spec> {
        Wp3W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wp4(&mut self) -> Wp4W<'_, Cr4Spec> {
        Wp4W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wp5(&mut self) -> Wp5W<'_, Cr4Spec> {
        Wp5W::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr4Spec;
impl crate::RegisterSpec for Cr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr4::R`](R) reader structure"]
impl crate::Readable for Cr4Spec {}
#[doc = "`write(|w| ..)` method takes [`cr4::W`](W) writer structure"]
impl crate::Writable for Cr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR4 to value 0"]
impl crate::Resettable for Cr4Spec {}
