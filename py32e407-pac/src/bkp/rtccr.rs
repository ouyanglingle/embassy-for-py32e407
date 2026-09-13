#[doc = "Register `RTCCR` reader"]
pub type R = crate::R<RtccrSpec>;
#[doc = "Register `RTCCR` writer"]
pub type W = crate::W<RtccrSpec>;
#[doc = "Field `CAL` reader - RTC calibration value"]
pub type CalR = crate::FieldReader;
#[doc = "Field `CAL` writer - RTC calibration value"]
pub type CalW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CCO` reader - Calibration clock output"]
pub type CcoR = crate::BitReader;
#[doc = "Field `CCO` writer - Calibration clock output"]
pub type CcoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASOE` reader - Alarm or second output enable"]
pub type AsoeR = crate::BitReader;
#[doc = "Field `ASOE` writer - Alarm or second output enable"]
pub type AsoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASOS` reader - Alarm or second output selection"]
pub type AsosR = crate::BitReader;
#[doc = "Field `ASOS` writer - Alarm or second output selection"]
pub type AsosW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - RTC calibration value"]
    #[inline(always)]
    pub fn cal(&self) -> CalR {
        CalR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Calibration clock output"]
    #[inline(always)]
    pub fn cco(&self) -> CcoR {
        CcoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    pub fn asoe(&self) -> AsoeR {
        AsoeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    pub fn asos(&self) -> AsosR {
        AsosR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - RTC calibration value"]
    #[inline(always)]
    pub fn cal(&mut self) -> CalW<'_, RtccrSpec> {
        CalW::new(self, 0)
    }
    #[doc = "Bit 7 - Calibration clock output"]
    #[inline(always)]
    pub fn cco(&mut self) -> CcoW<'_, RtccrSpec> {
        CcoW::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    pub fn asoe(&mut self) -> AsoeW<'_, RtccrSpec> {
        AsoeW::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    pub fn asos(&mut self) -> AsosW<'_, RtccrSpec> {
        AsosW::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtccrSpec;
impl crate::RegisterSpec for RtccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccr::R`](R) reader structure"]
impl crate::Readable for RtccrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtccr::W`](W) writer structure"]
impl crate::Writable for RtccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCR to value 0"]
impl crate::Resettable for RtccrSpec {}
