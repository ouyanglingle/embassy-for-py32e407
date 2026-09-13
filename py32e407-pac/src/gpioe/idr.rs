#[doc = "Register `IDR` reader"]
pub type R = crate::R<IdrSpec>;
#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `ID0` reader - GPIOE0 input data"]
pub type Id0R = crate::BitReader;
#[doc = "Field `ID1` reader - GPIOE1 input data"]
pub type Id1R = crate::BitReader;
#[doc = "Field `ID2` reader - GPIOE2 input data"]
pub type Id2R = crate::BitReader;
#[doc = "Field `ID3` reader - GPIOE3 input data"]
pub type Id3R = crate::BitReader;
#[doc = "Field `ID4` reader - GPIOE4 input data"]
pub type Id4R = crate::BitReader;
#[doc = "Field `ID5` reader - GPIOE5 input data"]
pub type Id5R = crate::BitReader;
#[doc = "Field `ID6` reader - GPIOE6 input data"]
pub type Id6R = crate::BitReader;
#[doc = "Field `ID7` reader - GPIOE7 input data"]
pub type Id7R = crate::BitReader;
#[doc = "Field `ID8` reader - GPIOE8 input data"]
pub type Id8R = crate::BitReader;
#[doc = "Field `ID9` reader - GPIOE9 input data"]
pub type Id9R = crate::BitReader;
#[doc = "Field `ID10` reader - GPIOE10 input data"]
pub type Id10R = crate::BitReader;
#[doc = "Field `ID11` reader - GPIOE11 input data"]
pub type Id11R = crate::BitReader;
#[doc = "Field `ID12` reader - GPIOE12 input data"]
pub type Id12R = crate::BitReader;
#[doc = "Field `ID13` reader - GPIOE13 input data"]
pub type Id13R = crate::BitReader;
#[doc = "Field `ID14` reader - GPIOE14 input data"]
pub type Id14R = crate::BitReader;
#[doc = "Field `ID15` reader - GPIOE15 input data"]
pub type Id15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - GPIOE0 input data"]
    #[inline(always)]
    pub fn id0(&self) -> Id0R {
        Id0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOE1 input data"]
    #[inline(always)]
    pub fn id1(&self) -> Id1R {
        Id1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOE2 input data"]
    #[inline(always)]
    pub fn id2(&self) -> Id2R {
        Id2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOE3 input data"]
    #[inline(always)]
    pub fn id3(&self) -> Id3R {
        Id3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOE4 input data"]
    #[inline(always)]
    pub fn id4(&self) -> Id4R {
        Id4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOE5 input data"]
    #[inline(always)]
    pub fn id5(&self) -> Id5R {
        Id5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOE6 input data"]
    #[inline(always)]
    pub fn id6(&self) -> Id6R {
        Id6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOE7 input data"]
    #[inline(always)]
    pub fn id7(&self) -> Id7R {
        Id7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOE8 input data"]
    #[inline(always)]
    pub fn id8(&self) -> Id8R {
        Id8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIOE9 input data"]
    #[inline(always)]
    pub fn id9(&self) -> Id9R {
        Id9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOE10 input data"]
    #[inline(always)]
    pub fn id10(&self) -> Id10R {
        Id10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIOE11 input data"]
    #[inline(always)]
    pub fn id11(&self) -> Id11R {
        Id11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIOE12 input data"]
    #[inline(always)]
    pub fn id12(&self) -> Id12R {
        Id12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIOE13 input data"]
    #[inline(always)]
    pub fn id13(&self) -> Id13R {
        Id13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIOE14 input data"]
    #[inline(always)]
    pub fn id14(&self) -> Id14R {
        Id14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIOE15 input data"]
    #[inline(always)]
    pub fn id15(&self) -> Id15R {
        Id15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IdrSpec {}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {}
