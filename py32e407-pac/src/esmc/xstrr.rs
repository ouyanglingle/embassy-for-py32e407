#[doc = "Register `XSTRR` reader"]
pub type R = crate::R<XstrrSpec>;
#[doc = "Register `XSTRR` writer"]
pub type W = crate::W<XstrrSpec>;
#[doc = "Field `XSMP` reader - xip sample sample"]
pub type XsmpR = crate::FieldReader;
#[doc = "Field `XSMP` writer - xip sample sample"]
pub type XsmpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - xip sample sample"]
    #[inline(always)]
    pub fn xsmp(&self) -> XsmpR {
        XsmpR::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - xip sample sample"]
    #[inline(always)]
    pub fn xsmp(&mut self) -> XsmpW<'_, XstrrSpec> {
        XsmpW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xstrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xstrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XstrrSpec;
impl crate::RegisterSpec for XstrrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xstrr::R`](R) reader structure"]
impl crate::Readable for XstrrSpec {}
#[doc = "`write(|w| ..)` method takes [`xstrr::W`](W) writer structure"]
impl crate::Writable for XstrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XSTRR to value 0"]
impl crate::Resettable for XstrrSpec {}
