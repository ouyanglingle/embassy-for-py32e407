#[doc = "Register `STATUSINT` reader"]
pub type R = crate::R<StatusintSpec>;
#[doc = "Register `STATUSINT` writer"]
pub type W = crate::W<StatusintSpec>;
#[doc = "Field `TFR` reader - OR of the contents of StatusTfr"]
pub type TfrR = crate::BitReader;
#[doc = "Field `BLOCK` reader - OR of the contents of StatusBlock"]
pub type BlockR = crate::BitReader;
#[doc = "Field `SRCT` reader - OR of the contents of StatusSrcTran"]
pub type SrctR = crate::BitReader;
#[doc = "Field `DSTT` reader - OR of the contents of StatusDstTran"]
pub type DsttR = crate::BitReader;
#[doc = "Field `ERR` reader - OR of the contents of StatusErr"]
pub type ErrR = crate::BitReader;
#[doc = "Field `HALFBLOCK` reader - OR of the contents of StatusHalfBlock"]
pub type HalfblockR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - OR of the contents of StatusTfr"]
    #[inline(always)]
    pub fn tfr(&self) -> TfrR {
        TfrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OR of the contents of StatusBlock"]
    #[inline(always)]
    pub fn block(&self) -> BlockR {
        BlockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OR of the contents of StatusSrcTran"]
    #[inline(always)]
    pub fn srct(&self) -> SrctR {
        SrctR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OR of the contents of StatusDstTran"]
    #[inline(always)]
    pub fn dstt(&self) -> DsttR {
        DsttR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OR of the contents of StatusErr"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OR of the contents of StatusHalfBlock"]
    #[inline(always)]
    pub fn halfblock(&self) -> HalfblockR {
        HalfblockR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`statusint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusintSpec;
impl crate::RegisterSpec for StatusintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusint::R`](R) reader structure"]
impl crate::Readable for StatusintSpec {}
#[doc = "`write(|w| ..)` method takes [`statusint::W`](W) writer structure"]
impl crate::Writable for StatusintSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUSINT to value 0"]
impl crate::Resettable for StatusintSpec {}
