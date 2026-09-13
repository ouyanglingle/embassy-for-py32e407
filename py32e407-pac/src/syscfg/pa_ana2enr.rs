#[doc = "Register `PA_ANA2ENR` reader"]
pub type R = crate::R<PaAna2enrSpec>;
#[doc = "Register `PA_ANA2ENR` writer"]
pub type W = crate::W<PaAna2enrSpec>;
#[doc = "Field `PA0_ENA2` reader - GPIOA0 ANA2 enable"]
pub type Pa0Ena2R = crate::BitReader;
#[doc = "Field `PA0_ENA2` writer - GPIOA0 ANA2 enable"]
pub type Pa0Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA1_ENA2` reader - GPIOA1 ANA2 enable"]
pub type Pa1Ena2R = crate::BitReader;
#[doc = "Field `PA1_ENA2` writer - GPIOA1 ANA2 enable"]
pub type Pa1Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA2_ENA2` reader - GPIOA2 ANA2 enable"]
pub type Pa2Ena2R = crate::BitReader;
#[doc = "Field `PA2_ENA2` writer - GPIOA2 ANA2 enable"]
pub type Pa2Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA3_ENA2` reader - GPIOA3 ANA2 enable"]
pub type Pa3Ena2R = crate::BitReader;
#[doc = "Field `PA3_ENA2` writer - GPIOA3 ANA2 enable"]
pub type Pa3Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA4_ENA2` reader - GPIOA4 ANA2 enable"]
pub type Pa4Ena2R = crate::BitReader;
#[doc = "Field `PA4_ENA2` writer - GPIOA4 ANA2 enable"]
pub type Pa4Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA5_ENA2` reader - GPIOA5 ANA2 enable"]
pub type Pa5Ena2R = crate::BitReader;
#[doc = "Field `PA5_ENA2` writer - GPIOA5 ANA2 enable"]
pub type Pa5Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA7_ENA2` reader - GPIOA7 ANA2 enable"]
pub type Pa7Ena2R = crate::BitReader;
#[doc = "Field `PA7_ENA2` writer - GPIOA7 ANA2 enable"]
pub type Pa7Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA10_ENA2` reader - GPIOA10 ANA2 enable"]
pub type Pa10Ena2R = crate::BitReader;
#[doc = "Field `PA10_ENA2` writer - GPIOA10 ANA2 enable"]
pub type Pa10Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOA0 ANA2 enable"]
    #[inline(always)]
    pub fn pa0_ena2(&self) -> Pa0Ena2R {
        Pa0Ena2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOA1 ANA2 enable"]
    #[inline(always)]
    pub fn pa1_ena2(&self) -> Pa1Ena2R {
        Pa1Ena2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOA2 ANA2 enable"]
    #[inline(always)]
    pub fn pa2_ena2(&self) -> Pa2Ena2R {
        Pa2Ena2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOA3 ANA2 enable"]
    #[inline(always)]
    pub fn pa3_ena2(&self) -> Pa3Ena2R {
        Pa3Ena2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOA4 ANA2 enable"]
    #[inline(always)]
    pub fn pa4_ena2(&self) -> Pa4Ena2R {
        Pa4Ena2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOA5 ANA2 enable"]
    #[inline(always)]
    pub fn pa5_ena2(&self) -> Pa5Ena2R {
        Pa5Ena2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOA7 ANA2 enable"]
    #[inline(always)]
    pub fn pa7_ena2(&self) -> Pa7Ena2R {
        Pa7Ena2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOA10 ANA2 enable"]
    #[inline(always)]
    pub fn pa10_ena2(&self) -> Pa10Ena2R {
        Pa10Ena2R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOA0 ANA2 enable"]
    #[inline(always)]
    pub fn pa0_ena2(&mut self) -> Pa0Ena2W<'_, PaAna2enrSpec> {
        Pa0Ena2W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOA1 ANA2 enable"]
    #[inline(always)]
    pub fn pa1_ena2(&mut self) -> Pa1Ena2W<'_, PaAna2enrSpec> {
        Pa1Ena2W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOA2 ANA2 enable"]
    #[inline(always)]
    pub fn pa2_ena2(&mut self) -> Pa2Ena2W<'_, PaAna2enrSpec> {
        Pa2Ena2W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOA3 ANA2 enable"]
    #[inline(always)]
    pub fn pa3_ena2(&mut self) -> Pa3Ena2W<'_, PaAna2enrSpec> {
        Pa3Ena2W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOA4 ANA2 enable"]
    #[inline(always)]
    pub fn pa4_ena2(&mut self) -> Pa4Ena2W<'_, PaAna2enrSpec> {
        Pa4Ena2W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOA5 ANA2 enable"]
    #[inline(always)]
    pub fn pa5_ena2(&mut self) -> Pa5Ena2W<'_, PaAna2enrSpec> {
        Pa5Ena2W::new(self, 5)
    }
    #[doc = "Bit 7 - GPIOA7 ANA2 enable"]
    #[inline(always)]
    pub fn pa7_ena2(&mut self) -> Pa7Ena2W<'_, PaAna2enrSpec> {
        Pa7Ena2W::new(self, 7)
    }
    #[doc = "Bit 10 - GPIOA10 ANA2 enable"]
    #[inline(always)]
    pub fn pa10_ena2(&mut self) -> Pa10Ena2W<'_, PaAna2enrSpec> {
        Pa10Ena2W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_ana2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_ana2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaAna2enrSpec;
impl crate::RegisterSpec for PaAna2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_ana2enr::R`](R) reader structure"]
impl crate::Readable for PaAna2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`pa_ana2enr::W`](W) writer structure"]
impl crate::Writable for PaAna2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PA_ANA2ENR to value 0"]
impl crate::Resettable for PaAna2enrSpec {}
