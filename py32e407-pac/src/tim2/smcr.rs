#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SmcrSpec>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SmcrSpec>;
#[doc = "Field `SMS_L` reader - "]
pub type SmsLR = crate::FieldReader;
#[doc = "Field `SMS_L` writer - "]
pub type SmsLW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OCCS` reader - "]
pub type OccsR = crate::BitReader;
#[doc = "Field `OCCS` writer - "]
pub type OccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_L` reader - "]
pub type TsLR = crate::FieldReader;
#[doc = "Field `TS_L` writer - "]
pub type TsLW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSM` reader - "]
pub type MsmR = crate::BitReader;
#[doc = "Field `MSM` writer - "]
pub type MsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETF` reader - "]
pub type EtfR = crate::FieldReader;
#[doc = "Field `ETF` writer - "]
pub type EtfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETPS` reader - "]
pub type EtpsR = crate::FieldReader;
#[doc = "Field `ETPS` writer - "]
pub type EtpsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECE` reader - "]
pub type EceR = crate::BitReader;
#[doc = "Field `ECE` writer - "]
pub type EceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETP` reader - "]
pub type EtpR = crate::BitReader;
#[doc = "Field `ETP` writer - "]
pub type EtpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMS_H` reader - "]
pub type SmsHR = crate::BitReader;
#[doc = "Field `SMS_H` writer - "]
pub type SmsHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_H` reader - "]
pub type TsHR = crate::FieldReader;
#[doc = "Field `TS_H` writer - "]
pub type TsHW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SMSPE` reader - "]
pub type SmspeR = crate::BitReader;
#[doc = "Field `SMSPE` writer - "]
pub type SmspeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMSPS` reader - "]
pub type SmspsR = crate::BitReader;
#[doc = "Field `SMSPS` writer - "]
pub type SmspsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sms_l(&self) -> SmsLR {
        SmsLR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn occs(&self) -> OccsR {
        OccsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ts_l(&self) -> TsLR {
        TsLR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn msm(&self) -> MsmR {
        MsmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn etf(&self) -> EtfR {
        EtfR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn etps(&self) -> EtpsR {
        EtpsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ece(&self) -> EceR {
        EceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn etp(&self) -> EtpR {
        EtpR::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn smspe(&self) -> SmspeR {
        SmspeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn smsps(&self) -> SmspsR {
        SmspsR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sms_l(&mut self) -> SmsLW<'_, SmcrSpec> {
        SmsLW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn occs(&mut self) -> OccsW<'_, SmcrSpec> {
        OccsW::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ts_l(&mut self) -> TsLW<'_, SmcrSpec> {
        TsLW::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn msm(&mut self) -> MsmW<'_, SmcrSpec> {
        MsmW::new(self, 7)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn etf(&mut self) -> EtfW<'_, SmcrSpec> {
        EtfW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn etps(&mut self) -> EtpsW<'_, SmcrSpec> {
        EtpsW::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ece(&mut self) -> EceW<'_, SmcrSpec> {
        EceW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn etp(&mut self) -> EtpW<'_, SmcrSpec> {
        EtpW::new(self, 15)
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
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn smspe(&mut self) -> SmspeW<'_, SmcrSpec> {
        SmspeW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn smsps(&mut self) -> SmspsW<'_, SmcrSpec> {
        SmspsW::new(self, 25)
    }
}
#[doc = "desc SMCR\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
