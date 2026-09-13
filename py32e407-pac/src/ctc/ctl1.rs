#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `RLVALUE` reader - CTC counter reload value"]
pub type RlvalueR = crate::FieldReader<u16>;
#[doc = "Field `RLVALUE` writer - CTC counter reload value"]
pub type RlvalueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CKLIM` reader - Clock calibration time base limit"]
pub type CklimR = crate::FieldReader;
#[doc = "Field `CKLIM` writer - Clock calibration time base limit"]
pub type CklimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REFPSC` reader - Reference signal source pre division"]
pub type RefpscR = crate::FieldReader;
#[doc = "Field `REFPSC` writer - Reference signal source pre division"]
pub type RefpscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REFSEL` reader - Selection of reference signal source"]
pub type RefselR = crate::FieldReader;
#[doc = "Field `REFSEL` writer - Selection of reference signal source"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REFPOL` reader - Reference signal source polarity"]
pub type RefpolR = crate::BitReader;
#[doc = "Field `REFPOL` writer - Reference signal source polarity"]
pub type RefpolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - CTC counter reload value"]
    #[inline(always)]
    pub fn rlvalue(&self) -> RlvalueR {
        RlvalueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Clock calibration time base limit"]
    #[inline(always)]
    pub fn cklim(&self) -> CklimR {
        CklimR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Reference signal source pre division"]
    #[inline(always)]
    pub fn refpsc(&self) -> RefpscR {
        RefpscR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Selection of reference signal source"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Reference signal source polarity"]
    #[inline(always)]
    pub fn refpol(&self) -> RefpolR {
        RefpolR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CTC counter reload value"]
    #[inline(always)]
    pub fn rlvalue(&mut self) -> RlvalueW<'_, Ctl1Spec> {
        RlvalueW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Clock calibration time base limit"]
    #[inline(always)]
    pub fn cklim(&mut self) -> CklimW<'_, Ctl1Spec> {
        CklimW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Reference signal source pre division"]
    #[inline(always)]
    pub fn refpsc(&mut self) -> RefpscW<'_, Ctl1Spec> {
        RefpscW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Selection of reference signal source"]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<'_, Ctl1Spec> {
        RefselW::new(self, 28)
    }
    #[doc = "Bit 31 - Reference signal source polarity"]
    #[inline(always)]
    pub fn refpol(&mut self) -> RefpolW<'_, Ctl1Spec> {
        RefpolW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {}
