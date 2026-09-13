#[doc = "Register `BRR` reader"]
pub type R = crate::R<BrrSpec>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `BR0` reader - GPIOF 0 output pin reset"]
pub type Br0R = crate::BitReader;
#[doc = "Field `BR0` writer - GPIOF 0 output pin reset"]
pub type Br0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR1` reader - GPIOF 1 output pin reset"]
pub type Br1R = crate::BitReader;
#[doc = "Field `BR1` writer - GPIOF 1 output pin reset"]
pub type Br1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR2` reader - GPIOF 2 output pin reset"]
pub type Br2R = crate::BitReader;
#[doc = "Field `BR2` writer - GPIOF 2 output pin reset"]
pub type Br2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR3` reader - GPIOF 3 output pin reset"]
pub type Br3R = crate::BitReader;
#[doc = "Field `BR3` writer - GPIOF 3 output pin reset"]
pub type Br3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR4` reader - GPIOF 4 output pin reset"]
pub type Br4R = crate::BitReader;
#[doc = "Field `BR4` writer - GPIOF 4 output pin reset"]
pub type Br4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR5` reader - GPIOF 5 output pin reset"]
pub type Br5R = crate::BitReader;
#[doc = "Field `BR5` writer - GPIOF 5 output pin reset"]
pub type Br5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOF 0 output pin reset"]
    #[inline(always)]
    pub fn br0(&self) -> Br0R {
        Br0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOF 1 output pin reset"]
    #[inline(always)]
    pub fn br1(&self) -> Br1R {
        Br1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOF 2 output pin reset"]
    #[inline(always)]
    pub fn br2(&self) -> Br2R {
        Br2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOF 3 output pin reset"]
    #[inline(always)]
    pub fn br3(&self) -> Br3R {
        Br3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOF 4 output pin reset"]
    #[inline(always)]
    pub fn br4(&self) -> Br4R {
        Br4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOF 5 output pin reset"]
    #[inline(always)]
    pub fn br5(&self) -> Br5R {
        Br5R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOF 0 output pin reset"]
    #[inline(always)]
    pub fn br0(&mut self) -> Br0W<'_, BrrSpec> {
        Br0W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOF 1 output pin reset"]
    #[inline(always)]
    pub fn br1(&mut self) -> Br1W<'_, BrrSpec> {
        Br1W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOF 2 output pin reset"]
    #[inline(always)]
    pub fn br2(&mut self) -> Br2W<'_, BrrSpec> {
        Br2W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOF 3 output pin reset"]
    #[inline(always)]
    pub fn br3(&mut self) -> Br3W<'_, BrrSpec> {
        Br3W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOF 4 output pin reset"]
    #[inline(always)]
    pub fn br4(&mut self) -> Br4W<'_, BrrSpec> {
        Br4W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOF 5 output pin reset"]
    #[inline(always)]
    pub fn br5(&mut self) -> Br5W<'_, BrrSpec> {
        Br5W::new(self, 5)
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
