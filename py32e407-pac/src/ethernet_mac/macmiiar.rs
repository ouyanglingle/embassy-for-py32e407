#[doc = "Register `MACMIIAR` reader"]
pub type R = crate::R<MacmiiarSpec>;
#[doc = "Register `MACMIIAR` writer"]
pub type W = crate::W<MacmiiarSpec>;
#[doc = "Field `MB` reader - MII busy"]
pub type MbR = crate::BitReader;
#[doc = "Field `MB` writer - MII busy"]
pub type MbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MW` reader - MII write"]
pub type MwR = crate::BitReader;
#[doc = "Field `MW` writer - MII write"]
pub type MwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR` reader - Clock range"]
pub type CrR = crate::FieldReader;
#[doc = "Field `CR` writer - Clock range"]
pub type CrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MR` reader - MII register"]
pub type MrR = crate::FieldReader;
#[doc = "Field `MR` writer - MII register"]
pub type MrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA` reader - PHY address"]
pub type PaR = crate::FieldReader;
#[doc = "Field `PA` writer - PHY address"]
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - MII busy"]
    #[inline(always)]
    pub fn mb(&self) -> MbR {
        MbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MII write"]
    #[inline(always)]
    pub fn mw(&self) -> MwR {
        MwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 6:10 - MII register"]
    #[inline(always)]
    pub fn mr(&self) -> MrR {
        MrR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MII busy"]
    #[inline(always)]
    pub fn mb(&mut self) -> MbW<'_, MacmiiarSpec> {
        MbW::new(self, 0)
    }
    #[doc = "Bit 1 - MII write"]
    #[inline(always)]
    pub fn mw(&mut self) -> MwW<'_, MacmiiarSpec> {
        MwW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    pub fn cr(&mut self) -> CrW<'_, MacmiiarSpec> {
        CrW::new(self, 2)
    }
    #[doc = "Bits 6:10 - MII register"]
    #[inline(always)]
    pub fn mr(&mut self) -> MrW<'_, MacmiiarSpec> {
        MrW::new(self, 6)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PaW<'_, MacmiiarSpec> {
        PaW::new(self, 11)
    }
}
#[doc = "Ethernet MAC MII address register (ETH_MACMIIAR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macmiiar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmiiar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacmiiarSpec;
impl crate::RegisterSpec for MacmiiarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmiiar::R`](R) reader structure"]
impl crate::Readable for MacmiiarSpec {}
#[doc = "`write(|w| ..)` method takes [`macmiiar::W`](W) writer structure"]
impl crate::Writable for MacmiiarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACMIIAR to value 0"]
impl crate::Resettable for MacmiiarSpec {}
