#[doc = "Register `TBBCNT` reader"]
pub type R = crate::R<TbbcntSpec>;
#[doc = "Register `TBBCNT` writer"]
pub type W = crate::W<TbbcntSpec>;
#[doc = "Field `TBBCNT` reader - desc TBBCNT"]
pub type TbbcntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc TBBCNT"]
    #[inline(always)]
    pub fn tbbcnt(&self) -> TbbcntR {
        TbbcntR::new(self.bits)
    }
}
impl W {}
#[doc = "desc TBBCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`tbbcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbbcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbbcntSpec;
impl crate::RegisterSpec for TbbcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbbcnt::R`](R) reader structure"]
impl crate::Readable for TbbcntSpec {}
#[doc = "`write(|w| ..)` method takes [`tbbcnt::W`](W) writer structure"]
impl crate::Writable for TbbcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBBCNT to value 0"]
impl crate::Resettable for TbbcntSpec {}
