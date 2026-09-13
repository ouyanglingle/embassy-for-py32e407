#[doc = "Register `DCR` reader"]
pub type R = crate::R<DcrSpec>;
#[doc = "Register `DCR` writer"]
pub type W = crate::W<DcrSpec>;
#[doc = "Field `DUMMY` reader - dummy cycyle set"]
pub type DummyR = crate::FieldReader;
#[doc = "Field `DUMMY` writer - dummy cycyle set"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NO_CMD` reader - no command"]
pub type NoCmdR = crate::BitReader;
#[doc = "Field `NO_CMD` writer - no command"]
pub type NoCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REC` reader - receive enable"]
pub type RecR = crate::BitReader;
#[doc = "Field `REC` writer - receive enable"]
pub type RecW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - dummy cycyle set"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - no command"]
    #[inline(always)]
    pub fn no_cmd(&self) -> NoCmdR {
        NoCmdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - receive enable"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - dummy cycyle set"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, DcrSpec> {
        DummyW::new(self, 0)
    }
    #[doc = "Bit 6 - no command"]
    #[inline(always)]
    pub fn no_cmd(&mut self) -> NoCmdW<'_, DcrSpec> {
        NoCmdW::new(self, 6)
    }
    #[doc = "Bit 7 - receive enable"]
    #[inline(always)]
    pub fn rec(&mut self) -> RecW<'_, DcrSpec> {
        RecW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrSpec;
impl crate::RegisterSpec for DcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dcr::R`](R) reader structure"]
impl crate::Readable for DcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcr::W`](W) writer structure"]
impl crate::Writable for DcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DcrSpec {}
