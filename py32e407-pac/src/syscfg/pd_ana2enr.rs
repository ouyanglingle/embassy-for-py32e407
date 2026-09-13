#[doc = "Register `PD_ANA2ENR` reader"]
pub type R = crate::R<PdAna2enrSpec>;
#[doc = "Register `PD_ANA2ENR` writer"]
pub type W = crate::W<PdAna2enrSpec>;
#[doc = "Field `PD10_ENA2` reader - GPIOD10_ANA2 enable"]
pub type Pd10Ena2R = crate::BitReader;
#[doc = "Field `PD10_ENA2` writer - GPIOD10_ANA2 enable"]
pub type Pd10Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD11_ENA2` reader - GPIOD11_ANA2 enable"]
pub type Pd11Ena2R = crate::BitReader;
#[doc = "Field `PD11_ENA2` writer - GPIOD11_ANA2 enable"]
pub type Pd11Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD12_ENA2` reader - GPIOD12_ANA2 enable"]
pub type Pd12Ena2R = crate::BitReader;
#[doc = "Field `PD12_ENA2` writer - GPIOD12_ANA2 enable"]
pub type Pd12Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD13_ENA2` reader - GPIOD13_ANA2 enable"]
pub type Pd13Ena2R = crate::BitReader;
#[doc = "Field `PD13_ENA2` writer - GPIOD13_ANA2 enable"]
pub type Pd13Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - GPIOD10_ANA2 enable"]
    #[inline(always)]
    pub fn pd10_ena2(&self) -> Pd10Ena2R {
        Pd10Ena2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIOD11_ANA2 enable"]
    #[inline(always)]
    pub fn pd11_ena2(&self) -> Pd11Ena2R {
        Pd11Ena2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIOD12_ANA2 enable"]
    #[inline(always)]
    pub fn pd12_ena2(&self) -> Pd12Ena2R {
        Pd12Ena2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIOD13_ANA2 enable"]
    #[inline(always)]
    pub fn pd13_ena2(&self) -> Pd13Ena2R {
        Pd13Ena2R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - GPIOD10_ANA2 enable"]
    #[inline(always)]
    pub fn pd10_ena2(&mut self) -> Pd10Ena2W<'_, PdAna2enrSpec> {
        Pd10Ena2W::new(self, 10)
    }
    #[doc = "Bit 11 - GPIOD11_ANA2 enable"]
    #[inline(always)]
    pub fn pd11_ena2(&mut self) -> Pd11Ena2W<'_, PdAna2enrSpec> {
        Pd11Ena2W::new(self, 11)
    }
    #[doc = "Bit 12 - GPIOD12_ANA2 enable"]
    #[inline(always)]
    pub fn pd12_ena2(&mut self) -> Pd12Ena2W<'_, PdAna2enrSpec> {
        Pd12Ena2W::new(self, 12)
    }
    #[doc = "Bit 13 - GPIOD13_ANA2 enable"]
    #[inline(always)]
    pub fn pd13_ena2(&mut self) -> Pd13Ena2W<'_, PdAna2enrSpec> {
        Pd13Ena2W::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_ana2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_ana2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdAna2enrSpec;
impl crate::RegisterSpec for PdAna2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_ana2enr::R`](R) reader structure"]
impl crate::Readable for PdAna2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`pd_ana2enr::W`](W) writer structure"]
impl crate::Writable for PdAna2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD_ANA2ENR to value 0"]
impl crate::Resettable for PdAna2enrSpec {}
