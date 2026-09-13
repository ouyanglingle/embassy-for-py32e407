#[doc = "Register `MACA2HR` reader"]
pub type R = crate::R<Maca2hrSpec>;
#[doc = "Register `MACA2HR` writer"]
pub type W = crate::W<Maca2hrSpec>;
#[doc = "Field `ETH_MACA2HR` reader - Ethernet MAC address 2 high register"]
pub type EthMaca2hrR = crate::FieldReader<u16>;
#[doc = "Field `ETH_MACA2HR` writer - Ethernet MAC address 2 high register"]
pub type EthMaca2hrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    #[doc = "Bits 0:15 - Ethernet MAC address 2 high register"]
    #[inline(always)]
    pub fn eth_maca2hr(&self) -> EthMaca2hrR {
        EthMaca2hrR::new((self.bits & 0xffff) as u16)
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
    #[doc = "Bits 0:15 - Ethernet MAC address 2 high register"]
    #[inline(always)]
    pub fn eth_maca2hr(&mut self) -> EthMaca2hrW<'_, Maca2hrSpec> {
        EthMaca2hrW::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    pub fn mbc(&mut self) -> MbcW<'_, Maca2hrSpec> {
        MbcW::new(self, 24)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    pub fn sa(&mut self) -> SaW<'_, Maca2hrSpec> {
        SaW::new(self, 30)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    pub fn ae(&mut self) -> AeW<'_, Maca2hrSpec> {
        AeW::new(self, 31)
    }
}
#[doc = "Ethernet MAC address 2 high register (ETH_MACA2HR)\n\nYou can [`read`](crate::Reg::read) this register and get [`maca2hr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2hr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maca2hrSpec;
impl crate::RegisterSpec for Maca2hrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca2hr::R`](R) reader structure"]
impl crate::Readable for Maca2hrSpec {}
#[doc = "`write(|w| ..)` method takes [`maca2hr::W`](W) writer structure"]
impl crate::Writable for Maca2hrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACA2HR to value 0"]
impl crate::Resettable for Maca2hrSpec {}
