#[doc = "Register `SOCR` reader"]
pub type R = crate::R<SocrSpec>;
#[doc = "Register `SOCR` writer"]
pub type W = crate::W<SocrSpec>;
#[doc = "Field `SPIMODE` reader - spi mode"]
pub type SpimodeR = crate::FieldReader;
#[doc = "Field `SPIMODE` writer - spi mode"]
pub type SpimodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SENM` reader - send mode enable"]
pub type SenmR = crate::BitReader;
#[doc = "Field `SENM` writer - send mode enable"]
pub type SenmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTICMD` reader - muti command"]
pub type MulticmdR = crate::BitReader;
#[doc = "Field `MULTICMD` writer - muti command"]
pub type MulticmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTIADDR` reader - muti address"]
pub type MultiaddrR = crate::BitReader;
#[doc = "Field `MULTIADDR` writer - muti address"]
pub type MultiaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRCMD` reader - ddr command"]
pub type DdrcmdR = crate::BitReader;
#[doc = "Field `DDRCMD` writer - ddr command"]
pub type DdrcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRADDR` reader - ddr address"]
pub type DdraddrR = crate::BitReader;
#[doc = "Field `DDRADDR` writer - ddr address"]
pub type DdraddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRDATA` reader - ddr data"]
pub type DdrdataR = crate::BitReader;
#[doc = "Field `DDRDATA` writer - ddr data"]
pub type DdrdataW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - spi mode"]
    #[inline(always)]
    pub fn spimode(&self) -> SpimodeR {
        SpimodeR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - send mode enable"]
    #[inline(always)]
    pub fn senm(&self) -> SenmR {
        SenmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - muti command"]
    #[inline(always)]
    pub fn multicmd(&self) -> MulticmdR {
        MulticmdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - muti address"]
    #[inline(always)]
    pub fn multiaddr(&self) -> MultiaddrR {
        MultiaddrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ddr command"]
    #[inline(always)]
    pub fn ddrcmd(&self) -> DdrcmdR {
        DdrcmdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ddr address"]
    #[inline(always)]
    pub fn ddraddr(&self) -> DdraddrR {
        DdraddrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ddr data"]
    #[inline(always)]
    pub fn ddrdata(&self) -> DdrdataR {
        DdrdataR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - spi mode"]
    #[inline(always)]
    pub fn spimode(&mut self) -> SpimodeW<'_, SocrSpec> {
        SpimodeW::new(self, 0)
    }
    #[doc = "Bit 2 - send mode enable"]
    #[inline(always)]
    pub fn senm(&mut self) -> SenmW<'_, SocrSpec> {
        SenmW::new(self, 2)
    }
    #[doc = "Bit 3 - muti command"]
    #[inline(always)]
    pub fn multicmd(&mut self) -> MulticmdW<'_, SocrSpec> {
        MulticmdW::new(self, 3)
    }
    #[doc = "Bit 4 - muti address"]
    #[inline(always)]
    pub fn multiaddr(&mut self) -> MultiaddrW<'_, SocrSpec> {
        MultiaddrW::new(self, 4)
    }
    #[doc = "Bit 5 - ddr command"]
    #[inline(always)]
    pub fn ddrcmd(&mut self) -> DdrcmdW<'_, SocrSpec> {
        DdrcmdW::new(self, 5)
    }
    #[doc = "Bit 6 - ddr address"]
    #[inline(always)]
    pub fn ddraddr(&mut self) -> DdraddrW<'_, SocrSpec> {
        DdraddrW::new(self, 6)
    }
    #[doc = "Bit 7 - ddr data"]
    #[inline(always)]
    pub fn ddrdata(&mut self) -> DdrdataW<'_, SocrSpec> {
        DdrdataW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`socr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`socr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocrSpec;
impl crate::RegisterSpec for SocrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`socr::R`](R) reader structure"]
impl crate::Readable for SocrSpec {}
#[doc = "`write(|w| ..)` method takes [`socr::W`](W) writer structure"]
impl crate::Writable for SocrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOCR to value 0"]
impl crate::Resettable for SocrSpec {}
