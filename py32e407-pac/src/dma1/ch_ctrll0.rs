#[doc = "Register `CH_CTRLL0` reader"]
pub type R = crate::R<ChCtrll0Spec>;
#[doc = "Register `CH_CTRLL0` writer"]
pub type W = crate::W<ChCtrll0Spec>;
#[doc = "Field `INT_EN` reader - Interrupt Enable Bit"]
pub type IntEnR = crate::BitReader;
#[doc = "Field `INT_EN` writer - Interrupt Enable Bit"]
pub type IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_TR_WIDTH` reader - Destination Transfer Width"]
pub type DstTrWidthR = crate::FieldReader;
#[doc = "Field `DST_TR_WIDTH` writer - Destination Transfer Width"]
pub type DstTrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRC_TR_WIDTH` reader - Source Transfer Width"]
pub type SrcTrWidthR = crate::FieldReader;
#[doc = "Field `SRC_TR_WIDTH` writer - Source Transfer Width"]
pub type SrcTrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DINC` reader - Destination Address Increment"]
pub type DincR = crate::FieldReader;
#[doc = "Field `DINC` writer - Destination Address Increment"]
pub type DincW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SINC` reader - Source Address Increment"]
pub type SincR = crate::FieldReader;
#[doc = "Field `SINC` writer - Source Address Increment"]
pub type SincW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DST_MSIZE` reader - Destination Burst Transaction Length"]
pub type DstMsizeR = crate::FieldReader;
#[doc = "Field `DST_MSIZE` writer - Destination Burst Transaction Length"]
pub type DstMsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRC_MSIZE` reader - Source Burst Transaction Length"]
pub type SrcMsizeR = crate::FieldReader;
#[doc = "Field `SRC_MSIZE` writer - Source Burst Transaction Length"]
pub type SrcMsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TT` reader - Transfer Type"]
pub type TtR = crate::FieldReader;
#[doc = "Field `TT` writer - Transfer Type"]
pub type TtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Interrupt Enable Bit"]
    #[inline(always)]
    pub fn int_en(&self) -> IntEnR {
        IntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Destination Transfer Width"]
    #[inline(always)]
    pub fn dst_tr_width(&self) -> DstTrWidthR {
        DstTrWidthR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - Source Transfer Width"]
    #[inline(always)]
    pub fn src_tr_width(&self) -> SrcTrWidthR {
        SrcTrWidthR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Destination Address Increment"]
    #[inline(always)]
    pub fn dinc(&self) -> DincR {
        DincR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Source Address Increment"]
    #[inline(always)]
    pub fn sinc(&self) -> SincR {
        SincR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length"]
    #[inline(always)]
    pub fn dst_msize(&self) -> DstMsizeR {
        DstMsizeR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length"]
    #[inline(always)]
    pub fn src_msize(&self) -> SrcMsizeR {
        SrcMsizeR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Transfer Type"]
    #[inline(always)]
    pub fn tt(&self) -> TtR {
        TtR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable Bit"]
    #[inline(always)]
    pub fn int_en(&mut self) -> IntEnW<'_, ChCtrll0Spec> {
        IntEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Destination Transfer Width"]
    #[inline(always)]
    pub fn dst_tr_width(&mut self) -> DstTrWidthW<'_, ChCtrll0Spec> {
        DstTrWidthW::new(self, 1)
    }
    #[doc = "Bits 4:6 - Source Transfer Width"]
    #[inline(always)]
    pub fn src_tr_width(&mut self) -> SrcTrWidthW<'_, ChCtrll0Spec> {
        SrcTrWidthW::new(self, 4)
    }
    #[doc = "Bits 7:8 - Destination Address Increment"]
    #[inline(always)]
    pub fn dinc(&mut self) -> DincW<'_, ChCtrll0Spec> {
        DincW::new(self, 7)
    }
    #[doc = "Bits 9:10 - Source Address Increment"]
    #[inline(always)]
    pub fn sinc(&mut self) -> SincW<'_, ChCtrll0Spec> {
        SincW::new(self, 9)
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length"]
    #[inline(always)]
    pub fn dst_msize(&mut self) -> DstMsizeW<'_, ChCtrll0Spec> {
        DstMsizeW::new(self, 11)
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length"]
    #[inline(always)]
    pub fn src_msize(&mut self) -> SrcMsizeW<'_, ChCtrll0Spec> {
        SrcMsizeW::new(self, 14)
    }
    #[doc = "Bits 20:21 - Transfer Type"]
    #[inline(always)]
    pub fn tt(&mut self) -> TtW<'_, ChCtrll0Spec> {
        TtW::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctrll0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctrll0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChCtrll0Spec;
impl crate::RegisterSpec for ChCtrll0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_ctrll0::R`](R) reader structure"]
impl crate::Readable for ChCtrll0Spec {}
#[doc = "`write(|w| ..)` method takes [`ch_ctrll0::W`](W) writer structure"]
impl crate::Writable for ChCtrll0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_CTRLL0 to value 0"]
impl crate::Resettable for ChCtrll0Spec {}
