#[doc = "Register `TR2` reader"]
pub type R = crate::R<Tr2Spec>;
#[doc = "Register `TR2` writer"]
pub type W = crate::W<Tr2Spec>;
#[doc = "Field `LT2` reader - "]
pub type Lt2R = crate::FieldReader<u16>;
#[doc = "Field `LT2` writer - "]
pub type Lt2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HT2` reader - "]
pub type Ht2R = crate::FieldReader<u16>;
#[doc = "Field `HT2` writer - "]
pub type Ht2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn lt2(&self) -> Lt2R {
        Lt2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn ht2(&self) -> Ht2R {
        Ht2R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn lt2(&mut self) -> Lt2W<'_, Tr2Spec> {
        Lt2W::new(self, 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn ht2(&mut self) -> Ht2W<'_, Tr2Spec> {
        Ht2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tr2Spec;
impl crate::RegisterSpec for Tr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr2::R`](R) reader structure"]
impl crate::Readable for Tr2Spec {}
#[doc = "`write(|w| ..)` method takes [`tr2::W`](W) writer structure"]
impl crate::Writable for Tr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TR2 to value 0"]
impl crate::Resettable for Tr2Spec {}
