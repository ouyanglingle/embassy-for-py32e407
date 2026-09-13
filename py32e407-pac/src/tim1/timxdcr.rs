#[doc = "Register `TIMXDCR` reader"]
pub type R = crate::R<TimxdcrSpec>;
#[doc = "Register `TIMXDCR` writer"]
pub type W = crate::W<TimxdcrSpec>;
#[doc = "Field `DBA` reader - "]
pub type DbaR = crate::FieldReader;
#[doc = "Field `DBA` writer - "]
pub type DbaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBL` reader - "]
pub type DblR = crate::FieldReader;
#[doc = "Field `DBL` writer - "]
pub type DblW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn dba(&self) -> DbaR {
        DbaR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn dbl(&self) -> DblR {
        DblR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn dba(&mut self) -> DbaW<'_, TimxdcrSpec> {
        DbaW::new(self, 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn dbl(&mut self) -> DblW<'_, TimxdcrSpec> {
        DblW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`timxdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timxdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimxdcrSpec;
impl crate::RegisterSpec for TimxdcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`timxdcr::R`](R) reader structure"]
impl crate::Readable for TimxdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`timxdcr::W`](W) writer structure"]
impl crate::Writable for TimxdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMXDCR to value 0"]
impl crate::Resettable for TimxdcrSpec {}
