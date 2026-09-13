#[doc = "Register `APB2ENR` reader"]
pub type R = crate::R<Apb2enrSpec>;
#[doc = "Register `APB2ENR` writer"]
pub type W = crate::W<Apb2enrSpec>;
#[doc = "Field `SYSCFGEN` reader - "]
pub type SyscfgenR = crate::BitReader;
#[doc = "Field `SYSCFGEN` writer - "]
pub type SyscfgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1EN` reader - "]
pub type Adc1enR = crate::BitReader;
#[doc = "Field `ADC1EN` writer - "]
pub type Adc1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2EN` reader - "]
pub type Adc2enR = crate::BitReader;
#[doc = "Field `ADC2EN` writer - "]
pub type Adc2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1EN` reader - "]
pub type Tim1enR = crate::BitReader;
#[doc = "Field `TIM1EN` writer - "]
pub type Tim1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1EN` reader - "]
pub type Spi1enR = crate::BitReader;
#[doc = "Field `SPI1EN` writer - "]
pub type Spi1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8EN` reader - "]
pub type Tim8enR = crate::BitReader;
#[doc = "Field `TIM8EN` writer - "]
pub type Tim8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1EN` reader - "]
pub type Usart1enR = crate::BitReader;
#[doc = "Field `USART1EN` writer - "]
pub type Usart1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3EN` reader - "]
pub type Adc3enR = crate::BitReader;
#[doc = "Field `ADC3EN` writer - "]
pub type Adc3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM9EN` reader - "]
pub type Tim9enR = crate::BitReader;
#[doc = "Field `TIM9EN` writer - "]
pub type Tim9enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM10EN` reader - "]
pub type Tim10enR = crate::BitReader;
#[doc = "Field `TIM10EN` writer - "]
pub type Tim10enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM11EN` reader - "]
pub type Tim11enR = crate::BitReader;
#[doc = "Field `TIM11EN` writer - "]
pub type Tim11enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15EN` reader - "]
pub type Tim15enR = crate::BitReader;
#[doc = "Field `TIM15EN` writer - "]
pub type Tim15enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16EN` reader - "]
pub type Tim16enR = crate::BitReader;
#[doc = "Field `TIM16EN` writer - "]
pub type Tim16enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17EN` reader - "]
pub type Tim17enR = crate::BitReader;
#[doc = "Field `TIM17EN` writer - "]
pub type Tim17enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM19EN` reader - "]
pub type Tim19enR = crate::BitReader;
#[doc = "Field `TIM19EN` writer - "]
pub type Tim19enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGEN` reader - "]
pub type RngenR = crate::BitReader;
#[doc = "Field `RNGEN` writer - "]
pub type RngenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEN` reader - "]
pub type CompenR = crate::BitReader;
#[doc = "Field `COMPEN` writer - "]
pub type CompenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPEN` reader - "]
pub type OpampenR = crate::BitReader;
#[doc = "Field `OPAMPEN` writer - "]
pub type OpampenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAEN` reader - OPAEN"]
pub type OpaenR = crate::BitReader;
#[doc = "Field `OPAEN` writer - OPAEN"]
pub type OpaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCEN` reader - "]
pub type LcdcenR = crate::BitReader;
#[doc = "Field `LCDCEN` writer - "]
pub type LcdcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SyscfgenR {
        SyscfgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adc1en(&self) -> Adc1enR {
        Adc1enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn adc2en(&self) -> Adc2enR {
        Adc2enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tim1en(&self) -> Tim1enR {
        Tim1enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tim8en(&self) -> Tim8enR {
        Tim8enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn usart1en(&self) -> Usart1enR {
        Usart1enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adc3en(&self) -> Adc3enR {
        Adc3enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tim9en(&self) -> Tim9enR {
        Tim9enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tim10en(&self) -> Tim10enR {
        Tim10enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tim11en(&self) -> Tim11enR {
        Tim11enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tim15en(&self) -> Tim15enR {
        Tim15enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tim16en(&self) -> Tim16enR {
        Tim16enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tim17en(&self) -> Tim17enR {
        Tim17enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tim19en(&self) -> Tim19enR {
        Tim19enR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rngen(&self) -> RngenR {
        RngenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn compen(&self) -> CompenR {
        CompenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn opampen(&self) -> OpampenR {
        OpampenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 28 - OPAEN"]
    #[inline(always)]
    pub fn opaen(&self) -> OpaenR {
        OpaenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lcdcen(&self) -> LcdcenR {
        LcdcenR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SyscfgenW<'_, Apb2enrSpec> {
        SyscfgenW::new(self, 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adc1en(&mut self) -> Adc1enW<'_, Apb2enrSpec> {
        Adc1enW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn adc2en(&mut self) -> Adc2enW<'_, Apb2enrSpec> {
        Adc2enW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> Tim1enW<'_, Apb2enrSpec> {
        Tim1enW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> Spi1enW<'_, Apb2enrSpec> {
        Spi1enW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tim8en(&mut self) -> Tim8enW<'_, Apb2enrSpec> {
        Tim8enW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> Usart1enW<'_, Apb2enrSpec> {
        Usart1enW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adc3en(&mut self) -> Adc3enW<'_, Apb2enrSpec> {
        Adc3enW::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tim9en(&mut self) -> Tim9enW<'_, Apb2enrSpec> {
        Tim9enW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tim10en(&mut self) -> Tim10enW<'_, Apb2enrSpec> {
        Tim10enW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tim11en(&mut self) -> Tim11enW<'_, Apb2enrSpec> {
        Tim11enW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tim15en(&mut self) -> Tim15enW<'_, Apb2enrSpec> {
        Tim15enW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tim16en(&mut self) -> Tim16enW<'_, Apb2enrSpec> {
        Tim16enW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tim17en(&mut self) -> Tim17enW<'_, Apb2enrSpec> {
        Tim17enW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tim19en(&mut self) -> Tim19enW<'_, Apb2enrSpec> {
        Tim19enW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rngen(&mut self) -> RngenW<'_, Apb2enrSpec> {
        RngenW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn compen(&mut self) -> CompenW<'_, Apb2enrSpec> {
        CompenW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn opampen(&mut self) -> OpampenW<'_, Apb2enrSpec> {
        OpampenW::new(self, 28)
    }
    #[doc = "Bit 28 - OPAEN"]
    #[inline(always)]
    pub fn opaen(&mut self) -> OpaenW<'_, Apb2enrSpec> {
        OpaenW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lcdcen(&mut self) -> LcdcenW<'_, Apb2enrSpec> {
        LcdcenW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2enrSpec;
impl crate::RegisterSpec for Apb2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2enr::R`](R) reader structure"]
impl crate::Readable for Apb2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure"]
impl crate::Writable for Apb2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for Apb2enrSpec {}
