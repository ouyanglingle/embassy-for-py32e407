#[doc = "Register `SGLRQSRC` reader"]
pub type R = crate::R<SglrqsrcSpec>;
#[doc = "Register `SGLRQSRC` writer"]
pub type W = crate::W<SglrqsrcSpec>;
#[doc = "Field `SRC_SGLREQ` reader - Source Single Transaction Request"]
pub type SrcSglreqR = crate::FieldReader;
#[doc = "Field `SRC_SGLREQ` writer - Source Single Transaction Request"]
pub type SrcSglreqW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SRC_SGLREQ_WE` reader - Source Single Transaction Request write enable"]
pub type SrcSglreqWeR = crate::FieldReader;
#[doc = "Field `SRC_SGLREQ_WE` writer - Source Single Transaction Request write enable"]
pub type SrcSglreqWeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Source Single Transaction Request"]
    #[inline(always)]
    pub fn src_sglreq(&self) -> SrcSglreqR {
        SrcSglreqR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Source Single Transaction Request write enable"]
    #[inline(always)]
    pub fn src_sglreq_we(&self) -> SrcSglreqWeR {
        SrcSglreqWeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Source Single Transaction Request"]
    #[inline(always)]
    pub fn src_sglreq(&mut self) -> SrcSglreqW<'_, SglrqsrcSpec> {
        SrcSglreqW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Source Single Transaction Request write enable"]
    #[inline(always)]
    pub fn src_sglreq_we(&mut self) -> SrcSglreqWeW<'_, SglrqsrcSpec> {
        SrcSglreqWeW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sglrqsrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sglrqsrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SglrqsrcSpec;
impl crate::RegisterSpec for SglrqsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sglrqsrc::R`](R) reader structure"]
impl crate::Readable for SglrqsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`sglrqsrc::W`](W) writer structure"]
impl crate::Writable for SglrqsrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGLRQSRC to value 0"]
impl crate::Resettable for SglrqsrcSpec {}
