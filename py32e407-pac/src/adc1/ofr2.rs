#[doc = "Register `OFR2` reader"]
pub type R = crate::R<Ofr2Spec>;
#[doc = "Register `OFR2` writer"]
pub type W = crate::W<Ofr2Spec>;
#[doc = "Field `OFFSET2` reader - "]
pub type Offset2R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET2` writer - "]
pub type Offset2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `OFFSET2_CH` reader - "]
pub type Offset2ChR = crate::FieldReader;
#[doc = "Field `OFFSET2_CH` writer - "]
pub type Offset2ChW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OFFSET2_EN` reader - "]
pub type Offset2EnR = crate::BitReader;
#[doc = "Field `OFFSET2_EN` writer - "]
pub type Offset2EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn offset2(&self) -> Offset2R {
        Offset2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 26:30"]
    #[inline(always)]
    pub fn offset2_ch(&self) -> Offset2ChR {
        Offset2ChR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn offset2_en(&self) -> Offset2EnR {
        Offset2EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn offset2(&mut self) -> Offset2W<'_, Ofr2Spec> {
        Offset2W::new(self, 0)
    }
    #[doc = "Bits 26:30"]
    #[inline(always)]
    pub fn offset2_ch(&mut self) -> Offset2ChW<'_, Ofr2Spec> {
        Offset2ChW::new(self, 26)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn offset2_en(&mut self) -> Offset2EnW<'_, Ofr2Spec> {
        Offset2EnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ofr2Spec;
impl crate::RegisterSpec for Ofr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ofr2::R`](R) reader structure"]
impl crate::Readable for Ofr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ofr2::W`](W) writer structure"]
impl crate::Writable for Ofr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OFR2 to value 0"]
impl crate::Resettable for Ofr2Spec {}
