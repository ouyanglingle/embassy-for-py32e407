#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `EOP` reader - End of operation"]
pub type EopR = crate::BitReader;
#[doc = "Field `EOP` writer - End of operation"]
pub type EopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPERR` reader - Write protection error"]
pub type WrperrR = crate::BitReader;
#[doc = "Field `WRPERR` writer - Write protection error"]
pub type WrperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USRLOCK` reader - Userdata Lock"]
pub type UsrlockR = crate::BitReader;
#[doc = "Field `RDERR` reader - "]
pub type RderrR = crate::BitReader;
#[doc = "Field `RDERR` writer - "]
pub type RderrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTVERR` reader - Option and trimming bits loading validity error"]
pub type OptverrR = crate::BitReader;
#[doc = "Field `OPTVERR` writer - Option and trimming bits loading validity error"]
pub type OptverrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSY0` reader - Bank0 Busy"]
pub type Bsy0R = crate::BitReader;
#[doc = "Field `BSY1` reader - Bank1 Busy"]
pub type Bsy1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EopR {
        EopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    pub fn wrperr(&self) -> WrperrR {
        WrperrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 13 - Userdata Lock"]
    #[inline(always)]
    pub fn usrlock(&self) -> UsrlockR {
        UsrlockR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rderr(&self) -> RderrR {
        RderrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Option and trimming bits loading validity error"]
    #[inline(always)]
    pub fn optverr(&self) -> OptverrR {
        OptverrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Bank0 Busy"]
    #[inline(always)]
    pub fn bsy0(&self) -> Bsy0R {
        Bsy0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank1 Busy"]
    #[inline(always)]
    pub fn bsy1(&self) -> Bsy1R {
        Bsy1R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    pub fn eop(&mut self) -> EopW<'_, SrSpec> {
        EopW::new(self, 0)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    pub fn wrperr(&mut self) -> WrperrW<'_, SrSpec> {
        WrperrW::new(self, 4)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rderr(&mut self) -> RderrW<'_, SrSpec> {
        RderrW::new(self, 14)
    }
    #[doc = "Bit 15 - Option and trimming bits loading validity error"]
    #[inline(always)]
    pub fn optverr(&mut self) -> OptverrW<'_, SrSpec> {
        OptverrW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
