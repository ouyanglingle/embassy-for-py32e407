#[doc = "Register `LCKR` reader"]
pub type R = crate::R<LckrSpec>;
#[doc = "Register `LCKR` writer"]
pub type W = crate::W<LckrSpec>;
#[doc = "Field `LCK0` reader - GPIOE lock(bit0)"]
pub type Lck0R = crate::BitReader;
#[doc = "Field `LCK0` writer - GPIOE lock(bit0)"]
pub type Lck0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK1` reader - GPIOE lock(bit1)"]
pub type Lck1R = crate::BitReader;
#[doc = "Field `LCK1` writer - GPIOE lock(bit1)"]
pub type Lck1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK2` reader - GPIOE lock(bit2)"]
pub type Lck2R = crate::BitReader;
#[doc = "Field `LCK2` writer - GPIOE lock(bit2)"]
pub type Lck2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK3` reader - GPIOE lock(bit3)"]
pub type Lck3R = crate::BitReader;
#[doc = "Field `LCK3` writer - GPIOE lock(bit3)"]
pub type Lck3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK4` reader - GPIOE lock(bit4)"]
pub type Lck4R = crate::BitReader;
#[doc = "Field `LCK4` writer - GPIOE lock(bit4)"]
pub type Lck4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK5` reader - GPIOE lock(bit5)"]
pub type Lck5R = crate::BitReader;
#[doc = "Field `LCK5` writer - GPIOE lock(bit5)"]
pub type Lck5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK6` reader - GPIOE lock(bit6)"]
pub type Lck6R = crate::BitReader;
#[doc = "Field `LCK6` writer - GPIOE lock(bit6)"]
pub type Lck6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK7` reader - GPIOE lock(bit7)"]
pub type Lck7R = crate::BitReader;
#[doc = "Field `LCK7` writer - GPIOE lock(bit7)"]
pub type Lck7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK8` reader - GPIOE lock(bit8)"]
pub type Lck8R = crate::BitReader;
#[doc = "Field `LCK8` writer - GPIOE lock(bit8)"]
pub type Lck8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK9` reader - GPIOE lock(bit9)"]
pub type Lck9R = crate::BitReader;
#[doc = "Field `LCK9` writer - GPIOE lock(bit9)"]
pub type Lck9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK10` reader - GPIOE lock(bit10)"]
pub type Lck10R = crate::BitReader;
#[doc = "Field `LCK10` writer - GPIOE lock(bit10)"]
pub type Lck10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK11` reader - GPIOE lock(bit11)"]
pub type Lck11R = crate::BitReader;
#[doc = "Field `LCK11` writer - GPIOE lock(bit11)"]
pub type Lck11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK12` reader - GPIOE lock(bit12)"]
pub type Lck12R = crate::BitReader;
#[doc = "Field `LCK12` writer - GPIOE lock(bit12)"]
pub type Lck12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK13` reader - GPIOE lock(bit13)"]
pub type Lck13R = crate::BitReader;
#[doc = "Field `LCK13` writer - GPIOE lock(bit13)"]
pub type Lck13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK14` reader - GPIOE lock(bit14)"]
pub type Lck14R = crate::BitReader;
#[doc = "Field `LCK14` writer - GPIOE lock(bit14)"]
pub type Lck14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK15` reader - GPIOE lock(bit15)"]
pub type Lck15R = crate::BitReader;
#[doc = "Field `LCK15` writer - GPIOE lock(bit15)"]
pub type Lck15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCKK` reader - GPIOE lock key"]
pub type LckkR = crate::BitReader;
#[doc = "Field `LCKK` writer - GPIOE lock key"]
pub type LckkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOE lock(bit0)"]
    #[inline(always)]
    pub fn lck0(&self) -> Lck0R {
        Lck0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOE lock(bit1)"]
    #[inline(always)]
    pub fn lck1(&self) -> Lck1R {
        Lck1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOE lock(bit2)"]
    #[inline(always)]
    pub fn lck2(&self) -> Lck2R {
        Lck2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOE lock(bit3)"]
    #[inline(always)]
    pub fn lck3(&self) -> Lck3R {
        Lck3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOE lock(bit4)"]
    #[inline(always)]
    pub fn lck4(&self) -> Lck4R {
        Lck4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOE lock(bit5)"]
    #[inline(always)]
    pub fn lck5(&self) -> Lck5R {
        Lck5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOE lock(bit6)"]
    #[inline(always)]
    pub fn lck6(&self) -> Lck6R {
        Lck6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOE lock(bit7)"]
    #[inline(always)]
    pub fn lck7(&self) -> Lck7R {
        Lck7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOE lock(bit8)"]
    #[inline(always)]
    pub fn lck8(&self) -> Lck8R {
        Lck8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIOE lock(bit9)"]
    #[inline(always)]
    pub fn lck9(&self) -> Lck9R {
        Lck9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOE lock(bit10)"]
    #[inline(always)]
    pub fn lck10(&self) -> Lck10R {
        Lck10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIOE lock(bit11)"]
    #[inline(always)]
    pub fn lck11(&self) -> Lck11R {
        Lck11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIOE lock(bit12)"]
    #[inline(always)]
    pub fn lck12(&self) -> Lck12R {
        Lck12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIOE lock(bit13)"]
    #[inline(always)]
    pub fn lck13(&self) -> Lck13R {
        Lck13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIOE lock(bit14)"]
    #[inline(always)]
    pub fn lck14(&self) -> Lck14R {
        Lck14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIOE lock(bit15)"]
    #[inline(always)]
    pub fn lck15(&self) -> Lck15R {
        Lck15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIOE lock key"]
    #[inline(always)]
    pub fn lckk(&self) -> LckkR {
        LckkR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOE lock(bit0)"]
    #[inline(always)]
    pub fn lck0(&mut self) -> Lck0W<'_, LckrSpec> {
        Lck0W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOE lock(bit1)"]
    #[inline(always)]
    pub fn lck1(&mut self) -> Lck1W<'_, LckrSpec> {
        Lck1W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOE lock(bit2)"]
    #[inline(always)]
    pub fn lck2(&mut self) -> Lck2W<'_, LckrSpec> {
        Lck2W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOE lock(bit3)"]
    #[inline(always)]
    pub fn lck3(&mut self) -> Lck3W<'_, LckrSpec> {
        Lck3W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOE lock(bit4)"]
    #[inline(always)]
    pub fn lck4(&mut self) -> Lck4W<'_, LckrSpec> {
        Lck4W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOE lock(bit5)"]
    #[inline(always)]
    pub fn lck5(&mut self) -> Lck5W<'_, LckrSpec> {
        Lck5W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOE lock(bit6)"]
    #[inline(always)]
    pub fn lck6(&mut self) -> Lck6W<'_, LckrSpec> {
        Lck6W::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOE lock(bit7)"]
    #[inline(always)]
    pub fn lck7(&mut self) -> Lck7W<'_, LckrSpec> {
        Lck7W::new(self, 7)
    }
    #[doc = "Bit 8 - GPIOE lock(bit8)"]
    #[inline(always)]
    pub fn lck8(&mut self) -> Lck8W<'_, LckrSpec> {
        Lck8W::new(self, 8)
    }
    #[doc = "Bit 9 - GPIOE lock(bit9)"]
    #[inline(always)]
    pub fn lck9(&mut self) -> Lck9W<'_, LckrSpec> {
        Lck9W::new(self, 9)
    }
    #[doc = "Bit 10 - GPIOE lock(bit10)"]
    #[inline(always)]
    pub fn lck10(&mut self) -> Lck10W<'_, LckrSpec> {
        Lck10W::new(self, 10)
    }
    #[doc = "Bit 11 - GPIOE lock(bit11)"]
    #[inline(always)]
    pub fn lck11(&mut self) -> Lck11W<'_, LckrSpec> {
        Lck11W::new(self, 11)
    }
    #[doc = "Bit 12 - GPIOE lock(bit12)"]
    #[inline(always)]
    pub fn lck12(&mut self) -> Lck12W<'_, LckrSpec> {
        Lck12W::new(self, 12)
    }
    #[doc = "Bit 13 - GPIOE lock(bit13)"]
    #[inline(always)]
    pub fn lck13(&mut self) -> Lck13W<'_, LckrSpec> {
        Lck13W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIOE lock(bit14)"]
    #[inline(always)]
    pub fn lck14(&mut self) -> Lck14W<'_, LckrSpec> {
        Lck14W::new(self, 14)
    }
    #[doc = "Bit 15 - GPIOE lock(bit15)"]
    #[inline(always)]
    pub fn lck15(&mut self) -> Lck15W<'_, LckrSpec> {
        Lck15W::new(self, 15)
    }
    #[doc = "Bit 16 - GPIOE lock key"]
    #[inline(always)]
    pub fn lckk(&mut self) -> LckkW<'_, LckrSpec> {
        LckkW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LckrSpec;
impl crate::RegisterSpec for LckrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lckr::R`](R) reader structure"]
impl crate::Readable for LckrSpec {}
#[doc = "`write(|w| ..)` method takes [`lckr::W`](W) writer structure"]
impl crate::Writable for LckrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCKR to value 0"]
impl crate::Resettable for LckrSpec {}
