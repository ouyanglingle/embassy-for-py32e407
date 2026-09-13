#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `EOSMPIE` reader - "]
pub type EosmpieR = crate::BitReader;
#[doc = "Field `EOSMPIE` writer - "]
pub type EosmpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCIE` reader - "]
pub type EocieR = crate::BitReader;
#[doc = "Field `EOCIE` writer - "]
pub type EocieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSIE` reader - "]
pub type EosieR = crate::BitReader;
#[doc = "Field `EOSIE` writer - "]
pub type EosieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOCIE` reader - "]
pub type JeocieR = crate::BitReader;
#[doc = "Field `JEOCIE` writer - "]
pub type JeocieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOSIE` reader - "]
pub type JeosieR = crate::BitReader;
#[doc = "Field `JEOSIE` writer - "]
pub type JeosieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1IE` reader - "]
pub type Awd1ieR = crate::BitReader;
#[doc = "Field `AWD1IE` writer - "]
pub type Awd1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD2IE` reader - "]
pub type Awd2ieR = crate::BitReader;
#[doc = "Field `AWD2IE` writer - "]
pub type Awd2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD3IE` reader - "]
pub type Awd3ieR = crate::BitReader;
#[doc = "Field `AWD3IE` writer - "]
pub type Awd3ieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn eosmpie(&self) -> EosmpieR {
        EosmpieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn eocie(&self) -> EocieR {
        EocieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn eosie(&self) -> EosieR {
        EosieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn jeocie(&self) -> JeocieR {
        JeocieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn jeosie(&self) -> JeosieR {
        JeosieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn awd1ie(&self) -> Awd1ieR {
        Awd1ieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn awd2ie(&self) -> Awd2ieR {
        Awd2ieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn awd3ie(&self) -> Awd3ieR {
        Awd3ieR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn eosmpie(&mut self) -> EosmpieW<'_, IerSpec> {
        EosmpieW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn eocie(&mut self) -> EocieW<'_, IerSpec> {
        EocieW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn eosie(&mut self) -> EosieW<'_, IerSpec> {
        EosieW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn jeocie(&mut self) -> JeocieW<'_, IerSpec> {
        JeocieW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn jeosie(&mut self) -> JeosieW<'_, IerSpec> {
        JeosieW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn awd1ie(&mut self) -> Awd1ieW<'_, IerSpec> {
        Awd1ieW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn awd2ie(&mut self) -> Awd2ieW<'_, IerSpec> {
        Awd2ieW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn awd3ie(&mut self) -> Awd3ieW<'_, IerSpec> {
        Awd3ieW::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
