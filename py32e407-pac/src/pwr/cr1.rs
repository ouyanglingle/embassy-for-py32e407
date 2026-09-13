#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `LPMS` reader - "]
pub type LpmsR = crate::FieldReader;
#[doc = "Field `LPMS` writer - "]
pub type LpmsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DBP` reader - "]
pub type DbpR = crate::BitReader;
#[doc = "Field `DBP` writer - "]
pub type DbpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOS` reader - "]
pub type VosR = crate::FieldReader;
#[doc = "Field `VOS` writer - "]
pub type VosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BKPVR_VOS` reader - "]
pub type BkpvrVosR = crate::FieldReader;
#[doc = "Field `BKPVR_VOS` writer - "]
pub type BkpvrVosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SRAM_RETV` reader - SRAM_RETV"]
pub type SramRetvR = crate::FieldReader;
#[doc = "Field `SRAM_RETV` writer - SRAM_RETV"]
pub type SramRetvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPR` reader - "]
pub type LprR = crate::BitReader;
#[doc = "Field `LPR` writer - "]
pub type LprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSION_CTRL` reader - "]
pub type HsionCtrlR = crate::BitReader;
#[doc = "Field `HSION_CTRL` writer - "]
pub type HsionCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLS_WUPT` reader - "]
pub type FlsWuptR = crate::FieldReader;
#[doc = "Field `FLS_WUPT` writer - "]
pub type FlsWuptW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STDBY_MRRDY_WAIT` reader - "]
pub type StdbyMrrdyWaitR = crate::FieldReader;
#[doc = "Field `STDBY_MRRDY_WAIT` writer - "]
pub type StdbyMrrdyWaitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lpms(&self) -> LpmsR {
        LpmsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dbp(&self) -> DbpR {
        DbpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn vos(&self) -> VosR {
        VosR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn bkpvr_vos(&self) -> BkpvrVosR {
        BkpvrVosR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 11:12 - SRAM_RETV"]
    #[inline(always)]
    pub fn sram_retv(&self) -> SramRetvR {
        SramRetvR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn lpr(&self) -> LprR {
        LprR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn hsion_ctrl(&self) -> HsionCtrlR {
        HsionCtrlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn fls_wupt(&self) -> FlsWuptR {
        FlsWuptR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn stdby_mrrdy_wait(&self) -> StdbyMrrdyWaitR {
        StdbyMrrdyWaitR::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lpms(&mut self) -> LpmsW<'_, Cr1Spec> {
        LpmsW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dbp(&mut self) -> DbpW<'_, Cr1Spec> {
        DbpW::new(self, 8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn vos(&mut self) -> VosW<'_, Cr1Spec> {
        VosW::new(self, 9)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn bkpvr_vos(&mut self) -> BkpvrVosW<'_, Cr1Spec> {
        BkpvrVosW::new(self, 11)
    }
    #[doc = "Bits 11:12 - SRAM_RETV"]
    #[inline(always)]
    pub fn sram_retv(&mut self) -> SramRetvW<'_, Cr1Spec> {
        SramRetvW::new(self, 11)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn lpr(&mut self) -> LprW<'_, Cr1Spec> {
        LprW::new(self, 14)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn hsion_ctrl(&mut self) -> HsionCtrlW<'_, Cr1Spec> {
        HsionCtrlW::new(self, 16)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn fls_wupt(&mut self) -> FlsWuptW<'_, Cr1Spec> {
        FlsWuptW::new(self, 17)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn stdby_mrrdy_wait(&mut self) -> StdbyMrrdyWaitW<'_, Cr1Spec> {
        StdbyMrrdyWaitW::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
