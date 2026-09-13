#[doc = "Register `DMARPDR` reader"]
pub type R = crate::R<DmarpdrSpec>;
#[doc = "Register `DMARPDR` writer"]
pub type W = crate::W<DmarpdrSpec>;
#[doc = "Field `RPD` reader - Receive poll demand"]
pub type RpdR = crate::FieldReader<u32>;
#[doc = "Field `RPD` writer - Receive poll demand"]
pub type RpdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    pub fn rpd(&self) -> RpdR {
        RpdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    pub fn rpd(&mut self) -> RpdW<'_, DmarpdrSpec> {
        RpdW::new(self, 0)
    }
}
#[doc = "EHERNET DMA receive poll demand register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmarpdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarpdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmarpdrSpec;
impl crate::RegisterSpec for DmarpdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarpdr::R`](R) reader structure"]
impl crate::Readable for DmarpdrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmarpdr::W`](W) writer structure"]
impl crate::Writable for DmarpdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMARPDR to value 0"]
impl crate::Resettable for DmarpdrSpec {}
