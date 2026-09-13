#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `EOSMP` reader - "]
pub type EosmpR = crate::BitReader;
#[doc = "Field `EOSMP` writer - "]
pub type EosmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC` reader - "]
pub type EocR = crate::BitReader;
#[doc = "Field `EOC` writer - "]
pub type EocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS` reader - "]
pub type EosR = crate::BitReader;
#[doc = "Field `EOS` writer - "]
pub type EosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOC` reader - "]
pub type JeocR = crate::BitReader;
#[doc = "Field `JEOC` writer - "]
pub type JeocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOS` reader - "]
pub type JeosR = crate::BitReader;
#[doc = "Field `JEOS` writer - "]
pub type JeosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1` reader - "]
pub type Awd1R = crate::BitReader;
#[doc = "Field `AWD1` writer - "]
pub type Awd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD2` reader - "]
pub type Awd2R = crate::BitReader;
#[doc = "Field `AWD2` writer - "]
pub type Awd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD3` reader - "]
pub type Awd3R = crate::BitReader;
#[doc = "Field `AWD3` writer - "]
pub type Awd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JSTRT` reader - "]
pub type JstrtR = crate::BitReader;
#[doc = "Field `JSTRT` writer - "]
pub type JstrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRT` reader - "]
pub type StrtR = crate::BitReader;
#[doc = "Field `STRT` writer - "]
pub type StrtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn eosmp(&self) -> EosmpR {
        EosmpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn eos(&self) -> EosR {
        EosR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn jeoc(&self) -> JeocR {
        JeocR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn jeos(&self) -> JeosR {
        JeosR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn awd1(&self) -> Awd1R {
        Awd1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn awd2(&self) -> Awd2R {
        Awd2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn awd3(&self) -> Awd3R {
        Awd3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn jstrt(&self) -> JstrtR {
        JstrtR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn strt(&self) -> StrtR {
        StrtR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn eosmp(&mut self) -> EosmpW<'_, IsrSpec> {
        EosmpW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EocW<'_, IsrSpec> {
        EocW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn eos(&mut self) -> EosW<'_, IsrSpec> {
        EosW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn jeoc(&mut self) -> JeocW<'_, IsrSpec> {
        JeocW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn jeos(&mut self) -> JeosW<'_, IsrSpec> {
        JeosW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn awd1(&mut self) -> Awd1W<'_, IsrSpec> {
        Awd1W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn awd2(&mut self) -> Awd2W<'_, IsrSpec> {
        Awd2W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn awd3(&mut self) -> Awd3W<'_, IsrSpec> {
        Awd3W::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn jstrt(&mut self) -> JstrtW<'_, IsrSpec> {
        JstrtW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn strt(&mut self) -> StrtW<'_, IsrSpec> {
        StrtW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
