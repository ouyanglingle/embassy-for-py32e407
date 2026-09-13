#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `CKOKIE` reader - Clock calibration completion interrupt enable"]
pub type CkokieR = crate::BitReader;
#[doc = "Field `CKOKIE` writer - Clock calibration completion interrupt enable"]
pub type CkokieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKWARNIE` reader - Clock calibration warning interrupt enable"]
pub type CkwarnieR = crate::BitReader;
#[doc = "Field `CKWARNIE` writer - Clock calibration warning interrupt enable"]
pub type CkwarnieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EREFIE` reader - Expected reference signal interrupt enable"]
pub type ErefieR = crate::BitReader;
#[doc = "Field `EREFIE` writer - Expected reference signal interrupt enable"]
pub type ErefieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTEN` reader - CTC count enable"]
pub type CntenR = crate::BitReader;
#[doc = "Field `CNTEN` writer - CTC count enable"]
pub type CntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOTRIM` reader - Automatic trim mode"]
pub type AutotrimR = crate::BitReader;
#[doc = "Field `AUTOTRIM` writer - Automatic trim mode"]
pub type AutotrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREFPUL` reader - Software generates synchronous reference signal pulses"]
pub type SwrefpulR = crate::BitReader;
#[doc = "Field `SWREFPUL` writer - Software generates synchronous reference signal pulses"]
pub type SwrefpulW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIMVALUE` reader - Trim Value"]
pub type TrimvalueR = crate::FieldReader;
#[doc = "Field `TRIMVALUE` writer - Trim Value"]
pub type TrimvalueW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Clock calibration completion interrupt enable"]
    #[inline(always)]
    pub fn ckokie(&self) -> CkokieR {
        CkokieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock calibration warning interrupt enable"]
    #[inline(always)]
    pub fn ckwarnie(&self) -> CkwarnieR {
        CkwarnieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected reference signal interrupt enable"]
    #[inline(always)]
    pub fn erefie(&self) -> ErefieR {
        ErefieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - CTC count enable"]
    #[inline(always)]
    pub fn cnten(&self) -> CntenR {
        CntenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic trim mode"]
    #[inline(always)]
    pub fn autotrim(&self) -> AutotrimR {
        AutotrimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software generates synchronous reference signal pulses"]
    #[inline(always)]
    pub fn swrefpul(&self) -> SwrefpulR {
        SwrefpulR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Trim Value"]
    #[inline(always)]
    pub fn trimvalue(&self) -> TrimvalueR {
        TrimvalueR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock calibration completion interrupt enable"]
    #[inline(always)]
    pub fn ckokie(&mut self) -> CkokieW<'_, Ctl0Spec> {
        CkokieW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock calibration warning interrupt enable"]
    #[inline(always)]
    pub fn ckwarnie(&mut self) -> CkwarnieW<'_, Ctl0Spec> {
        CkwarnieW::new(self, 1)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, Ctl0Spec> {
        ErrieW::new(self, 2)
    }
    #[doc = "Bit 3 - Expected reference signal interrupt enable"]
    #[inline(always)]
    pub fn erefie(&mut self) -> ErefieW<'_, Ctl0Spec> {
        ErefieW::new(self, 3)
    }
    #[doc = "Bit 5 - CTC count enable"]
    #[inline(always)]
    pub fn cnten(&mut self) -> CntenW<'_, Ctl0Spec> {
        CntenW::new(self, 5)
    }
    #[doc = "Bit 6 - Automatic trim mode"]
    #[inline(always)]
    pub fn autotrim(&mut self) -> AutotrimW<'_, Ctl0Spec> {
        AutotrimW::new(self, 6)
    }
    #[doc = "Bit 7 - Software generates synchronous reference signal pulses"]
    #[inline(always)]
    pub fn swrefpul(&mut self) -> SwrefpulW<'_, Ctl0Spec> {
        SwrefpulW::new(self, 7)
    }
    #[doc = "Bits 8:14 - Trim Value"]
    #[inline(always)]
    pub fn trimvalue(&mut self) -> TrimvalueW<'_, Ctl0Spec> {
        TrimvalueW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {}
