#[doc = "Register `OFR4` reader"]
pub type R = crate::R<Ofr4Spec>;
#[doc = "Register `OFR4` writer"]
pub type W = crate::W<Ofr4Spec>;
#[doc = "Field `OFFSET4` reader - "]
pub type Offset4R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET4` writer - "]
pub type Offset4W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `OFFSET4_CH` reader - "]
pub type Offset4ChR = crate::FieldReader;
#[doc = "Field `OFFSET4_CH` writer - "]
pub type Offset4ChW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OFFSET4_EN` reader - "]
pub type Offset4EnR = crate::BitReader;
#[doc = "Field `OFFSET4_EN` writer - "]
pub type Offset4EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn offset4(&self) -> Offset4R {
        Offset4R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 26:30"]
    #[inline(always)]
    pub fn offset4_ch(&self) -> Offset4ChR {
        Offset4ChR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn offset4_en(&self) -> Offset4EnR {
        Offset4EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn offset4(&mut self) -> Offset4W<'_, Ofr4Spec> {
        Offset4W::new(self, 0)
    }
    #[doc = "Bits 26:30"]
    #[inline(always)]
    pub fn offset4_ch(&mut self) -> Offset4ChW<'_, Ofr4Spec> {
        Offset4ChW::new(self, 26)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn offset4_en(&mut self) -> Offset4EnW<'_, Ofr4Spec> {
        Offset4EnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ofr4Spec;
impl crate::RegisterSpec for Ofr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ofr4::R`](R) reader structure"]
impl crate::Readable for Ofr4Spec {}
#[doc = "`write(|w| ..)` method takes [`ofr4::W`](W) writer structure"]
impl crate::Writable for Ofr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OFR4 to value 0"]
impl crate::Resettable for Ofr4Spec {}
