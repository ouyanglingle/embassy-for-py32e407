#[doc = "Register `CFGR3` reader"]
pub type R = crate::R<Cfgr3Spec>;
#[doc = "Register `CFGR3` writer"]
pub type W = crate::W<Cfgr3Spec>;
#[doc = "Field `PLLSRC` reader - "]
pub type PllsrcR = crate::BitReader;
#[doc = "Field `PLLSRC` writer - "]
pub type PllsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLXTPRE` reader - "]
pub type PllxtpreR = crate::BitReader;
#[doc = "Field `PLLXTPRE` writer - "]
pub type PllxtpreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLPREDIV` reader - "]
pub type PllpredivR = crate::FieldReader;
#[doc = "Field `PLLPREDIV` writer - "]
pub type PllpredivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLPOSTDIV` reader - "]
pub type PllpostdivR = crate::FieldReader;
#[doc = "Field `PLLPOSTDIV` writer - "]
pub type PllpostdivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLFBDIV` reader - "]
pub type PllfbdivR = crate::FieldReader;
#[doc = "Field `PLLFBDIV` writer - "]
pub type PllfbdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PLLCPGAIN` reader - "]
pub type PllcpgainR = crate::FieldReader;
#[doc = "Field `PLLCPGAIN` writer - "]
pub type PllcpgainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PllsrcR {
        PllsrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pllxtpre(&self) -> PllxtpreR {
        PllxtpreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pllprediv(&self) -> PllpredivR {
        PllpredivR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pllpostdiv(&self) -> PllpostdivR {
        PllpostdivR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pllfbdiv(&self) -> PllfbdivR {
        PllfbdivR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pllcpgain(&self) -> PllcpgainR {
        PllcpgainR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PllsrcW<'_, Cfgr3Spec> {
        PllsrcW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pllxtpre(&mut self) -> PllxtpreW<'_, Cfgr3Spec> {
        PllxtpreW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pllprediv(&mut self) -> PllpredivW<'_, Cfgr3Spec> {
        PllpredivW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pllpostdiv(&mut self) -> PllpostdivW<'_, Cfgr3Spec> {
        PllpostdivW::new(self, 4)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pllfbdiv(&mut self) -> PllfbdivW<'_, Cfgr3Spec> {
        PllfbdivW::new(self, 8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pllcpgain(&mut self) -> PllcpgainW<'_, Cfgr3Spec> {
        PllcpgainW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr3Spec;
impl crate::RegisterSpec for Cfgr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr3::R`](R) reader structure"]
impl crate::Readable for Cfgr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure"]
impl crate::Writable for Cfgr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for Cfgr3Spec {}
