#[doc = "Register `PA_ENS` reader"]
pub type R = crate::R<PaEnsSpec>;
#[doc = "Register `PA_ENS` writer"]
pub type W = crate::W<PaEnsSpec>;
#[doc = "Field `PA_ENS` reader - GPIOA noise filter enable"]
pub type PaEnsR = crate::FieldReader<u16>;
#[doc = "Field `PA_ENS` writer - GPIOA noise filter enable"]
pub type PaEnsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIOA noise filter enable"]
    #[inline(always)]
    pub fn pa_ens(&self) -> PaEnsR {
        PaEnsR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIOA noise filter enable"]
    #[inline(always)]
    pub fn pa_ens(&mut self) -> PaEnsW<'_, PaEnsSpec> {
        PaEnsW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_ens::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_ens::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaEnsSpec;
impl crate::RegisterSpec for PaEnsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_ens::R`](R) reader structure"]
impl crate::Readable for PaEnsSpec {}
#[doc = "`write(|w| ..)` method takes [`pa_ens::W`](W) writer structure"]
impl crate::Writable for PaEnsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PA_ENS to value 0"]
impl crate::Resettable for PaEnsSpec {}
