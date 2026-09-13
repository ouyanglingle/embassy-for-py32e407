#[doc = "Register `RXCRCR` reader"]
pub type R = crate::R<RxcrcrSpec>;
#[doc = "Register `RXCRCR` writer"]
pub type W = crate::W<RxcrcrSpec>;
#[doc = "Field `RXCRC` reader - rcv crc reg"]
pub type RxcrcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - rcv crc reg"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RxcrcR {
        RxcrcR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxcrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxcrcrSpec;
impl crate::RegisterSpec for RxcrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxcrcr::R`](R) reader structure"]
impl crate::Readable for RxcrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxcrcr::W`](W) writer structure"]
impl crate::Writable for RxcrcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXCRCR to value 0"]
impl crate::Resettable for RxcrcrSpec {}
