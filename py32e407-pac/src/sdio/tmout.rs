#[doc = "Register `TMOUT` reader"]
pub type R = crate::R<TmoutSpec>;
#[doc = "Register `TMOUT` writer"]
pub type W = crate::W<TmoutSpec>;
#[doc = "Field `RESPTIME` reader - desc RESPTIME"]
pub type ResptimeR = crate::FieldReader;
#[doc = "Field `RESPTIME` writer - desc RESPTIME"]
pub type ResptimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATATIME` reader - desc DATATIME"]
pub type DatatimeR = crate::FieldReader<u32>;
#[doc = "Field `DATATIME` writer - desc DATATIME"]
pub type DatatimeW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - desc RESPTIME"]
    #[inline(always)]
    pub fn resptime(&self) -> ResptimeR {
        ResptimeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - desc DATATIME"]
    #[inline(always)]
    pub fn datatime(&self) -> DatatimeR {
        DatatimeR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc RESPTIME"]
    #[inline(always)]
    pub fn resptime(&mut self) -> ResptimeW<'_, TmoutSpec> {
        ResptimeW::new(self, 0)
    }
    #[doc = "Bits 8:31 - desc DATATIME"]
    #[inline(always)]
    pub fn datatime(&mut self) -> DatatimeW<'_, TmoutSpec> {
        DatatimeW::new(self, 8)
    }
}
#[doc = "desc TMOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`tmout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmoutSpec;
impl crate::RegisterSpec for TmoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmout::R`](R) reader structure"]
impl crate::Readable for TmoutSpec {}
#[doc = "`write(|w| ..)` method takes [`tmout::W`](W) writer structure"]
impl crate::Writable for TmoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMOUT to value 0"]
impl crate::Resettable for TmoutSpec {}
