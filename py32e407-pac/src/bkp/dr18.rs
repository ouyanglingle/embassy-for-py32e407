#[doc = "Register `DR18` reader"]
pub type R = crate::R<Dr18Spec>;
#[doc = "Register `DR18` writer"]
pub type W = crate::W<Dr18Spec>;
#[doc = "Field `D` reader - bakeup data byte18"]
pub type DR = crate::FieldReader<u32>;
#[doc = "Field `D` writer - bakeup data byte18"]
pub type DW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bakeup data byte18"]
    #[inline(always)]
    pub fn d(&self) -> DR {
        DR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bakeup data byte18"]
    #[inline(always)]
    pub fn d(&mut self) -> DW<'_, Dr18Spec> {
        DW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dr18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr18Spec;
impl crate::RegisterSpec for Dr18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr18::R`](R) reader structure"]
impl crate::Readable for Dr18Spec {}
#[doc = "`write(|w| ..)` method takes [`dr18::W`](W) writer structure"]
impl crate::Writable for Dr18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR18 to value 0"]
impl crate::Resettable for Dr18Spec {}
