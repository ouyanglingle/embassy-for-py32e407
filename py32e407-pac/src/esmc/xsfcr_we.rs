#[doc = "Register `XSFCR_WE` reader"]
pub type R = crate::R<XsfcrWeSpec>;
#[doc = "Register `XSFCR_WE` writer"]
pub type W = crate::W<XsfcrWeSpec>;
#[doc = "Field `XINS_WE` reader - XIP write command"]
pub type XinsWeR = crate::FieldReader;
#[doc = "Field `XINS_WE` writer - XIP write command"]
pub type XinsWeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - XIP write command"]
    #[inline(always)]
    pub fn xins_we(&self) -> XinsWeR {
        XinsWeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - XIP write command"]
    #[inline(always)]
    pub fn xins_we(&mut self) -> XinsWeW<'_, XsfcrWeSpec> {
        XinsWeW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xsfcr_we::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xsfcr_we::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XsfcrWeSpec;
impl crate::RegisterSpec for XsfcrWeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xsfcr_we::R`](R) reader structure"]
impl crate::Readable for XsfcrWeSpec {}
#[doc = "`write(|w| ..)` method takes [`xsfcr_we::W`](W) writer structure"]
impl crate::Writable for XsfcrWeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XSFCR_WE to value 0"]
impl crate::Resettable for XsfcrWeSpec {}
