#[doc = "Register `MACA1HR` reader"]
pub type R = crate::R<Maca1hrSpec>;
#[doc = "Register `MACA1HR` writer"]
pub type W = crate::W<Maca1hrSpec>;
#[doc = "Field `MACA1H` reader - MAC address1 high"]
pub type Maca1hR = crate::FieldReader<u16>;
#[doc = "Field `MACA1H` writer - MAC address1 high"]
pub type Maca1hW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC` reader - Mask byte control"]
pub type MbcR = crate::FieldReader;
#[doc = "Field `MBC` writer - Mask byte control"]
pub type MbcW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA` reader - Source address"]
pub type SaR = crate::BitReader;
#[doc = "Field `SA` writer - Source address"]
pub type SaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE` reader - Address enable"]
pub type AeR = crate::BitReader;
#[doc = "Field `AE` writer - Address enable"]
pub type AeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MAC address1 high"]
    #[inline(always)]
    pub fn maca1h(&self) -> Maca1hR {
        Maca1hR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    pub fn mbc(&self) -> MbcR {
        MbcR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    pub fn ae(&self) -> AeR {
        AeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address1 high"]
    #[inline(always)]
    pub fn maca1h(&mut self) -> Maca1hW<'_, Maca1hrSpec> {
        Maca1hW::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    pub fn mbc(&mut self) -> MbcW<'_, Maca1hrSpec> {
        MbcW::new(self, 24)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    pub fn sa(&mut self) -> SaW<'_, Maca1hrSpec> {
        SaW::new(self, 30)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    pub fn ae(&mut self) -> AeW<'_, Maca1hrSpec> {
        AeW::new(self, 31)
    }
}
#[doc = "Ethernet MAC address 1 high register (ETH_MACA1HR)\n\nYou can [`read`](crate::Reg::read) this register and get [`maca1hr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1hr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maca1hrSpec;
impl crate::RegisterSpec for Maca1hrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca1hr::R`](R) reader structure"]
impl crate::Readable for Maca1hrSpec {}
#[doc = "`write(|w| ..)` method takes [`maca1hr::W`](W) writer structure"]
impl crate::Writable for Maca1hrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACA1HR to value 0"]
impl crate::Resettable for Maca1hrSpec {}
