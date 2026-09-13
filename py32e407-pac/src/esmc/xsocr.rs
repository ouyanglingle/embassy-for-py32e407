#[doc = "Register `XSOCR` reader"]
pub type R = crate::R<XsocrSpec>;
#[doc = "Register `XSOCR` writer"]
pub type W = crate::W<XsocrSpec>;
#[doc = "Field `SPI_MODE` reader - xip read spi mode"]
pub type SpiModeR = crate::FieldReader;
#[doc = "Field `SPI_MODE` writer - xip read spi mode"]
pub type SpiModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SEND_M` reader - xip read send mode enable"]
pub type SendMR = crate::BitReader;
#[doc = "Field `SEND_M` writer - xip read send mode enable"]
pub type SendMW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTICMD` reader - xip read muti command"]
pub type MulticmdR = crate::BitReader;
#[doc = "Field `MULTICMD` writer - xip read muti command"]
pub type MulticmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTIADDR` reader - xip read muti address"]
pub type MultiaddrR = crate::BitReader;
#[doc = "Field `MULTIADDR` writer - xip read muti address"]
pub type MultiaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRCMD` reader - xip read ddr command"]
pub type DdrcmdR = crate::BitReader;
#[doc = "Field `DDRCMD` writer - xip read ddr command"]
pub type DdrcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRADDR` reader - xip read ddr address"]
pub type DdraddrR = crate::BitReader;
#[doc = "Field `DDRADDR` writer - xip read ddr address"]
pub type DdraddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRDATA` reader - xip read ddr data"]
pub type DdrdataR = crate::BitReader;
#[doc = "Field `DDRDATA` writer - xip read ddr data"]
pub type DdrdataW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - xip read spi mode"]
    #[inline(always)]
    pub fn spi_mode(&self) -> SpiModeR {
        SpiModeR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - xip read send mode enable"]
    #[inline(always)]
    pub fn send_m(&self) -> SendMR {
        SendMR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - xip read muti command"]
    #[inline(always)]
    pub fn multicmd(&self) -> MulticmdR {
        MulticmdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - xip read muti address"]
    #[inline(always)]
    pub fn multiaddr(&self) -> MultiaddrR {
        MultiaddrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - xip read ddr command"]
    #[inline(always)]
    pub fn ddrcmd(&self) -> DdrcmdR {
        DdrcmdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - xip read ddr address"]
    #[inline(always)]
    pub fn ddraddr(&self) -> DdraddrR {
        DdraddrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - xip read ddr data"]
    #[inline(always)]
    pub fn ddrdata(&self) -> DdrdataR {
        DdrdataR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - xip read spi mode"]
    #[inline(always)]
    pub fn spi_mode(&mut self) -> SpiModeW<'_, XsocrSpec> {
        SpiModeW::new(self, 0)
    }
    #[doc = "Bit 2 - xip read send mode enable"]
    #[inline(always)]
    pub fn send_m(&mut self) -> SendMW<'_, XsocrSpec> {
        SendMW::new(self, 2)
    }
    #[doc = "Bit 3 - xip read muti command"]
    #[inline(always)]
    pub fn multicmd(&mut self) -> MulticmdW<'_, XsocrSpec> {
        MulticmdW::new(self, 3)
    }
    #[doc = "Bit 4 - xip read muti address"]
    #[inline(always)]
    pub fn multiaddr(&mut self) -> MultiaddrW<'_, XsocrSpec> {
        MultiaddrW::new(self, 4)
    }
    #[doc = "Bit 5 - xip read ddr command"]
    #[inline(always)]
    pub fn ddrcmd(&mut self) -> DdrcmdW<'_, XsocrSpec> {
        DdrcmdW::new(self, 5)
    }
    #[doc = "Bit 6 - xip read ddr address"]
    #[inline(always)]
    pub fn ddraddr(&mut self) -> DdraddrW<'_, XsocrSpec> {
        DdraddrW::new(self, 6)
    }
    #[doc = "Bit 7 - xip read ddr data"]
    #[inline(always)]
    pub fn ddrdata(&mut self) -> DdrdataW<'_, XsocrSpec> {
        DdrdataW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xsocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xsocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XsocrSpec;
impl crate::RegisterSpec for XsocrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xsocr::R`](R) reader structure"]
impl crate::Readable for XsocrSpec {}
#[doc = "`write(|w| ..)` method takes [`xsocr::W`](W) writer structure"]
impl crate::Writable for XsocrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XSOCR to value 0"]
impl crate::Resettable for XsocrSpec {}
