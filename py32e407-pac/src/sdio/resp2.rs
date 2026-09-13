#[doc = "Register `RESP2` reader"]
pub type R = crate::R<Resp2Spec>;
#[doc = "Register `RESP2` writer"]
pub type W = crate::W<Resp2Spec>;
#[doc = "Field `CARDSTATUS3` reader - desc CARDSTATUS3"]
pub type Cardstatus3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc CARDSTATUS3"]
    #[inline(always)]
    pub fn cardstatus3(&self) -> Cardstatus3R {
        Cardstatus3R::new(self.bits)
    }
}
impl W {}
#[doc = "desc RESP2\n\nYou can [`read`](crate::Reg::read) this register and get [`resp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp2Spec;
impl crate::RegisterSpec for Resp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp2::R`](R) reader structure"]
impl crate::Readable for Resp2Spec {}
#[doc = "`write(|w| ..)` method takes [`resp2::W`](W) writer structure"]
impl crate::Writable for Resp2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESP2 to value 0"]
impl crate::Resettable for Resp2Spec {}
