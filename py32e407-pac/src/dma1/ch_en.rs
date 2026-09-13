#[doc = "Register `CH_EN` reader"]
pub type R = crate::R<ChEnSpec>;
#[doc = "Register `CH_EN` writer"]
pub type W = crate::W<ChEnSpec>;
#[doc = "Field `CH_EN` reader - Channel enable"]
pub type ChEnR = crate::FieldReader;
#[doc = "Field `CH_EN` writer - Channel enable"]
pub type ChEnW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CH_EN_WE` reader - Channel enable register"]
pub type ChEnWeR = crate::FieldReader;
#[doc = "Field `CH_EN_WE` writer - Channel enable register"]
pub type ChEnWeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Channel enable"]
    #[inline(always)]
    pub fn ch_en(&self) -> ChEnR {
        ChEnR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Channel enable register"]
    #[inline(always)]
    pub fn ch_en_we(&self) -> ChEnWeR {
        ChEnWeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Channel enable"]
    #[inline(always)]
    pub fn ch_en(&mut self) -> ChEnW<'_, ChEnSpec> {
        ChEnW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Channel enable register"]
    #[inline(always)]
    pub fn ch_en_we(&mut self) -> ChEnWeW<'_, ChEnSpec> {
        ChEnWeW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChEnSpec;
impl crate::RegisterSpec for ChEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_en::R`](R) reader structure"]
impl crate::Readable for ChEnSpec {}
#[doc = "`write(|w| ..)` method takes [`ch_en::W`](W) writer structure"]
impl crate::Writable for ChEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_EN to value 0"]
impl crate::Resettable for ChEnSpec {}
