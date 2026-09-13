#[doc = "Register `RESP1` reader"]
pub type R = crate::R<Resp1Spec>;
#[doc = "Register `RESP1` writer"]
pub type W = crate::W<Resp1Spec>;
#[doc = "Field `CARDSTATUS2` reader - desc CARDSTATUS2"]
pub type Cardstatus2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc CARDSTATUS2"]
    #[inline(always)]
    pub fn cardstatus2(&self) -> Cardstatus2R {
        Cardstatus2R::new(self.bits)
    }
}
impl W {}
#[doc = "desc RESP1\n\nYou can [`read`](crate::Reg::read) this register and get [`resp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp1Spec;
impl crate::RegisterSpec for Resp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp1::R`](R) reader structure"]
impl crate::Readable for Resp1Spec {}
#[doc = "`write(|w| ..)` method takes [`resp1::W`](W) writer structure"]
impl crate::Writable for Resp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESP1 to value 0"]
impl crate::Resettable for Resp1Spec {}
