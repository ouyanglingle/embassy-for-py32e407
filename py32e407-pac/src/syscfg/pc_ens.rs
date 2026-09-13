#[doc = "Register `PC_ENS` reader"]
pub type R = crate::R<PcEnsSpec>;
#[doc = "Register `PC_ENS` writer"]
pub type W = crate::W<PcEnsSpec>;
#[doc = "Field `PC_ENS` reader - GPIOC noise filter enable"]
pub type PcEnsR = crate::FieldReader<u16>;
#[doc = "Field `PC_ENS` writer - GPIOC noise filter enable"]
pub type PcEnsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIOC noise filter enable"]
    #[inline(always)]
    pub fn pc_ens(&self) -> PcEnsR {
        PcEnsR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIOC noise filter enable"]
    #[inline(always)]
    pub fn pc_ens(&mut self) -> PcEnsW<'_, PcEnsSpec> {
        PcEnsW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_ens::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_ens::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcEnsSpec;
impl crate::RegisterSpec for PcEnsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc_ens::R`](R) reader structure"]
impl crate::Readable for PcEnsSpec {}
#[doc = "`write(|w| ..)` method takes [`pc_ens::W`](W) writer structure"]
impl crate::Writable for PcEnsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PC_ENS to value 0"]
impl crate::Resettable for PcEnsSpec {}
