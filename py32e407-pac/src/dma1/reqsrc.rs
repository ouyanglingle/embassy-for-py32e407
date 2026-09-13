#[doc = "Register `REQSRC` reader"]
pub type R = crate::R<ReqsrcSpec>;
#[doc = "Register `REQSRC` writer"]
pub type W = crate::W<ReqsrcSpec>;
#[doc = "Field `SRC_REQ` reader - Source Software Transaction Request"]
pub type SrcReqR = crate::FieldReader;
#[doc = "Field `SRC_REQ` writer - Source Software Transaction Request"]
pub type SrcReqW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SRC_REQ_WE` reader - Source Software Transaction Request write enable"]
pub type SrcReqWeR = crate::FieldReader;
#[doc = "Field `SRC_REQ_WE` writer - Source Software Transaction Request write enable"]
pub type SrcReqWeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Source Software Transaction Request"]
    #[inline(always)]
    pub fn src_req(&self) -> SrcReqR {
        SrcReqR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Source Software Transaction Request write enable"]
    #[inline(always)]
    pub fn src_req_we(&self) -> SrcReqWeR {
        SrcReqWeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Source Software Transaction Request"]
    #[inline(always)]
    pub fn src_req(&mut self) -> SrcReqW<'_, ReqsrcSpec> {
        SrcReqW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Source Software Transaction Request write enable"]
    #[inline(always)]
    pub fn src_req_we(&mut self) -> SrcReqWeW<'_, ReqsrcSpec> {
        SrcReqWeW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reqsrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqsrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReqsrcSpec;
impl crate::RegisterSpec for ReqsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reqsrc::R`](R) reader structure"]
impl crate::Readable for ReqsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`reqsrc::W`](W) writer structure"]
impl crate::Writable for ReqsrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REQSRC to value 0"]
impl crate::Resettable for ReqsrcSpec {}
