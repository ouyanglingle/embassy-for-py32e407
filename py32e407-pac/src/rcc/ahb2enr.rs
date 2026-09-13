#[doc = "Register `AHB2ENR` reader"]
pub type R = crate::R<Ahb2enrSpec>;
#[doc = "Register `AHB2ENR` writer"]
pub type W = crate::W<Ahb2enrSpec>;
#[doc = "Field `IOPAEN` reader - "]
pub type IopaenR = crate::BitReader;
#[doc = "Field `IOPAEN` writer - "]
pub type IopaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPBEN` reader - "]
pub type IopbenR = crate::BitReader;
#[doc = "Field `IOPBEN` writer - "]
pub type IopbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPCEN` reader - "]
pub type IopcenR = crate::BitReader;
#[doc = "Field `IOPCEN` writer - "]
pub type IopcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPDEN` reader - "]
pub type IopdenR = crate::BitReader;
#[doc = "Field `IOPDEN` writer - "]
pub type IopdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPEEN` reader - "]
pub type IopeenR = crate::BitReader;
#[doc = "Field `IOPEEN` writer - "]
pub type IopeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPFEN` reader - "]
pub type IopfenR = crate::BitReader;
#[doc = "Field `IOPFEN` writer - "]
pub type IopfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOEN` reader - "]
pub type SdioenR = crate::BitReader;
#[doc = "Field `SDIOEN` writer - "]
pub type SdioenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESEN` reader - "]
pub type AesenR = crate::BitReader;
#[doc = "Field `AESEN` writer - "]
pub type AesenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB1EN` reader - "]
pub type Usb1enR = crate::BitReader;
#[doc = "Field `USB1EN` writer - "]
pub type Usb1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB2EN` reader - "]
pub type Usb2enR = crate::BitReader;
#[doc = "Field `USB2EN` writer - "]
pub type Usb2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHEN` reader - "]
pub type EthenR = crate::BitReader;
#[doc = "Field `ETHEN` writer - "]
pub type EthenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHTXEN` reader - ETHTXEN"]
pub type EthtxenR = crate::BitReader;
#[doc = "Field `ETHTXEN` writer - ETHTXEN"]
pub type EthtxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHRXEN` reader - ETHRXEN"]
pub type EthrxenR = crate::BitReader;
#[doc = "Field `ETHRXEN` writer - ETHRXEN"]
pub type EthrxenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn iopaen(&self) -> IopaenR {
        IopaenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn iopben(&self) -> IopbenR {
        IopbenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn iopcen(&self) -> IopcenR {
        IopcenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn iopden(&self) -> IopdenR {
        IopdenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn iopeen(&self) -> IopeenR {
        IopeenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn iopfen(&self) -> IopfenR {
        IopfenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sdioen(&self) -> SdioenR {
        SdioenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn aesen(&self) -> AesenR {
        AesenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn usb1en(&self) -> Usb1enR {
        Usb1enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn usb2en(&self) -> Usb2enR {
        Usb2enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ethen(&self) -> EthenR {
        EthenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ETHTXEN"]
    #[inline(always)]
    pub fn ethtxen(&self) -> EthtxenR {
        EthtxenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ETHRXEN"]
    #[inline(always)]
    pub fn ethrxen(&self) -> EthrxenR {
        EthrxenR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn iopaen(&mut self) -> IopaenW<'_, Ahb2enrSpec> {
        IopaenW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn iopben(&mut self) -> IopbenW<'_, Ahb2enrSpec> {
        IopbenW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn iopcen(&mut self) -> IopcenW<'_, Ahb2enrSpec> {
        IopcenW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn iopden(&mut self) -> IopdenW<'_, Ahb2enrSpec> {
        IopdenW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn iopeen(&mut self) -> IopeenW<'_, Ahb2enrSpec> {
        IopeenW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn iopfen(&mut self) -> IopfenW<'_, Ahb2enrSpec> {
        IopfenW::new(self, 7)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sdioen(&mut self) -> SdioenW<'_, Ahb2enrSpec> {
        SdioenW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn aesen(&mut self) -> AesenW<'_, Ahb2enrSpec> {
        AesenW::new(self, 10)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn usb1en(&mut self) -> Usb1enW<'_, Ahb2enrSpec> {
        Usb1enW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn usb2en(&mut self) -> Usb2enW<'_, Ahb2enrSpec> {
        Usb2enW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ethen(&mut self) -> EthenW<'_, Ahb2enrSpec> {
        EthenW::new(self, 15)
    }
    #[doc = "Bit 16 - ETHTXEN"]
    #[inline(always)]
    pub fn ethtxen(&mut self) -> EthtxenW<'_, Ahb2enrSpec> {
        EthtxenW::new(self, 16)
    }
    #[doc = "Bit 17 - ETHRXEN"]
    #[inline(always)]
    pub fn ethrxen(&mut self) -> EthrxenW<'_, Ahb2enrSpec> {
        EthrxenW::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2enrSpec;
impl crate::RegisterSpec for Ahb2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2enr::R`](R) reader structure"]
impl crate::Readable for Ahb2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure"]
impl crate::Writable for Ahb2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for Ahb2enrSpec {}
