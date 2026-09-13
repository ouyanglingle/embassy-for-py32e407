#[doc = "Register `SGLRQDST` reader"]
pub type R = crate::R<SglrqdstSpec>;
#[doc = "Register `SGLRQDST` writer"]
pub type W = crate::W<SglrqdstSpec>;
#[doc = "Field `DST_SGLREQ` reader - Destination Single Transaction Request"]
pub type DstSglreqR = crate::FieldReader;
#[doc = "Field `DST_SGLREQ` writer - Destination Single Transaction Request"]
pub type DstSglreqW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DST_SGLREQ_WE` reader - Destination Single Transaction Request write enable"]
pub type DstSglreqWeR = crate::FieldReader;
#[doc = "Field `DST_SGLREQ_WE` writer - Destination Single Transaction Request write enable"]
pub type DstSglreqWeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Destination Single Transaction Request"]
    #[inline(always)]
    pub fn dst_sglreq(&self) -> DstSglreqR {
        DstSglreqR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Destination Single Transaction Request write enable"]
    #[inline(always)]
    pub fn dst_sglreq_we(&self) -> DstSglreqWeR {
        DstSglreqWeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Destination Single Transaction Request"]
    #[inline(always)]
    pub fn dst_sglreq(&mut self) -> DstSglreqW<'_, SglrqdstSpec> {
        DstSglreqW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Destination Single Transaction Request write enable"]
    #[inline(always)]
    pub fn dst_sglreq_we(&mut self) -> DstSglreqWeW<'_, SglrqdstSpec> {
        DstSglreqWeW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sglrqdst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sglrqdst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SglrqdstSpec;
impl crate::RegisterSpec for SglrqdstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sglrqdst::R`](R) reader structure"]
impl crate::Readable for SglrqdstSpec {}
#[doc = "`write(|w| ..)` method takes [`sglrqdst::W`](W) writer structure"]
impl crate::Writable for SglrqdstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGLRQDST to value 0"]
impl crate::Resettable for SglrqdstSpec {}
