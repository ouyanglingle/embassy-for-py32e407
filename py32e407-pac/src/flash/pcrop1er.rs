#[doc = "Register `PCROP1ER` reader"]
pub type R = crate::R<Pcrop1erSpec>;
#[doc = "Register `PCROP1ER` writer"]
pub type W = crate::W<Pcrop1erSpec>;
#[doc = "Field `PCROP1ER` reader - "]
pub type Pcrop1erR = crate::FieldReader<u16>;
#[doc = "Field `PCROP1ER` writer - "]
pub type Pcrop1erW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn pcrop1er(&self) -> Pcrop1erR {
        Pcrop1erR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn pcrop1er(&mut self) -> Pcrop1erW<'_, Pcrop1erSpec> {
        Pcrop1erW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop1er::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1er::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcrop1erSpec;
impl crate::RegisterSpec for Pcrop1erSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop1er::R`](R) reader structure"]
impl crate::Readable for Pcrop1erSpec {}
#[doc = "`write(|w| ..)` method takes [`pcrop1er::W`](W) writer structure"]
impl crate::Writable for Pcrop1erSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCROP1ER to value 0"]
impl crate::Resettable for Pcrop1erSpec {}
