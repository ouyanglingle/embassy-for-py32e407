#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ADEN` reader - "]
pub type AdenR = crate::BitReader;
#[doc = "Field `ADEN` writer - "]
pub type AdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTART` reader - "]
pub type AdstartR = crate::BitReader;
#[doc = "Field `ADSTART` writer - "]
pub type AdstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADSTART` reader - "]
pub type JadstartR = crate::BitReader;
#[doc = "Field `JADSTART` writer - "]
pub type JadstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTP` reader - "]
pub type AdstpR = crate::BitReader;
#[doc = "Field `ADSTP` writer - "]
pub type AdstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTCAL` reader - "]
pub type RstcalR = crate::BitReader;
#[doc = "Field `RSTCAL` writer - "]
pub type RstcalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCAL` reader - "]
pub type AdcalR = crate::BitReader;
#[doc = "Field `ADCAL` writer - "]
pub type AdcalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aden(&self) -> AdenR {
        AdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adstart(&self) -> AdstartR {
        AdstartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn jadstart(&self) -> JadstartR {
        JadstartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adstp(&self) -> AdstpR {
        AdstpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rstcal(&self) -> RstcalR {
        RstcalR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adcal(&self) -> AdcalR {
        AdcalR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aden(&mut self) -> AdenW<'_, CrSpec> {
        AdenW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adstart(&mut self) -> AdstartW<'_, CrSpec> {
        AdstartW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn jadstart(&mut self) -> JadstartW<'_, CrSpec> {
        JadstartW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adstp(&mut self) -> AdstpW<'_, CrSpec> {
        AdstpW::new(self, 4)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rstcal(&mut self) -> RstcalW<'_, CrSpec> {
        RstcalW::new(self, 27)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adcal(&mut self) -> AdcalW<'_, CrSpec> {
        AdcalW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
