#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AfrlSpec>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AfrlSpec>;
#[doc = "Field `AFSEL0` reader - GPIOF0 AF select"]
pub type Afsel0R = crate::FieldReader;
#[doc = "Field `AFSEL0` writer - GPIOF0 AF select"]
pub type Afsel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL1` reader - GPIOF1 AF select"]
pub type Afsel1R = crate::FieldReader;
#[doc = "Field `AFSEL1` writer - GPIOF1 AF select"]
pub type Afsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL2` reader - GPIOF2 AF select"]
pub type Afsel2R = crate::FieldReader;
#[doc = "Field `AFSEL2` writer - GPIOF2 AF select"]
pub type Afsel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL3` reader - GPIOF3 AF select"]
pub type Afsel3R = crate::FieldReader;
#[doc = "Field `AFSEL3` writer - GPIOF3 AF select"]
pub type Afsel3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL4` reader - GPIOF4 AF select"]
pub type Afsel4R = crate::FieldReader;
#[doc = "Field `AFSEL4` writer - GPIOF4 AF select"]
pub type Afsel4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL5` reader - GPIOF5 AF select"]
pub type Afsel5R = crate::FieldReader;
#[doc = "Field `AFSEL5` writer - GPIOF5 AF select"]
pub type Afsel5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - GPIOF0 AF select"]
    #[inline(always)]
    pub fn afsel0(&self) -> Afsel0R {
        Afsel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - GPIOF1 AF select"]
    #[inline(always)]
    pub fn afsel1(&self) -> Afsel1R {
        Afsel1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - GPIOF2 AF select"]
    #[inline(always)]
    pub fn afsel2(&self) -> Afsel2R {
        Afsel2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - GPIOF3 AF select"]
    #[inline(always)]
    pub fn afsel3(&self) -> Afsel3R {
        Afsel3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - GPIOF4 AF select"]
    #[inline(always)]
    pub fn afsel4(&self) -> Afsel4R {
        Afsel4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - GPIOF5 AF select"]
    #[inline(always)]
    pub fn afsel5(&self) -> Afsel5R {
        Afsel5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - GPIOF0 AF select"]
    #[inline(always)]
    pub fn afsel0(&mut self) -> Afsel0W<'_, AfrlSpec> {
        Afsel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - GPIOF1 AF select"]
    #[inline(always)]
    pub fn afsel1(&mut self) -> Afsel1W<'_, AfrlSpec> {
        Afsel1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - GPIOF2 AF select"]
    #[inline(always)]
    pub fn afsel2(&mut self) -> Afsel2W<'_, AfrlSpec> {
        Afsel2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - GPIOF3 AF select"]
    #[inline(always)]
    pub fn afsel3(&mut self) -> Afsel3W<'_, AfrlSpec> {
        Afsel3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - GPIOF4 AF select"]
    #[inline(always)]
    pub fn afsel4(&mut self) -> Afsel4W<'_, AfrlSpec> {
        Afsel4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - GPIOF5 AF select"]
    #[inline(always)]
    pub fn afsel5(&mut self) -> Afsel5W<'_, AfrlSpec> {
        Afsel5W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrlSpec;
impl crate::RegisterSpec for AfrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AfrlSpec {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AfrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AfrlSpec {}
