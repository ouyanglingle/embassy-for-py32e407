#[doc = "Register `IMR2` reader"]
pub type R = crate::R<Imr2Spec>;
#[doc = "Register `IMR2` writer"]
pub type W = crate::W<Imr2Spec>;
#[doc = "Field `IMR32` reader - "]
pub type Imr32R = crate::BitReader;
#[doc = "Field `IMR32` writer - "]
pub type Imr32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR33` reader - "]
pub type Imr33R = crate::BitReader;
#[doc = "Field `IMR33` writer - "]
pub type Imr33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR34` reader - "]
pub type Imr34R = crate::BitReader;
#[doc = "Field `IMR34` writer - "]
pub type Imr34W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn imr32(&self) -> Imr32R {
        Imr32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn imr33(&self) -> Imr33R {
        Imr33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn imr34(&self) -> Imr34R {
        Imr34R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn imr32(&mut self) -> Imr32W<'_, Imr2Spec> {
        Imr32W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn imr33(&mut self) -> Imr33W<'_, Imr2Spec> {
        Imr33W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn imr34(&mut self) -> Imr34W<'_, Imr2Spec> {
        Imr34W::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imr2Spec;
impl crate::RegisterSpec for Imr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr2::R`](R) reader structure"]
impl crate::Readable for Imr2Spec {}
#[doc = "`write(|w| ..)` method takes [`imr2::W`](W) writer structure"]
impl crate::Writable for Imr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMR2 to value 0"]
impl crate::Resettable for Imr2Spec {}
