#[doc = "Register `INTC` reader"]
pub type R = crate::R<IntcSpec>;
#[doc = "Register `INTC` writer"]
pub type W = crate::W<IntcSpec>;
#[doc = "Field `CKOKIC` reader - CKOKIF interrupt clear bit"]
pub type CkokicR = crate::BitReader;
#[doc = "Field `CKOKIC` writer - CKOKIF interrupt clear bit"]
pub type CkokicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKWARNIC` reader - CKWARNIF interrupt clear bit"]
pub type CkwarnicR = crate::BitReader;
#[doc = "Field `CKWARNIC` writer - CKWARNIF interrupt clear bit"]
pub type CkwarnicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIC` reader - ERRIF interrupt clear bit"]
pub type ErricR = crate::BitReader;
#[doc = "Field `ERRIC` writer - ERRIF interrupt clear bit"]
pub type ErricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EREFIC` reader - EREFIF interrupt clear bit"]
pub type EreficR = crate::BitReader;
#[doc = "Field `EREFIC` writer - EREFIF interrupt clear bit"]
pub type EreficW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CKOKIF interrupt clear bit"]
    #[inline(always)]
    pub fn ckokic(&self) -> CkokicR {
        CkokicR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CKWARNIF interrupt clear bit"]
    #[inline(always)]
    pub fn ckwarnic(&self) -> CkwarnicR {
        CkwarnicR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ERRIF interrupt clear bit"]
    #[inline(always)]
    pub fn erric(&self) -> ErricR {
        ErricR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EREFIF interrupt clear bit"]
    #[inline(always)]
    pub fn erefic(&self) -> EreficR {
        EreficR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CKOKIF interrupt clear bit"]
    #[inline(always)]
    pub fn ckokic(&mut self) -> CkokicW<'_, IntcSpec> {
        CkokicW::new(self, 0)
    }
    #[doc = "Bit 1 - CKWARNIF interrupt clear bit"]
    #[inline(always)]
    pub fn ckwarnic(&mut self) -> CkwarnicW<'_, IntcSpec> {
        CkwarnicW::new(self, 1)
    }
    #[doc = "Bit 2 - ERRIF interrupt clear bit"]
    #[inline(always)]
    pub fn erric(&mut self) -> ErricW<'_, IntcSpec> {
        ErricW::new(self, 2)
    }
    #[doc = "Bit 3 - EREFIF interrupt clear bit"]
    #[inline(always)]
    pub fn erefic(&mut self) -> EreficW<'_, IntcSpec> {
        EreficW::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`intc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcSpec;
impl crate::RegisterSpec for IntcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc::R`](R) reader structure"]
impl crate::Readable for IntcSpec {}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for IntcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for IntcSpec {}
