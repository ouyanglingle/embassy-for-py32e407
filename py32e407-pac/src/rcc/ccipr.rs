#[doc = "Register `CCIPR` reader"]
pub type R = crate::R<CciprSpec>;
#[doc = "Register `CCIPR` writer"]
pub type W = crate::W<CciprSpec>;
#[doc = "Field `USART1SEL` reader - "]
pub type Usart1selR = crate::FieldReader;
#[doc = "Field `USART1SEL` writer - "]
pub type Usart1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USART2SEL` reader - "]
pub type Usart2selR = crate::FieldReader;
#[doc = "Field `USART2SEL` writer - "]
pub type Usart2selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USART3SEL` reader - "]
pub type Usart3selR = crate::FieldReader;
#[doc = "Field `USART3SEL` writer - "]
pub type Usart3selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPUART1SEL` reader - "]
pub type Lpuart1selR = crate::FieldReader;
#[doc = "Field `LPUART1SEL` writer - "]
pub type Lpuart1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPTIM1SEL` reader - "]
pub type Lptim1selR = crate::FieldReader;
#[doc = "Field `LPTIM1SEL` writer - "]
pub type Lptim1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP1SEL` reader - "]
pub type Comp1selR = crate::FieldReader;
#[doc = "Field `COMP1SEL` writer - "]
pub type Comp1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP2SEL` reader - "]
pub type Comp2selR = crate::FieldReader;
#[doc = "Field `COMP2SEL` writer - "]
pub type Comp2selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP3SEL` reader - "]
pub type Comp3selR = crate::FieldReader;
#[doc = "Field `COMP3SEL` writer - "]
pub type Comp3selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP4SEL` reader - "]
pub type Comp4selR = crate::FieldReader;
#[doc = "Field `COMP4SEL` writer - "]
pub type Comp4selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn usart1sel(&self) -> Usart1selR {
        Usart1selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn usart2sel(&self) -> Usart2selR {
        Usart2selR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn usart3sel(&self) -> Usart3selR {
        Usart3selR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> Lpuart1selR {
        Lpuart1selR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> Lptim1selR {
        Lptim1selR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn comp1sel(&self) -> Comp1selR {
        Comp1selR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn comp2sel(&self) -> Comp2selR {
        Comp2selR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn comp3sel(&self) -> Comp3selR {
        Comp3selR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn comp4sel(&self) -> Comp4selR {
        Comp4selR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn usart1sel(&mut self) -> Usart1selW<'_, CciprSpec> {
        Usart1selW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn usart2sel(&mut self) -> Usart2selW<'_, CciprSpec> {
        Usart2selW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn usart3sel(&mut self) -> Usart3selW<'_, CciprSpec> {
        Usart3selW::new(self, 4)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> Lpuart1selW<'_, CciprSpec> {
        Lpuart1selW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> Lptim1selW<'_, CciprSpec> {
        Lptim1selW::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn comp1sel(&mut self) -> Comp1selW<'_, CciprSpec> {
        Comp1selW::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn comp2sel(&mut self) -> Comp2selW<'_, CciprSpec> {
        Comp2selW::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn comp3sel(&mut self) -> Comp3selW<'_, CciprSpec> {
        Comp3selW::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn comp4sel(&mut self) -> Comp4selW<'_, CciprSpec> {
        Comp4selW::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CciprSpec;
impl crate::RegisterSpec for CciprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr::R`](R) reader structure"]
impl crate::Readable for CciprSpec {}
#[doc = "`write(|w| ..)` method takes [`ccipr::W`](W) writer structure"]
impl crate::Writable for CciprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CciprSpec {}
