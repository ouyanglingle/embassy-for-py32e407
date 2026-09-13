#[doc = "Register `RESPCMD` reader"]
pub type R = crate::R<RespcmdSpec>;
#[doc = "Register `RESPCMD` writer"]
pub type W = crate::W<RespcmdSpec>;
#[doc = "Field `RESPCMD` reader - desc RESPCMD"]
pub type RespcmdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - desc RESPCMD"]
    #[inline(always)]
    pub fn respcmd(&self) -> RespcmdR {
        RespcmdR::new((self.bits & 0x3f) as u8)
    }
}
impl W {}
#[doc = "desc RESPCMD\n\nYou can [`read`](crate::Reg::read) this register and get [`respcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`respcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RespcmdSpec;
impl crate::RegisterSpec for RespcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`respcmd::R`](R) reader structure"]
impl crate::Readable for RespcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`respcmd::W`](W) writer structure"]
impl crate::Writable for RespcmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESPCMD to value 0"]
impl crate::Resettable for RespcmdSpec {}
