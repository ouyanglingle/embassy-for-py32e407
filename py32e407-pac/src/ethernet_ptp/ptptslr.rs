#[doc = "Register `PTPTSLR` reader"]
pub type R = crate::R<PtptslrSpec>;
#[doc = "Register `PTPTSLR` writer"]
pub type W = crate::W<PtptslrSpec>;
#[doc = "Field `STSS` reader - System time subseconds"]
pub type StssR = crate::FieldReader<u32>;
#[doc = "Field `STPNS` reader - System time positive or negative sign"]
pub type StpnsR = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - System time subseconds"]
    #[inline(always)]
    pub fn stss(&self) -> StssR {
        StssR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - System time positive or negative sign"]
    #[inline(always)]
    pub fn stpns(&self) -> StpnsR {
        StpnsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Ethernet PTP time stamp low register (ETH_PTPTSLR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptslr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptslr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtptslrSpec;
impl crate::RegisterSpec for PtptslrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptslr::R`](R) reader structure"]
impl crate::Readable for PtptslrSpec {}
#[doc = "`write(|w| ..)` method takes [`ptptslr::W`](W) writer structure"]
impl crate::Writable for PtptslrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PTPTSLR to value 0"]
impl crate::Resettable for PtptslrSpec {}
