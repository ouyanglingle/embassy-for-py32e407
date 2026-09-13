#[doc = "Register `PE_ANA2ENR` reader"]
pub type R = crate::R<PeAna2enrSpec>;
#[doc = "Register `PE_ANA2ENR` writer"]
pub type W = crate::W<PeAna2enrSpec>;
#[doc = "Field `PE7_ENA2` reader - GPIOE7_ANA2 enable"]
pub type Pe7Ena2R = crate::BitReader;
#[doc = "Field `PE7_ENA2` writer - GPIOE7_ANA2 enable"]
pub type Pe7Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE8_ENA2` reader - GPIOE8_ANA2 enable"]
pub type Pe8Ena2R = crate::BitReader;
#[doc = "Field `PE8_ENA2` writer - GPIOE8_ANA2 enable"]
pub type Pe8Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE10_ENA2` reader - GPIOE10_ANA2 enable"]
pub type Pe10Ena2R = crate::BitReader;
#[doc = "Field `PE10_ENA2` writer - GPIOE10_ANA2 enable"]
pub type Pe10Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE11_ENA2` reader - GPIOE11_ANA2 enable"]
pub type Pe11Ena2R = crate::BitReader;
#[doc = "Field `PE11_ENA2` writer - GPIOE11_ANA2 enable"]
pub type Pe11Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE12_ENA2` reader - GPIOE12_ANA2 enable"]
pub type Pe12Ena2R = crate::BitReader;
#[doc = "Field `PE12_ENA2` writer - GPIOE12_ANA2 enable"]
pub type Pe12Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - GPIOE7_ANA2 enable"]
    #[inline(always)]
    pub fn pe7_ena2(&self) -> Pe7Ena2R {
        Pe7Ena2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOE8_ANA2 enable"]
    #[inline(always)]
    pub fn pe8_ena2(&self) -> Pe8Ena2R {
        Pe8Ena2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOE10_ANA2 enable"]
    #[inline(always)]
    pub fn pe10_ena2(&self) -> Pe10Ena2R {
        Pe10Ena2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIOE11_ANA2 enable"]
    #[inline(always)]
    pub fn pe11_ena2(&self) -> Pe11Ena2R {
        Pe11Ena2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIOE12_ANA2 enable"]
    #[inline(always)]
    pub fn pe12_ena2(&self) -> Pe12Ena2R {
        Pe12Ena2R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - GPIOE7_ANA2 enable"]
    #[inline(always)]
    pub fn pe7_ena2(&mut self) -> Pe7Ena2W<'_, PeAna2enrSpec> {
        Pe7Ena2W::new(self, 7)
    }
    #[doc = "Bit 8 - GPIOE8_ANA2 enable"]
    #[inline(always)]
    pub fn pe8_ena2(&mut self) -> Pe8Ena2W<'_, PeAna2enrSpec> {
        Pe8Ena2W::new(self, 8)
    }
    #[doc = "Bit 10 - GPIOE10_ANA2 enable"]
    #[inline(always)]
    pub fn pe10_ena2(&mut self) -> Pe10Ena2W<'_, PeAna2enrSpec> {
        Pe10Ena2W::new(self, 10)
    }
    #[doc = "Bit 11 - GPIOE11_ANA2 enable"]
    #[inline(always)]
    pub fn pe11_ena2(&mut self) -> Pe11Ena2W<'_, PeAna2enrSpec> {
        Pe11Ena2W::new(self, 11)
    }
    #[doc = "Bit 12 - GPIOE12_ANA2 enable"]
    #[inline(always)]
    pub fn pe12_ena2(&mut self) -> Pe12Ena2W<'_, PeAna2enrSpec> {
        Pe12Ena2W::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_ana2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_ana2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeAna2enrSpec;
impl crate::RegisterSpec for PeAna2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_ana2enr::R`](R) reader structure"]
impl crate::Readable for PeAna2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`pe_ana2enr::W`](W) writer structure"]
impl crate::Writable for PeAna2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_ANA2ENR to value 0"]
impl crate::Resettable for PeAna2enrSpec {}
