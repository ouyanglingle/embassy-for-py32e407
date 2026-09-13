#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `FUNC` reader - Function"]
pub type FuncR = crate::FieldReader;
#[doc = "Field `FUNC` writer - Function"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRECISION` reader - Precision required (number of iterations)"]
pub type PrecisionR = crate::FieldReader;
#[doc = "Field `PRECISION` writer - Precision required (number of iterations)"]
pub type PrecisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCALE` reader - Scaling factor"]
pub type ScaleR = crate::FieldReader;
#[doc = "Field `SCALE` writer - Scaling factor"]
pub type ScaleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IEN` reader - Enable interrupt"]
pub type IenR = crate::BitReader;
#[doc = "Field `IEN` writer - Enable interrupt"]
pub type IenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREN` reader - Enable DMA read channel"]
pub type DmarenR = crate::BitReader;
#[doc = "Field `DMAREN` writer - Enable DMA read channel"]
pub type DmarenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAWEN` reader - Enable DMA write channel"]
pub type DmawenR = crate::BitReader;
#[doc = "Field `DMAWEN` writer - Enable DMA write channel"]
pub type DmawenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRES` reader - Number of results in the CORDIC_RDATA register"]
pub type NresR = crate::BitReader;
#[doc = "Field `NRES` writer - Number of results in the CORDIC_RDATA register"]
pub type NresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NARGS` reader - Number of arguments expected by the CORDIC_WDATA register"]
pub type NargsR = crate::BitReader;
#[doc = "Field `NARGS` writer - Number of arguments expected by the CORDIC_WDATA register"]
pub type NargsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESSIZE` reader - Width of input data"]
pub type RessizeR = crate::BitReader;
#[doc = "Field `RESSIZE` writer - Width of input data"]
pub type RessizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARGSIZE` reader - Width of input data"]
pub type ArgsizeR = crate::BitReader;
#[doc = "Field `ARGSIZE` writer - Width of input data"]
pub type ArgsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRDY` reader - Result ready flag"]
pub type RrdyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Function"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Precision required (number of iterations)"]
    #[inline(always)]
    pub fn precision(&self) -> PrecisionR {
        PrecisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Scaling factor"]
    #[inline(always)]
    pub fn scale(&self) -> ScaleR {
        ScaleR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Enable interrupt"]
    #[inline(always)]
    pub fn ien(&self) -> IenR {
        IenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable DMA read channel"]
    #[inline(always)]
    pub fn dmaren(&self) -> DmarenR {
        DmarenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable DMA write channel"]
    #[inline(always)]
    pub fn dmawen(&self) -> DmawenR {
        DmawenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Number of results in the CORDIC_RDATA register"]
    #[inline(always)]
    pub fn nres(&self) -> NresR {
        NresR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Number of arguments expected by the CORDIC_WDATA register"]
    #[inline(always)]
    pub fn nargs(&self) -> NargsR {
        NargsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Width of input data"]
    #[inline(always)]
    pub fn ressize(&self) -> RessizeR {
        RessizeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Width of input data"]
    #[inline(always)]
    pub fn argsize(&self) -> ArgsizeR {
        ArgsizeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - Result ready flag"]
    #[inline(always)]
    pub fn rrdy(&self) -> RrdyR {
        RrdyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, CsrSpec> {
        FuncW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Precision required (number of iterations)"]
    #[inline(always)]
    pub fn precision(&mut self) -> PrecisionW<'_, CsrSpec> {
        PrecisionW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Scaling factor"]
    #[inline(always)]
    pub fn scale(&mut self) -> ScaleW<'_, CsrSpec> {
        ScaleW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable interrupt"]
    #[inline(always)]
    pub fn ien(&mut self) -> IenW<'_, CsrSpec> {
        IenW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable DMA read channel"]
    #[inline(always)]
    pub fn dmaren(&mut self) -> DmarenW<'_, CsrSpec> {
        DmarenW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable DMA write channel"]
    #[inline(always)]
    pub fn dmawen(&mut self) -> DmawenW<'_, CsrSpec> {
        DmawenW::new(self, 18)
    }
    #[doc = "Bit 19 - Number of results in the CORDIC_RDATA register"]
    #[inline(always)]
    pub fn nres(&mut self) -> NresW<'_, CsrSpec> {
        NresW::new(self, 19)
    }
    #[doc = "Bit 20 - Number of arguments expected by the CORDIC_WDATA register"]
    #[inline(always)]
    pub fn nargs(&mut self) -> NargsW<'_, CsrSpec> {
        NargsW::new(self, 20)
    }
    #[doc = "Bit 21 - Width of input data"]
    #[inline(always)]
    pub fn ressize(&mut self) -> RessizeW<'_, CsrSpec> {
        RessizeW::new(self, 21)
    }
    #[doc = "Bit 22 - Width of input data"]
    #[inline(always)]
    pub fn argsize(&mut self) -> ArgsizeW<'_, CsrSpec> {
        ArgsizeW::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
