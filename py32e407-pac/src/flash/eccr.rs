#[doc = "Register `ECCR` reader"]
pub type R = crate::R<EccrSpec>;
#[doc = "Register `ECCR` writer"]
pub type W = crate::W<EccrSpec>;
#[doc = "Field `ADDR_ECC` reader - "]
pub type AddrEccR = crate::FieldReader<u32>;
#[doc = "Field `BK_ECC` reader - "]
pub type BkEccR = crate::BitReader;
#[doc = "Field `SYSF_ECC` reader - "]
pub type SysfEccR = crate::BitReader;
#[doc = "Field `ECCCIE` reader - "]
pub type EcccieR = crate::BitReader;
#[doc = "Field `ECCCIE` writer - "]
pub type EcccieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCC` reader - "]
pub type EcccR = crate::BitReader;
#[doc = "Field `ECCC` writer - "]
pub type EcccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCD` reader - "]
pub type EccdR = crate::BitReader;
#[doc = "Field `ECCD` writer - "]
pub type EccdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn addr_ecc(&self) -> AddrEccR {
        AddrEccR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn bk_ecc(&self) -> BkEccR {
        BkEccR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SysfEccR {
        SysfEccR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ecccie(&self) -> EcccieR {
        EcccieR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn eccc(&self) -> EcccR {
        EcccR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn eccd(&self) -> EccdR {
        EccdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ecccie(&mut self) -> EcccieW<'_, EccrSpec> {
        EcccieW::new(self, 24)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn eccc(&mut self) -> EcccW<'_, EccrSpec> {
        EcccW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn eccd(&mut self) -> EccdW<'_, EccrSpec> {
        EccdW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccrSpec;
impl crate::RegisterSpec for EccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccr::R`](R) reader structure"]
impl crate::Readable for EccrSpec {}
#[doc = "`write(|w| ..)` method takes [`eccr::W`](W) writer structure"]
impl crate::Writable for EccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECCR to value 0"]
impl crate::Resettable for EccrSpec {}
