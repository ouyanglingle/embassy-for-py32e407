#[doc = "Register `LCD0_CMDR` reader"]
pub type R = crate::R<Lcd0CmdrSpec>;
#[doc = "Register `LCD0_CMDR` writer"]
pub type W = crate::W<Lcd0CmdrSpec>;
#[doc = "Field `CMD` reader - LCD0 command register"]
pub type CmdR = crate::FieldReader;
#[doc = "Field `CMD` writer - LCD0 command register"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - LCD0 command register"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LCD0 command register"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, Lcd0CmdrSpec> {
        CmdW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd0_cmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd0_cmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcd0CmdrSpec;
impl crate::RegisterSpec for Lcd0CmdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd0_cmdr::R`](R) reader structure"]
impl crate::Readable for Lcd0CmdrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd0_cmdr::W`](W) writer structure"]
impl crate::Writable for Lcd0CmdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD0_CMDR to value 0"]
impl crate::Resettable for Lcd0CmdrSpec {}
