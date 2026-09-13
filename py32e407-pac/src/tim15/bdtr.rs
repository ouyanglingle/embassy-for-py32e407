#[doc = "Register `BDTR` reader"]
pub type R = crate::R<BdtrSpec>;
#[doc = "Register `BDTR` writer"]
pub type W = crate::W<BdtrSpec>;
#[doc = "Field `DTG` reader - "]
pub type DtgR = crate::FieldReader;
#[doc = "Field `DTG` writer - "]
pub type DtgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOCK` reader - "]
pub type LockR = crate::FieldReader;
#[doc = "Field `LOCK` writer - "]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSSI` reader - "]
pub type OssiR = crate::BitReader;
#[doc = "Field `OSSI` writer - "]
pub type OssiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSR` reader - "]
pub type OssrR = crate::BitReader;
#[doc = "Field `OSSR` writer - "]
pub type OssrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKE` reader - "]
pub type BkeR = crate::BitReader;
#[doc = "Field `BKE` writer - "]
pub type BkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKP` reader - "]
pub type BkpR = crate::BitReader;
#[doc = "Field `BKP` writer - "]
pub type BkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AOE` reader - "]
pub type AoeR = crate::BitReader;
#[doc = "Field `AOE` writer - "]
pub type AoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOE` reader - "]
pub type MoeR = crate::BitReader;
#[doc = "Field `MOE` writer - "]
pub type MoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKF` reader - "]
pub type BkfR = crate::FieldReader;
#[doc = "Field `BKF` writer - "]
pub type BkfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BKDSRM` reader - "]
pub type BkdsrmR = crate::BitReader;
#[doc = "Field `BKDSRM` writer - "]
pub type BkdsrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKBID` reader - "]
pub type BkbidR = crate::BitReader;
#[doc = "Field `BKBID` writer - "]
pub type BkbidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dtg(&self) -> DtgR {
        DtgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ossi(&self) -> OssiR {
        OssiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ossr(&self) -> OssrR {
        OssrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bke(&self) -> BkeR {
        BkeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn aoe(&self) -> AoeR {
        AoeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn moe(&self) -> MoeR {
        MoeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn bkf(&self) -> BkfR {
        BkfR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bkdsrm(&self) -> BkdsrmR {
        BkdsrmR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn bkbid(&self) -> BkbidR {
        BkbidR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dtg(&mut self) -> DtgW<'_, BdtrSpec> {
        DtgW::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, BdtrSpec> {
        LockW::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ossi(&mut self) -> OssiW<'_, BdtrSpec> {
        OssiW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ossr(&mut self) -> OssrW<'_, BdtrSpec> {
        OssrW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bke(&mut self) -> BkeW<'_, BdtrSpec> {
        BkeW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<'_, BdtrSpec> {
        BkpW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn aoe(&mut self) -> AoeW<'_, BdtrSpec> {
        AoeW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn moe(&mut self) -> MoeW<'_, BdtrSpec> {
        MoeW::new(self, 15)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn bkf(&mut self) -> BkfW<'_, BdtrSpec> {
        BkfW::new(self, 16)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bkdsrm(&mut self) -> BkdsrmW<'_, BdtrSpec> {
        BkdsrmW::new(self, 26)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn bkbid(&mut self) -> BkbidW<'_, BdtrSpec> {
        BkbidW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`bdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdtrSpec;
impl crate::RegisterSpec for BdtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdtr::R`](R) reader structure"]
impl crate::Readable for BdtrSpec {}
#[doc = "`write(|w| ..)` method takes [`bdtr::W`](W) writer structure"]
impl crate::Writable for BdtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BDTR to value 0"]
impl crate::Resettable for BdtrSpec {}
