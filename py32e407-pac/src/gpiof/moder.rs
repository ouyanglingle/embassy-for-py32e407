#[doc = "Register `MODER` reader"]
pub type R = crate::R<ModerSpec>;
#[doc = "Register `MODER` writer"]
pub type W = crate::W<ModerSpec>;
#[doc = "Field `MODER0` reader - GPIOF0 MODE"]
pub type Moder0R = crate::FieldReader;
#[doc = "Field `MODER0` writer - GPIOF0 MODE"]
pub type Moder0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER1` reader - GPIOF1 MODE"]
pub type Moder1R = crate::FieldReader;
#[doc = "Field `MODER1` writer - GPIOF1 MODE"]
pub type Moder1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER2` reader - GPIOF2 MODE"]
pub type Moder2R = crate::FieldReader;
#[doc = "Field `MODER2` writer - GPIOF2 MODE"]
pub type Moder2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER3` reader - GPIOF3 MODE"]
pub type Moder3R = crate::FieldReader;
#[doc = "Field `MODER3` writer - GPIOF3 MODE"]
pub type Moder3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER4` reader - GPIOF4 MODE"]
pub type Moder4R = crate::FieldReader;
#[doc = "Field `MODER4` writer - GPIOF4 MODE"]
pub type Moder4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER5` reader - GPIOF5 MODE"]
pub type Moder5R = crate::FieldReader;
#[doc = "Field `MODER5` writer - GPIOF5 MODE"]
pub type Moder5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - GPIOF0 MODE"]
    #[inline(always)]
    pub fn moder0(&self) -> Moder0R {
        Moder0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIOF1 MODE"]
    #[inline(always)]
    pub fn moder1(&self) -> Moder1R {
        Moder1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIOF2 MODE"]
    #[inline(always)]
    pub fn moder2(&self) -> Moder2R {
        Moder2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIOF3 MODE"]
    #[inline(always)]
    pub fn moder3(&self) -> Moder3R {
        Moder3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIOF4 MODE"]
    #[inline(always)]
    pub fn moder4(&self) -> Moder4R {
        Moder4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIOF5 MODE"]
    #[inline(always)]
    pub fn moder5(&self) -> Moder5R {
        Moder5R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIOF0 MODE"]
    #[inline(always)]
    pub fn moder0(&mut self) -> Moder0W<'_, ModerSpec> {
        Moder0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIOF1 MODE"]
    #[inline(always)]
    pub fn moder1(&mut self) -> Moder1W<'_, ModerSpec> {
        Moder1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIOF2 MODE"]
    #[inline(always)]
    pub fn moder2(&mut self) -> Moder2W<'_, ModerSpec> {
        Moder2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIOF3 MODE"]
    #[inline(always)]
    pub fn moder3(&mut self) -> Moder3W<'_, ModerSpec> {
        Moder3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIOF4 MODE"]
    #[inline(always)]
    pub fn moder4(&mut self) -> Moder4W<'_, ModerSpec> {
        Moder4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIOF5 MODE"]
    #[inline(always)]
    pub fn moder5(&mut self) -> Moder5W<'_, ModerSpec> {
        Moder5W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModerSpec;
impl crate::RegisterSpec for ModerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moder::R`](R) reader structure"]
impl crate::Readable for ModerSpec {}
#[doc = "`write(|w| ..)` method takes [`moder::W`](W) writer structure"]
impl crate::Writable for ModerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODER to value 0"]
impl crate::Resettable for ModerSpec {}
