#[doc = "Register `PF_ENS` reader"]
pub type R = crate::R<PfEnsSpec>;
#[doc = "Register `PF_ENS` writer"]
pub type W = crate::W<PfEnsSpec>;
#[doc = "Field `PF_ENS` reader - GPIOF noise filter enable"]
pub type PfEnsR = crate::FieldReader;
#[doc = "Field `PF_ENS` writer - GPIOF noise filter enable"]
pub type PfEnsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - GPIOF noise filter enable"]
    #[inline(always)]
    pub fn pf_ens(&self) -> PfEnsR {
        PfEnsR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - GPIOF noise filter enable"]
    #[inline(always)]
    pub fn pf_ens(&mut self) -> PfEnsW<'_, PfEnsSpec> {
        PfEnsW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_ens::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_ens::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfEnsSpec;
impl crate::RegisterSpec for PfEnsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_ens::R`](R) reader structure"]
impl crate::Readable for PfEnsSpec {}
#[doc = "`write(|w| ..)` method takes [`pf_ens::W`](W) writer structure"]
impl crate::Writable for PfEnsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_ENS to value 0"]
impl crate::Resettable for PfEnsSpec {}
