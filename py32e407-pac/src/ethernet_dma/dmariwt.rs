#[doc = "Register `DMARIWT` reader"]
pub type R = crate::R<DmariwtSpec>;
#[doc = "Register `DMARIWT` writer"]
pub type W = crate::W<DmariwtSpec>;
#[doc = "Field `RIWT` reader - RIWT"]
pub type RiwtR = crate::FieldReader;
#[doc = "Field `RIWT` writer - RIWT"]
pub type RiwtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RIWT"]
    #[inline(always)]
    pub fn riwt(&self) -> RiwtR {
        RiwtR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RIWT"]
    #[inline(always)]
    pub fn riwt(&mut self) -> RiwtW<'_, DmariwtSpec> {
        RiwtW::new(self, 0)
    }
}
#[doc = "ETH DMA receive interrupt watchdog timer\n\nYou can [`read`](crate::Reg::read) this register and get [`dmariwt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmariwt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmariwtSpec;
impl crate::RegisterSpec for DmariwtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmariwt::R`](R) reader structure"]
impl crate::Readable for DmariwtSpec {}
#[doc = "`write(|w| ..)` method takes [`dmariwt::W`](W) writer structure"]
impl crate::Writable for DmariwtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMARIWT to value 0"]
impl crate::Resettable for DmariwtSpec {}
