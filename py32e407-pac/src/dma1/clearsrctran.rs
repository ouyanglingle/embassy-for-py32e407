#[doc = "Register `CLEARSRCTRAN` reader"]
pub type R = crate::R<ClearsrctranSpec>;
#[doc = "Register `CLEARSRCTRAN` writer"]
pub type W = crate::W<ClearsrctranSpec>;
#[doc = "Field `CLEAR` reader - Clear for IntSrcTran Interrupt"]
pub type ClearR = crate::FieldReader;
#[doc = "Field `CLEAR` writer - Clear for IntSrcTran Interrupt"]
pub type ClearW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Clear for IntSrcTran Interrupt"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clear for IntSrcTran Interrupt"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<'_, ClearsrctranSpec> {
        ClearW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clearsrctran::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearsrctran::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearsrctranSpec;
impl crate::RegisterSpec for ClearsrctranSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clearsrctran::R`](R) reader structure"]
impl crate::Readable for ClearsrctranSpec {}
#[doc = "`write(|w| ..)` method takes [`clearsrctran::W`](W) writer structure"]
impl crate::Writable for ClearsrctranSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLEARSRCTRAN to value 0"]
impl crate::Resettable for ClearsrctranSpec {}
