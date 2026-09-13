#[doc = "Register `ODR` reader"]
pub type R = crate::R<OdrSpec>;
#[doc = "Register `ODR` writer"]
pub type W = crate::W<OdrSpec>;
#[doc = "Field `OD0` reader - GPIOF0 output data"]
pub type Od0R = crate::BitReader;
#[doc = "Field `OD0` writer - GPIOF0 output data"]
pub type Od0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD1` reader - GPIOF1 output data"]
pub type Od1R = crate::BitReader;
#[doc = "Field `OD1` writer - GPIOF1 output data"]
pub type Od1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD2` reader - GPIOF2 output data"]
pub type Od2R = crate::BitReader;
#[doc = "Field `OD2` writer - GPIOF2 output data"]
pub type Od2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD3` reader - GPIOF3 output data"]
pub type Od3R = crate::BitReader;
#[doc = "Field `OD3` writer - GPIOF3 output data"]
pub type Od3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD4` reader - GPIOF4 output data"]
pub type Od4R = crate::BitReader;
#[doc = "Field `OD4` writer - GPIOF4 output data"]
pub type Od4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD5` reader - GPIOF5 output data"]
pub type Od5R = crate::BitReader;
#[doc = "Field `OD5` writer - GPIOF5 output data"]
pub type Od5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOF0 output data"]
    #[inline(always)]
    pub fn od0(&self) -> Od0R {
        Od0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOF1 output data"]
    #[inline(always)]
    pub fn od1(&self) -> Od1R {
        Od1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOF2 output data"]
    #[inline(always)]
    pub fn od2(&self) -> Od2R {
        Od2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOF3 output data"]
    #[inline(always)]
    pub fn od3(&self) -> Od3R {
        Od3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOF4 output data"]
    #[inline(always)]
    pub fn od4(&self) -> Od4R {
        Od4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOF5 output data"]
    #[inline(always)]
    pub fn od5(&self) -> Od5R {
        Od5R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOF0 output data"]
    #[inline(always)]
    pub fn od0(&mut self) -> Od0W<'_, OdrSpec> {
        Od0W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOF1 output data"]
    #[inline(always)]
    pub fn od1(&mut self) -> Od1W<'_, OdrSpec> {
        Od1W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOF2 output data"]
    #[inline(always)]
    pub fn od2(&mut self) -> Od2W<'_, OdrSpec> {
        Od2W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOF3 output data"]
    #[inline(always)]
    pub fn od3(&mut self) -> Od3W<'_, OdrSpec> {
        Od3W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOF4 output data"]
    #[inline(always)]
    pub fn od4(&mut self) -> Od4W<'_, OdrSpec> {
        Od4W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOF5 output data"]
    #[inline(always)]
    pub fn od5(&mut self) -> Od5W<'_, OdrSpec> {
        Od5W::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`odr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdrSpec;
impl crate::RegisterSpec for OdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odr::R`](R) reader structure"]
impl crate::Readable for OdrSpec {}
#[doc = "`write(|w| ..)` method takes [`odr::W`](W) writer structure"]
impl crate::Writable for OdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ODR to value 0"]
impl crate::Resettable for OdrSpec {}
