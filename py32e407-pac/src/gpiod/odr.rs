#[doc = "Register `ODR` reader"]
pub type R = crate::R<OdrSpec>;
#[doc = "Register `ODR` writer"]
pub type W = crate::W<OdrSpec>;
#[doc = "Field `OD0` reader - GPIOD0 output data"]
pub type Od0R = crate::BitReader;
#[doc = "Field `OD0` writer - GPIOD0 output data"]
pub type Od0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD1` reader - GPIOD1 output data"]
pub type Od1R = crate::BitReader;
#[doc = "Field `OD1` writer - GPIOD1 output data"]
pub type Od1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD2` reader - GPIOD2 output data"]
pub type Od2R = crate::BitReader;
#[doc = "Field `OD2` writer - GPIOD2 output data"]
pub type Od2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD3` reader - GPIOD3 output data"]
pub type Od3R = crate::BitReader;
#[doc = "Field `OD3` writer - GPIOD3 output data"]
pub type Od3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD4` reader - GPIOD4 output data"]
pub type Od4R = crate::BitReader;
#[doc = "Field `OD4` writer - GPIOD4 output data"]
pub type Od4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD5` reader - GPIOD5 output data"]
pub type Od5R = crate::BitReader;
#[doc = "Field `OD5` writer - GPIOD5 output data"]
pub type Od5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD6` reader - GPIOD6 output data"]
pub type Od6R = crate::BitReader;
#[doc = "Field `OD6` writer - GPIOD6 output data"]
pub type Od6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD7` reader - GPIOD7 output data"]
pub type Od7R = crate::BitReader;
#[doc = "Field `OD7` writer - GPIOD7 output data"]
pub type Od7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD8` reader - GPIOD8 output data"]
pub type Od8R = crate::BitReader;
#[doc = "Field `OD8` writer - GPIOD8 output data"]
pub type Od8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD9` reader - GPIOD9 output data"]
pub type Od9R = crate::BitReader;
#[doc = "Field `OD9` writer - GPIOD9 output data"]
pub type Od9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD10` reader - GPIOD10 output data"]
pub type Od10R = crate::BitReader;
#[doc = "Field `OD10` writer - GPIOD10 output data"]
pub type Od10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD11` reader - GPIOD11 output data"]
pub type Od11R = crate::BitReader;
#[doc = "Field `OD11` writer - GPIOD11 output data"]
pub type Od11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD12` reader - GPIOD12 output data"]
pub type Od12R = crate::BitReader;
#[doc = "Field `OD12` writer - GPIOD12 output data"]
pub type Od12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD13` reader - GPIOD13 output data"]
pub type Od13R = crate::BitReader;
#[doc = "Field `OD13` writer - GPIOD13 output data"]
pub type Od13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD14` reader - GPIOD14 output data"]
pub type Od14R = crate::BitReader;
#[doc = "Field `OD14` writer - GPIOD14 output data"]
pub type Od14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD15` reader - GPIOD15 output data"]
pub type Od15R = crate::BitReader;
#[doc = "Field `OD15` writer - GPIOD15 output data"]
pub type Od15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOD0 output data"]
    #[inline(always)]
    pub fn od0(&self) -> Od0R {
        Od0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOD1 output data"]
    #[inline(always)]
    pub fn od1(&self) -> Od1R {
        Od1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOD2 output data"]
    #[inline(always)]
    pub fn od2(&self) -> Od2R {
        Od2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOD3 output data"]
    #[inline(always)]
    pub fn od3(&self) -> Od3R {
        Od3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOD4 output data"]
    #[inline(always)]
    pub fn od4(&self) -> Od4R {
        Od4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOD5 output data"]
    #[inline(always)]
    pub fn od5(&self) -> Od5R {
        Od5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOD6 output data"]
    #[inline(always)]
    pub fn od6(&self) -> Od6R {
        Od6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOD7 output data"]
    #[inline(always)]
    pub fn od7(&self) -> Od7R {
        Od7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOD8 output data"]
    #[inline(always)]
    pub fn od8(&self) -> Od8R {
        Od8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIOD9 output data"]
    #[inline(always)]
    pub fn od9(&self) -> Od9R {
        Od9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOD10 output data"]
    #[inline(always)]
    pub fn od10(&self) -> Od10R {
        Od10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIOD11 output data"]
    #[inline(always)]
    pub fn od11(&self) -> Od11R {
        Od11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIOD12 output data"]
    #[inline(always)]
    pub fn od12(&self) -> Od12R {
        Od12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIOD13 output data"]
    #[inline(always)]
    pub fn od13(&self) -> Od13R {
        Od13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIOD14 output data"]
    #[inline(always)]
    pub fn od14(&self) -> Od14R {
        Od14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIOD15 output data"]
    #[inline(always)]
    pub fn od15(&self) -> Od15R {
        Od15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOD0 output data"]
    #[inline(always)]
    pub fn od0(&mut self) -> Od0W<'_, OdrSpec> {
        Od0W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOD1 output data"]
    #[inline(always)]
    pub fn od1(&mut self) -> Od1W<'_, OdrSpec> {
        Od1W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOD2 output data"]
    #[inline(always)]
    pub fn od2(&mut self) -> Od2W<'_, OdrSpec> {
        Od2W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOD3 output data"]
    #[inline(always)]
    pub fn od3(&mut self) -> Od3W<'_, OdrSpec> {
        Od3W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOD4 output data"]
    #[inline(always)]
    pub fn od4(&mut self) -> Od4W<'_, OdrSpec> {
        Od4W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOD5 output data"]
    #[inline(always)]
    pub fn od5(&mut self) -> Od5W<'_, OdrSpec> {
        Od5W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOD6 output data"]
    #[inline(always)]
    pub fn od6(&mut self) -> Od6W<'_, OdrSpec> {
        Od6W::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOD7 output data"]
    #[inline(always)]
    pub fn od7(&mut self) -> Od7W<'_, OdrSpec> {
        Od7W::new(self, 7)
    }
    #[doc = "Bit 8 - GPIOD8 output data"]
    #[inline(always)]
    pub fn od8(&mut self) -> Od8W<'_, OdrSpec> {
        Od8W::new(self, 8)
    }
    #[doc = "Bit 9 - GPIOD9 output data"]
    #[inline(always)]
    pub fn od9(&mut self) -> Od9W<'_, OdrSpec> {
        Od9W::new(self, 9)
    }
    #[doc = "Bit 10 - GPIOD10 output data"]
    #[inline(always)]
    pub fn od10(&mut self) -> Od10W<'_, OdrSpec> {
        Od10W::new(self, 10)
    }
    #[doc = "Bit 11 - GPIOD11 output data"]
    #[inline(always)]
    pub fn od11(&mut self) -> Od11W<'_, OdrSpec> {
        Od11W::new(self, 11)
    }
    #[doc = "Bit 12 - GPIOD12 output data"]
    #[inline(always)]
    pub fn od12(&mut self) -> Od12W<'_, OdrSpec> {
        Od12W::new(self, 12)
    }
    #[doc = "Bit 13 - GPIOD13 output data"]
    #[inline(always)]
    pub fn od13(&mut self) -> Od13W<'_, OdrSpec> {
        Od13W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIOD14 output data"]
    #[inline(always)]
    pub fn od14(&mut self) -> Od14W<'_, OdrSpec> {
        Od14W::new(self, 14)
    }
    #[doc = "Bit 15 - GPIOD15 output data"]
    #[inline(always)]
    pub fn od15(&mut self) -> Od15W<'_, OdrSpec> {
        Od15W::new(self, 15)
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
