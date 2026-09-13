#[doc = "Register `TRISE` reader"]
pub type R = crate::R<TriseSpec>;
#[doc = "Register `TRISE` writer"]
pub type W = crate::W<TriseSpec>;
#[doc = "Field `TRISE` reader - desc TRISE"]
pub type TriseR = crate::FieldReader;
#[doc = "Field `TRISE` writer - desc TRISE"]
pub type TriseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `THOLDDATA` reader - THOLDDATA"]
pub type TholddataR = crate::FieldReader;
#[doc = "Field `THOLDDATA` writer - THOLDDATA"]
pub type TholddataW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `THOLDDATA_SEL` reader - THOLDDATA_SEL"]
pub type TholddataSelR = crate::BitReader;
#[doc = "Field `THOLDDATA_SEL` writer - THOLDDATA_SEL"]
pub type TholddataSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - desc TRISE"]
    #[inline(always)]
    pub fn trise(&self) -> TriseR {
        TriseR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 7:11 - THOLDDATA"]
    #[inline(always)]
    pub fn tholddata(&self) -> TholddataR {
        TholddataR::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bit 12 - THOLDDATA_SEL"]
    #[inline(always)]
    pub fn tholddata_sel(&self) -> TholddataSelR {
        TholddataSelR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc TRISE"]
    #[inline(always)]
    pub fn trise(&mut self) -> TriseW<'_, TriseSpec> {
        TriseW::new(self, 0)
    }
    #[doc = "Bits 7:11 - THOLDDATA"]
    #[inline(always)]
    pub fn tholddata(&mut self) -> TholddataW<'_, TriseSpec> {
        TholddataW::new(self, 7)
    }
    #[doc = "Bit 12 - THOLDDATA_SEL"]
    #[inline(always)]
    pub fn tholddata_sel(&mut self) -> TholddataSelW<'_, TriseSpec> {
        TholddataSelW::new(self, 12)
    }
}
#[doc = "desc TRISE\n\nYou can [`read`](crate::Reg::read) this register and get [`trise::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trise::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TriseSpec;
impl crate::RegisterSpec for TriseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trise::R`](R) reader structure"]
impl crate::Readable for TriseSpec {}
#[doc = "`write(|w| ..)` method takes [`trise::W`](W) writer structure"]
impl crate::Writable for TriseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRISE to value 0"]
impl crate::Resettable for TriseSpec {}
