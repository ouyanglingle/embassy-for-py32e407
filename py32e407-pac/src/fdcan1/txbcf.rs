#[doc = "Register `TXBCF` reader"]
pub type R = crate::R<TxbcfSpec>;
#[doc = "Register `TXBCF` writer"]
pub type W = crate::W<TxbcfSpec>;
#[doc = "Field `CF0` reader - Cancellation Finished"]
pub type Cf0R = crate::BitReader;
#[doc = "Field `CF1` reader - Cancellation Finished"]
pub type Cf1R = crate::BitReader;
#[doc = "Field `CF2` reader - Cancellation Finished"]
pub type Cf2R = crate::BitReader;
#[doc = "Field `CF3` reader - Cancellation Finished"]
pub type Cf3R = crate::BitReader;
#[doc = "Field `CF4` reader - Cancellation Finished"]
pub type Cf4R = crate::BitReader;
#[doc = "Field `CF5` reader - Cancellation Finished"]
pub type Cf5R = crate::BitReader;
#[doc = "Field `CF6` reader - Cancellation Finished"]
pub type Cf6R = crate::BitReader;
#[doc = "Field `CF7` reader - Cancellation Finished"]
pub type Cf7R = crate::BitReader;
#[doc = "Field `CF8` reader - Cancellation Finished"]
pub type Cf8R = crate::BitReader;
#[doc = "Field `CF9` reader - Cancellation Finished"]
pub type Cf9R = crate::BitReader;
#[doc = "Field `CF10` reader - Cancellation Finished"]
pub type Cf10R = crate::BitReader;
#[doc = "Field `CF11` reader - Cancellation Finished"]
pub type Cf11R = crate::BitReader;
#[doc = "Field `CF12` reader - Cancellation Finished"]
pub type Cf12R = crate::BitReader;
#[doc = "Field `CF13` reader - Cancellation Finished"]
pub type Cf13R = crate::BitReader;
#[doc = "Field `CF14` reader - Cancellation Finished"]
pub type Cf14R = crate::BitReader;
#[doc = "Field `CF15` reader - Cancellation Finished"]
pub type Cf15R = crate::BitReader;
#[doc = "Field `CF16` reader - Cancellation Finished"]
pub type Cf16R = crate::BitReader;
#[doc = "Field `CF17` reader - Cancellation Finished"]
pub type Cf17R = crate::BitReader;
#[doc = "Field `CF18` reader - Cancellation Finished"]
pub type Cf18R = crate::BitReader;
#[doc = "Field `CF19` reader - Cancellation Finished"]
pub type Cf19R = crate::BitReader;
#[doc = "Field `CF20` reader - Cancellation Finished"]
pub type Cf20R = crate::BitReader;
#[doc = "Field `CF21` reader - Cancellation Finished"]
pub type Cf21R = crate::BitReader;
#[doc = "Field `CF22` reader - Cancellation Finished"]
pub type Cf22R = crate::BitReader;
#[doc = "Field `CF23` reader - Cancellation Finished"]
pub type Cf23R = crate::BitReader;
#[doc = "Field `CF24` reader - Cancellation Finished"]
pub type Cf24R = crate::BitReader;
#[doc = "Field `CF25` reader - Cancellation Finished"]
pub type Cf25R = crate::BitReader;
#[doc = "Field `CF26` reader - Cancellation Finished"]
pub type Cf26R = crate::BitReader;
#[doc = "Field `CF27` reader - Cancellation Finished"]
pub type Cf27R = crate::BitReader;
#[doc = "Field `CF28` reader - Cancellation Finished"]
pub type Cf28R = crate::BitReader;
#[doc = "Field `CF29` reader - Cancellation Finished"]
pub type Cf29R = crate::BitReader;
#[doc = "Field `CF30` reader - Cancellation Finished"]
pub type Cf30R = crate::BitReader;
#[doc = "Field `CF31` reader - Cancellation Finished"]
pub type Cf31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf0(&self) -> Cf0R {
        Cf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf1(&self) -> Cf1R {
        Cf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf2(&self) -> Cf2R {
        Cf2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf3(&self) -> Cf3R {
        Cf3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf4(&self) -> Cf4R {
        Cf4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf5(&self) -> Cf5R {
        Cf5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf6(&self) -> Cf6R {
        Cf6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf7(&self) -> Cf7R {
        Cf7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf8(&self) -> Cf8R {
        Cf8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf9(&self) -> Cf9R {
        Cf9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf10(&self) -> Cf10R {
        Cf10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf11(&self) -> Cf11R {
        Cf11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf12(&self) -> Cf12R {
        Cf12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf13(&self) -> Cf13R {
        Cf13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf14(&self) -> Cf14R {
        Cf14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf15(&self) -> Cf15R {
        Cf15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf16(&self) -> Cf16R {
        Cf16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf17(&self) -> Cf17R {
        Cf17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf18(&self) -> Cf18R {
        Cf18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf19(&self) -> Cf19R {
        Cf19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf20(&self) -> Cf20R {
        Cf20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf21(&self) -> Cf21R {
        Cf21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf22(&self) -> Cf22R {
        Cf22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf23(&self) -> Cf23R {
        Cf23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf24(&self) -> Cf24R {
        Cf24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf25(&self) -> Cf25R {
        Cf25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf26(&self) -> Cf26R {
        Cf26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf27(&self) -> Cf27R {
        Cf27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf28(&self) -> Cf28R {
        Cf28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf29(&self) -> Cf29R {
        Cf29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf30(&self) -> Cf30R {
        Cf30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf31(&self) -> Cf31R {
        Cf31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcfSpec;
impl crate::RegisterSpec for TxbcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcf::R`](R) reader structure"]
impl crate::Readable for TxbcfSpec {}
#[doc = "`write(|w| ..)` method takes [`txbcf::W`](W) writer structure"]
impl crate::Writable for TxbcfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXBCF to value 0"]
impl crate::Resettable for TxbcfSpec {}
