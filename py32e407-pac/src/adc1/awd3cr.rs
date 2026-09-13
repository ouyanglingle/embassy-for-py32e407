#[doc = "Register `AWD3CR` reader"]
pub type R = crate::R<Awd3crSpec>;
#[doc = "Register `AWD3CR` writer"]
pub type W = crate::W<Awd3crSpec>;
#[doc = "Field `AWD3CH` reader - "]
pub type Awd3chR = crate::FieldReader<u32>;
#[doc = "Field `AWD3CH` writer - "]
pub type Awd3chW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn awd3ch(&self) -> Awd3chR {
        Awd3chR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn awd3ch(&mut self) -> Awd3chW<'_, Awd3crSpec> {
        Awd3chW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`awd3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awd3crSpec;
impl crate::RegisterSpec for Awd3crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awd3cr::R`](R) reader structure"]
impl crate::Readable for Awd3crSpec {}
#[doc = "`write(|w| ..)` method takes [`awd3cr::W`](W) writer structure"]
impl crate::Writable for Awd3crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWD3CR to value 0"]
impl crate::Resettable for Awd3crSpec {}
