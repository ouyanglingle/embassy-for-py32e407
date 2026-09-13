#[doc = "Register `XMODE_WE` reader"]
pub type R = crate::R<XmodeWeSpec>;
#[doc = "Register `XMODE_WE` writer"]
pub type W = crate::W<XmodeWeSpec>;
#[doc = "Field `XMODE_WE` reader - xip send modereg_we"]
pub type XmodeWeR = crate::FieldReader;
#[doc = "Field `XMODE_WE` writer - xip send modereg_we"]
pub type XmodeWeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - xip send modereg_we"]
    #[inline(always)]
    pub fn xmode_we(&self) -> XmodeWeR {
        XmodeWeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - xip send modereg_we"]
    #[inline(always)]
    pub fn xmode_we(&mut self) -> XmodeWeW<'_, XmodeWeSpec> {
        XmodeWeW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xmode_we::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xmode_we::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XmodeWeSpec;
impl crate::RegisterSpec for XmodeWeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xmode_we::R`](R) reader structure"]
impl crate::Readable for XmodeWeSpec {}
#[doc = "`write(|w| ..)` method takes [`xmode_we::W`](W) writer structure"]
impl crate::Writable for XmodeWeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XMODE_WE to value 0"]
impl crate::Resettable for XmodeWeSpec {}
