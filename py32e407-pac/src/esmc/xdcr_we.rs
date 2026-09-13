#[doc = "Register `XDCR_WE` reader"]
pub type R = crate::R<XdcrWeSpec>;
#[doc = "Register `XDCR_WE` writer"]
pub type W = crate::W<XdcrWeSpec>;
#[doc = "Field `XDUMMY` reader - xip write dummy cycle set"]
pub type XdummyR = crate::FieldReader;
#[doc = "Field `XDUMMY` writer - xip write dummy cycle set"]
pub type XdummyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NO_CMD` reader - xip write no command"]
pub type NoCmdR = crate::BitReader;
#[doc = "Field `NO_CMD` writer - xip write no command"]
pub type NoCmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - xip write dummy cycle set"]
    #[inline(always)]
    pub fn xdummy(&self) -> XdummyR {
        XdummyR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - xip write no command"]
    #[inline(always)]
    pub fn no_cmd(&self) -> NoCmdR {
        NoCmdR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - xip write dummy cycle set"]
    #[inline(always)]
    pub fn xdummy(&mut self) -> XdummyW<'_, XdcrWeSpec> {
        XdummyW::new(self, 0)
    }
    #[doc = "Bit 6 - xip write no command"]
    #[inline(always)]
    pub fn no_cmd(&mut self) -> NoCmdW<'_, XdcrWeSpec> {
        NoCmdW::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xdcr_we::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xdcr_we::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XdcrWeSpec;
impl crate::RegisterSpec for XdcrWeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xdcr_we::R`](R) reader structure"]
impl crate::Readable for XdcrWeSpec {}
#[doc = "`write(|w| ..)` method takes [`xdcr_we::W`](W) writer structure"]
impl crate::Writable for XdcrWeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XDCR_WE to value 0"]
impl crate::Resettable for XdcrWeSpec {}
