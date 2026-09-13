#[doc = "Register `TIMEOUTR` reader"]
pub type R = crate::R<TimeoutrSpec>;
#[doc = "Register `TIMEOUTR` writer"]
pub type W = crate::W<TimeoutrSpec>;
#[doc = "Field `TIMEOUTA` reader - TIMEOUTA"]
pub type TimeoutaR = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUTA` writer - TIMEOUTA"]
pub type TimeoutaW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TIDLE` reader - TIDLE"]
pub type TidleR = crate::BitReader;
#[doc = "Field `TIDLE` writer - TIDLE"]
pub type TidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTEN` reader - TIMEOUTEN"]
pub type TimeoutenR = crate::BitReader;
#[doc = "Field `TIMEOUTEN` writer - TIMEOUTEN"]
pub type TimeoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTB` reader - TIMEOUTB"]
pub type TimeoutbR = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUTB` writer - TIMEOUTB"]
pub type TimeoutbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TEXTEN` reader - TEXTEN"]
pub type TextenR = crate::BitReader;
#[doc = "Field `TEXTEN` writer - TEXTEN"]
pub type TextenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - TIMEOUTA"]
    #[inline(always)]
    pub fn timeouta(&self) -> TimeoutaR {
        TimeoutaR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - TIDLE"]
    #[inline(always)]
    pub fn tidle(&self) -> TidleR {
        TidleR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - TIMEOUTEN"]
    #[inline(always)]
    pub fn timeouten(&self) -> TimeoutenR {
        TimeoutenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - TIMEOUTB"]
    #[inline(always)]
    pub fn timeoutb(&self) -> TimeoutbR {
        TimeoutbR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - TEXTEN"]
    #[inline(always)]
    pub fn texten(&self) -> TextenR {
        TextenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - TIMEOUTA"]
    #[inline(always)]
    pub fn timeouta(&mut self) -> TimeoutaW<'_, TimeoutrSpec> {
        TimeoutaW::new(self, 0)
    }
    #[doc = "Bit 12 - TIDLE"]
    #[inline(always)]
    pub fn tidle(&mut self) -> TidleW<'_, TimeoutrSpec> {
        TidleW::new(self, 12)
    }
    #[doc = "Bit 15 - TIMEOUTEN"]
    #[inline(always)]
    pub fn timeouten(&mut self) -> TimeoutenW<'_, TimeoutrSpec> {
        TimeoutenW::new(self, 15)
    }
    #[doc = "Bits 16:27 - TIMEOUTB"]
    #[inline(always)]
    pub fn timeoutb(&mut self) -> TimeoutbW<'_, TimeoutrSpec> {
        TimeoutbW::new(self, 16)
    }
    #[doc = "Bit 31 - TEXTEN"]
    #[inline(always)]
    pub fn texten(&mut self) -> TextenW<'_, TimeoutrSpec> {
        TextenW::new(self, 31)
    }
}
#[doc = "I2C timeout register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeoutr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeoutr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeoutrSpec;
impl crate::RegisterSpec for TimeoutrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeoutr::R`](R) reader structure"]
impl crate::Readable for TimeoutrSpec {}
#[doc = "`write(|w| ..)` method takes [`timeoutr::W`](W) writer structure"]
impl crate::Writable for TimeoutrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMEOUTR to value 0"]
impl crate::Resettable for TimeoutrSpec {}
