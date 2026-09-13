#[doc = "Register `TCBCNT` reader"]
pub type R = crate::R<TcbcntSpec>;
#[doc = "Register `TCBCNT` writer"]
pub type W = crate::W<TcbcntSpec>;
#[doc = "Field `TCBCNT` reader - desc TCBCNT"]
pub type TcbcntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc TCBCNT"]
    #[inline(always)]
    pub fn tcbcnt(&self) -> TcbcntR {
        TcbcntR::new(self.bits)
    }
}
impl W {}
#[doc = "desc TCBCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`tcbcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcbcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcbcntSpec;
impl crate::RegisterSpec for TcbcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcbcnt::R`](R) reader structure"]
impl crate::Readable for TcbcntSpec {}
#[doc = "`write(|w| ..)` method takes [`tcbcnt::W`](W) writer structure"]
impl crate::Writable for TcbcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCBCNT to value 0"]
impl crate::Resettable for TcbcntSpec {}
