#[doc = "Register `IFR1` reader"]
pub type R = crate::R<Ifr1Spec>;
#[doc = "Register `IFR1` writer"]
pub type W = crate::W<Ifr1Spec>;
#[doc = "Field `CMDIF` reader - command done interrupt flag"]
pub type CmdifR = crate::BitReader;
#[doc = "Field `CMDIF` writer - command done interrupt flag"]
pub type CmdifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFONEIF` reader - rxfifo not empty interrupt flag"]
pub type RxfifoneifR = crate::BitReader;
#[doc = "Field `RXFIFOHFIF` reader - rxfifo half full interrupt flag"]
pub type RxfifohfifR = crate::BitReader;
#[doc = "Field `RXFIFOFIF` reader - rxfifo full interrupt flag"]
pub type RxfifofifR = crate::BitReader;
#[doc = "Field `IDLEDONEIF` reader - idle interrupt flag"]
pub type IdledoneifR = crate::BitReader;
#[doc = "Field `DATAWAITIF` reader - data wait interrupt flag"]
pub type DatawaitifR = crate::BitReader;
#[doc = "Field `RXFIFOWIF` reader - rx fifo word interrupt flag"]
pub type RxfifowifR = crate::BitReader;
#[doc = "Field `ADDRDONEIF` reader - address done interrupt flag"]
pub type AddrdoneifR = crate::BitReader;
#[doc = "Field `ADDRDONEIF` writer - address done interrupt flag"]
pub type AddrdoneifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - command done interrupt flag"]
    #[inline(always)]
    pub fn cmdif(&self) -> CmdifR {
        CmdifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - rxfifo not empty interrupt flag"]
    #[inline(always)]
    pub fn rxfifoneif(&self) -> RxfifoneifR {
        RxfifoneifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - rxfifo half full interrupt flag"]
    #[inline(always)]
    pub fn rxfifohfif(&self) -> RxfifohfifR {
        RxfifohfifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - rxfifo full interrupt flag"]
    #[inline(always)]
    pub fn rxfifofif(&self) -> RxfifofifR {
        RxfifofifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - idle interrupt flag"]
    #[inline(always)]
    pub fn idledoneif(&self) -> IdledoneifR {
        IdledoneifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - data wait interrupt flag"]
    #[inline(always)]
    pub fn datawaitif(&self) -> DatawaitifR {
        DatawaitifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - rx fifo word interrupt flag"]
    #[inline(always)]
    pub fn rxfifowif(&self) -> RxfifowifR {
        RxfifowifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - address done interrupt flag"]
    #[inline(always)]
    pub fn addrdoneif(&self) -> AddrdoneifR {
        AddrdoneifR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - command done interrupt flag"]
    #[inline(always)]
    pub fn cmdif(&mut self) -> CmdifW<'_, Ifr1Spec> {
        CmdifW::new(self, 0)
    }
    #[doc = "Bit 7 - address done interrupt flag"]
    #[inline(always)]
    pub fn addrdoneif(&mut self) -> AddrdoneifW<'_, Ifr1Spec> {
        AddrdoneifW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ifr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ifr1Spec;
impl crate::RegisterSpec for Ifr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ifr1::R`](R) reader structure"]
impl crate::Readable for Ifr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ifr1::W`](W) writer structure"]
impl crate::Writable for Ifr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFR1 to value 0"]
impl crate::Resettable for Ifr1Spec {}
