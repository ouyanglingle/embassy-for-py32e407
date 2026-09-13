#[doc = "Register `MMCTIMR` reader"]
pub type R = crate::R<MmctimrSpec>;
#[doc = "Register `MMCTIMR` writer"]
pub type W = crate::W<MmctimrSpec>;
#[doc = "Field `TGFSCM` reader - Transmitted good frames single collision mask"]
pub type TgfscmR = crate::BitReader;
#[doc = "Field `TGFSCM` writer - Transmitted good frames single collision mask"]
pub type TgfscmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGFMSCM` reader - Transmitted good frames more single collision mask"]
pub type TgfmscmR = crate::BitReader;
#[doc = "Field `TGFMSCM` writer - Transmitted good frames more single collision mask"]
pub type TgfmscmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGFM` reader - Transmitted good frames mask"]
pub type TgfmR = crate::BitReader;
#[doc = "Field `TGFM` writer - Transmitted good frames mask"]
pub type TgfmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision mask"]
    #[inline(always)]
    pub fn tgfscm(&self) -> TgfscmR {
        TgfscmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more single collision mask"]
    #[inline(always)]
    pub fn tgfmscm(&self) -> TgfmscmR {
        TgfmscmR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames mask"]
    #[inline(always)]
    pub fn tgfm(&self) -> TgfmR {
        TgfmR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Transmitted good frames single collision mask"]
    #[inline(always)]
    pub fn tgfscm(&mut self) -> TgfscmW<'_, MmctimrSpec> {
        TgfscmW::new(self, 14)
    }
    #[doc = "Bit 15 - Transmitted good frames more single collision mask"]
    #[inline(always)]
    pub fn tgfmscm(&mut self) -> TgfmscmW<'_, MmctimrSpec> {
        TgfmscmW::new(self, 15)
    }
    #[doc = "Bit 21 - Transmitted good frames mask"]
    #[inline(always)]
    pub fn tgfm(&mut self) -> TgfmW<'_, MmctimrSpec> {
        TgfmW::new(self, 21)
    }
}
#[doc = "Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR)\n\nYou can [`read`](crate::Reg::read) this register and get [`mmctimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmctimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmctimrSpec;
impl crate::RegisterSpec for MmctimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctimr::R`](R) reader structure"]
impl crate::Readable for MmctimrSpec {}
#[doc = "`write(|w| ..)` method takes [`mmctimr::W`](W) writer structure"]
impl crate::Writable for MmctimrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMCTIMR to value 0"]
impl crate::Resettable for MmctimrSpec {}
