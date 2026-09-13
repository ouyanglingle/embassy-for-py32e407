#[doc = "Register `DR23` reader"]
pub type R = crate::R<Dr23Spec>;
#[doc = "Register `DR23` writer"]
pub type W = crate::W<Dr23Spec>;
#[doc = "Field `D` reader - bakeup data byte23"]
pub type DR = crate::FieldReader<u32>;
#[doc = "Field `D` writer - bakeup data byte23"]
pub type DW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bakeup data byte23"]
    #[inline(always)]
    pub fn d(&self) -> DR {
        DR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bakeup data byte23"]
    #[inline(always)]
    pub fn d(&mut self) -> DW<'_, Dr23Spec> {
        DW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dr23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr23Spec;
impl crate::RegisterSpec for Dr23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr23::R`](R) reader structure"]
impl crate::Readable for Dr23Spec {}
#[doc = "`write(|w| ..)` method takes [`dr23::W`](W) writer structure"]
impl crate::Writable for Dr23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR23 to value 0"]
impl crate::Resettable for Dr23Spec {}
