#[doc = "Register `DMACHTDR` reader"]
pub type R = crate::R<DmachtdrSpec>;
#[doc = "Register `DMACHTDR` writer"]
pub type W = crate::W<DmachtdrSpec>;
#[doc = "Field `HTDAP` reader - Host transmit descriptor address pointer"]
pub type HtdapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host transmit descriptor address pointer"]
    #[inline(always)]
    pub fn htdap(&self) -> HtdapR {
        HtdapR::new(self.bits)
    }
}
impl W {}
#[doc = "Ethernet DMA current host transmit descriptor register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmachtdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmachtdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmachtdrSpec;
impl crate::RegisterSpec for DmachtdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmachtdr::R`](R) reader structure"]
impl crate::Readable for DmachtdrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmachtdr::W`](W) writer structure"]
impl crate::Writable for DmachtdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACHTDR to value 0"]
impl crate::Resettable for DmachtdrSpec {}
