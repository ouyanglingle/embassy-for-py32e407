#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `RXWMARK` reader - desc RXWMARK"]
pub type RxwmarkR = crate::BitReader;
#[doc = "Field `TXWMARK` reader - desc TXWMARK"]
pub type TxwmarkR = crate::BitReader;
#[doc = "Field `FIFOE` reader - desc FIFOE"]
pub type FifoeR = crate::BitReader;
#[doc = "Field `FIFOF` reader - desc FIFOF"]
pub type FifofR = crate::BitReader;
#[doc = "Field `CMDFSM` reader - desc CMDFSM"]
pub type CmdfsmR = crate::FieldReader;
#[doc = "Field `CARDPRESENT` reader - desc CARDPRESENT"]
pub type CardpresentR = crate::BitReader;
#[doc = "Field `CARDBSY` reader - desc CARDBSY"]
pub type CardbsyR = crate::BitReader;
#[doc = "Field `FIFOCNT` reader - desc FIFOCNT"]
pub type FifocntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - desc RXWMARK"]
    #[inline(always)]
    pub fn rxwmark(&self) -> RxwmarkR {
        RxwmarkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TXWMARK"]
    #[inline(always)]
    pub fn txwmark(&self) -> TxwmarkR {
        TxwmarkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc FIFOE"]
    #[inline(always)]
    pub fn fifoe(&self) -> FifoeR {
        FifoeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc FIFOF"]
    #[inline(always)]
    pub fn fifof(&self) -> FifofR {
        FifofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - desc CMDFSM"]
    #[inline(always)]
    pub fn cmdfsm(&self) -> CmdfsmR {
        CmdfsmR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - desc CARDPRESENT"]
    #[inline(always)]
    pub fn cardpresent(&self) -> CardpresentR {
        CardpresentR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CARDBSY"]
    #[inline(always)]
    pub fn cardbsy(&self) -> CardbsyR {
        CardbsyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 17:29 - desc FIFOCNT"]
    #[inline(always)]
    pub fn fifocnt(&self) -> FifocntR {
        FifocntR::new(((self.bits >> 17) & 0x1fff) as u16)
    }
}
impl W {}
#[doc = "desc STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
