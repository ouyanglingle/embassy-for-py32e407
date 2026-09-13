#[doc = "Register `MODER` reader"]
pub type R = crate::R<ModerSpec>;
#[doc = "Register `MODER` writer"]
pub type W = crate::W<ModerSpec>;
#[doc = "Field `MODER0` reader - GPIOB0 MODE"]
pub type Moder0R = crate::FieldReader;
#[doc = "Field `MODER0` writer - GPIOB0 MODE"]
pub type Moder0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER1` reader - GPIOB1 MODE"]
pub type Moder1R = crate::FieldReader;
#[doc = "Field `MODER1` writer - GPIOB1 MODE"]
pub type Moder1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER2` reader - GPIOB2 MODE"]
pub type Moder2R = crate::FieldReader;
#[doc = "Field `MODER2` writer - GPIOB2 MODE"]
pub type Moder2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER3` reader - GPIOB3 MODE"]
pub type Moder3R = crate::FieldReader;
#[doc = "Field `MODER3` writer - GPIOB3 MODE"]
pub type Moder3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER4` reader - GPIOB4 MODE"]
pub type Moder4R = crate::FieldReader;
#[doc = "Field `MODER4` writer - GPIOB4 MODE"]
pub type Moder4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER5` reader - GPIOB5 MODE"]
pub type Moder5R = crate::FieldReader;
#[doc = "Field `MODER5` writer - GPIOB5 MODE"]
pub type Moder5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER6` reader - GPIOB6 MODE"]
pub type Moder6R = crate::FieldReader;
#[doc = "Field `MODER6` writer - GPIOB6 MODE"]
pub type Moder6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER7` reader - GPIOB7 MODE"]
pub type Moder7R = crate::FieldReader;
#[doc = "Field `MODER7` writer - GPIOB7 MODE"]
pub type Moder7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER8` reader - GPIOB8 MODE"]
pub type Moder8R = crate::FieldReader;
#[doc = "Field `MODER8` writer - GPIOB8 MODE"]
pub type Moder8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER9` reader - GPIOB9 MODE"]
pub type Moder9R = crate::FieldReader;
#[doc = "Field `MODER9` writer - GPIOB9 MODE"]
pub type Moder9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER10` reader - GPIOB10 MODE"]
pub type Moder10R = crate::FieldReader;
#[doc = "Field `MODER10` writer - GPIOB10 MODE"]
pub type Moder10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER11` reader - GPIOB11 MODE"]
pub type Moder11R = crate::FieldReader;
#[doc = "Field `MODER11` writer - GPIOB11 MODE"]
pub type Moder11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER12` reader - GPIOB12 MODE"]
pub type Moder12R = crate::FieldReader;
#[doc = "Field `MODER12` writer - GPIOB12 MODE"]
pub type Moder12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER13` reader - GPIOB13 MODE"]
pub type Moder13R = crate::FieldReader;
#[doc = "Field `MODER13` writer - GPIOB13 MODE"]
pub type Moder13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER14` reader - GPIOB14 MODE"]
pub type Moder14R = crate::FieldReader;
#[doc = "Field `MODER14` writer - GPIOB14 MODE"]
pub type Moder14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER15` reader - GPIOB15 MODE"]
pub type Moder15R = crate::FieldReader;
#[doc = "Field `MODER15` writer - GPIOB15 MODE"]
pub type Moder15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - GPIOB0 MODE"]
    #[inline(always)]
    pub fn moder0(&self) -> Moder0R {
        Moder0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIOB1 MODE"]
    #[inline(always)]
    pub fn moder1(&self) -> Moder1R {
        Moder1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIOB2 MODE"]
    #[inline(always)]
    pub fn moder2(&self) -> Moder2R {
        Moder2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIOB3 MODE"]
    #[inline(always)]
    pub fn moder3(&self) -> Moder3R {
        Moder3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIOB4 MODE"]
    #[inline(always)]
    pub fn moder4(&self) -> Moder4R {
        Moder4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIOB5 MODE"]
    #[inline(always)]
    pub fn moder5(&self) -> Moder5R {
        Moder5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIOB6 MODE"]
    #[inline(always)]
    pub fn moder6(&self) -> Moder6R {
        Moder6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIOB7 MODE"]
    #[inline(always)]
    pub fn moder7(&self) -> Moder7R {
        Moder7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPIOB8 MODE"]
    #[inline(always)]
    pub fn moder8(&self) -> Moder8R {
        Moder8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPIOB9 MODE"]
    #[inline(always)]
    pub fn moder9(&self) -> Moder9R {
        Moder9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPIOB10 MODE"]
    #[inline(always)]
    pub fn moder10(&self) -> Moder10R {
        Moder10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - GPIOB11 MODE"]
    #[inline(always)]
    pub fn moder11(&self) -> Moder11R {
        Moder11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GPIOB12 MODE"]
    #[inline(always)]
    pub fn moder12(&self) -> Moder12R {
        Moder12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - GPIOB13 MODE"]
    #[inline(always)]
    pub fn moder13(&self) -> Moder13R {
        Moder13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - GPIOB14 MODE"]
    #[inline(always)]
    pub fn moder14(&self) -> Moder14R {
        Moder14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - GPIOB15 MODE"]
    #[inline(always)]
    pub fn moder15(&self) -> Moder15R {
        Moder15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIOB0 MODE"]
    #[inline(always)]
    pub fn moder0(&mut self) -> Moder0W<'_, ModerSpec> {
        Moder0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIOB1 MODE"]
    #[inline(always)]
    pub fn moder1(&mut self) -> Moder1W<'_, ModerSpec> {
        Moder1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIOB2 MODE"]
    #[inline(always)]
    pub fn moder2(&mut self) -> Moder2W<'_, ModerSpec> {
        Moder2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIOB3 MODE"]
    #[inline(always)]
    pub fn moder3(&mut self) -> Moder3W<'_, ModerSpec> {
        Moder3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIOB4 MODE"]
    #[inline(always)]
    pub fn moder4(&mut self) -> Moder4W<'_, ModerSpec> {
        Moder4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIOB5 MODE"]
    #[inline(always)]
    pub fn moder5(&mut self) -> Moder5W<'_, ModerSpec> {
        Moder5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIOB6 MODE"]
    #[inline(always)]
    pub fn moder6(&mut self) -> Moder6W<'_, ModerSpec> {
        Moder6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIOB7 MODE"]
    #[inline(always)]
    pub fn moder7(&mut self) -> Moder7W<'_, ModerSpec> {
        Moder7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - GPIOB8 MODE"]
    #[inline(always)]
    pub fn moder8(&mut self) -> Moder8W<'_, ModerSpec> {
        Moder8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - GPIOB9 MODE"]
    #[inline(always)]
    pub fn moder9(&mut self) -> Moder9W<'_, ModerSpec> {
        Moder9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - GPIOB10 MODE"]
    #[inline(always)]
    pub fn moder10(&mut self) -> Moder10W<'_, ModerSpec> {
        Moder10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - GPIOB11 MODE"]
    #[inline(always)]
    pub fn moder11(&mut self) -> Moder11W<'_, ModerSpec> {
        Moder11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - GPIOB12 MODE"]
    #[inline(always)]
    pub fn moder12(&mut self) -> Moder12W<'_, ModerSpec> {
        Moder12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - GPIOB13 MODE"]
    #[inline(always)]
    pub fn moder13(&mut self) -> Moder13W<'_, ModerSpec> {
        Moder13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - GPIOB14 MODE"]
    #[inline(always)]
    pub fn moder14(&mut self) -> Moder14W<'_, ModerSpec> {
        Moder14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - GPIOB15 MODE"]
    #[inline(always)]
    pub fn moder15(&mut self) -> Moder15W<'_, ModerSpec> {
        Moder15W::new(self, 30)
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
