#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `DBG_SLEEP` reader - "]
pub type DbgSleepR = crate::BitReader;
#[doc = "Field `DBG_SLEEP` writer - "]
pub type DbgSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STOP` reader - "]
pub type DbgStopR = crate::BitReader;
#[doc = "Field `DBG_STOP` writer - "]
pub type DbgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STDBY` reader - "]
pub type DbgStdbyR = crate::BitReader;
#[doc = "Field `DBG_STDBY` writer - "]
pub type DbgStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_IOEN` reader - "]
pub type TraceIoenR = crate::BitReader;
#[doc = "Field `TRACE_IOEN` writer - "]
pub type TraceIoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_MODE` reader - "]
pub type TraceModeR = crate::FieldReader;
#[doc = "Field `TRACE_MODE` writer - "]
pub type TraceModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DBG_IWDG_STOP` reader - "]
pub type DbgIwdgStopR = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - "]
pub type DbgIwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG_STOP` reader - "]
pub type DbgWwdgStopR = crate::BitReader;
#[doc = "Field `DBG_WWDG_STOP` writer - "]
pub type DbgWwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM1_STOP` reader - "]
pub type DbgTim1StopR = crate::BitReader;
#[doc = "Field `DBG_TIM1_STOP` writer - "]
pub type DbgTim1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM2_STOP` reader - "]
pub type DbgTim2StopR = crate::BitReader;
#[doc = "Field `DBG_TIM2_STOP` writer - "]
pub type DbgTim2StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM3_STOP` reader - "]
pub type DbgTim3StopR = crate::BitReader;
#[doc = "Field `DBG_TIM3_STOP` writer - "]
pub type DbgTim3StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM4_STOP` reader - "]
pub type DbgTim4StopR = crate::BitReader;
#[doc = "Field `DBG_TIM4_STOP` writer - "]
pub type DbgTim4StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_CAN1_STOP` reader - "]
pub type DbgCan1StopR = crate::BitReader;
#[doc = "Field `DBG_CAN1_STOP` writer - "]
pub type DbgCan1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` reader - "]
pub type DbgI2c1SmbusTimeoutR = crate::BitReader;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` writer - "]
pub type DbgI2c1SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` reader - "]
pub type DbgI2c2SmbusTimeoutR = crate::BitReader;
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` writer - "]
pub type DbgI2c2SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM8_STOP` reader - "]
pub type DbgTim8StopR = crate::BitReader;
#[doc = "Field `DBG_TIM8_STOP` writer - "]
pub type DbgTim8StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM5_STOP` reader - "]
pub type DbgTim5StopR = crate::BitReader;
#[doc = "Field `DBG_TIM5_STOP` writer - "]
pub type DbgTim5StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM6_STOP` reader - "]
pub type DbgTim6StopR = crate::BitReader;
#[doc = "Field `DBG_TIM6_STOP` writer - "]
pub type DbgTim6StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM7_STOP` reader - "]
pub type DbgTim7StopR = crate::BitReader;
#[doc = "Field `DBG_TIM7_STOP` writer - "]
pub type DbgTim7StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM12_STOP` reader - "]
pub type DbgTim12StopR = crate::BitReader;
#[doc = "Field `DBG_TIM12_STOP` writer - "]
pub type DbgTim12StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM13_STOP` reader - "]
pub type DbgTim13StopR = crate::BitReader;
#[doc = "Field `DBG_TIM13_STOP` writer - "]
pub type DbgTim13StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM14_STOP` reader - "]
pub type DbgTim14StopR = crate::BitReader;
#[doc = "Field `DBG_TIM14_STOP` writer - "]
pub type DbgTim14StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM9_STOP` reader - "]
pub type DbgTim9StopR = crate::BitReader;
#[doc = "Field `DBG_TIM9_STOP` writer - "]
pub type DbgTim9StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM10_STOP` reader - "]
pub type DbgTim10StopR = crate::BitReader;
#[doc = "Field `DBG_TIM10_STOP` writer - "]
pub type DbgTim10StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM11_STOP` reader - "]
pub type DbgTim11StopR = crate::BitReader;
#[doc = "Field `DBG_TIM11_STOP` writer - "]
pub type DbgTim11StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DbgSleepR {
        DbgSleepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DbgStopR {
        DbgStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dbg_stdby(&self) -> DbgStdbyR {
        DbgStdbyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TraceIoenR {
        TraceIoenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TraceModeR {
        TraceModeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DbgIwdgStopR {
        DbgIwdgStopR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DbgWwdgStopR {
        DbgWwdgStopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DbgTim1StopR {
        DbgTim1StopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DbgTim2StopR {
        DbgTim2StopR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DbgTim3StopR {
        DbgTim3StopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DbgTim4StopR {
        DbgTim4StopR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dbg_can1_stop(&self) -> DbgCan1StopR {
        DbgCan1StopR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DbgI2c1SmbusTimeoutR {
        DbgI2c1SmbusTimeoutR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&self) -> DbgI2c2SmbusTimeoutR {
        DbgI2c2SmbusTimeoutR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dbg_tim8_stop(&self) -> DbgTim8StopR {
        DbgTim8StopR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DbgTim5StopR {
        DbgTim5StopR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DbgTim6StopR {
        DbgTim6StopR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DbgTim7StopR {
        DbgTim7StopR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dbg_tim12_stop(&self) -> DbgTim12StopR {
        DbgTim12StopR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dbg_tim13_stop(&self) -> DbgTim13StopR {
        DbgTim13StopR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DbgTim14StopR {
        DbgTim14StopR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dbg_tim9_stop(&self) -> DbgTim9StopR {
        DbgTim9StopR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dbg_tim10_stop(&self) -> DbgTim10StopR {
        DbgTim10StopR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dbg_tim11_stop(&self) -> DbgTim11StopR {
        DbgTim11StopR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dbg_sleep(&mut self) -> DbgSleepW<'_, Cr1Spec> {
        DbgSleepW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DbgStopW<'_, Cr1Spec> {
        DbgStopW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dbg_stdby(&mut self) -> DbgStdbyW<'_, Cr1Spec> {
        DbgStdbyW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn trace_ioen(&mut self) -> TraceIoenW<'_, Cr1Spec> {
        TraceIoenW::new(self, 5)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn trace_mode(&mut self) -> TraceModeW<'_, Cr1Spec> {
        TraceModeW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DbgIwdgStopW<'_, Cr1Spec> {
        DbgIwdgStopW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DbgWwdgStopW<'_, Cr1Spec> {
        DbgWwdgStopW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DbgTim1StopW<'_, Cr1Spec> {
        DbgTim1StopW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DbgTim2StopW<'_, Cr1Spec> {
        DbgTim2StopW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DbgTim3StopW<'_, Cr1Spec> {
        DbgTim3StopW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&mut self) -> DbgTim4StopW<'_, Cr1Spec> {
        DbgTim4StopW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dbg_can1_stop(&mut self) -> DbgCan1StopW<'_, Cr1Spec> {
        DbgCan1StopW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DbgI2c1SmbusTimeoutW<'_, Cr1Spec> {
        DbgI2c1SmbusTimeoutW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&mut self) -> DbgI2c2SmbusTimeoutW<'_, Cr1Spec> {
        DbgI2c2SmbusTimeoutW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dbg_tim8_stop(&mut self) -> DbgTim8StopW<'_, Cr1Spec> {
        DbgTim8StopW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&mut self) -> DbgTim5StopW<'_, Cr1Spec> {
        DbgTim5StopW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&mut self) -> DbgTim6StopW<'_, Cr1Spec> {
        DbgTim6StopW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DbgTim7StopW<'_, Cr1Spec> {
        DbgTim7StopW::new(self, 20)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dbg_tim12_stop(&mut self) -> DbgTim12StopW<'_, Cr1Spec> {
        DbgTim12StopW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dbg_tim13_stop(&mut self) -> DbgTim13StopW<'_, Cr1Spec> {
        DbgTim13StopW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&mut self) -> DbgTim14StopW<'_, Cr1Spec> {
        DbgTim14StopW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dbg_tim9_stop(&mut self) -> DbgTim9StopW<'_, Cr1Spec> {
        DbgTim9StopW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dbg_tim10_stop(&mut self) -> DbgTim10StopW<'_, Cr1Spec> {
        DbgTim10StopW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dbg_tim11_stop(&mut self) -> DbgTim11StopW<'_, Cr1Spec> {
        DbgTim11StopW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
