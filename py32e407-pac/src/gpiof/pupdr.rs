#[doc = "Register `PUPDR` reader"]
pub type R = crate::R<PupdrSpec>;
#[doc = "Register `PUPDR` writer"]
pub type W = crate::W<PupdrSpec>;
#[doc = "Field `PUPD0` reader - GPIOF0 pull up/pull down"]
pub type Pupd0R = crate::FieldReader;
#[doc = "Field `PUPD0` writer - GPIOF0 pull up/pull down"]
pub type Pupd0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD1` reader - GPIOF1 pull up/pull down"]
pub type Pupd1R = crate::FieldReader;
#[doc = "Field `PUPD1` writer - GPIOF1 pull up/pull down"]
pub type Pupd1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD2` reader - GPIOF2 pull up/pull down"]
pub type Pupd2R = crate::FieldReader;
#[doc = "Field `PUPD2` writer - GPIOF2 pull up/pull down"]
pub type Pupd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD3` reader - GPIOF3 pull up/pull down"]
pub type Pupd3R = crate::FieldReader;
#[doc = "Field `PUPD3` writer - GPIOF3 pull up/pull down"]
pub type Pupd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD4` reader - GPIOF4 pull up/pull down"]
pub type Pupd4R = crate::FieldReader;
#[doc = "Field `PUPD4` writer - GPIOF4 pull up/pull down"]
pub type Pupd4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD5` reader - GPIOF5 pull up/pull down"]
pub type Pupd5R = crate::FieldReader;
#[doc = "Field `PUPD5` writer - GPIOF5 pull up/pull down"]
pub type Pupd5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - GPIOF0 pull up/pull down"]
    #[inline(always)]
    pub fn pupd0(&self) -> Pupd0R {
        Pupd0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIOF1 pull up/pull down"]
    #[inline(always)]
    pub fn pupd1(&self) -> Pupd1R {
        Pupd1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIOF2 pull up/pull down"]
    #[inline(always)]
    pub fn pupd2(&self) -> Pupd2R {
        Pupd2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIOF3 pull up/pull down"]
    #[inline(always)]
    pub fn pupd3(&self) -> Pupd3R {
        Pupd3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIOF4 pull up/pull down"]
    #[inline(always)]
    pub fn pupd4(&self) -> Pupd4R {
        Pupd4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIOF5 pull up/pull down"]
    #[inline(always)]
    pub fn pupd5(&self) -> Pupd5R {
        Pupd5R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIOF0 pull up/pull down"]
    #[inline(always)]
    pub fn pupd0(&mut self) -> Pupd0W<'_, PupdrSpec> {
        Pupd0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIOF1 pull up/pull down"]
    #[inline(always)]
    pub fn pupd1(&mut self) -> Pupd1W<'_, PupdrSpec> {
        Pupd1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIOF2 pull up/pull down"]
    #[inline(always)]
    pub fn pupd2(&mut self) -> Pupd2W<'_, PupdrSpec> {
        Pupd2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIOF3 pull up/pull down"]
    #[inline(always)]
    pub fn pupd3(&mut self) -> Pupd3W<'_, PupdrSpec> {
        Pupd3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIOF4 pull up/pull down"]
    #[inline(always)]
    pub fn pupd4(&mut self) -> Pupd4W<'_, PupdrSpec> {
        Pupd4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIOF5 pull up/pull down"]
    #[inline(always)]
    pub fn pupd5(&mut self) -> Pupd5W<'_, PupdrSpec> {
        Pupd5W::new(self, 10)
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
