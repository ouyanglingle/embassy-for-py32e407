#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `DBG_I2C3_SMBUS_TIMEOUT` reader - "]
pub type DbgI2c3SmbusTimeoutR = crate::BitReader;
#[doc = "Field `DBG_I2C3_SMBUS_TIMEOUT` writer - "]
pub type DbgI2c3SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C4_SMBUS_TIMEOUT` reader - "]
pub type DbgI2c4SmbusTimeoutR = crate::BitReader;
#[doc = "Field `DBG_I2C4_SMBUS_TIMEOUT` writer - "]
pub type DbgI2c4SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_CAN2_STOP` reader - "]
pub type DbgCan2StopR = crate::BitReader;
#[doc = "Field `DBG_CAN2_STOP` writer - "]
pub type DbgCan2StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM15_STOP` reader - "]
pub type DbgTim15StopR = crate::BitReader;
#[doc = "Field `DBG_TIM15_STOP` writer - "]
pub type DbgTim15StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM16_STOP` reader - "]
pub type DbgTim16StopR = crate::BitReader;
#[doc = "Field `DBG_TIM16_STOP` writer - "]
pub type DbgTim16StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM17_STOP` reader - "]
pub type DbgTim17StopR = crate::BitReader;
#[doc = "Field `DBG_TIM17_STOP` writer - "]
pub type DbgTim17StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM18_STOP` reader - "]
pub type DbgTim18StopR = crate::BitReader;
#[doc = "Field `DBG_TIM18_STOP` writer - "]
pub type DbgTim18StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM19_STOP` reader - "]
pub type DbgTim19StopR = crate::BitReader;
#[doc = "Field `DBG_TIM19_STOP` writer - "]
pub type DbgTim19StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM_STOP` reader - "]
pub type DbgLptimStopR = crate::BitReader;
#[doc = "Field `DBG_LPTIM_STOP` writer - "]
pub type DbgLptimStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dbg_i2c3_smbus_timeout(&self) -> DbgI2c3SmbusTimeoutR {
        DbgI2c3SmbusTimeoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dbg_i2c4_smbus_timeout(&self) -> DbgI2c4SmbusTimeoutR {
        DbgI2c4SmbusTimeoutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dbg_can2_stop(&self) -> DbgCan2StopR {
        DbgCan2StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DbgTim15StopR {
        DbgTim15StopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DbgTim16StopR {
        DbgTim16StopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DbgTim17StopR {
        DbgTim17StopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dbg_tim18_stop(&self) -> DbgTim18StopR {
        DbgTim18StopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dbg_tim19_stop(&self) -> DbgTim19StopR {
        DbgTim19StopR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dbg_lptim_stop(&self) -> DbgLptimStopR {
        DbgLptimStopR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dbg_i2c3_smbus_timeout(&mut self) -> DbgI2c3SmbusTimeoutW<'_, Cr2Spec> {
        DbgI2c3SmbusTimeoutW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dbg_i2c4_smbus_timeout(&mut self) -> DbgI2c4SmbusTimeoutW<'_, Cr2Spec> {
        DbgI2c4SmbusTimeoutW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dbg_can2_stop(&mut self) -> DbgCan2StopW<'_, Cr2Spec> {
        DbgCan2StopW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DbgTim15StopW<'_, Cr2Spec> {
        DbgTim15StopW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DbgTim16StopW<'_, Cr2Spec> {
        DbgTim16StopW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&mut self) -> DbgTim17StopW<'_, Cr2Spec> {
        DbgTim17StopW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dbg_tim18_stop(&mut self) -> DbgTim18StopW<'_, Cr2Spec> {
        DbgTim18StopW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dbg_tim19_stop(&mut self) -> DbgTim19StopW<'_, Cr2Spec> {
        DbgTim19StopW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dbg_lptim_stop(&mut self) -> DbgLptimStopW<'_, Cr2Spec> {
        DbgLptimStopW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
