#[doc = "Register `DIER` reader"]
pub type R = crate::R<DierSpec>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DierSpec>;
#[doc = "Field `UIE` reader - "]
pub type UieR = crate::BitReader;
#[doc = "Field `UIE` writer - "]
pub type UieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IE` reader - "]
pub type Cc1ieR = crate::BitReader;
#[doc = "Field `CC1IE` writer - "]
pub type Cc1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IE` reader - "]
pub type Cc2ieR = crate::BitReader;
#[doc = "Field `CC2IE` writer - "]
pub type Cc2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IE` reader - "]
pub type Cc3ieR = crate::BitReader;
#[doc = "Field `CC3IE` writer - "]
pub type Cc3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IE` reader - "]
pub type Cc4ieR = crate::BitReader;
#[doc = "Field `CC4IE` writer - "]
pub type Cc4ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - "]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - "]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDE` reader - "]
pub type UdeR = crate::BitReader;
#[doc = "Field `UDE` writer - "]
pub type UdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1DE` reader - "]
pub type Cc1deR = crate::BitReader;
#[doc = "Field `CC1DE` writer - "]
pub type Cc1deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2DE` reader - "]
pub type Cc2deR = crate::BitReader;
#[doc = "Field `CC2DE` writer - "]
pub type Cc2deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3DE` reader - "]
pub type Cc3deR = crate::BitReader;
#[doc = "Field `CC3DE` writer - "]
pub type Cc3deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4DE` reader - "]
pub type Cc4deR = crate::BitReader;
#[doc = "Field `CC4DE` writer - "]
pub type Cc4deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDE` reader - "]
pub type TdeR = crate::BitReader;
#[doc = "Field `TDE` writer - "]
pub type TdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDXIE` reader - "]
pub type IdxieR = crate::BitReader;
#[doc = "Field `IDXIE` writer - "]
pub type IdxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRIE` reader - "]
pub type DirieR = crate::BitReader;
#[doc = "Field `DIRIE` writer - "]
pub type DirieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IERRIE` reader - "]
pub type IerrieR = crate::BitReader;
#[doc = "Field `IERRIE` writer - "]
pub type IerrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERRIE` reader - "]
pub type TerrieR = crate::BitReader;
#[doc = "Field `TERRIE` writer - "]
pub type TerrieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cc1ie(&self) -> Cc1ieR {
        Cc1ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cc2ie(&self) -> Cc2ieR {
        Cc2ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cc3ie(&self) -> Cc3ieR {
        Cc3ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cc4ie(&self) -> Cc4ieR {
        Cc4ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ude(&self) -> UdeR {
        UdeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cc1de(&self) -> Cc1deR {
        Cc1deR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cc2de(&self) -> Cc2deR {
        Cc2deR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cc3de(&self) -> Cc3deR {
        Cc3deR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cc4de(&self) -> Cc4deR {
        Cc4deR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn idxie(&self) -> IdxieR {
        IdxieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dirie(&self) -> DirieR {
        DirieR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ierrie(&self) -> IerrieR {
        IerrieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn terrie(&self) -> TerrieR {
        TerrieR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uie(&mut self) -> UieW<'_, DierSpec> {
        UieW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cc1ie(&mut self) -> Cc1ieW<'_, DierSpec> {
        Cc1ieW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cc2ie(&mut self) -> Cc2ieW<'_, DierSpec> {
        Cc2ieW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cc3ie(&mut self) -> Cc3ieW<'_, DierSpec> {
        Cc3ieW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cc4ie(&mut self) -> Cc4ieW<'_, DierSpec> {
        Cc4ieW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<'_, DierSpec> {
        TieW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ude(&mut self) -> UdeW<'_, DierSpec> {
        UdeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cc1de(&mut self) -> Cc1deW<'_, DierSpec> {
        Cc1deW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cc2de(&mut self) -> Cc2deW<'_, DierSpec> {
        Cc2deW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cc3de(&mut self) -> Cc3deW<'_, DierSpec> {
        Cc3deW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cc4de(&mut self) -> Cc4deW<'_, DierSpec> {
        Cc4deW::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tde(&mut self) -> TdeW<'_, DierSpec> {
        TdeW::new(self, 14)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn idxie(&mut self) -> IdxieW<'_, DierSpec> {
        IdxieW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dirie(&mut self) -> DirieW<'_, DierSpec> {
        DirieW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ierrie(&mut self) -> IerrieW<'_, DierSpec> {
        IerrieW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn terrie(&mut self) -> TerrieW<'_, DierSpec> {
        TerrieW::new(self, 23)
    }
}
#[doc = "desc DIER\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DierSpec;
impl crate::RegisterSpec for DierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier::R`](R) reader structure"]
impl crate::Readable for DierSpec {}
#[doc = "`write(|w| ..)` method takes [`dier::W`](W) writer structure"]
impl crate::Writable for DierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DierSpec {}
