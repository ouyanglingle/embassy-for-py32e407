#[doc = "Register `APB1ENR2` reader"]
pub type R = crate::R<Apb1enr2Spec>;
#[doc = "Register `APB1ENR2` writer"]
pub type W = crate::W<Apb1enr2Spec>;
#[doc = "Field `LPTIM1EN` reader - "]
pub type Lptim1enR = crate::BitReader;
#[doc = "Field `LPTIM1EN` writer - "]
pub type Lptim1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1EN` reader - "]
pub type Lpuart1enR = crate::BitReader;
#[doc = "Field `LPUART1EN` writer - "]
pub type Lpuart1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4EN` reader - "]
pub type I2c4enR = crate::BitReader;
#[doc = "Field `I2C4EN` writer - "]
pub type I2c4enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lptim1en(&self) -> Lptim1enR {
        Lptim1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> Lpuart1enR {
        Lpuart1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2c4enR {
        I2c4enR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lptim1en(&mut self) -> Lptim1enW<'_, Apb1enr2Spec> {
        Lptim1enW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> Lpuart1enW<'_, Apb1enr2Spec> {
        Lpuart1enW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2c4enW<'_, Apb1enr2Spec> {
        I2c4enW::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1enr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1enr2Spec;
impl crate::RegisterSpec for Apb1enr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1enr2::R`](R) reader structure"]
impl crate::Readable for Apb1enr2Spec {}
#[doc = "`write(|w| ..)` method takes [`apb1enr2::W`](W) writer structure"]
impl crate::Writable for Apb1enr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1ENR2 to value 0"]
impl crate::Resettable for Apb1enr2Spec {}
