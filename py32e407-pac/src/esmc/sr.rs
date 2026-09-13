#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `SSACTIVE` reader - slave select active"]
pub type SsactiveR = crate::BitReader;
#[doc = "Field `RXBUSY` reader - rx busy state"]
pub type RxbusyR = crate::BitReader;
#[doc = "Field `TXBUSY` reader - tx busy state"]
pub type TxbusyR = crate::BitReader;
#[doc = "Field `XIPBUSY` reader - xip busy state"]
pub type XipbusyR = crate::BitReader;
#[doc = "Field `IDLE_STATE` reader - idle state"]
pub type IdleStateR = crate::BitReader;
#[doc = "Field `TXFIFOOV` reader - txfifo overflow state"]
pub type TxfifoovR = crate::BitReader;
#[doc = "Field `DATAWAIT` reader - data wait"]
pub type DatawaitR = crate::BitReader;
#[doc = "Field `SPIF` reader - spi interrupt flag"]
pub type SpifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - slave select active"]
    #[inline(always)]
    pub fn ssactive(&self) -> SsactiveR {
        SsactiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - rx busy state"]
    #[inline(always)]
    pub fn rxbusy(&self) -> RxbusyR {
        RxbusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tx busy state"]
    #[inline(always)]
    pub fn txbusy(&self) -> TxbusyR {
        TxbusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - xip busy state"]
    #[inline(always)]
    pub fn xipbusy(&self) -> XipbusyR {
        XipbusyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - idle state"]
    #[inline(always)]
    pub fn idle_state(&self) -> IdleStateR {
        IdleStateR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - txfifo overflow state"]
    #[inline(always)]
    pub fn txfifoov(&self) -> TxfifoovR {
        TxfifoovR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - data wait"]
    #[inline(always)]
    pub fn datawait(&self) -> DatawaitR {
        DatawaitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - spi interrupt flag"]
    #[inline(always)]
    pub fn spif(&self) -> SpifR {
        SpifR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
