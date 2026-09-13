#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `EN0` reader - LCD0 enable"]
pub type En0R = crate::BitReader;
#[doc = "Field `EN0` writer - LCD0 enable"]
pub type En0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN1` reader - LCD1 enable"]
pub type En1R = crate::BitReader;
#[doc = "Field `EN1` writer - LCD1 enable"]
pub type En1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN2` reader - LCD2 enable"]
pub type En2R = crate::BitReader;
#[doc = "Field `EN2` writer - LCD2 enable"]
pub type En2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN3` reader - LCD3 enable"]
pub type En3R = crate::BitReader;
#[doc = "Field `EN3` writer - LCD3 enable"]
pub type En3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE0` reader - LCD0 mode"]
pub type Mode0R = crate::BitReader;
#[doc = "Field `MODE0` writer - LCD0 mode"]
pub type Mode0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE1` reader - LCD1 mode"]
pub type Mode1R = crate::BitReader;
#[doc = "Field `MODE1` writer - LCD1 mode"]
pub type Mode1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE2` reader - LCD2 mode"]
pub type Mode2R = crate::BitReader;
#[doc = "Field `MODE2` writer - LCD2 mode"]
pub type Mode2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE3` reader - LCD3 mode"]
pub type Mode3R = crate::BitReader;
#[doc = "Field `MODE3` writer - LCD3 mode"]
pub type Mode3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCD0 enable"]
    #[inline(always)]
    pub fn en0(&self) -> En0R {
        En0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD1 enable"]
    #[inline(always)]
    pub fn en1(&self) -> En1R {
        En1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD2 enable"]
    #[inline(always)]
    pub fn en2(&self) -> En2R {
        En2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD3 enable"]
    #[inline(always)]
    pub fn en3(&self) -> En3R {
        En3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD0 mode"]
    #[inline(always)]
    pub fn mode0(&self) -> Mode0R {
        Mode0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD1 mode"]
    #[inline(always)]
    pub fn mode1(&self) -> Mode1R {
        Mode1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD2 mode"]
    #[inline(always)]
    pub fn mode2(&self) -> Mode2R {
        Mode2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD3 mode"]
    #[inline(always)]
    pub fn mode3(&self) -> Mode3R {
        Mode3R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD0 enable"]
    #[inline(always)]
    pub fn en0(&mut self) -> En0W<'_, CrSpec> {
        En0W::new(self, 0)
    }
    #[doc = "Bit 1 - LCD1 enable"]
    #[inline(always)]
    pub fn en1(&mut self) -> En1W<'_, CrSpec> {
        En1W::new(self, 1)
    }
    #[doc = "Bit 2 - LCD2 enable"]
    #[inline(always)]
    pub fn en2(&mut self) -> En2W<'_, CrSpec> {
        En2W::new(self, 2)
    }
    #[doc = "Bit 3 - LCD3 enable"]
    #[inline(always)]
    pub fn en3(&mut self) -> En3W<'_, CrSpec> {
        En3W::new(self, 3)
    }
    #[doc = "Bit 4 - LCD0 mode"]
    #[inline(always)]
    pub fn mode0(&mut self) -> Mode0W<'_, CrSpec> {
        Mode0W::new(self, 4)
    }
    #[doc = "Bit 5 - LCD1 mode"]
    #[inline(always)]
    pub fn mode1(&mut self) -> Mode1W<'_, CrSpec> {
        Mode1W::new(self, 5)
    }
    #[doc = "Bit 6 - LCD2 mode"]
    #[inline(always)]
    pub fn mode2(&mut self) -> Mode2W<'_, CrSpec> {
        Mode2W::new(self, 6)
    }
    #[doc = "Bit 7 - LCD3 mode"]
    #[inline(always)]
    pub fn mode3(&mut self) -> Mode3W<'_, CrSpec> {
        Mode3W::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
