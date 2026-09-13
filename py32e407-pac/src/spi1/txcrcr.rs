#[doc = "Register `TXCRCR` reader"]
pub type R = crate::R<TxcrcrSpec>;
#[doc = "Register `TXCRCR` writer"]
pub type W = crate::W<TxcrcrSpec>;
#[doc = "Field `TXCRC` reader - tx crc reg"]
pub type TxcrcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - tx crc reg"]
    #[inline(always)]
    pub fn txcrc(&self) -> TxcrcR {
        TxcrcR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`txcrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txcrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxcrcrSpec;
impl crate::RegisterSpec for TxcrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txcrcr::R`](R) reader structure"]
impl crate::Readable for TxcrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`txcrcr::W`](W) writer structure"]
impl crate::Writable for TxcrcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXCRCR to value 0"]
impl crate::Resettable for TxcrcrSpec {}
