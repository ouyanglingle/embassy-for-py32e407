#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `DMAUDR1` reader - "]
pub type Dmaudr1R = crate::BitReader;
#[doc = "Field `DMAUDR1` writer - "]
pub type Dmaudr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAUDR2` reader - "]
pub type Dmaudr2R = crate::BitReader;
#[doc = "Field `DMAUDR2` writer - "]
pub type Dmaudr2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dmaudr1(&self) -> Dmaudr1R {
        Dmaudr1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dmaudr2(&self) -> Dmaudr2R {
        Dmaudr2R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dmaudr1(&mut self) -> Dmaudr1W<'_, SrSpec> {
        Dmaudr1W::new(self, 13)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dmaudr2(&mut self) -> Dmaudr2W<'_, SrSpec> {
        Dmaudr2W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
