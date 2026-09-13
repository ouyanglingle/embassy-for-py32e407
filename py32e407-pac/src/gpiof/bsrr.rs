#[doc = "Register `BSRR` reader"]
pub type R = crate::R<BsrrSpec>;
#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BsrrSpec>;
#[doc = "Field `BS0` reader - GPIOF0 output set"]
pub type Bs0R = crate::BitReader;
#[doc = "Field `BS0` writer - GPIOF0 output set"]
pub type Bs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS1` reader - GPIOF1 output set"]
pub type Bs1R = crate::BitReader;
#[doc = "Field `BS1` writer - GPIOF1 output set"]
pub type Bs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS2` reader - GPIOF2 output set"]
pub type Bs2R = crate::BitReader;
#[doc = "Field `BS2` writer - GPIOF2 output set"]
pub type Bs2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS3` reader - GPIOF3 output set"]
pub type Bs3R = crate::BitReader;
#[doc = "Field `BS3` writer - GPIOF3 output set"]
pub type Bs3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS4` reader - GPIOF4 output set"]
pub type Bs4R = crate::BitReader;
#[doc = "Field `BS4` writer - GPIOF4 output set"]
pub type Bs4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS5` reader - GPIOF5 output set"]
pub type Bs5R = crate::BitReader;
#[doc = "Field `BS5` writer - GPIOF5 output set"]
pub type Bs5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR0` reader - GPIOF0 outut reset"]
pub type Br0R = crate::BitReader;
#[doc = "Field `BR0` writer - GPIOF0 outut reset"]
pub type Br0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR1` reader - GPIOF1 outut reset"]
pub type Br1R = crate::BitReader;
#[doc = "Field `BR1` writer - GPIOF1 outut reset"]
pub type Br1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR2` reader - GPIOF2 outut reset"]
pub type Br2R = crate::BitReader;
#[doc = "Field `BR2` writer - GPIOF2 outut reset"]
pub type Br2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR3` reader - GPIOF3 outut reset"]
pub type Br3R = crate::BitReader;
#[doc = "Field `BR3` writer - GPIOF3 outut reset"]
pub type Br3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR4` reader - GPIOF4 outut reset"]
pub type Br4R = crate::BitReader;
#[doc = "Field `BR4` writer - GPIOF4 outut reset"]
pub type Br4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR5` reader - GPIOF5 outut reset"]
pub type Br5R = crate::BitReader;
#[doc = "Field `BR5` writer - GPIOF5 outut reset"]
pub type Br5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - GPIOF0 output set"]
    #[inline(always)]
    pub fn bs0(&self) -> Bs0R {
        Bs0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOF1 output set"]
    #[inline(always)]
    pub fn bs1(&self) -> Bs1R {
        Bs1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOF2 output set"]
    #[inline(always)]
    pub fn bs2(&self) -> Bs2R {
        Bs2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOF3 output set"]
    #[inline(always)]
    pub fn bs3(&self) -> Bs3R {
        Bs3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOF4 output set"]
    #[inline(always)]
    pub fn bs4(&self) -> Bs4R {
        Bs4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOF5 output set"]
    #[inline(always)]
    pub fn bs5(&self) -> Bs5R {
        Bs5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIOF0 outut reset"]
    #[inline(always)]
    pub fn br0(&self) -> Br0R {
        Br0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIOF1 outut reset"]
    #[inline(always)]
    pub fn br1(&self) -> Br1R {
        Br1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIOF2 outut reset"]
    #[inline(always)]
    pub fn br2(&self) -> Br2R {
        Br2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIOF3 outut reset"]
    #[inline(always)]
    pub fn br3(&self) -> Br3R {
        Br3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIOF4 outut reset"]
    #[inline(always)]
    pub fn br4(&self) -> Br4R {
        Br4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIOF5 outut reset"]
    #[inline(always)]
    pub fn br5(&self) -> Br5R {
        Br5R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - GPIOF0 output set"]
    #[inline(always)]
    pub fn bs0(&mut self) -> Bs0W<'_, BsrrSpec> {
        Bs0W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOF1 output set"]
    #[inline(always)]
    pub fn bs1(&mut self) -> Bs1W<'_, BsrrSpec> {
        Bs1W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOF2 output set"]
    #[inline(always)]
    pub fn bs2(&mut self) -> Bs2W<'_, BsrrSpec> {
        Bs2W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOF3 output set"]
    #[inline(always)]
    pub fn bs3(&mut self) -> Bs3W<'_, BsrrSpec> {
        Bs3W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOF4 output set"]
    #[inline(always)]
    pub fn bs4(&mut self) -> Bs4W<'_, BsrrSpec> {
        Bs4W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOF5 output set"]
    #[inline(always)]
    pub fn bs5(&mut self) -> Bs5W<'_, BsrrSpec> {
        Bs5W::new(self, 6)
    }
    #[doc = "Bit 17 - GPIOF0 outut reset"]
    #[inline(always)]
    pub fn br0(&mut self) -> Br0W<'_, BsrrSpec> {
        Br0W::new(self, 17)
    }
    #[doc = "Bit 18 - GPIOF1 outut reset"]
    #[inline(always)]
    pub fn br1(&mut self) -> Br1W<'_, BsrrSpec> {
        Br1W::new(self, 18)
    }
    #[doc = "Bit 19 - GPIOF2 outut reset"]
    #[inline(always)]
    pub fn br2(&mut self) -> Br2W<'_, BsrrSpec> {
        Br2W::new(self, 19)
    }
    #[doc = "Bit 20 - GPIOF3 outut reset"]
    #[inline(always)]
    pub fn br3(&mut self) -> Br3W<'_, BsrrSpec> {
        Br3W::new(self, 20)
    }
    #[doc = "Bit 21 - GPIOF4 outut reset"]
    #[inline(always)]
    pub fn br4(&mut self) -> Br4W<'_, BsrrSpec> {
        Br4W::new(self, 21)
    }
    #[doc = "Bit 22 - GPIOF5 outut reset"]
    #[inline(always)]
    pub fn br5(&mut self) -> Br5W<'_, BsrrSpec> {
        Br5W::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`bsrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsrrSpec;
impl crate::RegisterSpec for BsrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsrr::R`](R) reader structure"]
impl crate::Readable for BsrrSpec {}
#[doc = "`write(|w| ..)` method takes [`bsrr::W`](W) writer structure"]
impl crate::Writable for BsrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BsrrSpec {}
