#[doc = "Register `IER1` reader"]
pub type R = crate::R<Ier1Spec>;
#[doc = "Register `IER1` writer"]
pub type W = crate::W<Ier1Spec>;
#[doc = "Field `CMDIE` reader - command done interrupt enable"]
pub type CmdieR = crate::BitReader;
#[doc = "Field `CMDIE` writer - command done interrupt enable"]
pub type CmdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFONEIE` reader - rx fifo not empty interrupt enable"]
pub type RxfifoneieR = crate::BitReader;
#[doc = "Field `RXFIFONEIE` writer - rx fifo not empty interrupt enable"]
pub type RxfifoneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOHFIE` reader - rx fifo half interrupt enable"]
pub type RxfifohfieR = crate::BitReader;
#[doc = "Field `RXFIFOHFIE` writer - rx fifo half interrupt enable"]
pub type RxfifohfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOFIE` reader - rx fifo full interrupt enable"]
pub type RxfifofieR = crate::BitReader;
#[doc = "Field `RXFIFOFIE` writer - rx fifo full interrupt enable"]
pub type RxfifofieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEDONEIE` reader - idle interrupt enable"]
pub type IdledoneieR = crate::BitReader;
#[doc = "Field `IDLEDONEIE` writer - idle interrupt enable"]
pub type IdledoneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAWAITIE` reader - busy interrupt enable"]
pub type DatawaitieR = crate::BitReader;
#[doc = "Field `DATAWAITIE` writer - busy interrupt enable"]
pub type DatawaitieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOWIE` reader - rx fifo word interrupt enable"]
pub type RxfifowieR = crate::BitReader;
#[doc = "Field `RXFIFOWIE` writer - rx fifo word interrupt enable"]
pub type RxfifowieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRDONEIE` reader - address done interrupt enable"]
pub type AddrdoneieR = crate::BitReader;
#[doc = "Field `ADDRDONEIE` writer - address done interrupt enable"]
pub type AddrdoneieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - command done interrupt enable"]
    #[inline(always)]
    pub fn cmdie(&self) -> CmdieR {
        CmdieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - rx fifo not empty interrupt enable"]
    #[inline(always)]
    pub fn rxfifoneie(&self) -> RxfifoneieR {
        RxfifoneieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - rx fifo half interrupt enable"]
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RxfifohfieR {
        RxfifohfieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - rx fifo full interrupt enable"]
    #[inline(always)]
    pub fn rxfifofie(&self) -> RxfifofieR {
        RxfifofieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - idle interrupt enable"]
    #[inline(always)]
    pub fn idledoneie(&self) -> IdledoneieR {
        IdledoneieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - busy interrupt enable"]
    #[inline(always)]
    pub fn datawaitie(&self) -> DatawaitieR {
        DatawaitieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - rx fifo word interrupt enable"]
    #[inline(always)]
    pub fn rxfifowie(&self) -> RxfifowieR {
        RxfifowieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - address done interrupt enable"]
    #[inline(always)]
    pub fn addrdoneie(&self) -> AddrdoneieR {
        AddrdoneieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - command done interrupt enable"]
    #[inline(always)]
    pub fn cmdie(&mut self) -> CmdieW<'_, Ier1Spec> {
        CmdieW::new(self, 0)
    }
    #[doc = "Bit 1 - rx fifo not empty interrupt enable"]
    #[inline(always)]
    pub fn rxfifoneie(&mut self) -> RxfifoneieW<'_, Ier1Spec> {
        RxfifoneieW::new(self, 1)
    }
    #[doc = "Bit 2 - rx fifo half interrupt enable"]
    #[inline(always)]
    pub fn rxfifohfie(&mut self) -> RxfifohfieW<'_, Ier1Spec> {
        RxfifohfieW::new(self, 2)
    }
    #[doc = "Bit 3 - rx fifo full interrupt enable"]
    #[inline(always)]
    pub fn rxfifofie(&mut self) -> RxfifofieW<'_, Ier1Spec> {
        RxfifofieW::new(self, 3)
    }
    #[doc = "Bit 4 - idle interrupt enable"]
    #[inline(always)]
    pub fn idledoneie(&mut self) -> IdledoneieW<'_, Ier1Spec> {
        IdledoneieW::new(self, 4)
    }
    #[doc = "Bit 5 - busy interrupt enable"]
    #[inline(always)]
    pub fn datawaitie(&mut self) -> DatawaitieW<'_, Ier1Spec> {
        DatawaitieW::new(self, 5)
    }
    #[doc = "Bit 6 - rx fifo word interrupt enable"]
    #[inline(always)]
    pub fn rxfifowie(&mut self) -> RxfifowieW<'_, Ier1Spec> {
        RxfifowieW::new(self, 6)
    }
    #[doc = "Bit 7 - address done interrupt enable"]
    #[inline(always)]
    pub fn addrdoneie(&mut self) -> AddrdoneieW<'_, Ier1Spec> {
        AddrdoneieW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ier1Spec;
impl crate::RegisterSpec for Ier1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ier1::R`](R) reader structure"]
impl crate::Readable for Ier1Spec {}
#[doc = "`write(|w| ..)` method takes [`ier1::W`](W) writer structure"]
impl crate::Writable for Ier1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER1 to value 0"]
impl crate::Resettable for Ier1Spec {}
