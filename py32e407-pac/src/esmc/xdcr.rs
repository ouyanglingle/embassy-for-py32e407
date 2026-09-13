#[doc = "Register `XDCR` reader"]
pub type R = crate::R<XdcrSpec>;
#[doc = "Register `XDCR` writer"]
pub type W = crate::W<XdcrSpec>;
#[doc = "Field `XDUMMY` reader - xip read dummy cycle set"]
pub type XdummyR = crate::FieldReader;
#[doc = "Field `XDUMMY` writer - xip read dummy cycle set"]
pub type XdummyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NO_CMD` reader - xip read no command"]
pub type NoCmdR = crate::BitReader;
#[doc = "Field `NO_CMD` writer - xip read no command"]
pub type NoCmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - xip read dummy cycle set"]
    #[inline(always)]
    pub fn xdummy(&self) -> XdummyR {
        XdummyR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - xip read no command"]
    #[inline(always)]
    pub fn no_cmd(&self) -> NoCmdR {
        NoCmdR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - xip read dummy cycle set"]
    #[inline(always)]
    pub fn xdummy(&mut self) -> XdummyW<'_, XdcrSpec> {
        XdummyW::new(self, 0)
    }
    #[doc = "Bit 6 - xip read no command"]
    #[inline(always)]
    pub fn no_cmd(&mut self) -> NoCmdW<'_, XdcrSpec> {
        NoCmdW::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XdcrSpec;
impl crate::RegisterSpec for XdcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xdcr::R`](R) reader structure"]
impl crate::Readable for XdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`xdcr::W`](W) writer structure"]
impl crate::Writable for XdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XDCR to value 0"]
impl crate::Resettable for XdcrSpec {}
