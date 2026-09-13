#[doc = "Register `REQDST` reader"]
pub type R = crate::R<ReqdstSpec>;
#[doc = "Register `REQDST` writer"]
pub type W = crate::W<ReqdstSpec>;
#[doc = "Field `DST_REQ` reader - Destination Software Transaction Request"]
pub type DstReqR = crate::FieldReader;
#[doc = "Field `DST_REQ` writer - Destination Software Transaction Request"]
pub type DstReqW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DST_REQ_WE` reader - Destination Software Transaction Request write enable"]
pub type DstReqWeR = crate::FieldReader;
#[doc = "Field `DST_REQ_WE` writer - Destination Software Transaction Request write enable"]
pub type DstReqWeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Destination Software Transaction Request"]
    #[inline(always)]
    pub fn dst_req(&self) -> DstReqR {
        DstReqR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Destination Software Transaction Request write enable"]
    #[inline(always)]
    pub fn dst_req_we(&self) -> DstReqWeR {
        DstReqWeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Destination Software Transaction Request"]
    #[inline(always)]
    pub fn dst_req(&mut self) -> DstReqW<'_, ReqdstSpec> {
        DstReqW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Destination Software Transaction Request write enable"]
    #[inline(always)]
    pub fn dst_req_we(&mut self) -> DstReqWeW<'_, ReqdstSpec> {
        DstReqWeW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reqdst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqdst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReqdstSpec;
impl crate::RegisterSpec for ReqdstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reqdst::R`](R) reader structure"]
impl crate::Readable for ReqdstSpec {}
#[doc = "`write(|w| ..)` method takes [`reqdst::W`](W) writer structure"]
impl crate::Writable for ReqdstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REQDST to value 0"]
impl crate::Resettable for ReqdstSpec {}
