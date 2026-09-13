#[doc = "Register `PF_ANA2ENR` reader"]
pub type R = crate::R<PfAna2enrSpec>;
#[doc = "Register `PF_ANA2ENR` writer"]
pub type W = crate::W<PfAna2enrSpec>;
#[doc = "Field `PF0_ENA2` reader - GPIOF0_ANA2 enable"]
pub type Pf0Ena2R = crate::BitReader;
#[doc = "Field `PF0_ENA2` writer - GPIOF0_ANA2 enable"]
pub type Pf0Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF1_ENA2` reader - GPIOF1_ANA2 enable"]
pub type Pf1Ena2R = crate::BitReader;
#[doc = "Field `PF1_ENA2` writer - GPIOF1_ANA2 enable"]
pub type Pf1Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOF0_ANA2 enable"]
    #[inline(always)]
    pub fn pf0_ena2(&self) -> Pf0Ena2R {
        Pf0Ena2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOF1_ANA2 enable"]
    #[inline(always)]
    pub fn pf1_ena2(&self) -> Pf1Ena2R {
        Pf1Ena2R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOF0_ANA2 enable"]
    #[inline(always)]
    pub fn pf0_ena2(&mut self) -> Pf0Ena2W<'_, PfAna2enrSpec> {
        Pf0Ena2W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOF1_ANA2 enable"]
    #[inline(always)]
    pub fn pf1_ena2(&mut self) -> Pf1Ena2W<'_, PfAna2enrSpec> {
        Pf1Ena2W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_ana2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_ana2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfAna2enrSpec;
impl crate::RegisterSpec for PfAna2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_ana2enr::R`](R) reader structure"]
impl crate::Readable for PfAna2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`pf_ana2enr::W`](W) writer structure"]
impl crate::Writable for PfAna2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_ANA2ENR to value 0"]
impl crate::Resettable for PfAna2enrSpec {}
