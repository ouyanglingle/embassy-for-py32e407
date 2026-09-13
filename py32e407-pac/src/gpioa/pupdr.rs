#[doc = "Register `PUPDR` reader"]
pub type R = crate::R<PupdrSpec>;
#[doc = "Register `PUPDR` writer"]
pub type W = crate::W<PupdrSpec>;
#[doc = "Field `PUPD0` reader - GPIOA0 pull up/pull down"]
pub type Pupd0R = crate::FieldReader;
#[doc = "Field `PUPD0` writer - GPIOA0 pull up/pull down"]
pub type Pupd0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD1` reader - GPIOA1 pull up/pull down"]
pub type Pupd1R = crate::FieldReader;
#[doc = "Field `PUPD1` writer - GPIOA1 pull up/pull down"]
pub type Pupd1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD2` reader - GPIOA2 pull up/pull down"]
pub type Pupd2R = crate::FieldReader;
#[doc = "Field `PUPD2` writer - GPIOA2 pull up/pull down"]
pub type Pupd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD3` reader - GPIOA3 pull up/pull down"]
pub type Pupd3R = crate::FieldReader;
#[doc = "Field `PUPD3` writer - GPIOA3 pull up/pull down"]
pub type Pupd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD4` reader - GPIOA4 pull up/pull down"]
pub type Pupd4R = crate::FieldReader;
#[doc = "Field `PUPD4` writer - GPIOA4 pull up/pull down"]
pub type Pupd4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD5` reader - GPIOA5 pull up/pull down"]
pub type Pupd5R = crate::FieldReader;
#[doc = "Field `PUPD5` writer - GPIOA5 pull up/pull down"]
pub type Pupd5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD6` reader - GPIOA6 pull up/pull down"]
pub type Pupd6R = crate::FieldReader;
#[doc = "Field `PUPD6` writer - GPIOA6 pull up/pull down"]
pub type Pupd6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD7` reader - GPIOA7 pull up/pull down"]
pub type Pupd7R = crate::FieldReader;
#[doc = "Field `PUPD7` writer - GPIOA7 pull up/pull down"]
pub type Pupd7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD8` reader - GPIOA8 pull up/pull down"]
pub type Pupd8R = crate::FieldReader;
#[doc = "Field `PUPD8` writer - GPIOA8 pull up/pull down"]
pub type Pupd8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD9` reader - GPIOA9 pull up/pull down"]
pub type Pupd9R = crate::FieldReader;
#[doc = "Field `PUPD9` writer - GPIOA9 pull up/pull down"]
pub type Pupd9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD10` reader - GPIOA10 pull up/pull down"]
pub type Pupd10R = crate::FieldReader;
#[doc = "Field `PUPD10` writer - GPIOA10 pull up/pull down"]
pub type Pupd10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD11` reader - GPIOA11 pull up/pull down"]
pub type Pupd11R = crate::FieldReader;
#[doc = "Field `PUPD11` writer - GPIOA11 pull up/pull down"]
pub type Pupd11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD12` reader - GPIOA12 pull up/pull down"]
pub type Pupd12R = crate::FieldReader;
#[doc = "Field `PUPD12` writer - GPIOA12 pull up/pull down"]
pub type Pupd12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD13` reader - GPIOA13 pull up/pull down"]
pub type Pupd13R = crate::FieldReader;
#[doc = "Field `PUPD13` writer - GPIOA13 pull up/pull down"]
pub type Pupd13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD14` reader - GPIOA14 pull up/pull down"]
pub type Pupd14R = crate::FieldReader;
#[doc = "Field `PUPD14` writer - GPIOA14 pull up/pull down"]
pub type Pupd14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD15` reader - GPIOA15 pull up/pull down"]
pub type Pupd15R = crate::FieldReader;
#[doc = "Field `PUPD15` writer - GPIOA15 pull up/pull down"]
pub type Pupd15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - GPIOA0 pull up/pull down"]
    #[inline(always)]
    pub fn pupd0(&self) -> Pupd0R {
        Pupd0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIOA1 pull up/pull down"]
    #[inline(always)]
    pub fn pupd1(&self) -> Pupd1R {
        Pupd1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIOA2 pull up/pull down"]
    #[inline(always)]
    pub fn pupd2(&self) -> Pupd2R {
        Pupd2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIOA3 pull up/pull down"]
    #[inline(always)]
    pub fn pupd3(&self) -> Pupd3R {
        Pupd3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIOA4 pull up/pull down"]
    #[inline(always)]
    pub fn pupd4(&self) -> Pupd4R {
        Pupd4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIOA5 pull up/pull down"]
    #[inline(always)]
    pub fn pupd5(&self) -> Pupd5R {
        Pupd5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIOA6 pull up/pull down"]
    #[inline(always)]
    pub fn pupd6(&self) -> Pupd6R {
        Pupd6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIOA7 pull up/pull down"]
    #[inline(always)]
    pub fn pupd7(&self) -> Pupd7R {
        Pupd7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPIOA8 pull up/pull down"]
    #[inline(always)]
    pub fn pupd8(&self) -> Pupd8R {
        Pupd8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPIOA9 pull up/pull down"]
    #[inline(always)]
    pub fn pupd9(&self) -> Pupd9R {
        Pupd9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPIOA10 pull up/pull down"]
    #[inline(always)]
    pub fn pupd10(&self) -> Pupd10R {
        Pupd10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - GPIOA11 pull up/pull down"]
    #[inline(always)]
    pub fn pupd11(&self) -> Pupd11R {
        Pupd11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GPIOA12 pull up/pull down"]
    #[inline(always)]
    pub fn pupd12(&self) -> Pupd12R {
        Pupd12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - GPIOA13 pull up/pull down"]
    #[inline(always)]
    pub fn pupd13(&self) -> Pupd13R {
        Pupd13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - GPIOA14 pull up/pull down"]
    #[inline(always)]
    pub fn pupd14(&self) -> Pupd14R {
        Pupd14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - GPIOA15 pull up/pull down"]
    #[inline(always)]
    pub fn pupd15(&self) -> Pupd15R {
        Pupd15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIOA0 pull up/pull down"]
    #[inline(always)]
    pub fn pupd0(&mut self) -> Pupd0W<'_, PupdrSpec> {
        Pupd0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIOA1 pull up/pull down"]
    #[inline(always)]
    pub fn pupd1(&mut self) -> Pupd1W<'_, PupdrSpec> {
        Pupd1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIOA2 pull up/pull down"]
    #[inline(always)]
    pub fn pupd2(&mut self) -> Pupd2W<'_, PupdrSpec> {
        Pupd2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIOA3 pull up/pull down"]
    #[inline(always)]
    pub fn pupd3(&mut self) -> Pupd3W<'_, PupdrSpec> {
        Pupd3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIOA4 pull up/pull down"]
    #[inline(always)]
    pub fn pupd4(&mut self) -> Pupd4W<'_, PupdrSpec> {
        Pupd4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIOA5 pull up/pull down"]
    #[inline(always)]
    pub fn pupd5(&mut self) -> Pupd5W<'_, PupdrSpec> {
        Pupd5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIOA6 pull up/pull down"]
    #[inline(always)]
    pub fn pupd6(&mut self) -> Pupd6W<'_, PupdrSpec> {
        Pupd6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIOA7 pull up/pull down"]
    #[inline(always)]
    pub fn pupd7(&mut self) -> Pupd7W<'_, PupdrSpec> {
        Pupd7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - GPIOA8 pull up/pull down"]
    #[inline(always)]
    pub fn pupd8(&mut self) -> Pupd8W<'_, PupdrSpec> {
        Pupd8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - GPIOA9 pull up/pull down"]
    #[inline(always)]
    pub fn pupd9(&mut self) -> Pupd9W<'_, PupdrSpec> {
        Pupd9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - GPIOA10 pull up/pull down"]
    #[inline(always)]
    pub fn pupd10(&mut self) -> Pupd10W<'_, PupdrSpec> {
        Pupd10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - GPIOA11 pull up/pull down"]
    #[inline(always)]
    pub fn pupd11(&mut self) -> Pupd11W<'_, PupdrSpec> {
        Pupd11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - GPIOA12 pull up/pull down"]
    #[inline(always)]
    pub fn pupd12(&mut self) -> Pupd12W<'_, PupdrSpec> {
        Pupd12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - GPIOA13 pull up/pull down"]
    #[inline(always)]
    pub fn pupd13(&mut self) -> Pupd13W<'_, PupdrSpec> {
        Pupd13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - GPIOA14 pull up/pull down"]
    #[inline(always)]
    pub fn pupd14(&mut self) -> Pupd14W<'_, PupdrSpec> {
        Pupd14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - GPIOA15 pull up/pull down"]
    #[inline(always)]
    pub fn pupd15(&mut self) -> Pupd15W<'_, PupdrSpec> {
        Pupd15W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pupdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PupdrSpec;
impl crate::RegisterSpec for PupdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pupdr::R`](R) reader structure"]
impl crate::Readable for PupdrSpec {}
#[doc = "`write(|w| ..)` method takes [`pupdr::W`](W) writer structure"]
impl crate::Writable for PupdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PUPDR to value 0"]
impl crate::Resettable for PupdrSpec {}
