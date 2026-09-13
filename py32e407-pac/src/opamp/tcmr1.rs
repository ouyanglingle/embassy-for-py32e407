#[doc = "Register `TCMR1` reader"]
pub type R = crate::R<Tcmr1Spec>;
#[doc = "Register `TCMR1` writer"]
pub type W = crate::W<Tcmr1Spec>;
#[doc = "Field `VMS_SEL` reader - OPAMPx reverse phase input secondary selection"]
pub type VmsSelR = crate::BitReader;
#[doc = "Field `VMS_SEL` writer - OPAMPx reverse phase input secondary selection"]
pub type VmsSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPS_SEL` reader - OPAMPx non inverting input secondary selection"]
pub type VpsSelR = crate::FieldReader;
#[doc = "Field `VPS_SEL` writer - OPAMPx non inverting input secondary selection"]
pub type VpsSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `T1CM_EN` reader - TIM1 Control Multiplexing Mode Enable"]
pub type T1cmEnR = crate::BitReader;
#[doc = "Field `T1CM_EN` writer - TIM1 Control Multiplexing Mode Enable"]
pub type T1cmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T8CM_EN` reader - TIM8 Control Multiplexing Mode Enable"]
pub type T8cmEnR = crate::BitReader;
#[doc = "Field `T8CM_EN` writer - TIM8 Control Multiplexing Mode Enable"]
pub type T8cmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - OPAMP1_TCMR register Lock"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - OPAMP1_TCMR register Lock"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OPAMPx reverse phase input secondary selection"]
    #[inline(always)]
    pub fn vms_sel(&self) -> VmsSelR {
        VmsSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - OPAMPx non inverting input secondary selection"]
    #[inline(always)]
    pub fn vps_sel(&self) -> VpsSelR {
        VpsSelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - TIM1 Control Multiplexing Mode Enable"]
    #[inline(always)]
    pub fn t1cm_en(&self) -> T1cmEnR {
        T1cmEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM8 Control Multiplexing Mode Enable"]
    #[inline(always)]
    pub fn t8cm_en(&self) -> T8cmEnR {
        T8cmEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - OPAMP1_TCMR register Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPAMPx reverse phase input secondary selection"]
    #[inline(always)]
    pub fn vms_sel(&mut self) -> VmsSelW<'_, Tcmr1Spec> {
        VmsSelW::new(self, 0)
    }
    #[doc = "Bits 1:2 - OPAMPx non inverting input secondary selection"]
    #[inline(always)]
    pub fn vps_sel(&mut self) -> VpsSelW<'_, Tcmr1Spec> {
        VpsSelW::new(self, 1)
    }
    #[doc = "Bit 3 - TIM1 Control Multiplexing Mode Enable"]
    #[inline(always)]
    pub fn t1cm_en(&mut self) -> T1cmEnW<'_, Tcmr1Spec> {
        T1cmEnW::new(self, 3)
    }
    #[doc = "Bit 4 - TIM8 Control Multiplexing Mode Enable"]
    #[inline(always)]
    pub fn t8cm_en(&mut self) -> T8cmEnW<'_, Tcmr1Spec> {
        T8cmEnW::new(self, 4)
    }
    #[doc = "Bit 31 - OPAMP1_TCMR register Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, Tcmr1Spec> {
        LockW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tcmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcmr1Spec;
impl crate::RegisterSpec for Tcmr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcmr1::R`](R) reader structure"]
impl crate::Readable for Tcmr1Spec {}
#[doc = "`write(|w| ..)` method takes [`tcmr1::W`](W) writer structure"]
impl crate::Writable for Tcmr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCMR1 to value 0"]
impl crate::Resettable for Tcmr1Spec {}
