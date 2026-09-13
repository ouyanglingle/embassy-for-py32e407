#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<Cfgr1Spec>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<Cfgr1Spec>;
#[doc = "Field `MCOPRE` reader - "]
pub type McopreR = crate::FieldReader;
#[doc = "Field `MCOPRE` writer - "]
pub type McopreW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HSI48ON` reader - "]
pub type Hsi48onR = crate::BitReader;
#[doc = "Field `HSI48ON` writer - "]
pub type Hsi48onW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSI48RDY` reader - "]
pub type Hsi48rdyR = crate::BitReader;
#[doc = "Field `HSI48TRIM` reader - "]
pub type Hsi48trimR = crate::FieldReader;
#[doc = "Field `HSI48TRIM` writer - "]
pub type Hsi48trimW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HSI48CAL` reader - "]
pub type Hsi48calR = crate::FieldReader<u16>;
#[doc = "Field `PVDSEL` reader - "]
pub type PvdselR = crate::FieldReader;
#[doc = "Field `PVDSEL` writer - "]
pub type PvdselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USBSELHSI48` reader - "]
pub type Usbselhsi48R = crate::BitReader;
#[doc = "Field `USBSELHSI48` writer - "]
pub type Usbselhsi48W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn mcopre(&self) -> McopreR {
        McopreR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn hsi48on(&self) -> Hsi48onR {
        Hsi48onR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> Hsi48rdyR {
        Hsi48rdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn hsi48trim(&self) -> Hsi48trimR {
        Hsi48trimR::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn hsi48cal(&self) -> Hsi48calR {
        Hsi48calR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn pvdsel(&self) -> PvdselR {
        PvdselR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn usbselhsi48(&self) -> Usbselhsi48R {
        Usbselhsi48R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn mcopre(&mut self) -> McopreW<'_, Cfgr1Spec> {
        McopreW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn hsi48on(&mut self) -> Hsi48onW<'_, Cfgr1Spec> {
        Hsi48onW::new(self, 4)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn hsi48trim(&mut self) -> Hsi48trimW<'_, Cfgr1Spec> {
        Hsi48trimW::new(self, 9)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn pvdsel(&mut self) -> PvdselW<'_, Cfgr1Spec> {
        PvdselW::new(self, 29)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn usbselhsi48(&mut self) -> Usbselhsi48W<'_, Cfgr1Spec> {
        Usbselhsi48W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr1Spec;
impl crate::RegisterSpec for Cfgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for Cfgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for Cfgr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for Cfgr1Spec {}
