#[doc = "Register `XSFCR` reader"]
pub type R = crate::R<XsfcrSpec>;
#[doc = "Register `XSFCR` writer"]
pub type W = crate::W<XsfcrSpec>;
#[doc = "Field `XINS` reader - XIP read command"]
pub type XinsR = crate::FieldReader;
#[doc = "Field `XINS` writer - XIP read command"]
pub type XinsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - XIP read command"]
    #[inline(always)]
    pub fn xins(&self) -> XinsR {
        XinsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - XIP read command"]
    #[inline(always)]
    pub fn xins(&mut self) -> XinsW<'_, XsfcrSpec> {
        XinsW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xsfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xsfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XsfcrSpec;
impl crate::RegisterSpec for XsfcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xsfcr::R`](R) reader structure"]
impl crate::Readable for XsfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`xsfcr::W`](W) writer structure"]
impl crate::Writable for XsfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XSFCR to value 0"]
impl crate::Resettable for XsfcrSpec {}
