#[doc = "Register `DOR2` reader"]
pub type R = crate::R<Dor2Spec>;
#[doc = "Register `DOR2` writer"]
pub type W = crate::W<Dor2Spec>;
#[doc = "Field `DACC2DOR` reader - "]
pub type Dacc2dorR = crate::FieldReader<u16>;
#[doc = "Field `DACC2DOR` writer - "]
pub type Dacc2dorW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dacc2dor(&self) -> Dacc2dorR {
        Dacc2dorR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dacc2dor(&mut self) -> Dacc2dorW<'_, Dor2Spec> {
        Dacc2dorW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dor2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dor2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dor2Spec;
impl crate::RegisterSpec for Dor2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dor2::R`](R) reader structure"]
impl crate::Readable for Dor2Spec {}
#[doc = "`write(|w| ..)` method takes [`dor2::W`](W) writer structure"]
impl crate::Writable for Dor2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOR2 to value 0"]
impl crate::Resettable for Dor2Spec {}
