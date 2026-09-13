#[doc = "Register `PTPTSHR` reader"]
pub type R = crate::R<PtptshrSpec>;
#[doc = "Register `PTPTSHR` writer"]
pub type W = crate::W<PtptshrSpec>;
#[doc = "Field `STS` reader - System time second"]
pub type StsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - System time second"]
    #[inline(always)]
    pub fn sts(&self) -> StsR {
        StsR::new(self.bits)
    }
}
impl W {}
#[doc = "Ethernet PTP time stamp high register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptshr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptshr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtptshrSpec;
impl crate::RegisterSpec for PtptshrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptshr::R`](R) reader structure"]
impl crate::Readable for PtptshrSpec {}
#[doc = "`write(|w| ..)` method takes [`ptptshr::W`](W) writer structure"]
impl crate::Writable for PtptshrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PTPTSHR to value 0"]
impl crate::Resettable for PtptshrSpec {}
