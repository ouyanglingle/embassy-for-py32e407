#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `EN1` reader - "]
pub type En1R = crate::BitReader;
#[doc = "Field `EN1` writer - "]
pub type En1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOFF1` reader - "]
pub type Boff1R = crate::BitReader;
#[doc = "Field `BOFF1` writer - "]
pub type Boff1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN1` reader - "]
pub type Ten1R = crate::BitReader;
#[doc = "Field `TEN1` writer - "]
pub type Ten1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEL1` reader - "]
pub type Tsel1R = crate::FieldReader;
#[doc = "Field `TSEL1` writer - "]
pub type Tsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WAVE1` reader - "]
pub type Wave1R = crate::FieldReader;
#[doc = "Field `WAVE1` writer - "]
pub type Wave1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAMP1` reader - "]
pub type Mamp1R = crate::FieldReader;
#[doc = "Field `MAMP1` writer - "]
pub type Mamp1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAEN1` reader - "]
pub type Dmaen1R = crate::BitReader;
#[doc = "Field `DMAEN1` writer - "]
pub type Dmaen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAUDRIE1` reader - "]
pub type Dmaudrie1R = crate::BitReader;
#[doc = "Field `DMAUDRIE1` writer - "]
pub type Dmaudrie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1CEN` reader - "]
pub type Dac1cenR = crate::BitReader;
#[doc = "Field `DAC1CEN` writer - "]
pub type Dac1cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN2` reader - "]
pub type En2R = crate::BitReader;
#[doc = "Field `EN2` writer - "]
pub type En2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOFF2` reader - "]
pub type Boff2R = crate::BitReader;
#[doc = "Field `BOFF2` writer - "]
pub type Boff2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN2` reader - "]
pub type Ten2R = crate::BitReader;
#[doc = "Field `TEN2` writer - "]
pub type Ten2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEL2` reader - "]
pub type Tsel2R = crate::FieldReader;
#[doc = "Field `TSEL2` writer - "]
pub type Tsel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WAVE2` reader - "]
pub type Wave2R = crate::FieldReader;
#[doc = "Field `WAVE2` writer - "]
pub type Wave2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAMP2` reader - "]
pub type Mamp2R = crate::FieldReader;
#[doc = "Field `MAMP2` writer - "]
pub type Mamp2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAEN2` reader - "]
pub type Dmaen2R = crate::BitReader;
#[doc = "Field `DMAEN2` writer - "]
pub type Dmaen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAUDRIE2` reader - "]
pub type Dmaudrie2R = crate::BitReader;
#[doc = "Field `DMAUDRIE2` writer - "]
pub type Dmaudrie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC2CEN` reader - "]
pub type Dac2cenR = crate::BitReader;
#[doc = "Field `DAC2CEN` writer - "]
pub type Dac2cenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en1(&self) -> En1R {
        En1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn boff1(&self) -> Boff1R {
        Boff1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten1(&self) -> Ten1R {
        Ten1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn tsel1(&self) -> Tsel1R {
        Tsel1R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn wave1(&self) -> Wave1R {
        Wave1R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn mamp1(&self) -> Mamp1R {
        Mamp1R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dmaen1(&self) -> Dmaen1R {
        Dmaen1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> Dmaudrie1R {
        Dmaudrie1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dac1cen(&self) -> Dac1cenR {
        Dac1cenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn en2(&self) -> En2R {
        En2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn boff2(&self) -> Boff2R {
        Boff2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten2(&self) -> Ten2R {
        Ten2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    pub fn tsel2(&self) -> Tsel2R {
        Tsel2R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn wave2(&self) -> Wave2R {
        Wave2R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:28"]
    #[inline(always)]
    pub fn mamp2(&self) -> Mamp2R {
        Mamp2R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dmaen2(&self) -> Dmaen2R {
        Dmaen2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dmaudrie2(&self) -> Dmaudrie2R {
        Dmaudrie2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dac2cen(&self) -> Dac2cenR {
        Dac2cenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en1(&mut self) -> En1W<'_, CrSpec> {
        En1W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn boff1(&mut self) -> Boff1W<'_, CrSpec> {
        Boff1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten1(&mut self) -> Ten1W<'_, CrSpec> {
        Ten1W::new(self, 2)
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn tsel1(&mut self) -> Tsel1W<'_, CrSpec> {
        Tsel1W::new(self, 3)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn wave1(&mut self) -> Wave1W<'_, CrSpec> {
        Wave1W::new(self, 7)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn mamp1(&mut self) -> Mamp1W<'_, CrSpec> {
        Mamp1W::new(self, 9)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dmaen1(&mut self) -> Dmaen1W<'_, CrSpec> {
        Dmaen1W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dmaudrie1(&mut self) -> Dmaudrie1W<'_, CrSpec> {
        Dmaudrie1W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dac1cen(&mut self) -> Dac1cenW<'_, CrSpec> {
        Dac1cenW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn en2(&mut self) -> En2W<'_, CrSpec> {
        En2W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn boff2(&mut self) -> Boff2W<'_, CrSpec> {
        Boff2W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten2(&mut self) -> Ten2W<'_, CrSpec> {
        Ten2W::new(self, 18)
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    pub fn tsel2(&mut self) -> Tsel2W<'_, CrSpec> {
        Tsel2W::new(self, 19)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn wave2(&mut self) -> Wave2W<'_, CrSpec> {
        Wave2W::new(self, 23)
    }
    #[doc = "Bits 25:28"]
    #[inline(always)]
    pub fn mamp2(&mut self) -> Mamp2W<'_, CrSpec> {
        Mamp2W::new(self, 25)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dmaen2(&mut self) -> Dmaen2W<'_, CrSpec> {
        Dmaen2W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dmaudrie2(&mut self) -> Dmaudrie2W<'_, CrSpec> {
        Dmaudrie2W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dac2cen(&mut self) -> Dac2cenW<'_, CrSpec> {
        Dac2cenW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
