#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `LBDL` reader - LIN break detection length"]
pub type LbdlR = crate::BitReader;
#[doc = "Field `LBDL` writer - LIN break detection length"]
pub type LbdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDIE` reader - lin break enable"]
pub type LbdieR = crate::BitReader;
#[doc = "Field `LBDIE` writer - lin break enable"]
pub type LbdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBCL` reader - lbcl"]
pub type LbclR = crate::BitReader;
#[doc = "Field `LBCL` writer - lbcl"]
pub type LbclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - cpha"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - cpha"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - cpol"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - cpol"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN` reader - clk en"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - clk en"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - stop bit"]
pub type StopR = crate::FieldReader;
#[doc = "Field `STOP` writer - stop bit"]
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LINEN` reader - lin enable"]
pub type LinenR = crate::BitReader;
#[doc = "Field `LINEN` writer - lin enable"]
pub type LinenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD` reader - address"]
pub type AddR = crate::FieldReader;
#[doc = "Field `ADD` writer - address"]
pub type AddW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DEM` reader - dem"]
pub type DemR = crate::BitReader;
#[doc = "Field `DEM` writer - dem"]
pub type DemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEP` reader - dep"]
pub type DepR = crate::BitReader;
#[doc = "Field `DEP` writer - dep"]
pub type DepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOEN` reader - RTOEN"]
pub type RtoenR = crate::BitReader;
#[doc = "Field `RTOEN` writer - RTOEN"]
pub type RtoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    pub fn lbdl(&self) -> LbdlR {
        LbdlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - lin break enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LbdieR {
        LbdieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - lbcl"]
    #[inline(always)]
    pub fn lbcl(&self) -> LbclR {
        LbclR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - cpha"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - cpol"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - clk en"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - stop bit"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - lin enable"]
    #[inline(always)]
    pub fn linen(&self) -> LinenR {
        LinenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:22 - address"]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bit 23 - dem"]
    #[inline(always)]
    pub fn dem(&self) -> DemR {
        DemR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - dep"]
    #[inline(always)]
    pub fn dep(&self) -> DepR {
        DepR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RTOEN"]
    #[inline(always)]
    pub fn rtoen(&self) -> RtoenR {
        RtoenR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    pub fn lbdl(&mut self) -> LbdlW<'_, Cr2Spec> {
        LbdlW::new(self, 5)
    }
    #[doc = "Bit 6 - lin break enable"]
    #[inline(always)]
    pub fn lbdie(&mut self) -> LbdieW<'_, Cr2Spec> {
        LbdieW::new(self, 6)
    }
    #[doc = "Bit 8 - lbcl"]
    #[inline(always)]
    pub fn lbcl(&mut self) -> LbclW<'_, Cr2Spec> {
        LbclW::new(self, 8)
    }
    #[doc = "Bit 9 - cpha"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, Cr2Spec> {
        CphaW::new(self, 9)
    }
    #[doc = "Bit 10 - cpol"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, Cr2Spec> {
        CpolW::new(self, 10)
    }
    #[doc = "Bit 11 - clk en"]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<'_, Cr2Spec> {
        ClkenW::new(self, 11)
    }
    #[doc = "Bits 12:13 - stop bit"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, Cr2Spec> {
        StopW::new(self, 12)
    }
    #[doc = "Bit 14 - lin enable"]
    #[inline(always)]
    pub fn linen(&mut self) -> LinenW<'_, Cr2Spec> {
        LinenW::new(self, 14)
    }
    #[doc = "Bits 15:22 - address"]
    #[inline(always)]
    pub fn add(&mut self) -> AddW<'_, Cr2Spec> {
        AddW::new(self, 15)
    }
    #[doc = "Bit 23 - dem"]
    #[inline(always)]
    pub fn dem(&mut self) -> DemW<'_, Cr2Spec> {
        DemW::new(self, 23)
    }
    #[doc = "Bit 24 - dep"]
    #[inline(always)]
    pub fn dep(&mut self) -> DepW<'_, Cr2Spec> {
        DepW::new(self, 24)
    }
    #[doc = "Bit 25 - RTOEN"]
    #[inline(always)]
    pub fn rtoen(&mut self) -> RtoenW<'_, Cr2Spec> {
        RtoenW::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
