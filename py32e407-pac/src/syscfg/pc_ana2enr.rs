#[doc = "Register `PC_ANA2ENR` reader"]
pub type R = crate::R<PcAna2enrSpec>;
#[doc = "Register `PC_ANA2ENR` writer"]
pub type W = crate::W<PcAna2enrSpec>;
#[doc = "Field `PC0_ENA2` reader - GPIOC0 ANA2 enable"]
pub type Pc0Ena2R = crate::BitReader;
#[doc = "Field `PC0_ENA2` writer - GPIOC0 ANA2 enable"]
pub type Pc0Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC1_ENA2` reader - GPIOC1 ANA2 enable"]
pub type Pc1Ena2R = crate::BitReader;
#[doc = "Field `PC1_ENA2` writer - GPIOC1 ANA2 enable"]
pub type Pc1Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC2_ENA2` reader - GPIOC2 ANA2 enable"]
pub type Pc2Ena2R = crate::BitReader;
#[doc = "Field `PC2_ENA2` writer - GPIOC2 ANA2 enable"]
pub type Pc2Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC3_ENA2` reader - GPIOC3 ANA2 enable"]
pub type Pc3Ena2R = crate::BitReader;
#[doc = "Field `PC3_ENA2` writer - GPIOC3 ANA2 enable"]
pub type Pc3Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC14_ENA2` reader - GPIOC14 ANA2 enable"]
pub type Pc14Ena2R = crate::BitReader;
#[doc = "Field `PC14_ENA2` writer - GPIOC14 ANA2 enable"]
pub type Pc14Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC15_ENA2` reader - GPIOC15 ANA2 enable"]
pub type Pc15Ena2R = crate::BitReader;
#[doc = "Field `PC15_ENA2` writer - GPIOC15 ANA2 enable"]
pub type Pc15Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOC0 ANA2 enable"]
    #[inline(always)]
    pub fn pc0_ena2(&self) -> Pc0Ena2R {
        Pc0Ena2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOC1 ANA2 enable"]
    #[inline(always)]
    pub fn pc1_ena2(&self) -> Pc1Ena2R {
        Pc1Ena2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOC2 ANA2 enable"]
    #[inline(always)]
    pub fn pc2_ena2(&self) -> Pc2Ena2R {
        Pc2Ena2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOC3 ANA2 enable"]
    #[inline(always)]
    pub fn pc3_ena2(&self) -> Pc3Ena2R {
        Pc3Ena2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIOC14 ANA2 enable"]
    #[inline(always)]
    pub fn pc14_ena2(&self) -> Pc14Ena2R {
        Pc14Ena2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIOC15 ANA2 enable"]
    #[inline(always)]
    pub fn pc15_ena2(&self) -> Pc15Ena2R {
        Pc15Ena2R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOC0 ANA2 enable"]
    #[inline(always)]
    pub fn pc0_ena2(&mut self) -> Pc0Ena2W<'_, PcAna2enrSpec> {
        Pc0Ena2W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOC1 ANA2 enable"]
    #[inline(always)]
    pub fn pc1_ena2(&mut self) -> Pc1Ena2W<'_, PcAna2enrSpec> {
        Pc1Ena2W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOC2 ANA2 enable"]
    #[inline(always)]
    pub fn pc2_ena2(&mut self) -> Pc2Ena2W<'_, PcAna2enrSpec> {
        Pc2Ena2W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOC3 ANA2 enable"]
    #[inline(always)]
    pub fn pc3_ena2(&mut self) -> Pc3Ena2W<'_, PcAna2enrSpec> {
        Pc3Ena2W::new(self, 3)
    }
    #[doc = "Bit 14 - GPIOC14 ANA2 enable"]
    #[inline(always)]
    pub fn pc14_ena2(&mut self) -> Pc14Ena2W<'_, PcAna2enrSpec> {
        Pc14Ena2W::new(self, 14)
    }
    #[doc = "Bit 15 - GPIOC15 ANA2 enable"]
    #[inline(always)]
    pub fn pc15_ena2(&mut self) -> Pc15Ena2W<'_, PcAna2enrSpec> {
        Pc15Ena2W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_ana2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_ana2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcAna2enrSpec;
impl crate::RegisterSpec for PcAna2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc_ana2enr::R`](R) reader structure"]
impl crate::Readable for PcAna2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`pc_ana2enr::W`](W) writer structure"]
impl crate::Writable for PcAna2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PC_ANA2ENR to value 0"]
impl crate::Resettable for PcAna2enrSpec {}
