#[doc = "Register `DMABMR` reader"]
pub type R = crate::R<DmabmrSpec>;
#[doc = "Register `DMABMR` writer"]
pub type W = crate::W<DmabmrSpec>;
#[doc = "Field `SR` reader - Software reset"]
pub type SrR = crate::BitReader;
#[doc = "Field `SR` writer - Software reset"]
pub type SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA` reader - DMA Arbitration"]
pub type DaR = crate::BitReader;
#[doc = "Field `DA` writer - DMA Arbitration"]
pub type DaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - Descriptor skip length"]
pub type DslR = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor skip length"]
pub type DslW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PBL` reader - Programmable burst length"]
pub type PblR = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable burst length"]
pub type PblW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RTPR` reader - Rx Tx priority ratio"]
pub type RtprR = crate::FieldReader;
#[doc = "Field `RTPR` writer - Rx Tx priority ratio"]
pub type RtprW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FB` reader - Fixed burst"]
pub type FbR = crate::BitReader;
#[doc = "Field `FB` writer - Fixed burst"]
pub type FbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP` reader - Rx DMA PBL"]
pub type RdpR = crate::FieldReader;
#[doc = "Field `RDP` writer - Rx DMA PBL"]
pub type RdpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `USP` reader - Use separate PBL"]
pub type UspR = crate::BitReader;
#[doc = "Field `USP` writer - Use separate PBL"]
pub type UspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPM` reader - 4xPBL mode"]
pub type FpmR = crate::BitReader;
#[doc = "Field `FPM` writer - 4xPBL mode"]
pub type FpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAB` reader - Address-aligned beats"]
pub type AabR = crate::BitReader;
#[doc = "Field `AAB` writer - Address-aligned beats"]
pub type AabW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        SrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dsl(&self) -> DslR {
        DslR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pbl(&self) -> PblR {
        PblR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Rx Tx priority ratio"]
    #[inline(always)]
    pub fn rtpr(&self) -> RtprR {
        RtprR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fb(&self) -> FbR {
        FbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rdp(&self) -> RdpR {
        RdpR::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    pub fn usp(&self) -> UspR {
        UspR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 4xPBL mode"]
    #[inline(always)]
    pub fn fpm(&self) -> FpmR {
        FpmR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    pub fn aab(&self) -> AabR {
        AabR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn sr(&mut self) -> SrW<'_, DmabmrSpec> {
        SrW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    pub fn da(&mut self) -> DaW<'_, DmabmrSpec> {
        DaW::new(self, 1)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dsl(&mut self) -> DslW<'_, DmabmrSpec> {
        DslW::new(self, 2)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pbl(&mut self) -> PblW<'_, DmabmrSpec> {
        PblW::new(self, 8)
    }
    #[doc = "Bits 14:15 - Rx Tx priority ratio"]
    #[inline(always)]
    pub fn rtpr(&mut self) -> RtprW<'_, DmabmrSpec> {
        RtprW::new(self, 14)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fb(&mut self) -> FbW<'_, DmabmrSpec> {
        FbW::new(self, 16)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RdpW<'_, DmabmrSpec> {
        RdpW::new(self, 17)
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    pub fn usp(&mut self) -> UspW<'_, DmabmrSpec> {
        UspW::new(self, 23)
    }
    #[doc = "Bit 24 - 4xPBL mode"]
    #[inline(always)]
    pub fn fpm(&mut self) -> FpmW<'_, DmabmrSpec> {
        FpmW::new(self, 24)
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    pub fn aab(&mut self) -> AabW<'_, DmabmrSpec> {
        AabW::new(self, 25)
    }
}
#[doc = "Ethernet DMA bus mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmabmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmabmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmabmrSpec;
impl crate::RegisterSpec for DmabmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmabmr::R`](R) reader structure"]
impl crate::Readable for DmabmrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmabmr::W`](W) writer structure"]
impl crate::Writable for DmabmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMABMR to value 0"]
impl crate::Resettable for DmabmrSpec {}
