#[doc = "Register `PB_ANA2ENR` reader"]
pub type R = crate::R<PbAna2enrSpec>;
#[doc = "Register `PB_ANA2ENR` writer"]
pub type W = crate::W<PbAna2enrSpec>;
#[doc = "Field `PB0_ENA2` reader - GPIOB0 ANA2 enable"]
pub type Pb0Ena2R = crate::BitReader;
#[doc = "Field `PB0_ENA2` writer - GPIOB0 ANA2 enable"]
pub type Pb0Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB1_ENA2` reader - GPIOB1 ANA2 enable"]
pub type Pb1Ena2R = crate::BitReader;
#[doc = "Field `PB1_ENA2` writer - GPIOB1 ANA2 enable"]
pub type Pb1Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB2_ENA2` reader - GPIOB2 ANA2 enable"]
pub type Pb2Ena2R = crate::BitReader;
#[doc = "Field `PB2_ENA2` writer - GPIOB2 ANA2 enable"]
pub type Pb2Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOB0 ANA2 enable"]
    #[inline(always)]
    pub fn pb0_ena2(&self) -> Pb0Ena2R {
        Pb0Ena2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOB1 ANA2 enable"]
    #[inline(always)]
    pub fn pb1_ena2(&self) -> Pb1Ena2R {
        Pb1Ena2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOB2 ANA2 enable"]
    #[inline(always)]
    pub fn pb2_ena2(&self) -> Pb2Ena2R {
        Pb2Ena2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOB0 ANA2 enable"]
    #[inline(always)]
    pub fn pb0_ena2(&mut self) -> Pb0Ena2W<'_, PbAna2enrSpec> {
        Pb0Ena2W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOB1 ANA2 enable"]
    #[inline(always)]
    pub fn pb1_ena2(&mut self) -> Pb1Ena2W<'_, PbAna2enrSpec> {
        Pb1Ena2W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOB2 ANA2 enable"]
    #[inline(always)]
    pub fn pb2_ena2(&mut self) -> Pb2Ena2W<'_, PbAna2enrSpec> {
        Pb2Ena2W::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_ana2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_ana2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbAna2enrSpec;
impl crate::RegisterSpec for PbAna2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_ana2enr::R`](R) reader structure"]
impl crate::Readable for PbAna2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`pb_ana2enr::W`](W) writer structure"]
impl crate::Writable for PbAna2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PB_ANA2ENR to value 0"]
impl crate::Resettable for PbAna2enrSpec {}
