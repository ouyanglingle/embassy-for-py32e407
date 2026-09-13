#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `EN` reader - COMP1 enable bit"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - COMP1 enable bit"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_VSEL` reader - VREFCMP DIV SOURCE voltage select"]
pub type CompVselR = crate::FieldReader;
#[doc = "Field `COMP_VSEL` writer - VREFCMP DIV SOURCE voltage select"]
pub type CompVselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INNSEL` reader - COMP1 negative input select"]
pub type InnselR = crate::FieldReader;
#[doc = "Field `INNSEL` writer - COMP1 negative input select"]
pub type InnselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INPSEL` reader - COMP1 positive input select"]
pub type InpselR = crate::BitReader;
#[doc = "Field `INPSEL` writer - COMP1 positive input select"]
pub type InpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINMODE` reader - COMP1 window mode enable"]
pub type WinmodeR = crate::BitReader;
#[doc = "Field `WINMODE` writer - COMP1 window mode enable"]
pub type WinmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKSEL` reader - Comparator 1 blanking signal selection"]
pub type BlankselR = crate::FieldReader;
#[doc = "Field `BLANKSEL` writer - Comparator 1 blanking signal selection"]
pub type BlankselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `POLARITY` reader - COMP1 output polarity selection"]
pub type PolarityR = crate::BitReader;
#[doc = "Field `POLARITY` writer - COMP1 output polarity selection"]
pub type PolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - COMP1 hysteresis function enable control"]
pub type HystR = crate::BitReader;
#[doc = "Field `HYST` writer - COMP1 hysteresis function enable control"]
pub type HystW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRMODE` reader - COMP1 power mode selection"]
pub type PwrmodeR = crate::FieldReader;
#[doc = "Field `PWRMODE` writer - COMP1 power mode selection"]
pub type PwrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP_VCDIV` reader - COMP1, COMP2 partial voltage selection"]
pub type CompVcdivR = crate::FieldReader;
#[doc = "Field `COMP_VCDIV` writer - COMP1, COMP2 partial voltage selection"]
pub type CompVcdivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `COMP_VCDIV_EN` reader - VREFCMP voltage divider enable signal"]
pub type CompVcdivEnR = crate::BitReader;
#[doc = "Field `COMP_VCDIV_EN` writer - VREFCMP voltage divider enable signal"]
pub type CompVcdivEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_VCSEL` reader - VREFCMP reference source selection"]
pub type CompVcselR = crate::BitReader;
#[doc = "Field `COMP_VCSEL` writer - VREFCMP reference source selection"]
pub type CompVcselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM_EXTI_OUT_SEL` reader - COMP1 output to TIM/EXTI selection"]
pub type TimExtiOutSelR = crate::BitReader;
#[doc = "Field `TIM_EXTI_OUT_SEL` writer - COMP1 output to TIM/EXTI selection"]
pub type TimExtiOutSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_OUT` reader - COMP1 output status"]
pub type Comp1OutR = crate::BitReader;
#[doc = "Field `LOCK` reader - COMP1_CSR registerlock"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - COMP1_CSR registerlock"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - COMP1 enable bit"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - VREFCMP DIV SOURCE voltage select"]
    #[inline(always)]
    pub fn comp_vsel(&self) -> CompVselR {
        CompVselR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - COMP1 negative input select"]
    #[inline(always)]
    pub fn innsel(&self) -> InnselR {
        InnselR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 9 - COMP1 positive input select"]
    #[inline(always)]
    pub fn inpsel(&self) -> InpselR {
        InpselR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - COMP1 window mode enable"]
    #[inline(always)]
    pub fn winmode(&self) -> WinmodeR {
        WinmodeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Comparator 1 blanking signal selection"]
    #[inline(always)]
    pub fn blanksel(&self) -> BlankselR {
        BlankselR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - COMP1 output polarity selection"]
    #[inline(always)]
    pub fn polarity(&self) -> PolarityR {
        PolarityR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - COMP1 hysteresis function enable control"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:19 - COMP1 power mode selection"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PwrmodeR {
        PwrmodeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:25 - COMP1, COMP2 partial voltage selection"]
    #[inline(always)]
    pub fn comp_vcdiv(&self) -> CompVcdivR {
        CompVcdivR::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - VREFCMP voltage divider enable signal"]
    #[inline(always)]
    pub fn comp_vcdiv_en(&self) -> CompVcdivEnR {
        CompVcdivEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - VREFCMP reference source selection"]
    #[inline(always)]
    pub fn comp_vcsel(&self) -> CompVcselR {
        CompVcselR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - COMP1 output to TIM/EXTI selection"]
    #[inline(always)]
    pub fn tim_exti_out_sel(&self) -> TimExtiOutSelR {
        TimExtiOutSelR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - COMP1 output status"]
    #[inline(always)]
    pub fn comp1_out(&self) -> Comp1OutR {
        Comp1OutR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - COMP1_CSR registerlock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COMP1 enable bit"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CsrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - VREFCMP DIV SOURCE voltage select"]
    #[inline(always)]
    pub fn comp_vsel(&mut self) -> CompVselW<'_, CsrSpec> {
        CompVselW::new(self, 1)
    }
    #[doc = "Bits 3:5 - COMP1 negative input select"]
    #[inline(always)]
    pub fn innsel(&mut self) -> InnselW<'_, CsrSpec> {
        InnselW::new(self, 3)
    }
    #[doc = "Bit 9 - COMP1 positive input select"]
    #[inline(always)]
    pub fn inpsel(&mut self) -> InpselW<'_, CsrSpec> {
        InpselW::new(self, 9)
    }
    #[doc = "Bit 11 - COMP1 window mode enable"]
    #[inline(always)]
    pub fn winmode(&mut self) -> WinmodeW<'_, CsrSpec> {
        WinmodeW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Comparator 1 blanking signal selection"]
    #[inline(always)]
    pub fn blanksel(&mut self) -> BlankselW<'_, CsrSpec> {
        BlankselW::new(self, 12)
    }
    #[doc = "Bit 15 - COMP1 output polarity selection"]
    #[inline(always)]
    pub fn polarity(&mut self) -> PolarityW<'_, CsrSpec> {
        PolarityW::new(self, 15)
    }
    #[doc = "Bit 16 - COMP1 hysteresis function enable control"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<'_, CsrSpec> {
        HystW::new(self, 16)
    }
    #[doc = "Bits 18:19 - COMP1 power mode selection"]
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PwrmodeW<'_, CsrSpec> {
        PwrmodeW::new(self, 18)
    }
    #[doc = "Bits 20:25 - COMP1, COMP2 partial voltage selection"]
    #[inline(always)]
    pub fn comp_vcdiv(&mut self) -> CompVcdivW<'_, CsrSpec> {
        CompVcdivW::new(self, 20)
    }
    #[doc = "Bit 26 - VREFCMP voltage divider enable signal"]
    #[inline(always)]
    pub fn comp_vcdiv_en(&mut self) -> CompVcdivEnW<'_, CsrSpec> {
        CompVcdivEnW::new(self, 26)
    }
    #[doc = "Bit 27 - VREFCMP reference source selection"]
    #[inline(always)]
    pub fn comp_vcsel(&mut self) -> CompVcselW<'_, CsrSpec> {
        CompVcselW::new(self, 27)
    }
    #[doc = "Bit 29 - COMP1 output to TIM/EXTI selection"]
    #[inline(always)]
    pub fn tim_exti_out_sel(&mut self) -> TimExtiOutSelW<'_, CsrSpec> {
        TimExtiOutSelW::new(self, 29)
    }
    #[doc = "Bit 31 - COMP1_CSR registerlock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, CsrSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
