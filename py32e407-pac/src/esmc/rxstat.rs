#[doc = "Register `RXSTAT` reader"]
pub type R = crate::R<RxstatSpec>;
#[doc = "Register `RXSTAT` writer"]
pub type W = crate::W<RxstatSpec>;
#[doc = "Field `RXSTAT` reader - rx state"]
pub type RxstatR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - rx state"]
    #[inline(always)]
    pub fn rxstat(&self) -> RxstatR {
        RxstatR::new(self.bits)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rxstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxstatSpec;
impl crate::RegisterSpec for RxstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxstat::R`](R) reader structure"]
impl crate::Readable for RxstatSpec {}
#[doc = "`write(|w| ..)` method takes [`rxstat::W`](W) writer structure"]
impl crate::Writable for RxstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXSTAT to value 0"]
impl crate::Resettable for RxstatSpec {}
