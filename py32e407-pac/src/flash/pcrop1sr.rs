#[doc = "Register `PCROP1SR` reader"]
pub type R = crate::R<Pcrop1srSpec>;
#[doc = "Register `PCROP1SR` writer"]
pub type W = crate::W<Pcrop1srSpec>;
#[doc = "Field `PCROP1SR` reader - "]
pub type Pcrop1srR = crate::FieldReader<u16>;
#[doc = "Field `PCROP1SR` writer - "]
pub type Pcrop1srW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn pcrop1sr(&self) -> Pcrop1srR {
        Pcrop1srR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn pcrop1sr(&mut self) -> Pcrop1srW<'_, Pcrop1srSpec> {
        Pcrop1srW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop1sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcrop1srSpec;
impl crate::RegisterSpec for Pcrop1srSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop1sr::R`](R) reader structure"]
impl crate::Readable for Pcrop1srSpec {}
#[doc = "`write(|w| ..)` method takes [`pcrop1sr::W`](W) writer structure"]
impl crate::Writable for Pcrop1srSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCROP1SR to value 0"]
impl crate::Resettable for Pcrop1srSpec {}
