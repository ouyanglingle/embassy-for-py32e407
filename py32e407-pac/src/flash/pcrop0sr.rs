#[doc = "Register `PCROP0SR` reader"]
pub type R = crate::R<Pcrop0srSpec>;
#[doc = "Register `PCROP0SR` writer"]
pub type W = crate::W<Pcrop0srSpec>;
#[doc = "Field `PCROP0SR` reader - "]
pub type Pcrop0srR = crate::FieldReader<u16>;
#[doc = "Field `PCROP0SR` writer - "]
pub type Pcrop0srW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn pcrop0sr(&self) -> Pcrop0srR {
        Pcrop0srR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn pcrop0sr(&mut self) -> Pcrop0srW<'_, Pcrop0srSpec> {
        Pcrop0srW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop0sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop0sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcrop0srSpec;
impl crate::RegisterSpec for Pcrop0srSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop0sr::R`](R) reader structure"]
impl crate::Readable for Pcrop0srSpec {}
#[doc = "`write(|w| ..)` method takes [`pcrop0sr::W`](W) writer structure"]
impl crate::Writable for Pcrop0srSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCROP0SR to value 0"]
impl crate::Resettable for Pcrop0srSpec {}
