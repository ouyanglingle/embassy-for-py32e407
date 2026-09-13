#[doc = "Register `RESP3` reader"]
pub type R = crate::R<Resp3Spec>;
#[doc = "Register `RESP3` writer"]
pub type W = crate::W<Resp3Spec>;
#[doc = "Field `CARDSTATUS4` reader - desc CARDSTATUS4"]
pub type Cardstatus4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc CARDSTATUS4"]
    #[inline(always)]
    pub fn cardstatus4(&self) -> Cardstatus4R {
        Cardstatus4R::new(self.bits)
    }
}
impl W {}
#[doc = "desc RESP3\n\nYou can [`read`](crate::Reg::read) this register and get [`resp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp3Spec;
impl crate::RegisterSpec for Resp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp3::R`](R) reader structure"]
impl crate::Readable for Resp3Spec {}
#[doc = "`write(|w| ..)` method takes [`resp3::W`](W) writer structure"]
impl crate::Writable for Resp3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESP3 to value 0"]
impl crate::Resettable for Resp3Spec {}
