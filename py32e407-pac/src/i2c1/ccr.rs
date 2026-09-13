#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Field `CCR` reader - desc CCR"]
pub type CcrR = crate::FieldReader<u16>;
#[doc = "Field `CCR` writer - desc CCR"]
pub type CcrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `FP` reader - FP"]
pub type FpR = crate::BitReader;
#[doc = "Field `FP` writer - FP"]
pub type FpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY` reader - desc DUTY"]
pub type DutyR = crate::BitReader;
#[doc = "Field `DUTY` writer - desc DUTY"]
pub type DutyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FS` reader - desc FS"]
pub type FsR = crate::BitReader;
#[doc = "Field `FS` writer - desc FS"]
pub type FsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - desc CCR"]
    #[inline(always)]
    pub fn ccr(&self) -> CcrR {
        CcrR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 13 - FP"]
    #[inline(always)]
    pub fn fp(&self) -> FpR {
        FpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc DUTY"]
    #[inline(always)]
    pub fn duty(&self) -> DutyR {
        DutyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc FS"]
    #[inline(always)]
    pub fn fs(&self) -> FsR {
        FsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc CCR"]
    #[inline(always)]
    pub fn ccr(&mut self) -> CcrW<'_, CcrSpec> {
        CcrW::new(self, 0)
    }
    #[doc = "Bit 13 - FP"]
    #[inline(always)]
    pub fn fp(&mut self) -> FpW<'_, CcrSpec> {
        FpW::new(self, 13)
    }
    #[doc = "Bit 14 - desc DUTY"]
    #[inline(always)]
    pub fn duty(&mut self) -> DutyW<'_, CcrSpec> {
        DutyW::new(self, 14)
    }
    #[doc = "Bit 15 - desc FS"]
    #[inline(always)]
    pub fn fs(&mut self) -> FsW<'_, CcrSpec> {
        FsW::new(self, 15)
    }
}
#[doc = "desc CCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {}
