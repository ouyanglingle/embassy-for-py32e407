#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `RXNEIE` reader - Enable Received Data Available Interrupt"]
pub type RxneieR = crate::BitReader;
#[doc = "Field `RXNEIE` writer - Enable Received Data Available Interrupt"]
pub type RxneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDREIE` reader - Enable Transmit Holding Register Empty Interrupt"]
pub type TdreieR = crate::BitReader;
#[doc = "Field `TDREIE` writer - Enable Transmit Holding Register Empty Interrupt"]
pub type TdreieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIE` reader - Enable Receiver Line Status Interrupt"]
pub type LsieR = crate::BitReader;
#[doc = "Field `LSIE` writer - Enable Receiver Line Status Interrupt"]
pub type LsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSYERRIE` reader - Enable Busyerr state interruption"]
pub type BusyerrieR = crate::BitReader;
#[doc = "Field `BUSYERRIE` writer - Enable Busyerr state interruption"]
pub type BusyerrieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Received Data Available Interrupt"]
    #[inline(always)]
    pub fn rxneie(&self) -> RxneieR {
        RxneieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Transmit Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn tdreie(&self) -> TdreieR {
        TdreieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt"]
    #[inline(always)]
    pub fn lsie(&self) -> LsieR {
        LsieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Busyerr state interruption"]
    #[inline(always)]
    pub fn busyerrie(&self) -> BusyerrieR {
        BusyerrieR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Received Data Available Interrupt"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RxneieW<'_, Cr2Spec> {
        RxneieW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Transmit Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn tdreie(&mut self) -> TdreieW<'_, Cr2Spec> {
        TdreieW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt"]
    #[inline(always)]
    pub fn lsie(&mut self) -> LsieW<'_, Cr2Spec> {
        LsieW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Busyerr state interruption"]
    #[inline(always)]
    pub fn busyerrie(&mut self) -> BusyerrieW<'_, Cr2Spec> {
        BusyerrieW::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
