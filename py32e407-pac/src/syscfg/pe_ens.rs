#[doc = "Register `PE_ENS` reader"]
pub type R = crate::R<PeEnsSpec>;
#[doc = "Register `PE_ENS` writer"]
pub type W = crate::W<PeEnsSpec>;
#[doc = "Field `PE_ENS` reader - GPIOE noise filter enable"]
pub type PeEnsR = crate::FieldReader<u16>;
#[doc = "Field `PE_ENS` writer - GPIOE noise filter enable"]
pub type PeEnsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIOE noise filter enable"]
    #[inline(always)]
    pub fn pe_ens(&self) -> PeEnsR {
        PeEnsR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIOE noise filter enable"]
    #[inline(always)]
    pub fn pe_ens(&mut self) -> PeEnsW<'_, PeEnsSpec> {
        PeEnsW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_ens::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_ens::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeEnsSpec;
impl crate::RegisterSpec for PeEnsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_ens::R`](R) reader structure"]
impl crate::Readable for PeEnsSpec {}
#[doc = "`write(|w| ..)` method takes [`pe_ens::W`](W) writer structure"]
impl crate::Writable for PeEnsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_ENS to value 0"]
impl crate::Resettable for PeEnsSpec {}
