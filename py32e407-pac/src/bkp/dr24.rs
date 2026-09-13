#[doc = "Register `DR24` reader"]
pub type R = crate::R<Dr24Spec>;
#[doc = "Register `DR24` writer"]
pub type W = crate::W<Dr24Spec>;
#[doc = "Field `D` reader - bakeup data byte24"]
pub type DR = crate::FieldReader<u32>;
#[doc = "Field `D` writer - bakeup data byte24"]
pub type DW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bakeup data byte24"]
    #[inline(always)]
    pub fn d(&self) -> DR {
        DR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bakeup data byte24"]
    #[inline(always)]
    pub fn d(&mut self) -> DW<'_, Dr24Spec> {
        DW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dr24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr24Spec;
impl crate::RegisterSpec for Dr24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr24::R`](R) reader structure"]
impl crate::Readable for Dr24Spec {}
#[doc = "`write(|w| ..)` method takes [`dr24::W`](W) writer structure"]
impl crate::Writable for Dr24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR24 to value 0"]
impl crate::Resettable for Dr24Spec {}
