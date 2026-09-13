#[doc = "Register `DOR1` reader"]
pub type R = crate::R<Dor1Spec>;
#[doc = "Register `DOR1` writer"]
pub type W = crate::W<Dor1Spec>;
#[doc = "Field `DACC1DOR` reader - "]
pub type Dacc1dorR = crate::FieldReader<u16>;
#[doc = "Field `DACC1DOR` writer - "]
pub type Dacc1dorW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dacc1dor(&self) -> Dacc1dorR {
        Dacc1dorR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dacc1dor(&mut self) -> Dacc1dorW<'_, Dor1Spec> {
        Dacc1dorW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dor1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dor1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dor1Spec;
impl crate::RegisterSpec for Dor1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dor1::R`](R) reader structure"]
impl crate::Readable for Dor1Spec {}
#[doc = "`write(|w| ..)` method takes [`dor1::W`](W) writer structure"]
impl crate::Writable for Dor1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOR1 to value 0"]
impl crate::Resettable for Dor1Spec {}
