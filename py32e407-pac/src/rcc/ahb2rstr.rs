#[doc = "Register `AHB2RSTR` reader"]
pub type R = crate::R<Ahb2rstrSpec>;
#[doc = "Register `AHB2RSTR` writer"]
pub type W = crate::W<Ahb2rstrSpec>;
#[doc = "Field `IOPARST` reader - "]
pub type IoparstR = crate::BitReader;
#[doc = "Field `IOPARST` writer - "]
pub type IoparstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPBRST` reader - "]
pub type IopbrstR = crate::BitReader;
#[doc = "Field `IOPBRST` writer - "]
pub type IopbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPCRST` reader - "]
pub type IopcrstR = crate::BitReader;
#[doc = "Field `IOPCRST` writer - "]
pub type IopcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPDRST` reader - "]
pub type IopdrstR = crate::BitReader;
#[doc = "Field `IOPDRST` writer - "]
pub type IopdrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPERST` reader - "]
pub type IoperstR = crate::BitReader;
#[doc = "Field `IOPERST` writer - "]
pub type IoperstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPFRST` reader - "]
pub type IopfrstR = crate::BitReader;
#[doc = "Field `IOPFRST` writer - "]
pub type IopfrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIORST` reader - "]
pub type SdiorstR = crate::BitReader;
#[doc = "Field `SDIORST` writer - "]
pub type SdiorstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESRST` reader - "]
pub type AesrstR = crate::BitReader;
#[doc = "Field `AESRST` writer - "]
pub type AesrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB1RST` reader - "]
pub type Usb1rstR = crate::BitReader;
#[doc = "Field `USB1RST` writer - "]
pub type Usb1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB2RST` reader - "]
pub type Usb2rstR = crate::BitReader;
#[doc = "Field `USB2RST` writer - "]
pub type Usb2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHRST` reader - "]
pub type EthrstR = crate::BitReader;
#[doc = "Field `ETHRST` writer - "]
pub type EthrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ioparst(&self) -> IoparstR {
        IoparstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn iopbrst(&self) -> IopbrstR {
        IopbrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn iopcrst(&self) -> IopcrstR {
        IopcrstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn iopdrst(&self) -> IopdrstR {
        IopdrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ioperst(&self) -> IoperstR {
        IoperstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn iopfrst(&self) -> IopfrstR {
        IopfrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sdiorst(&self) -> SdiorstR {
        SdiorstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn aesrst(&self) -> AesrstR {
        AesrstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn usb1rst(&self) -> Usb1rstR {
        Usb1rstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn usb2rst(&self) -> Usb2rstR {
        Usb2rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ethrst(&self) -> EthrstR {
        EthrstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ioparst(&mut self) -> IoparstW<'_, Ahb2rstrSpec> {
        IoparstW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn iopbrst(&mut self) -> IopbrstW<'_, Ahb2rstrSpec> {
        IopbrstW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn iopcrst(&mut self) -> IopcrstW<'_, Ahb2rstrSpec> {
        IopcrstW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn iopdrst(&mut self) -> IopdrstW<'_, Ahb2rstrSpec> {
        IopdrstW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ioperst(&mut self) -> IoperstW<'_, Ahb2rstrSpec> {
        IoperstW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn iopfrst(&mut self) -> IopfrstW<'_, Ahb2rstrSpec> {
        IopfrstW::new(self, 7)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sdiorst(&mut self) -> SdiorstW<'_, Ahb2rstrSpec> {
        SdiorstW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn aesrst(&mut self) -> AesrstW<'_, Ahb2rstrSpec> {
        AesrstW::new(self, 10)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn usb1rst(&mut self) -> Usb1rstW<'_, Ahb2rstrSpec> {
        Usb1rstW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn usb2rst(&mut self) -> Usb2rstW<'_, Ahb2rstrSpec> {
        Usb2rstW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ethrst(&mut self) -> EthrstW<'_, Ahb2rstrSpec> {
        EthrstW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2rstrSpec;
impl crate::RegisterSpec for Ahb2rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2rstr::R`](R) reader structure"]
impl crate::Readable for Ahb2rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure"]
impl crate::Writable for Ahb2rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for Ahb2rstrSpec {}
