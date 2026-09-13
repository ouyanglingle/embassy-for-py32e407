#[doc = "Register `FS_DCTL` reader"]
pub type R = crate::R<FsDctlSpec>;
#[doc = "Register `FS_DCTL` writer"]
pub type W = crate::W<FsDctlSpec>;
#[doc = "Field `RWUSIG` reader - Remote wakeup signaling"]
pub type RwusigR = crate::BitReader;
#[doc = "Field `RWUSIG` writer - Remote wakeup signaling"]
pub type RwusigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIS` reader - Soft disconnect"]
pub type SdisR = crate::BitReader;
#[doc = "Field `SDIS` writer - Soft disconnect"]
pub type SdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINSTS` reader - Global IN NAK status"]
pub type GinstsR = crate::BitReader;
#[doc = "Field `GONSTS` reader - Global OUT NAK status"]
pub type GonstsR = crate::BitReader;
#[doc = "Field `TCTL` reader - Test control"]
pub type TctlR = crate::FieldReader;
#[doc = "Field `TCTL` writer - Test control"]
pub type TctlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SGINAK` reader - Set global IN NAK"]
pub type SginakR = crate::BitReader;
#[doc = "Field `SGINAK` writer - Set global IN NAK"]
pub type SginakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGINAK` reader - Clear global IN NAK"]
pub type CginakR = crate::BitReader;
#[doc = "Field `CGINAK` writer - Clear global IN NAK"]
pub type CginakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGONAK` reader - Set global OUT NAK"]
pub type SgonakR = crate::BitReader;
#[doc = "Field `SGONAK` writer - Set global OUT NAK"]
pub type SgonakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGONAK` reader - Clear global OUT NAK"]
pub type CgonakR = crate::BitReader;
#[doc = "Field `CGONAK` writer - Clear global OUT NAK"]
pub type CgonakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POPRGDNE` reader - Power-on programming done"]
pub type PoprgdneR = crate::BitReader;
#[doc = "Field `POPRGDNE` writer - Power-on programming done"]
pub type PoprgdneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(&self) -> RwusigR {
        RwusigR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sdis(&self) -> SdisR {
        SdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global IN NAK status"]
    #[inline(always)]
    pub fn ginsts(&self) -> GinstsR {
        GinstsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK status"]
    #[inline(always)]
    pub fn gonsts(&self) -> GonstsR {
        GonstsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tctl(&self) -> TctlR {
        TctlR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    pub fn sginak(&self) -> SginakR {
        SginakR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    pub fn cginak(&self) -> CginakR {
        CginakR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    pub fn sgonak(&self) -> SgonakR {
        SgonakR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgonak(&self) -> CgonakR {
        CgonakR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(&self) -> PoprgdneR {
        PoprgdneR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(&mut self) -> RwusigW<'_, FsDctlSpec> {
        RwusigW::new(self, 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sdis(&mut self) -> SdisW<'_, FsDctlSpec> {
        SdisW::new(self, 1)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tctl(&mut self) -> TctlW<'_, FsDctlSpec> {
        TctlW::new(self, 4)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    pub fn sginak(&mut self) -> SginakW<'_, FsDctlSpec> {
        SginakW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    pub fn cginak(&mut self) -> CginakW<'_, FsDctlSpec> {
        CginakW::new(self, 8)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    pub fn sgonak(&mut self) -> SgonakW<'_, FsDctlSpec> {
        SgonakW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgonak(&mut self) -> CgonakW<'_, FsDctlSpec> {
        CgonakW::new(self, 10)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(&mut self) -> PoprgdneW<'_, FsDctlSpec> {
        PoprgdneW::new(self, 11)
    }
}
#[doc = "OTG_FS device control register (OTG_FS_DCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_dctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_dctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsDctlSpec;
impl crate::RegisterSpec for FsDctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_dctl::R`](R) reader structure"]
impl crate::Readable for FsDctlSpec {}
#[doc = "`write(|w| ..)` method takes [`fs_dctl::W`](W) writer structure"]
impl crate::Writable for FsDctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FS_DCTL to value 0"]
impl crate::Resettable for FsDctlSpec {}
