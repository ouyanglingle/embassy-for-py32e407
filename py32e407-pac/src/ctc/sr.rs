#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `CKOKIF` reader - Clock calibration successful interrupt flag bit"]
pub type CkokifR = crate::BitReader;
#[doc = "Field `CKWARNIF` reader - Clock calibration warning interrupt flag bit"]
pub type CkwarnifR = crate::BitReader;
#[doc = "Field `ERRIF` reader - Error interrupt flag bit"]
pub type ErrifR = crate::BitReader;
#[doc = "Field `EREFIF` reader - Expected reference interrupt flag bit"]
pub type ErefifR = crate::BitReader;
#[doc = "Field `CKERR` reader - Clock calibration error bit"]
pub type CkerrR = crate::BitReader;
#[doc = "Field `REFMISS` reader - Loss of synchronous reference pulse signal"]
pub type RefmissR = crate::BitReader;
#[doc = "Field `TRIMERR` reader - Calibration value error bit"]
pub type TrimerrR = crate::BitReader;
#[doc = "Field `REFDIR` reader - CTC calibration clock counting direction"]
pub type RefdirR = crate::BitReader;
#[doc = "Field `REFCAP` reader - CTC counter capture value"]
pub type RefcapR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Clock calibration successful interrupt flag bit"]
    #[inline(always)]
    pub fn ckokif(&self) -> CkokifR {
        CkokifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock calibration warning interrupt flag bit"]
    #[inline(always)]
    pub fn ckwarnif(&self) -> CkwarnifR {
        CkwarnifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt flag bit"]
    #[inline(always)]
    pub fn errif(&self) -> ErrifR {
        ErrifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected reference interrupt flag bit"]
    #[inline(always)]
    pub fn erefif(&self) -> ErefifR {
        ErefifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock calibration error bit"]
    #[inline(always)]
    pub fn ckerr(&self) -> CkerrR {
        CkerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Loss of synchronous reference pulse signal"]
    #[inline(always)]
    pub fn refmiss(&self) -> RefmissR {
        RefmissR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Calibration value error bit"]
    #[inline(always)]
    pub fn trimerr(&self) -> TrimerrR {
        TrimerrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - CTC calibration clock counting direction"]
    #[inline(always)]
    pub fn refdir(&self) -> RefdirR {
        RefdirR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - CTC counter capture value"]
    #[inline(always)]
    pub fn refcap(&self) -> RefcapR {
        RefcapR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
