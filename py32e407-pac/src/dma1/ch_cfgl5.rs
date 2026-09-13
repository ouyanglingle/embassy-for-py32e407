#[doc = "Register `CH_CFGL5` reader"]
pub type R = crate::R<ChCfgl5Spec>;
#[doc = "Register `CH_CFGL5` writer"]
pub type W = crate::W<ChCfgl5Spec>;
#[doc = "Field `CH_PRIOR` reader - Channel Priority"]
pub type ChPriorR = crate::FieldReader;
#[doc = "Field `CH_PRIOR` writer - Channel Priority"]
pub type ChPriorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH_SUSP` reader - Channel Suspend"]
pub type ChSuspR = crate::BitReader;
#[doc = "Field `CH_SUSP` writer - Channel Suspend"]
pub type ChSuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_EMPTY` reader - Channel FIFO status"]
pub type FifoEmptyR = crate::BitReader;
#[doc = "Field `HS_SEL_DST` reader - Destination Software or Hardware Handshaking Select"]
pub type HsSelDstR = crate::BitReader;
#[doc = "Field `HS_SEL_DST` writer - Destination Software or Hardware Handshaking Select"]
pub type HsSelDstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS_SEL_SRC` reader - Source Software or Hardware Handshaking Select"]
pub type HsSelSrcR = crate::BitReader;
#[doc = "Field `HS_SEL_SRC` writer - Source Software or Hardware Handshaking Select"]
pub type HsSelSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_HS_POL` reader - Destination Handshaking Interface Polarity"]
pub type DstHsPolR = crate::BitReader;
#[doc = "Field `DST_HS_POL` writer - Destination Handshaking Interface Polarity"]
pub type DstHsPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_HS_POL` reader - Source Handshaking Interface Polarity"]
pub type SrcHsPolR = crate::BitReader;
#[doc = "Field `SRC_HS_POL` writer - Source Handshaking Interface Polarity"]
pub type SrcHsPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAX_ABRST` reader - Maximum AMBA Burst Length."]
pub type MaxAbrstR = crate::FieldReader<u16>;
#[doc = "Field `MAX_ABRST` writer - Maximum AMBA Burst Length."]
pub type MaxAbrstW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RELOAD_SRC` reader - Automatic Source Reload"]
pub type ReloadSrcR = crate::BitReader;
#[doc = "Field `RELOAD_SRC` writer - Automatic Source Reload"]
pub type ReloadSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RELOAD_DST` reader - Automatic Destination Reload"]
pub type ReloadDstR = crate::BitReader;
#[doc = "Field `RELOAD_DST` writer - Automatic Destination Reload"]
pub type ReloadDstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 5:7 - Channel Priority"]
    #[inline(always)]
    pub fn ch_prior(&self) -> ChPriorR {
        ChPriorR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Channel Suspend"]
    #[inline(always)]
    pub fn ch_susp(&self) -> ChSuspR {
        ChSuspR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel FIFO status"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FifoEmptyR {
        FifoEmptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_dst(&self) -> HsSelDstR {
        HsSelDstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_src(&self) -> HsSelSrcR {
        HsSelSrcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn dst_hs_pol(&self) -> DstHsPolR {
        DstHsPolR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn src_hs_pol(&self) -> SrcHsPolR {
        SrcHsPolR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:29 - Maximum AMBA Burst Length."]
    #[inline(always)]
    pub fn max_abrst(&self) -> MaxAbrstR {
        MaxAbrstR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - Automatic Source Reload"]
    #[inline(always)]
    pub fn reload_src(&self) -> ReloadSrcR {
        ReloadSrcR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Automatic Destination Reload"]
    #[inline(always)]
    pub fn reload_dst(&self) -> ReloadDstR {
        ReloadDstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 5:7 - Channel Priority"]
    #[inline(always)]
    pub fn ch_prior(&mut self) -> ChPriorW<'_, ChCfgl5Spec> {
        ChPriorW::new(self, 5)
    }
    #[doc = "Bit 8 - Channel Suspend"]
    #[inline(always)]
    pub fn ch_susp(&mut self) -> ChSuspW<'_, ChCfgl5Spec> {
        ChSuspW::new(self, 8)
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_dst(&mut self) -> HsSelDstW<'_, ChCfgl5Spec> {
        HsSelDstW::new(self, 10)
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_src(&mut self) -> HsSelSrcW<'_, ChCfgl5Spec> {
        HsSelSrcW::new(self, 11)
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn dst_hs_pol(&mut self) -> DstHsPolW<'_, ChCfgl5Spec> {
        DstHsPolW::new(self, 18)
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn src_hs_pol(&mut self) -> SrcHsPolW<'_, ChCfgl5Spec> {
        SrcHsPolW::new(self, 19)
    }
    #[doc = "Bits 20:29 - Maximum AMBA Burst Length."]
    #[inline(always)]
    pub fn max_abrst(&mut self) -> MaxAbrstW<'_, ChCfgl5Spec> {
        MaxAbrstW::new(self, 20)
    }
    #[doc = "Bit 30 - Automatic Source Reload"]
    #[inline(always)]
    pub fn reload_src(&mut self) -> ReloadSrcW<'_, ChCfgl5Spec> {
        ReloadSrcW::new(self, 30)
    }
    #[doc = "Bit 31 - Automatic Destination Reload"]
    #[inline(always)]
    pub fn reload_dst(&mut self) -> ReloadDstW<'_, ChCfgl5Spec> {
        ReloadDstW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfgl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfgl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChCfgl5Spec;
impl crate::RegisterSpec for ChCfgl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_cfgl5::R`](R) reader structure"]
impl crate::Readable for ChCfgl5Spec {}
#[doc = "`write(|w| ..)` method takes [`ch_cfgl5::W`](W) writer structure"]
impl crate::Writable for ChCfgl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_CFGL5 to value 0"]
impl crate::Resettable for ChCfgl5Spec {}
