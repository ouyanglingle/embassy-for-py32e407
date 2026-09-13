#[doc = "Register `OTYPER` reader"]
pub type R = crate::R<OtyperSpec>;
#[doc = "Register `OTYPER` writer"]
pub type W = crate::W<OtyperSpec>;
#[doc = "Field `OT0` reader - GPIOE0 Output type"]
pub type Ot0R = crate::BitReader;
#[doc = "Field `OT0` writer - GPIOE0 Output type"]
pub type Ot0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT1` reader - GPIOE1 Output type"]
pub type Ot1R = crate::BitReader;
#[doc = "Field `OT1` writer - GPIOE1 Output type"]
pub type Ot1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT2` reader - GPIOE2 Output type"]
pub type Ot2R = crate::BitReader;
#[doc = "Field `OT2` writer - GPIOE2 Output type"]
pub type Ot2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT3` reader - GPIOE3 Output type"]
pub type Ot3R = crate::BitReader;
#[doc = "Field `OT3` writer - GPIOE3 Output type"]
pub type Ot3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT4` reader - GPIOE4 Output type"]
pub type Ot4R = crate::BitReader;
#[doc = "Field `OT4` writer - GPIOE4 Output type"]
pub type Ot4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT5` reader - GPIOE5 Output type"]
pub type Ot5R = crate::BitReader;
#[doc = "Field `OT5` writer - GPIOE5 Output type"]
pub type Ot5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT6` reader - GPIOE6 Output type"]
pub type Ot6R = crate::BitReader;
#[doc = "Field `OT6` writer - GPIOE6 Output type"]
pub type Ot6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT7` reader - GPIOE7 Output type"]
pub type Ot7R = crate::BitReader;
#[doc = "Field `OT7` writer - GPIOE7 Output type"]
pub type Ot7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT8` reader - GPIOE8 Output type"]
pub type Ot8R = crate::BitReader;
#[doc = "Field `OT8` writer - GPIOE8 Output type"]
pub type Ot8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT9` reader - GPIOE9 Output type"]
pub type Ot9R = crate::BitReader;
#[doc = "Field `OT9` writer - GPIOE9 Output type"]
pub type Ot9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT10` reader - GPIOE10 Output type"]
pub type Ot10R = crate::BitReader;
#[doc = "Field `OT10` writer - GPIOE10 Output type"]
pub type Ot10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT11` reader - GPIOE11 Output type"]
pub type Ot11R = crate::BitReader;
#[doc = "Field `OT11` writer - GPIOE11 Output type"]
pub type Ot11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT12` reader - GPIOE12 Output type"]
pub type Ot12R = crate::BitReader;
#[doc = "Field `OT12` writer - GPIOE12 Output type"]
pub type Ot12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT13` reader - GPIOE13 Output type"]
pub type Ot13R = crate::BitReader;
#[doc = "Field `OT13` writer - GPIOE13 Output type"]
pub type Ot13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT14` reader - GPIOE14 Output type"]
pub type Ot14R = crate::BitReader;
#[doc = "Field `OT14` writer - GPIOE14 Output type"]
pub type Ot14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT15` reader - GPIOE15 Output type"]
pub type Ot15R = crate::BitReader;
#[doc = "Field `OT15` writer - GPIOE15 Output type"]
pub type Ot15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOE0 Output type"]
    #[inline(always)]
    pub fn ot0(&self) -> Ot0R {
        Ot0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOE1 Output type"]
    #[inline(always)]
    pub fn ot1(&self) -> Ot1R {
        Ot1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOE2 Output type"]
    #[inline(always)]
    pub fn ot2(&self) -> Ot2R {
        Ot2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOE3 Output type"]
    #[inline(always)]
    pub fn ot3(&self) -> Ot3R {
        Ot3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOE4 Output type"]
    #[inline(always)]
    pub fn ot4(&self) -> Ot4R {
        Ot4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOE5 Output type"]
    #[inline(always)]
    pub fn ot5(&self) -> Ot5R {
        Ot5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOE6 Output type"]
    #[inline(always)]
    pub fn ot6(&self) -> Ot6R {
        Ot6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOE7 Output type"]
    #[inline(always)]
    pub fn ot7(&self) -> Ot7R {
        Ot7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOE8 Output type"]
    #[inline(always)]
    pub fn ot8(&self) -> Ot8R {
        Ot8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIOE9 Output type"]
    #[inline(always)]
    pub fn ot9(&self) -> Ot9R {
        Ot9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOE10 Output type"]
    #[inline(always)]
    pub fn ot10(&self) -> Ot10R {
        Ot10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIOE11 Output type"]
    #[inline(always)]
    pub fn ot11(&self) -> Ot11R {
        Ot11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIOE12 Output type"]
    #[inline(always)]
    pub fn ot12(&self) -> Ot12R {
        Ot12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIOE13 Output type"]
    #[inline(always)]
    pub fn ot13(&self) -> Ot13R {
        Ot13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIOE14 Output type"]
    #[inline(always)]
    pub fn ot14(&self) -> Ot14R {
        Ot14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIOE15 Output type"]
    #[inline(always)]
    pub fn ot15(&self) -> Ot15R {
        Ot15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOE0 Output type"]
    #[inline(always)]
    pub fn ot0(&mut self) -> Ot0W<'_, OtyperSpec> {
        Ot0W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOE1 Output type"]
    #[inline(always)]
    pub fn ot1(&mut self) -> Ot1W<'_, OtyperSpec> {
        Ot1W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOE2 Output type"]
    #[inline(always)]
    pub fn ot2(&mut self) -> Ot2W<'_, OtyperSpec> {
        Ot2W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOE3 Output type"]
    #[inline(always)]
    pub fn ot3(&mut self) -> Ot3W<'_, OtyperSpec> {
        Ot3W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOE4 Output type"]
    #[inline(always)]
    pub fn ot4(&mut self) -> Ot4W<'_, OtyperSpec> {
        Ot4W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOE5 Output type"]
    #[inline(always)]
    pub fn ot5(&mut self) -> Ot5W<'_, OtyperSpec> {
        Ot5W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOE6 Output type"]
    #[inline(always)]
    pub fn ot6(&mut self) -> Ot6W<'_, OtyperSpec> {
        Ot6W::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOE7 Output type"]
    #[inline(always)]
    pub fn ot7(&mut self) -> Ot7W<'_, OtyperSpec> {
        Ot7W::new(self, 7)
    }
    #[doc = "Bit 8 - GPIOE8 Output type"]
    #[inline(always)]
    pub fn ot8(&mut self) -> Ot8W<'_, OtyperSpec> {
        Ot8W::new(self, 8)
    }
    #[doc = "Bit 9 - GPIOE9 Output type"]
    #[inline(always)]
    pub fn ot9(&mut self) -> Ot9W<'_, OtyperSpec> {
        Ot9W::new(self, 9)
    }
    #[doc = "Bit 10 - GPIOE10 Output type"]
    #[inline(always)]
    pub fn ot10(&mut self) -> Ot10W<'_, OtyperSpec> {
        Ot10W::new(self, 10)
    }
    #[doc = "Bit 11 - GPIOE11 Output type"]
    #[inline(always)]
    pub fn ot11(&mut self) -> Ot11W<'_, OtyperSpec> {
        Ot11W::new(self, 11)
    }
    #[doc = "Bit 12 - GPIOE12 Output type"]
    #[inline(always)]
    pub fn ot12(&mut self) -> Ot12W<'_, OtyperSpec> {
        Ot12W::new(self, 12)
    }
    #[doc = "Bit 13 - GPIOE13 Output type"]
    #[inline(always)]
    pub fn ot13(&mut self) -> Ot13W<'_, OtyperSpec> {
        Ot13W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIOE14 Output type"]
    #[inline(always)]
    pub fn ot14(&mut self) -> Ot14W<'_, OtyperSpec> {
        Ot14W::new(self, 14)
    }
    #[doc = "Bit 15 - GPIOE15 Output type"]
    #[inline(always)]
    pub fn ot15(&mut self) -> Ot15W<'_, OtyperSpec> {
        Ot15W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`otyper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtyperSpec;
impl crate::RegisterSpec for OtyperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otyper::R`](R) reader structure"]
impl crate::Readable for OtyperSpec {}
#[doc = "`write(|w| ..)` method takes [`otyper::W`](W) writer structure"]
impl crate::Writable for OtyperSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTYPER to value 0"]
impl crate::Resettable for OtyperSpec {}
