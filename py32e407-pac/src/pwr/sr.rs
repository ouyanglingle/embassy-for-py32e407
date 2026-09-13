#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `WUF1` reader - "]
pub type Wuf1R = crate::BitReader;
#[doc = "Field `WUF2` reader - "]
pub type Wuf2R = crate::BitReader;
#[doc = "Field `WUF3` reader - "]
pub type Wuf3R = crate::BitReader;
#[doc = "Field `WUF4` reader - "]
pub type Wuf4R = crate::BitReader;
#[doc = "Field `WUF5` reader - "]
pub type Wuf5R = crate::BitReader;
#[doc = "Field `SBF` reader - "]
pub type SbfR = crate::BitReader;
#[doc = "Field `MR_RDY` reader - MR_RDY"]
pub type MrRdyR = crate::BitReader;
#[doc = "Field `MR_RDY` writer - MR_RDY"]
pub type MrRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDO` reader - "]
pub type PvdoR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wuf1(&self) -> Wuf1R {
        Wuf1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wuf2(&self) -> Wuf2R {
        Wuf2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wuf3(&self) -> Wuf3R {
        Wuf3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wuf4(&self) -> Wuf4R {
        Wuf4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wuf5(&self) -> Wuf5R {
        Wuf5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sbf(&self) -> SbfR {
        SbfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MR_RDY"]
    #[inline(always)]
    pub fn mr_rdy(&self) -> MrRdyR {
        MrRdyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pvdo(&self) -> PvdoR {
        PvdoR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - MR_RDY"]
    #[inline(always)]
    pub fn mr_rdy(&mut self) -> MrRdyW<'_, SrSpec> {
        MrRdyW::new(self, 9)
    }
}
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
