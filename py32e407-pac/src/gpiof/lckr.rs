#[doc = "Register `LCKR` reader"]
pub type R = crate::R<LckrSpec>;
#[doc = "Register `LCKR` writer"]
pub type W = crate::W<LckrSpec>;
#[doc = "Field `LCK0` reader - GPIOF lock(bit0)"]
pub type Lck0R = crate::BitReader;
#[doc = "Field `LCK0` writer - GPIOF lock(bit0)"]
pub type Lck0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK1` reader - GPIOF lock(bit1)"]
pub type Lck1R = crate::BitReader;
#[doc = "Field `LCK1` writer - GPIOF lock(bit1)"]
pub type Lck1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK2` reader - GPIOF lock(bit2)"]
pub type Lck2R = crate::BitReader;
#[doc = "Field `LCK2` writer - GPIOF lock(bit2)"]
pub type Lck2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK3` reader - GPIOF lock(bit3)"]
pub type Lck3R = crate::BitReader;
#[doc = "Field `LCK3` writer - GPIOF lock(bit3)"]
pub type Lck3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK4` reader - GPIOF lock(bit4)"]
pub type Lck4R = crate::BitReader;
#[doc = "Field `LCK4` writer - GPIOF lock(bit4)"]
pub type Lck4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK5` reader - GPIOF lock(bit5)"]
pub type Lck5R = crate::BitReader;
#[doc = "Field `LCK5` writer - GPIOF lock(bit5)"]
pub type Lck5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCKK` reader - GPIOF lock key"]
pub type LckkR = crate::BitReader;
#[doc = "Field `LCKK` writer - GPIOF lock key"]
pub type LckkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOF lock(bit0)"]
    #[inline(always)]
    pub fn lck0(&self) -> Lck0R {
        Lck0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOF lock(bit1)"]
    #[inline(always)]
    pub fn lck1(&self) -> Lck1R {
        Lck1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOF lock(bit2)"]
    #[inline(always)]
    pub fn lck2(&self) -> Lck2R {
        Lck2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOF lock(bit3)"]
    #[inline(always)]
    pub fn lck3(&self) -> Lck3R {
        Lck3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOF lock(bit4)"]
    #[inline(always)]
    pub fn lck4(&self) -> Lck4R {
        Lck4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOF lock(bit5)"]
    #[inline(always)]
    pub fn lck5(&self) -> Lck5R {
        Lck5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIOF lock key"]
    #[inline(always)]
    pub fn lckk(&self) -> LckkR {
        LckkR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOF lock(bit0)"]
    #[inline(always)]
    pub fn lck0(&mut self) -> Lck0W<'_, LckrSpec> {
        Lck0W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOF lock(bit1)"]
    #[inline(always)]
    pub fn lck1(&mut self) -> Lck1W<'_, LckrSpec> {
        Lck1W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOF lock(bit2)"]
    #[inline(always)]
    pub fn lck2(&mut self) -> Lck2W<'_, LckrSpec> {
        Lck2W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOF lock(bit3)"]
    #[inline(always)]
    pub fn lck3(&mut self) -> Lck3W<'_, LckrSpec> {
        Lck3W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOF lock(bit4)"]
    #[inline(always)]
    pub fn lck4(&mut self) -> Lck4W<'_, LckrSpec> {
        Lck4W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOF lock(bit5)"]
    #[inline(always)]
    pub fn lck5(&mut self) -> Lck5W<'_, LckrSpec> {
        Lck5W::new(self, 5)
    }
    #[doc = "Bit 16 - GPIOF lock key"]
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
