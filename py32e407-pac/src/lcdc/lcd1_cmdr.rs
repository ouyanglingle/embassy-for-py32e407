#[doc = "Register `LCD1_CMDR` reader"]
pub type R = crate::R<Lcd1CmdrSpec>;
#[doc = "Register `LCD1_CMDR` writer"]
pub type W = crate::W<Lcd1CmdrSpec>;
#[doc = "Field `CMD` reader - LCD1 command register"]
pub type CmdR = crate::FieldReader;
#[doc = "Field `CMD` writer - LCD1 command register"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - LCD1 command register"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LCD1 command register"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, Lcd1CmdrSpec> {
        CmdW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd1_cmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd1_cmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcd1CmdrSpec;
impl crate::RegisterSpec for Lcd1CmdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd1_cmdr::R`](R) reader structure"]
impl crate::Readable for Lcd1CmdrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd1_cmdr::W`](W) writer structure"]
impl crate::Writable for Lcd1CmdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD1_CMDR to value 0"]
impl crate::Resettable for Lcd1CmdrSpec {}
