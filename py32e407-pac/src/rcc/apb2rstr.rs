#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<Apb2rstrSpec>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<Apb2rstrSpec>;
#[doc = "Field `SYSCFGRST` reader - "]
pub type SyscfgrstR = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - "]
pub type SyscfgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1RST` reader - "]
pub type Adc1rstR = crate::BitReader;
#[doc = "Field `ADC1RST` writer - "]
pub type Adc1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2RST` reader - "]
pub type Adc2rstR = crate::BitReader;
#[doc = "Field `ADC2RST` writer - "]
pub type Adc2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1RST` reader - "]
pub type Tim1rstR = crate::BitReader;
#[doc = "Field `TIM1RST` writer - "]
pub type Tim1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - "]
pub type Spi1rstR = crate::BitReader;
#[doc = "Field `SPI1RST` writer - "]
pub type Spi1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8RST` reader - "]
pub type Tim8rstR = crate::BitReader;
#[doc = "Field `TIM8RST` writer - "]
pub type Tim8rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - "]
pub type Usart1rstR = crate::BitReader;
#[doc = "Field `USART1RST` writer - "]
pub type Usart1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3RST` reader - "]
pub type Adc3rstR = crate::BitReader;
#[doc = "Field `ADC3RST` writer - "]
pub type Adc3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM9RST` reader - "]
pub type Tim9rstR = crate::BitReader;
#[doc = "Field `TIM9RST` writer - "]
pub type Tim9rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM10RST` reader - "]
pub type Tim10rstR = crate::BitReader;
#[doc = "Field `TIM10RST` writer - "]
pub type Tim10rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM11RST` reader - "]
pub type Tim11rstR = crate::BitReader;
#[doc = "Field `TIM11RST` writer - "]
pub type Tim11rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15RST` reader - "]
pub type Tim15rstR = crate::BitReader;
#[doc = "Field `TIM15RST` writer - "]
pub type Tim15rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16RST` reader - "]
pub type Tim16rstR = crate::BitReader;
#[doc = "Field `TIM16RST` writer - "]
pub type Tim16rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17RST` reader - "]
pub type Tim17rstR = crate::BitReader;
#[doc = "Field `TIM17RST` writer - "]
pub type Tim17rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM19RST` reader - "]
pub type Tim19rstR = crate::BitReader;
#[doc = "Field `TIM19RST` writer - "]
pub type Tim19rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGRST` reader - "]
pub type RngrstR = crate::BitReader;
#[doc = "Field `RNGRST` writer - "]
pub type RngrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPRST` reader - "]
pub type ComprstR = crate::BitReader;
#[doc = "Field `COMPRST` writer - "]
pub type ComprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPRST` reader - "]
pub type OpamprstR = crate::BitReader;
#[doc = "Field `OPAMPRST` writer - "]
pub type OpamprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCRST` reader - "]
pub type LcdcrstR = crate::BitReader;
#[doc = "Field `LCDCRST` writer - "]
pub type LcdcrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SyscfgrstR {
        SyscfgrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adc1rst(&self) -> Adc1rstR {
        Adc1rstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn adc2rst(&self) -> Adc2rstR {
        Adc2rstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tim1rst(&self) -> Tim1rstR {
        Tim1rstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        Spi1rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tim8rst(&self) -> Tim8rstR {
        Tim8rstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn usart1rst(&self) -> Usart1rstR {
        Usart1rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adc3rst(&self) -> Adc3rstR {
        Adc3rstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tim9rst(&self) -> Tim9rstR {
        Tim9rstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tim10rst(&self) -> Tim10rstR {
        Tim10rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tim11rst(&self) -> Tim11rstR {
        Tim11rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tim15rst(&self) -> Tim15rstR {
        Tim15rstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tim16rst(&self) -> Tim16rstR {
        Tim16rstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tim17rst(&self) -> Tim17rstR {
        Tim17rstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tim19rst(&self) -> Tim19rstR {
        Tim19rstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rngrst(&self) -> RngrstR {
        RngrstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn comprst(&self) -> ComprstR {
        ComprstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn opamprst(&self) -> OpamprstR {
        OpamprstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lcdcrst(&self) -> LcdcrstR {
        LcdcrstR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SyscfgrstW<'_, Apb2rstrSpec> {
        SyscfgrstW::new(self, 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adc1rst(&mut self) -> Adc1rstW<'_, Apb2rstrSpec> {
        Adc1rstW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn adc2rst(&mut self) -> Adc2rstW<'_, Apb2rstrSpec> {
        Adc2rstW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tim1rst(&mut self) -> Tim1rstW<'_, Apb2rstrSpec> {
        Tim1rstW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> Spi1rstW<'_, Apb2rstrSpec> {
        Spi1rstW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tim8rst(&mut self) -> Tim8rstW<'_, Apb2rstrSpec> {
        Tim8rstW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> Usart1rstW<'_, Apb2rstrSpec> {
        Usart1rstW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adc3rst(&mut self) -> Adc3rstW<'_, Apb2rstrSpec> {
        Adc3rstW::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tim9rst(&mut self) -> Tim9rstW<'_, Apb2rstrSpec> {
        Tim9rstW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tim10rst(&mut self) -> Tim10rstW<'_, Apb2rstrSpec> {
        Tim10rstW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tim11rst(&mut self) -> Tim11rstW<'_, Apb2rstrSpec> {
        Tim11rstW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tim15rst(&mut self) -> Tim15rstW<'_, Apb2rstrSpec> {
        Tim15rstW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tim16rst(&mut self) -> Tim16rstW<'_, Apb2rstrSpec> {
        Tim16rstW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tim17rst(&mut self) -> Tim17rstW<'_, Apb2rstrSpec> {
        Tim17rstW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tim19rst(&mut self) -> Tim19rstW<'_, Apb2rstrSpec> {
        Tim19rstW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rngrst(&mut self) -> RngrstW<'_, Apb2rstrSpec> {
        RngrstW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn comprst(&mut self) -> ComprstW<'_, Apb2rstrSpec> {
        ComprstW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn opamprst(&mut self) -> OpamprstW<'_, Apb2rstrSpec> {
        OpamprstW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lcdcrst(&mut self) -> LcdcrstW<'_, Apb2rstrSpec> {
        LcdcrstW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2rstrSpec;
impl crate::RegisterSpec for Apb2rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rstr::R`](R) reader structure"]
impl crate::Readable for Apb2rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure"]
impl crate::Writable for Apb2rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for Apb2rstrSpec {}
