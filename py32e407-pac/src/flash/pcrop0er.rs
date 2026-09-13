#[doc = "Register `PCROP0ER` reader"]
pub type R = crate::R<Pcrop0erSpec>;
#[doc = "Register `PCROP0ER` writer"]
pub type W = crate::W<Pcrop0erSpec>;
#[doc = "Field `PCROP0ER` reader - "]
pub type Pcrop0erR = crate::FieldReader<u16>;
#[doc = "Field `PCROP0ER` writer - "]
pub type Pcrop0erW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn pcrop0er(&self) -> Pcrop0erR {
        Pcrop0erR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn pcrop0er(&mut self) -> Pcrop0erW<'_, Pcrop0erSpec> {
        Pcrop0erW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop0er::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop0er::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcrop0erSpec;
impl crate::RegisterSpec for Pcrop0erSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop0er::R`](R) reader structure"]
impl crate::Readable for Pcrop0erSpec {}
#[doc = "`write(|w| ..)` method takes [`pcrop0er::W`](W) writer structure"]
impl crate::Writable for Pcrop0erSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCROP0ER to value 0"]
impl crate::Resettable for Pcrop0erSpec {}
