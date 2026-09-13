#[doc = "Register `DHR12LD` reader"]
pub type R = crate::R<Dhr12ldSpec>;
#[doc = "Register `DHR12LD` writer"]
pub type W = crate::W<Dhr12ldSpec>;
#[doc = "Field `DACC1DHR` reader - "]
pub type Dacc1dhrR = crate::FieldReader<u16>;
#[doc = "Field `DACC1DHR` writer - "]
pub type Dacc1dhrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DACC2DHR` reader - "]
pub type Dacc2dhrR = crate::FieldReader<u16>;
#[doc = "Field `DACC2DHR` writer - "]
pub type Dacc2dhrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15"]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> Dacc1dhrR {
        Dacc1dhrR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> Dacc2dhrR {
        Dacc2dhrR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15"]
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> Dacc1dhrW<'_, Dhr12ldSpec> {
        Dacc1dhrW::new(self, 4)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> Dacc2dhrW<'_, Dhr12ldSpec> {
        Dacc2dhrW::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12ld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12ld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dhr12ldSpec;
impl crate::RegisterSpec for Dhr12ldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr12ld::R`](R) reader structure"]
impl crate::Readable for Dhr12ldSpec {}
#[doc = "`write(|w| ..)` method takes [`dhr12ld::W`](W) writer structure"]
impl crate::Writable for Dhr12ldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DHR12LD to value 0"]
impl crate::Resettable for Dhr12ldSpec {}
