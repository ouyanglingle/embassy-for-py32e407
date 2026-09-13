#[doc = "Register `OFR3` reader"]
pub type R = crate::R<Ofr3Spec>;
#[doc = "Register `OFR3` writer"]
pub type W = crate::W<Ofr3Spec>;
#[doc = "Field `OFFSET3` reader - "]
pub type Offset3R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET3` writer - "]
pub type Offset3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `OFFSET3_CH` reader - "]
pub type Offset3ChR = crate::FieldReader;
#[doc = "Field `OFFSET3_CH` writer - "]
pub type Offset3ChW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OFFSET3_EN` reader - "]
pub type Offset3EnR = crate::BitReader;
#[doc = "Field `OFFSET3_EN` writer - "]
pub type Offset3EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn offset3(&self) -> Offset3R {
        Offset3R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 26:30"]
    #[inline(always)]
    pub fn offset3_ch(&self) -> Offset3ChR {
        Offset3ChR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn offset3_en(&self) -> Offset3EnR {
        Offset3EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn offset3(&mut self) -> Offset3W<'_, Ofr3Spec> {
        Offset3W::new(self, 0)
    }
    #[doc = "Bits 26:30"]
    #[inline(always)]
    pub fn offset3_ch(&mut self) -> Offset3ChW<'_, Ofr3Spec> {
        Offset3ChW::new(self, 26)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn offset3_en(&mut self) -> Offset3EnW<'_, Ofr3Spec> {
        Offset3EnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ofr3Spec;
impl crate::RegisterSpec for Ofr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ofr3::R`](R) reader structure"]
impl crate::Readable for Ofr3Spec {}
#[doc = "`write(|w| ..)` method takes [`ofr3::W`](W) writer structure"]
impl crate::Writable for Ofr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OFR3 to value 0"]
impl crate::Resettable for Ofr3Spec {}
