#[doc = "Register `PB_ENS` reader"]
pub type R = crate::R<PbEnsSpec>;
#[doc = "Register `PB_ENS` writer"]
pub type W = crate::W<PbEnsSpec>;
#[doc = "Field `PB_ENS` reader - GPIOB noise filter enable"]
pub type PbEnsR = crate::FieldReader<u16>;
#[doc = "Field `PB_ENS` writer - GPIOB noise filter enable"]
pub type PbEnsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIOB noise filter enable"]
    #[inline(always)]
    pub fn pb_ens(&self) -> PbEnsR {
        PbEnsR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIOB noise filter enable"]
    #[inline(always)]
    pub fn pb_ens(&mut self) -> PbEnsW<'_, PbEnsSpec> {
        PbEnsW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_ens::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_ens::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbEnsSpec;
impl crate::RegisterSpec for PbEnsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_ens::R`](R) reader structure"]
impl crate::Readable for PbEnsSpec {}
#[doc = "`write(|w| ..)` method takes [`pb_ens::W`](W) writer structure"]
impl crate::Writable for PbEnsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PB_ENS to value 0"]
impl crate::Resettable for PbEnsSpec {}
