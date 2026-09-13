#[doc = "Register `ADDR32` reader"]
pub type R = crate::R<Addr32Spec>;
#[doc = "Register `ADDR32` writer"]
pub type W = crate::W<Addr32Spec>;
#[doc = "Field `ADDR3` reader - address3"]
pub type Addr3R = crate::FieldReader;
#[doc = "Field `ADDR3` writer - address3"]
pub type Addr3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MREG` reader - Mode register"]
pub type MregR = crate::FieldReader;
#[doc = "Field `MREG` writer - Mode register"]
pub type MregW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SSXO` reader - Slave select enable"]
pub type SsxoR = crate::FieldReader;
#[doc = "Field `SSXO` writer - Slave select enable"]
pub type SsxoW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SS_CLR` reader - Slave select clear"]
pub type SsClrR = crate::BitReader;
#[doc = "Field `SS_CLR` writer - Slave select clear"]
pub type SsClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS_SET` reader - Slave select set"]
pub type SsSetR = crate::BitReader;
#[doc = "Field `SS_SET` writer - Slave select set"]
pub type SsSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - address3"]
    #[inline(always)]
    pub fn addr3(&self) -> Addr3R {
        Addr3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Mode register"]
    #[inline(always)]
    pub fn mreg(&self) -> MregR {
        MregR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - Slave select enable"]
    #[inline(always)]
    pub fn ssxo(&self) -> SsxoR {
        SsxoR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Slave select clear"]
    #[inline(always)]
    pub fn ss_clr(&self) -> SsClrR {
        SsClrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slave select set"]
    #[inline(always)]
    pub fn ss_set(&self) -> SsSetR {
        SsSetR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - address3"]
    #[inline(always)]
    pub fn addr3(&mut self) -> Addr3W<'_, Addr32Spec> {
        Addr3W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Mode register"]
    #[inline(always)]
    pub fn mreg(&mut self) -> MregW<'_, Addr32Spec> {
        MregW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Slave select enable"]
    #[inline(always)]
    pub fn ssxo(&mut self) -> SsxoW<'_, Addr32Spec> {
        SsxoW::new(self, 16)
    }
    #[doc = "Bit 22 - Slave select clear"]
    #[inline(always)]
    pub fn ss_clr(&mut self) -> SsClrW<'_, Addr32Spec> {
        SsClrW::new(self, 22)
    }
    #[doc = "Bit 23 - Slave select set"]
    #[inline(always)]
    pub fn ss_set(&mut self) -> SsSetW<'_, Addr32Spec> {
        SsSetW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`addr32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr32Spec;
impl crate::RegisterSpec for Addr32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr32::R`](R) reader structure"]
impl crate::Readable for Addr32Spec {}
#[doc = "`write(|w| ..)` method takes [`addr32::W`](W) writer structure"]
impl crate::Writable for Addr32Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR32 to value 0"]
impl crate::Resettable for Addr32Spec {}
