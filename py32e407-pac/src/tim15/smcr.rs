#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SmcrSpec>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SmcrSpec>;
#[doc = "Field `SMS_L` reader - "]
pub type SmsLR = crate::FieldReader;
#[doc = "Field `SMS_L` writer - "]
pub type SmsLW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TS_L` reader - "]
pub type TsLR = crate::FieldReader;
#[doc = "Field `TS_L` writer - "]
pub type TsLW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MSM` reader - "]
pub type MsmR = crate::BitReader;
#[doc = "Field `MSM` writer - "]
pub type MsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMS_H` reader - "]
pub type SmsHR = crate::BitReader;
#[doc = "Field `SMS_H` writer - "]
pub type SmsHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_H` reader - "]
pub type TsHR = crate::FieldReader;
#[doc = "Field `TS_H` writer - "]
pub type TsHW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sms_l(&self) -> SmsLR {
        SmsLR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn ts_l(&self) -> TsLR {
        TsLR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn msm(&self) -> MsmR {
        MsmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sms_h(&self) -> SmsHR {
        SmsHR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn ts_h(&self) -> TsHR {
        TsHR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sms_l(&mut self) -> SmsLW<'_, SmcrSpec> {
        SmsLW::new(self, 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn ts_l(&mut self) -> TsLW<'_, SmcrSpec> {
        TsLW::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn msm(&mut self) -> MsmW<'_, SmcrSpec> {
        MsmW::new(self, 7)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sms_h(&mut self) -> SmsHW<'_, SmcrSpec> {
        SmsHW::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn ts_h(&mut self) -> TsHW<'_, SmcrSpec> {
        TsHW::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcrSpec;
impl crate::RegisterSpec for SmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SmcrSpec {}
