#[doc = "Register `OPTR1` reader"]
pub type R = crate::R<Optr1Spec>;
#[doc = "Register `OPTR1` writer"]
pub type W = crate::W<Optr1Spec>;
#[doc = "Field `RDP` reader - Read Protection"]
pub type RdpR = crate::FieldReader;
#[doc = "Field `RDP` writer - Read Protection"]
pub type RdpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IWDG_STOP` reader - IWDG_STOP"]
pub type IwdgStopR = crate::BitReader;
#[doc = "Field `IWDG_STOP` writer - IWDG_STOP"]
pub type IwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_STDBY` reader - IWDG_STDBY"]
pub type IwdgStdbyR = crate::BitReader;
#[doc = "Field `IWDG_STDBY` writer - IWDG_STDBY"]
pub type IwdgStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFB` reader - "]
pub type BfbR = crate::BitReader;
#[doc = "Field `BFB` writer - "]
pub type BfbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_SW` reader - IWDG Software Enable"]
pub type IwdgSwR = crate::BitReader;
#[doc = "Field `IWDG_SW` writer - IWDG Software Enable"]
pub type IwdgSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRST_STOP` reader - "]
pub type NrstStopR = crate::BitReader;
#[doc = "Field `NRST_STOP` writer - "]
pub type NrstStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRST_STDBY` reader - "]
pub type NrstStdbyR = crate::BitReader;
#[doc = "Field `NRST_STDBY` writer - "]
pub type NrstStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDG_SW` reader - WWDG_SW"]
pub type WwdgSwR = crate::BitReader;
#[doc = "Field `WWDG_SW` writer - WWDG_SW"]
pub type WwdgSwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Read Protection"]
    #[inline(always)]
    pub fn rdp(&self) -> RdpR {
        RdpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 9 - IWDG_STOP"]
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IwdgStopR {
        IwdgStopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IWDG_STDBY"]
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IwdgStdbyR {
        IwdgStdbyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bfb(&self) -> BfbR {
        BfbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IWDG Software Enable"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IwdgSwR {
        IwdgSwR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nrst_stop(&self) -> NrstStopR {
        NrstStopR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nrst_stdby(&self) -> NrstStdbyR {
        NrstStdbyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - WWDG_SW"]
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WwdgSwR {
        WwdgSwR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Protection"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RdpW<'_, Optr1Spec> {
        RdpW::new(self, 0)
    }
    #[doc = "Bit 9 - IWDG_STOP"]
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IwdgStopW<'_, Optr1Spec> {
        IwdgStopW::new(self, 9)
    }
    #[doc = "Bit 10 - IWDG_STDBY"]
    #[inline(always)]
    pub fn iwdg_stdby(&mut self) -> IwdgStdbyW<'_, Optr1Spec> {
        IwdgStdbyW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bfb(&mut self) -> BfbW<'_, Optr1Spec> {
        BfbW::new(self, 11)
    }
    #[doc = "Bit 12 - IWDG Software Enable"]
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IwdgSwW<'_, Optr1Spec> {
        IwdgSwW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nrst_stop(&mut self) -> NrstStopW<'_, Optr1Spec> {
        NrstStopW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nrst_stdby(&mut self) -> NrstStdbyW<'_, Optr1Spec> {
        NrstStdbyW::new(self, 14)
    }
    #[doc = "Bit 15 - WWDG_SW"]
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WwdgSwW<'_, Optr1Spec> {
        WwdgSwW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`optr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Optr1Spec;
impl crate::RegisterSpec for Optr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optr1::R`](R) reader structure"]
impl crate::Readable for Optr1Spec {}
#[doc = "`write(|w| ..)` method takes [`optr1::W`](W) writer structure"]
impl crate::Writable for Optr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPTR1 to value 0"]
impl crate::Resettable for Optr1Spec {}
