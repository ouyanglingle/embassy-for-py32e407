#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `FREQ` reader - desc FREQ"]
pub type FreqR = crate::FieldReader;
#[doc = "Field `FREQ` writer - desc FREQ"]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ITERREN` reader - desc ITERREN"]
pub type IterrenR = crate::BitReader;
#[doc = "Field `ITERREN` writer - desc ITERREN"]
pub type IterrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITEVTEN` reader - desc ITEVTEN"]
pub type ItevtenR = crate::BitReader;
#[doc = "Field `ITEVTEN` writer - desc ITEVTEN"]
pub type ItevtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITBUFEN` reader - desc ITBUFEN"]
pub type ItbufenR = crate::BitReader;
#[doc = "Field `ITBUFEN` writer - desc ITBUFEN"]
pub type ItbufenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - desc DMAEN"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - desc DMAEN"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAST` reader - desc LAST"]
pub type LastR = crate::BitReader;
#[doc = "Field `LAST` writer - desc LAST"]
pub type LastW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - desc FREQ"]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - desc ITERREN"]
    #[inline(always)]
    pub fn iterren(&self) -> IterrenR {
        IterrenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ITEVTEN"]
    #[inline(always)]
    pub fn itevten(&self) -> ItevtenR {
        ItevtenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ITBUFEN"]
    #[inline(always)]
    pub fn itbufen(&self) -> ItbufenR {
        ItbufenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc LAST"]
    #[inline(always)]
    pub fn last(&self) -> LastR {
        LastR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc FREQ"]
    #[inline(always)]
    pub fn freq(&mut self) -> FreqW<'_, Cr2Spec> {
        FreqW::new(self, 0)
    }
    #[doc = "Bit 8 - desc ITERREN"]
    #[inline(always)]
    pub fn iterren(&mut self) -> IterrenW<'_, Cr2Spec> {
        IterrenW::new(self, 8)
    }
    #[doc = "Bit 9 - desc ITEVTEN"]
    #[inline(always)]
    pub fn itevten(&mut self) -> ItevtenW<'_, Cr2Spec> {
        ItevtenW::new(self, 9)
    }
    #[doc = "Bit 10 - desc ITBUFEN"]
    #[inline(always)]
    pub fn itbufen(&mut self) -> ItbufenW<'_, Cr2Spec> {
        ItbufenW::new(self, 10)
    }
    #[doc = "Bit 11 - desc DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, Cr2Spec> {
        DmaenW::new(self, 11)
    }
    #[doc = "Bit 12 - desc LAST"]
    #[inline(always)]
    pub fn last(&mut self) -> LastW<'_, Cr2Spec> {
        LastW::new(self, 12)
    }
}
#[doc = "desc CR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
