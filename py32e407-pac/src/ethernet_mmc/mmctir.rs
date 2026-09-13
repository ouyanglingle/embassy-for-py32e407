#[doc = "Register `MMCTIR` reader"]
pub type R = crate::R<MmctirSpec>;
#[doc = "Register `MMCTIR` writer"]
pub type W = crate::W<MmctirSpec>;
#[doc = "Field `TGFSCS` reader - Transmitted good frames single collision status"]
pub type TgfscsR = crate::BitReader;
#[doc = "Field `TGFSCS` writer - Transmitted good frames single collision status"]
pub type TgfscsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGFMSCS` reader - Transmitted good frames more single collision status"]
pub type TgfmscsR = crate::BitReader;
#[doc = "Field `TGFMSCS` writer - Transmitted good frames more single collision status"]
pub type TgfmscsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGFS` reader - Transmitted good frames status"]
pub type TgfsR = crate::BitReader;
#[doc = "Field `TGFS` writer - Transmitted good frames status"]
pub type TgfsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision status"]
    #[inline(always)]
    pub fn tgfscs(&self) -> TgfscsR {
        TgfscsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more single collision status"]
    #[inline(always)]
    pub fn tgfmscs(&self) -> TgfmscsR {
        TgfmscsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames status"]
    #[inline(always)]
    pub fn tgfs(&self) -> TgfsR {
        TgfsR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Transmitted good frames single collision status"]
    #[inline(always)]
    pub fn tgfscs(&mut self) -> TgfscsW<'_, MmctirSpec> {
        TgfscsW::new(self, 14)
    }
    #[doc = "Bit 15 - Transmitted good frames more single collision status"]
    #[inline(always)]
    pub fn tgfmscs(&mut self) -> TgfmscsW<'_, MmctirSpec> {
        TgfmscsW::new(self, 15)
    }
    #[doc = "Bit 21 - Transmitted good frames status"]
    #[inline(always)]
    pub fn tgfs(&mut self) -> TgfsW<'_, MmctirSpec> {
        TgfsW::new(self, 21)
    }
}
#[doc = "Ethernet MMC transmit interrupt register (ETH_MMCTIR)\n\nYou can [`read`](crate::Reg::read) this register and get [`mmctir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmctir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmctirSpec;
impl crate::RegisterSpec for MmctirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctir::R`](R) reader structure"]
impl crate::Readable for MmctirSpec {}
#[doc = "`write(|w| ..)` method takes [`mmctir::W`](W) writer structure"]
impl crate::Writable for MmctirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMCTIR to value 0"]
impl crate::Resettable for MmctirSpec {}
