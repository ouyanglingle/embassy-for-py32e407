#[doc = "Register `EMR2` reader"]
pub type R = crate::R<Emr2Spec>;
#[doc = "Register `EMR2` writer"]
pub type W = crate::W<Emr2Spec>;
#[doc = "Field `EMR32` reader - "]
pub type Emr32R = crate::BitReader;
#[doc = "Field `EMR32` writer - "]
pub type Emr32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR33` reader - "]
pub type Emr33R = crate::BitReader;
#[doc = "Field `EMR33` writer - "]
pub type Emr33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR34` reader - "]
pub type Emr34R = crate::BitReader;
#[doc = "Field `EMR34` writer - "]
pub type Emr34W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn emr32(&self) -> Emr32R {
        Emr32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn emr33(&self) -> Emr33R {
        Emr33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn emr34(&self) -> Emr34R {
        Emr34R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn emr32(&mut self) -> Emr32W<'_, Emr2Spec> {
        Emr32W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn emr33(&mut self) -> Emr33W<'_, Emr2Spec> {
        Emr33W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn emr34(&mut self) -> Emr34W<'_, Emr2Spec> {
        Emr34W::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Emr2Spec;
impl crate::RegisterSpec for Emr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr2::R`](R) reader structure"]
impl crate::Readable for Emr2Spec {}
#[doc = "`write(|w| ..)` method takes [`emr2::W`](W) writer structure"]
impl crate::Writable for Emr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMR2 to value 0"]
impl crate::Resettable for Emr2Spec {}
