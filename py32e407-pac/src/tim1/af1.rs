#[doc = "Register `AF1` reader"]
pub type R = crate::R<Af1Spec>;
#[doc = "Register `AF1` writer"]
pub type W = crate::W<Af1Spec>;
#[doc = "Field `BKINE` reader - "]
pub type BkineR = crate::BitReader;
#[doc = "Field `BKINE` writer - "]
pub type BkineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1E` reader - "]
pub type Bkcmp1eR = crate::BitReader;
#[doc = "Field `BKCMP1E` writer - "]
pub type Bkcmp1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP2E` reader - "]
pub type Bkcmp2eR = crate::BitReader;
#[doc = "Field `BKCMP2E` writer - "]
pub type Bkcmp2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP3E` reader - "]
pub type Bkcmp3eR = crate::BitReader;
#[doc = "Field `BKCMP3E` writer - "]
pub type Bkcmp3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP4E` reader - "]
pub type Bkcmp4eR = crate::BitReader;
#[doc = "Field `BKCMP4E` writer - "]
pub type Bkcmp4eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP5E` reader - "]
pub type Bkcmp5eR = crate::BitReader;
#[doc = "Field `BKCMP5E` writer - "]
pub type Bkcmp5eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP6E` reader - "]
pub type Bkcmp6eR = crate::BitReader;
#[doc = "Field `BKCMP6E` writer - "]
pub type Bkcmp6eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP7E` reader - "]
pub type Bkcmp7eR = crate::BitReader;
#[doc = "Field `BKCMP7E` writer - "]
pub type Bkcmp7eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP8E` reader - "]
pub type Bkcmp8eR = crate::BitReader;
#[doc = "Field `BKCMP8E` writer - "]
pub type Bkcmp8eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKINP` reader - "]
pub type BkinpR = crate::BitReader;
#[doc = "Field `BKINP` writer - "]
pub type BkinpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1P` reader - "]
pub type Bkcmp1pR = crate::BitReader;
#[doc = "Field `BKCMP1P` writer - "]
pub type Bkcmp1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP2P` reader - "]
pub type Bkcmp2pR = crate::BitReader;
#[doc = "Field `BKCMP2P` writer - "]
pub type Bkcmp2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP3P` reader - "]
pub type Bkcmp3pR = crate::BitReader;
#[doc = "Field `BKCMP3P` writer - "]
pub type Bkcmp3pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP4P` reader - "]
pub type Bkcmp4pR = crate::BitReader;
#[doc = "Field `BKCMP4P` writer - "]
pub type Bkcmp4pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETRSEL` reader - "]
pub type EtrselR = crate::FieldReader;
#[doc = "Field `ETRSEL` writer - "]
pub type EtrselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bkine(&self) -> BkineR {
        BkineR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> Bkcmp1eR {
        Bkcmp1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> Bkcmp2eR {
        Bkcmp2eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn bkcmp3e(&self) -> Bkcmp3eR {
        Bkcmp3eR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bkcmp4e(&self) -> Bkcmp4eR {
        Bkcmp4eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bkcmp5e(&self) -> Bkcmp5eR {
        Bkcmp5eR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bkcmp6e(&self) -> Bkcmp6eR {
        Bkcmp6eR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bkcmp7e(&self) -> Bkcmp7eR {
        Bkcmp7eR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bkcmp8e(&self) -> Bkcmp8eR {
        Bkcmp8eR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bkinp(&self) -> BkinpR {
        BkinpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> Bkcmp1pR {
        Bkcmp1pR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> Bkcmp2pR {
        Bkcmp2pR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bkcmp3p(&self) -> Bkcmp3pR {
        Bkcmp3pR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn bkcmp4p(&self) -> Bkcmp4pR {
        Bkcmp4pR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn etrsel(&self) -> EtrselR {
        EtrselR::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bkine(&mut self) -> BkineW<'_, Af1Spec> {
        BkineW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bkcmp1e(&mut self) -> Bkcmp1eW<'_, Af1Spec> {
        Bkcmp1eW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bkcmp2e(&mut self) -> Bkcmp2eW<'_, Af1Spec> {
        Bkcmp2eW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn bkcmp3e(&mut self) -> Bkcmp3eW<'_, Af1Spec> {
        Bkcmp3eW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bkcmp4e(&mut self) -> Bkcmp4eW<'_, Af1Spec> {
        Bkcmp4eW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bkcmp5e(&mut self) -> Bkcmp5eW<'_, Af1Spec> {
        Bkcmp5eW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bkcmp6e(&mut self) -> Bkcmp6eW<'_, Af1Spec> {
        Bkcmp6eW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bkcmp7e(&mut self) -> Bkcmp7eW<'_, Af1Spec> {
        Bkcmp7eW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bkcmp8e(&mut self) -> Bkcmp8eW<'_, Af1Spec> {
        Bkcmp8eW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bkinp(&mut self) -> BkinpW<'_, Af1Spec> {
        BkinpW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bkcmp1p(&mut self) -> Bkcmp1pW<'_, Af1Spec> {
        Bkcmp1pW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bkcmp2p(&mut self) -> Bkcmp2pW<'_, Af1Spec> {
        Bkcmp2pW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bkcmp3p(&mut self) -> Bkcmp3pW<'_, Af1Spec> {
        Bkcmp3pW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn bkcmp4p(&mut self) -> Bkcmp4pW<'_, Af1Spec> {
        Bkcmp4pW::new(self, 13)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn etrsel(&mut self) -> EtrselW<'_, Af1Spec> {
        EtrselW::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Af1Spec;
impl crate::RegisterSpec for Af1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af1::R`](R) reader structure"]
impl crate::Readable for Af1Spec {}
#[doc = "`write(|w| ..)` method takes [`af1::W`](W) writer structure"]
impl crate::Writable for Af1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF1 to value 0"]
impl crate::Resettable for Af1Spec {}
