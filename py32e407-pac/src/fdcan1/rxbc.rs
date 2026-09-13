#[doc = "Register `RXBC` reader"]
pub type R = crate::R<RxbcSpec>;
#[doc = "Register `RXBC` writer"]
pub type W = crate::W<RxbcSpec>;
#[doc = "Field `RBSA` reader - Rx Buffer Start Address"]
pub type RbsaR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 2:15 - Rx Buffer Start Address"]
    #[inline(always)]
    pub fn rbsa(&self) -> RbsaR {
        RbsaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxbcSpec;
impl crate::RegisterSpec for RxbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxbc::R`](R) reader structure"]
impl crate::Readable for RxbcSpec {}
#[doc = "`write(|w| ..)` method takes [`rxbc::W`](W) writer structure"]
impl crate::Writable for RxbcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXBC to value 0"]
impl crate::Resettable for RxbcSpec {}
