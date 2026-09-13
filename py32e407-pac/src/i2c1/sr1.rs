#[doc = "Register `SR1` reader"]
pub type R = crate::R<Sr1Spec>;
#[doc = "Register `SR1` writer"]
pub type W = crate::W<Sr1Spec>;
#[doc = "Field `SB` reader - desc SB"]
pub type SbR = crate::BitReader;
#[doc = "Field `ADDR` reader - desc ADDR"]
pub type AddrR = crate::BitReader;
#[doc = "Field `BTF` reader - desc BTF"]
pub type BtfR = crate::BitReader;
#[doc = "Field `ADD10` reader - desc ADD10"]
pub type Add10R = crate::BitReader;
#[doc = "Field `STOPF` reader - desc STOPF"]
pub type StopfR = crate::BitReader;
#[doc = "Field `RXNE` reader - desc RXNE"]
pub type RxneR = crate::BitReader;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `BERR` reader - desc BERR"]
pub type BerrR = crate::BitReader;
#[doc = "Field `BERR` writer - desc BERR"]
pub type BerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLO` reader - desc ARLO"]
pub type ArloR = crate::BitReader;
#[doc = "Field `ARLO` writer - desc ARLO"]
pub type ArloW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF` reader - desc AF"]
pub type AfR = crate::BitReader;
#[doc = "Field `AF` writer - desc AF"]
pub type AfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR` reader - desc OVR"]
pub type OvrR = crate::BitReader;
#[doc = "Field `OVR` writer - desc OVR"]
pub type OvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECERR` reader - desc PECERR"]
pub type PecerrR = crate::BitReader;
#[doc = "Field `PECERR` writer - desc PECERR"]
pub type PecerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - desc TIMEOUT"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - desc TIMEOUT"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBALERT` reader - desc SMBALERT"]
pub type SmbalertR = crate::BitReader;
#[doc = "Field `SMBALERT` writer - desc SMBALERT"]
pub type SmbalertW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SB"]
    #[inline(always)]
    pub fn sb(&self) -> SbR {
        SbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc BTF"]
    #[inline(always)]
    pub fn btf(&self) -> BtfR {
        BtfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc ADD10"]
    #[inline(always)]
    pub fn add10(&self) -> Add10R {
        Add10R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc STOPF"]
    #[inline(always)]
    pub fn stopf(&self) -> StopfR {
        StopfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc BERR"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ARLO"]
    #[inline(always)]
    pub fn arlo(&self) -> ArloR {
        ArloR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc AF"]
    #[inline(always)]
    pub fn af(&self) -> AfR {
        AfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PECERR"]
    #[inline(always)]
    pub fn pecerr(&self) -> PecerrR {
        PecerrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - desc TIMEOUT"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc SMBALERT"]
    #[inline(always)]
    pub fn smbalert(&self) -> SmbalertR {
        SmbalertR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - desc BERR"]
    #[inline(always)]
    pub fn berr(&mut self) -> BerrW<'_, Sr1Spec> {
        BerrW::new(self, 8)
    }
    #[doc = "Bit 9 - desc ARLO"]
    #[inline(always)]
    pub fn arlo(&mut self) -> ArloW<'_, Sr1Spec> {
        ArloW::new(self, 9)
    }
    #[doc = "Bit 10 - desc AF"]
    #[inline(always)]
    pub fn af(&mut self) -> AfW<'_, Sr1Spec> {
        AfW::new(self, 10)
    }
    #[doc = "Bit 11 - desc OVR"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OvrW<'_, Sr1Spec> {
        OvrW::new(self, 11)
    }
    #[doc = "Bit 12 - desc PECERR"]
    #[inline(always)]
    pub fn pecerr(&mut self) -> PecerrW<'_, Sr1Spec> {
        PecerrW::new(self, 12)
    }
    #[doc = "Bit 14 - desc TIMEOUT"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<'_, Sr1Spec> {
        TimeoutW::new(self, 14)
    }
    #[doc = "Bit 15 - desc SMBALERT"]
    #[inline(always)]
    pub fn smbalert(&mut self) -> SmbalertW<'_, Sr1Spec> {
        SmbalertW::new(self, 15)
    }
}
#[doc = "desc SR1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr1Spec;
impl crate::RegisterSpec for Sr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for Sr1Spec {}
#[doc = "`write(|w| ..)` method takes [`sr1::W`](W) writer structure"]
impl crate::Writable for Sr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for Sr1Spec {}
