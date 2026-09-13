#[doc = "Register `ECR` reader"]
pub type R = crate::R<EcrSpec>;
#[doc = "Register `ECR` writer"]
pub type W = crate::W<EcrSpec>;
#[doc = "Field `IE` reader - "]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - "]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDIR` reader - "]
pub type IdirR = crate::BitReader;
#[doc = "Field `IDIR` writer - "]
pub type IdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIDX` reader - "]
pub type FidxR = crate::BitReader;
#[doc = "Field `FIDX` writer - "]
pub type FidxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPOS` reader - "]
pub type IposR = crate::FieldReader;
#[doc = "Field `IPOS` writer - "]
pub type IposW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PW` reader - "]
pub type PwR = crate::FieldReader;
#[doc = "Field `PW` writer - "]
pub type PwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWPRSC` reader - "]
pub type PwprscR = crate::FieldReader;
#[doc = "Field `PWPRSC` writer - "]
pub type PwprscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn idir(&self) -> IdirR {
        IdirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fidx(&self) -> FidxR {
        FidxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn ipos(&self) -> IposR {
        IposR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pw(&self) -> PwR {
        PwR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pwprsc(&self) -> PwprscR {
        PwprscR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, EcrSpec> {
        IeW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn idir(&mut self) -> IdirW<'_, EcrSpec> {
        IdirW::new(self, 1)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fidx(&mut self) -> FidxW<'_, EcrSpec> {
        FidxW::new(self, 5)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn ipos(&mut self) -> IposW<'_, EcrSpec> {
        IposW::new(self, 6)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pw(&mut self) -> PwW<'_, EcrSpec> {
        PwW::new(self, 16)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pwprsc(&mut self) -> PwprscW<'_, EcrSpec> {
        PwprscW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcrSpec;
impl crate::RegisterSpec for EcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecr::R`](R) reader structure"]
impl crate::Readable for EcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ecr::W`](W) writer structure"]
impl crate::Writable for EcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for EcrSpec {}
