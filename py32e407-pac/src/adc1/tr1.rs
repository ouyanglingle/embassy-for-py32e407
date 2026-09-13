#[doc = "Register `TR1` reader"]
pub type R = crate::R<Tr1Spec>;
#[doc = "Register `TR1` writer"]
pub type W = crate::W<Tr1Spec>;
#[doc = "Field `LT1` reader - "]
pub type Lt1R = crate::FieldReader<u16>;
#[doc = "Field `LT1` writer - "]
pub type Lt1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AWDFILT` reader - "]
pub type AwdfiltR = crate::FieldReader;
#[doc = "Field `AWDFILT` writer - "]
pub type AwdfiltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HT1` reader - "]
pub type Ht1R = crate::FieldReader<u16>;
#[doc = "Field `HT1` writer - "]
pub type Ht1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn lt1(&self) -> Lt1R {
        Lt1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn awdfilt(&self) -> AwdfiltR {
        AwdfiltR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn ht1(&self) -> Ht1R {
        Ht1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn lt1(&mut self) -> Lt1W<'_, Tr1Spec> {
        Lt1W::new(self, 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn awdfilt(&mut self) -> AwdfiltW<'_, Tr1Spec> {
        AwdfiltW::new(self, 12)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn ht1(&mut self) -> Ht1W<'_, Tr1Spec> {
        Ht1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tr1Spec;
impl crate::RegisterSpec for Tr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr1::R`](R) reader structure"]
impl crate::Readable for Tr1Spec {}
#[doc = "`write(|w| ..)` method takes [`tr1::W`](W) writer structure"]
impl crate::Writable for Tr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TR1 to value 0"]
impl crate::Resettable for Tr1Spec {}
