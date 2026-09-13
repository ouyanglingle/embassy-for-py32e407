#[doc = "Register `APB1ENR1` reader"]
pub type R = crate::R<Apb1enr1Spec>;
#[doc = "Register `APB1ENR1` writer"]
pub type W = crate::W<Apb1enr1Spec>;
#[doc = "Field `TIM2EN` reader - "]
pub type Tim2enR = crate::BitReader;
#[doc = "Field `TIM2EN` writer - "]
pub type Tim2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3EN` reader - "]
pub type Tim3enR = crate::BitReader;
#[doc = "Field `TIM3EN` writer - "]
pub type Tim3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4EN` reader - "]
pub type Tim4enR = crate::BitReader;
#[doc = "Field `TIM4EN` writer - "]
pub type Tim4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5EN` reader - "]
pub type Tim5enR = crate::BitReader;
#[doc = "Field `TIM5EN` writer - "]
pub type Tim5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6EN` reader - "]
pub type Tim6enR = crate::BitReader;
#[doc = "Field `TIM6EN` writer - "]
pub type Tim6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7EN` reader - "]
pub type Tim7enR = crate::BitReader;
#[doc = "Field `TIM7EN` writer - "]
pub type Tim7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM12EN` reader - "]
pub type Tim12enR = crate::BitReader;
#[doc = "Field `TIM12EN` writer - "]
pub type Tim12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM13EN` reader - "]
pub type Tim13enR = crate::BitReader;
#[doc = "Field `TIM13EN` writer - "]
pub type Tim13enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14EN` reader - "]
pub type Tim14enR = crate::BitReader;
#[doc = "Field `TIM14EN` writer - "]
pub type Tim14enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM18EN` reader - "]
pub type Tim18enR = crate::BitReader;
#[doc = "Field `TIM18EN` writer - "]
pub type Tim18enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGEN` reader - "]
pub type WwdgenR = crate::BitReader;
#[doc = "Field `WWDGEN` writer - "]
pub type WwdgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2EN` reader - "]
pub type Spi2enR = crate::BitReader;
#[doc = "Field `SPI2EN` writer - "]
pub type Spi2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3EN` reader - "]
pub type Spi3enR = crate::BitReader;
#[doc = "Field `SPI3EN` writer - "]
pub type Spi3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2EN` reader - "]
pub type Usart2enR = crate::BitReader;
#[doc = "Field `USART2EN` writer - "]
pub type Usart2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3EN` reader - "]
pub type Usart3enR = crate::BitReader;
#[doc = "Field `USART3EN` writer - "]
pub type Usart3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1EN` reader - "]
pub type Uart1enR = crate::BitReader;
#[doc = "Field `UART1EN` writer - "]
pub type Uart1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2EN` reader - "]
pub type Uart2enR = crate::BitReader;
#[doc = "Field `UART2EN` writer - "]
pub type Uart2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1EN` reader - "]
pub type I2c1enR = crate::BitReader;
#[doc = "Field `I2C1EN` writer - "]
pub type I2c1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2EN` reader - "]
pub type I2c2enR = crate::BitReader;
#[doc = "Field `I2C2EN` writer - "]
pub type I2c2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3EN` reader - "]
pub type Uart3enR = crate::BitReader;
#[doc = "Field `UART3EN` writer - "]
pub type Uart3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1EN` reader - "]
pub type Can1enR = crate::BitReader;
#[doc = "Field `CAN1EN` writer - "]
pub type Can1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN2EN` reader - "]
pub type Can2enR = crate::BitReader;
#[doc = "Field `CAN2EN` writer - "]
pub type Can2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPEN` reader - "]
pub type BkpenR = crate::BitReader;
#[doc = "Field `BKPEN` writer - "]
pub type BkpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWREN` reader - "]
pub type PwrenR = crate::BitReader;
#[doc = "Field `PWREN` writer - "]
pub type PwrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACEN` reader - "]
pub type DacenR = crate::BitReader;
#[doc = "Field `DACEN` writer - "]
pub type DacenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3EN` reader - "]
pub type I2c3enR = crate::BitReader;
#[doc = "Field `I2C3EN` writer - "]
pub type I2c3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCEN` reader - "]
pub type CtcenR = crate::BitReader;
#[doc = "Field `CTCEN` writer - "]
pub type CtcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tim2en(&self) -> Tim2enR {
        Tim2enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tim3en(&self) -> Tim3enR {
        Tim3enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tim4en(&self) -> Tim4enR {
        Tim4enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tim5en(&self) -> Tim5enR {
        Tim5enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tim6en(&self) -> Tim6enR {
        Tim6enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tim7en(&self) -> Tim7enR {
        Tim7enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tim12en(&self) -> Tim12enR {
        Tim12enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tim13en(&self) -> Tim13enR {
        Tim13enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tim14en(&self) -> Tim14enR {
        Tim14enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tim18en(&self) -> Tim18enR {
        Tim18enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WwdgenR {
        WwdgenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi2en(&self) -> Spi2enR {
        Spi2enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi3en(&self) -> Spi3enR {
        Spi3enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn usart2en(&self) -> Usart2enR {
        Usart2enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn usart3en(&self) -> Usart3enR {
        Usart3enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn uart1en(&self) -> Uart1enR {
        Uart1enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn uart2en(&self) -> Uart2enR {
        Uart2enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2c1enR {
        I2c1enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2c2enR {
        I2c2enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn uart3en(&self) -> Uart3enR {
        Uart3enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn can1en(&self) -> Can1enR {
        Can1enR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn can2en(&self) -> Can2enR {
        Can2enR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bkpen(&self) -> BkpenR {
        BkpenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pwren(&self) -> PwrenR {
        PwrenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dacen(&self) -> DacenR {
        DacenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2c3enR {
        I2c3enR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ctcen(&self) -> CtcenR {
        CtcenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tim2en(&mut self) -> Tim2enW<'_, Apb1enr1Spec> {
        Tim2enW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tim3en(&mut self) -> Tim3enW<'_, Apb1enr1Spec> {
        Tim3enW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tim4en(&mut self) -> Tim4enW<'_, Apb1enr1Spec> {
        Tim4enW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tim5en(&mut self) -> Tim5enW<'_, Apb1enr1Spec> {
        Tim5enW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tim6en(&mut self) -> Tim6enW<'_, Apb1enr1Spec> {
        Tim6enW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tim7en(&mut self) -> Tim7enW<'_, Apb1enr1Spec> {
        Tim7enW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tim12en(&mut self) -> Tim12enW<'_, Apb1enr1Spec> {
        Tim12enW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tim13en(&mut self) -> Tim13enW<'_, Apb1enr1Spec> {
        Tim13enW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tim14en(&mut self) -> Tim14enW<'_, Apb1enr1Spec> {
        Tim14enW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tim18en(&mut self) -> Tim18enW<'_, Apb1enr1Spec> {
        Tim18enW::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WwdgenW<'_, Apb1enr1Spec> {
        WwdgenW::new(self, 11)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi2en(&mut self) -> Spi2enW<'_, Apb1enr1Spec> {
        Spi2enW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi3en(&mut self) -> Spi3enW<'_, Apb1enr1Spec> {
        Spi3enW::new(self, 15)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn usart2en(&mut self) -> Usart2enW<'_, Apb1enr1Spec> {
        Usart2enW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn usart3en(&mut self) -> Usart3enW<'_, Apb1enr1Spec> {
        Usart3enW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn uart1en(&mut self) -> Uart1enW<'_, Apb1enr1Spec> {
        Uart1enW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn uart2en(&mut self) -> Uart2enW<'_, Apb1enr1Spec> {
        Uart2enW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2c1enW<'_, Apb1enr1Spec> {
        I2c1enW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2c2enW<'_, Apb1enr1Spec> {
        I2c2enW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn uart3en(&mut self) -> Uart3enW<'_, Apb1enr1Spec> {
        Uart3enW::new(self, 23)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn can1en(&mut self) -> Can1enW<'_, Apb1enr1Spec> {
        Can1enW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn can2en(&mut self) -> Can2enW<'_, Apb1enr1Spec> {
        Can2enW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bkpen(&mut self) -> BkpenW<'_, Apb1enr1Spec> {
        BkpenW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pwren(&mut self) -> PwrenW<'_, Apb1enr1Spec> {
        PwrenW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DacenW<'_, Apb1enr1Spec> {
        DacenW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2c3enW<'_, Apb1enr1Spec> {
        I2c3enW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ctcen(&mut self) -> CtcenW<'_, Apb1enr1Spec> {
        CtcenW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1enr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1enr1Spec;
impl crate::RegisterSpec for Apb1enr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1enr1::R`](R) reader structure"]
impl crate::Readable for Apb1enr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apb1enr1::W`](W) writer structure"]
impl crate::Writable for Apb1enr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1ENR1 to value 0"]
impl crate::Resettable for Apb1enr1Spec {}
