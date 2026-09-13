#[doc = "Register `BRR` reader"]
pub type R = crate::R<BrrSpec>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `BR0` reader - GPIOB 0 output pin reset"]
pub type Br0R = crate::BitReader;
#[doc = "Field `BR0` writer - GPIOB 0 output pin reset"]
pub type Br0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR1` reader - GPIOB 1 output pin reset"]
pub type Br1R = crate::BitReader;
#[doc = "Field `BR1` writer - GPIOB 1 output pin reset"]
pub type Br1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR2` reader - GPIOB 2 output pin reset"]
pub type Br2R = crate::BitReader;
#[doc = "Field `BR2` writer - GPIOB 2 output pin reset"]
pub type Br2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR3` reader - GPIOB 3 output pin reset"]
pub type Br3R = crate::BitReader;
#[doc = "Field `BR3` writer - GPIOB 3 output pin reset"]
pub type Br3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR4` reader - GPIOB 4 output pin reset"]
pub type Br4R = crate::BitReader;
#[doc = "Field `BR4` writer - GPIOB 4 output pin reset"]
pub type Br4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR5` reader - GPIOB 5 output pin reset"]
pub type Br5R = crate::BitReader;
#[doc = "Field `BR5` writer - GPIOB 5 output pin reset"]
pub type Br5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR6` reader - GPIOB 6 output pin reset"]
pub type Br6R = crate::BitReader;
#[doc = "Field `BR6` writer - GPIOB 6 output pin reset"]
pub type Br6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR7` reader - GPIOB 7 output pin reset"]
pub type Br7R = crate::BitReader;
#[doc = "Field `BR7` writer - GPIOB 7 output pin reset"]
pub type Br7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR8` reader - GPIOB 8 output pin reset"]
pub type Br8R = crate::BitReader;
#[doc = "Field `BR8` writer - GPIOB 8 output pin reset"]
pub type Br8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR9` reader - GPIOB 9 output pin reset"]
pub type Br9R = crate::BitReader;
#[doc = "Field `BR9` writer - GPIOB 9 output pin reset"]
pub type Br9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR10` reader - GPIOB 10 output pin reset"]
pub type Br10R = crate::BitReader;
#[doc = "Field `BR10` writer - GPIOB 10 output pin reset"]
pub type Br10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR11` reader - GPIOB 11 output pin reset"]
pub type Br11R = crate::BitReader;
#[doc = "Field `BR11` writer - GPIOB 11 output pin reset"]
pub type Br11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR12` reader - GPIOB 12 output pin reset"]
pub type Br12R = crate::BitReader;
#[doc = "Field `BR12` writer - GPIOB 12 output pin reset"]
pub type Br12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR13` reader - GPIOB 13 output pin reset"]
pub type Br13R = crate::BitReader;
#[doc = "Field `BR13` writer - GPIOB 13 output pin reset"]
pub type Br13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR14` reader - GPIOB 14 output pin reset"]
pub type Br14R = crate::BitReader;
#[doc = "Field `BR14` writer - GPIOB 14 output pin reset"]
pub type Br14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR15` reader - GPIOB 15 output pin reset"]
pub type Br15R = crate::BitReader;
#[doc = "Field `BR15` writer - GPIOB 15 output pin reset"]
pub type Br15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOB 0 output pin reset"]
    #[inline(always)]
    pub fn br0(&self) -> Br0R {
        Br0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOB 1 output pin reset"]
    #[inline(always)]
    pub fn br1(&self) -> Br1R {
        Br1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOB 2 output pin reset"]
    #[inline(always)]
    pub fn br2(&self) -> Br2R {
        Br2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOB 3 output pin reset"]
    #[inline(always)]
    pub fn br3(&self) -> Br3R {
        Br3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOB 4 output pin reset"]
    #[inline(always)]
    pub fn br4(&self) -> Br4R {
        Br4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOB 5 output pin reset"]
    #[inline(always)]
    pub fn br5(&self) -> Br5R {
        Br5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOB 6 output pin reset"]
    #[inline(always)]
    pub fn br6(&self) -> Br6R {
        Br6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOB 7 output pin reset"]
    #[inline(always)]
    pub fn br7(&self) -> Br7R {
        Br7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOB 8 output pin reset"]
    #[inline(always)]
    pub fn br8(&self) -> Br8R {
        Br8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIOB 9 output pin reset"]
    #[inline(always)]
    pub fn br9(&self) -> Br9R {
        Br9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOB 10 output pin reset"]
    #[inline(always)]
    pub fn br10(&self) -> Br10R {
        Br10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIOB 11 output pin reset"]
    #[inline(always)]
    pub fn br11(&self) -> Br11R {
        Br11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIOB 12 output pin reset"]
    #[inline(always)]
    pub fn br12(&self) -> Br12R {
        Br12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIOB 13 output pin reset"]
    #[inline(always)]
    pub fn br13(&self) -> Br13R {
        Br13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIOB 14 output pin reset"]
    #[inline(always)]
    pub fn br14(&self) -> Br14R {
        Br14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIOB 15 output pin reset"]
    #[inline(always)]
    pub fn br15(&self) -> Br15R {
        Br15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOB 0 output pin reset"]
    #[inline(always)]
    pub fn br0(&mut self) -> Br0W<'_, BrrSpec> {
        Br0W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOB 1 output pin reset"]
    #[inline(always)]
    pub fn br1(&mut self) -> Br1W<'_, BrrSpec> {
        Br1W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOB 2 output pin reset"]
    #[inline(always)]
    pub fn br2(&mut self) -> Br2W<'_, BrrSpec> {
        Br2W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOB 3 output pin reset"]
    #[inline(always)]
    pub fn br3(&mut self) -> Br3W<'_, BrrSpec> {
        Br3W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOB 4 output pin reset"]
    #[inline(always)]
    pub fn br4(&mut self) -> Br4W<'_, BrrSpec> {
        Br4W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOB 5 output pin reset"]
    #[inline(always)]
    pub fn br5(&mut self) -> Br5W<'_, BrrSpec> {
        Br5W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOB 6 output pin reset"]
    #[inline(always)]
    pub fn br6(&mut self) -> Br6W<'_, BrrSpec> {
        Br6W::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOB 7 output pin reset"]
    #[inline(always)]
    pub fn br7(&mut self) -> Br7W<'_, BrrSpec> {
        Br7W::new(self, 7)
    }
    #[doc = "Bit 8 - GPIOB 8 output pin reset"]
    #[inline(always)]
    pub fn br8(&mut self) -> Br8W<'_, BrrSpec> {
        Br8W::new(self, 8)
    }
    #[doc = "Bit 9 - GPIOB 9 output pin reset"]
    #[inline(always)]
    pub fn br9(&mut self) -> Br9W<'_, BrrSpec> {
        Br9W::new(self, 9)
    }
    #[doc = "Bit 10 - GPIOB 10 output pin reset"]
    #[inline(always)]
    pub fn br10(&mut self) -> Br10W<'_, BrrSpec> {
        Br10W::new(self, 10)
    }
    #[doc = "Bit 11 - GPIOB 11 output pin reset"]
    #[inline(always)]
    pub fn br11(&mut self) -> Br11W<'_, BrrSpec> {
        Br11W::new(self, 11)
    }
    #[doc = "Bit 12 - GPIOB 12 output pin reset"]
    #[inline(always)]
    pub fn br12(&mut self) -> Br12W<'_, BrrSpec> {
        Br12W::new(self, 12)
    }
    #[doc = "Bit 13 - GPIOB 13 output pin reset"]
    #[inline(always)]
    pub fn br13(&mut self) -> Br13W<'_, BrrSpec> {
        Br13W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIOB 14 output pin reset"]
    #[inline(always)]
    pub fn br14(&mut self) -> Br14W<'_, BrrSpec> {
        Br14W::new(self, 14)
    }
    #[doc = "Bit 15 - GPIOB 15 output pin reset"]
    #[inline(always)]
    pub fn br15(&mut self) -> Br15W<'_, BrrSpec> {
        Br15W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BrrSpec {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BrrSpec {}
