#[doc = "Register `DMAMFBOCR` reader"]
pub type R = crate::R<DmamfbocrSpec>;
#[doc = "Register `DMAMFBOCR` writer"]
pub type W = crate::W<DmamfbocrSpec>;
#[doc = "Field `MFC` reader - Missed frames by the controller"]
pub type MfcR = crate::FieldReader<u16>;
#[doc = "Field `OMFC` reader - Overflow bit for missed frame counter"]
pub type OmfcR = crate::BitReader;
#[doc = "Field `MFA` reader - Missed frames by the application"]
pub type MfaR = crate::FieldReader<u16>;
#[doc = "Field `OFOC` reader - Overflow bit for FIFO overflow counter"]
pub type OfocR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Missed frames by the controller"]
    #[inline(always)]
    pub fn mfc(&self) -> MfcR {
        MfcR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow bit for missed frame counter"]
    #[inline(always)]
    pub fn omfc(&self) -> OmfcR {
        OmfcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - Missed frames by the application"]
    #[inline(always)]
    pub fn mfa(&self) -> MfaR {
        MfaR::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow bit for FIFO overflow counter"]
    #[inline(always)]
    pub fn ofoc(&self) -> OfocR {
        OfocR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {}
#[doc = "Ethernet DMA missed frame and buffer overflow counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmamfbocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamfbocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmamfbocrSpec;
impl crate::RegisterSpec for DmamfbocrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamfbocr::R`](R) reader structure"]
impl crate::Readable for DmamfbocrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmamfbocr::W`](W) writer structure"]
impl crate::Writable for DmamfbocrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAMFBOCR to value 0"]
impl crate::Resettable for DmamfbocrSpec {}
