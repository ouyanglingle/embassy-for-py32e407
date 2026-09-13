#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AfrhSpec>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AfrhSpec>;
#[doc = "Field `AFSEL8` reader - GPIOB8 AF select"]
pub type Afsel8R = crate::FieldReader;
#[doc = "Field `AFSEL8` writer - GPIOB8 AF select"]
pub type Afsel8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL9` reader - GPIOB9 AF select"]
pub type Afsel9R = crate::FieldReader;
#[doc = "Field `AFSEL9` writer - GPIOB9 AF select"]
pub type Afsel9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL10` reader - GPIOB10 AF select"]
pub type Afsel10R = crate::FieldReader;
#[doc = "Field `AFSEL10` writer - GPIOB10 AF select"]
pub type Afsel10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL11` reader - GPIOB11 AF select"]
pub type Afsel11R = crate::FieldReader;
#[doc = "Field `AFSEL11` writer - GPIOB11 AF select"]
pub type Afsel11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL12` reader - GPIOB12 AF select"]
pub type Afsel12R = crate::FieldReader;
#[doc = "Field `AFSEL12` writer - GPIOB12 AF select"]
pub type Afsel12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL13` reader - GPIOB13 AF select"]
pub type Afsel13R = crate::FieldReader;
#[doc = "Field `AFSEL13` writer - GPIOB13 AF select"]
pub type Afsel13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL14` reader - GPIOB14 AF select"]
pub type Afsel14R = crate::FieldReader;
#[doc = "Field `AFSEL14` writer - GPIOB14 AF select"]
pub type Afsel14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL15` reader - GPIOB15 AF select"]
pub type Afsel15R = crate::FieldReader;
#[doc = "Field `AFSEL15` writer - GPIOB15 AF select"]
pub type Afsel15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - GPIOB8 AF select"]
    #[inline(always)]
    pub fn afsel8(&self) -> Afsel8R {
        Afsel8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - GPIOB9 AF select"]
    #[inline(always)]
    pub fn afsel9(&self) -> Afsel9R {
        Afsel9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - GPIOB10 AF select"]
    #[inline(always)]
    pub fn afsel10(&self) -> Afsel10R {
        Afsel10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - GPIOB11 AF select"]
    #[inline(always)]
    pub fn afsel11(&self) -> Afsel11R {
        Afsel11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - GPIOB12 AF select"]
    #[inline(always)]
    pub fn afsel12(&self) -> Afsel12R {
        Afsel12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - GPIOB13 AF select"]
    #[inline(always)]
    pub fn afsel13(&self) -> Afsel13R {
        Afsel13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - GPIOB14 AF select"]
    #[inline(always)]
    pub fn afsel14(&self) -> Afsel14R {
        Afsel14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - GPIOB15 AF select"]
    #[inline(always)]
    pub fn afsel15(&self) -> Afsel15R {
        Afsel15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - GPIOB8 AF select"]
    #[inline(always)]
    pub fn afsel8(&mut self) -> Afsel8W<'_, AfrhSpec> {
        Afsel8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - GPIOB9 AF select"]
    #[inline(always)]
    pub fn afsel9(&mut self) -> Afsel9W<'_, AfrhSpec> {
        Afsel9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - GPIOB10 AF select"]
    #[inline(always)]
    pub fn afsel10(&mut self) -> Afsel10W<'_, AfrhSpec> {
        Afsel10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - GPIOB11 AF select"]
    #[inline(always)]
    pub fn afsel11(&mut self) -> Afsel11W<'_, AfrhSpec> {
        Afsel11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - GPIOB12 AF select"]
    #[inline(always)]
    pub fn afsel12(&mut self) -> Afsel12W<'_, AfrhSpec> {
        Afsel12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - GPIOB13 AF select"]
    #[inline(always)]
    pub fn afsel13(&mut self) -> Afsel13W<'_, AfrhSpec> {
        Afsel13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - GPIOB14 AF select"]
    #[inline(always)]
    pub fn afsel14(&mut self) -> Afsel14W<'_, AfrhSpec> {
        Afsel14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - GPIOB15 AF select"]
    #[inline(always)]
    pub fn afsel15(&mut self) -> Afsel15W<'_, AfrhSpec> {
        Afsel15W::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrhSpec;
impl crate::RegisterSpec for AfrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrh::R`](R) reader structure"]
impl crate::Readable for AfrhSpec {}
#[doc = "`write(|w| ..)` method takes [`afrh::W`](W) writer structure"]
impl crate::Writable for AfrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFRH to value 0"]
impl crate::Resettable for AfrhSpec {}
