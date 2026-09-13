#[doc = "Register `XMODE` reader"]
pub type R = crate::R<XmodeSpec>;
#[doc = "Register `XMODE` writer"]
pub type W = crate::W<XmodeSpec>;
#[doc = "Field `XMODE` reader - xip send modereg"]
pub type XmodeR = crate::FieldReader;
#[doc = "Field `XMODE` writer - xip send modereg"]
pub type XmodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - xip send modereg"]
    #[inline(always)]
    pub fn xmode(&self) -> XmodeR {
        XmodeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - xip send modereg"]
    #[inline(always)]
    pub fn xmode(&mut self) -> XmodeW<'_, XmodeSpec> {
        XmodeW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XmodeSpec;
impl crate::RegisterSpec for XmodeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xmode::R`](R) reader structure"]
impl crate::Readable for XmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`xmode::W`](W) writer structure"]
impl crate::Writable for XmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XMODE to value 0"]
impl crate::Resettable for XmodeSpec {}
