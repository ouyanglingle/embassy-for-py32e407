#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `UIF` reader - "]
pub type UifR = crate::BitReader;
#[doc = "Field `UIF` writer - "]
pub type UifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IF` reader - "]
pub type Cc1ifR = crate::BitReader;
#[doc = "Field `CC1IF` writer - "]
pub type Cc1ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IF` reader - "]
pub type Cc2ifR = crate::BitReader;
#[doc = "Field `CC2IF` writer - "]
pub type Cc2ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIF` reader - "]
pub type ComifR = crate::BitReader;
#[doc = "Field `COMIF` writer - "]
pub type ComifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIF` reader - "]
pub type TifR = crate::BitReader;
#[doc = "Field `TIF` writer - "]
pub type TifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIF` reader - "]
pub type BifR = crate::BitReader;
#[doc = "Field `BIF` writer - "]
pub type BifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OF` reader - "]
pub type Cc1ofR = crate::BitReader;
#[doc = "Field `CC1OF` writer - "]
pub type Cc1ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2OF` reader - "]
pub type Cc2ofR = crate::BitReader;
#[doc = "Field `CC2OF` writer - "]
pub type Cc2ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC1IR` reader - "]
pub type Ic1irR = crate::BitReader;
#[doc = "Field `IC1IR` writer - "]
pub type Ic1irW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC2IR` reader - "]
pub type Ic2irR = crate::BitReader;
#[doc = "Field `IC2IR` writer - "]
pub type Ic2irW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC1IF` reader - "]
pub type Ic1ifR = crate::BitReader;
#[doc = "Field `IC1IF` writer - "]
pub type Ic1ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC2IF` reader - "]
pub type Ic2ifR = crate::BitReader;
#[doc = "Field `IC2IF` writer - "]
pub type Ic2ifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uif(&self) -> UifR {
        UifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cc1if(&self) -> Cc1ifR {
        Cc1ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cc2if(&self) -> Cc2ifR {
        Cc2ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn comif(&self) -> ComifR {
        ComifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tif(&self) -> TifR {
        TifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bif(&self) -> BifR {
        BifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cc1of(&self) -> Cc1ofR {
        Cc1ofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cc2of(&self) -> Cc2ofR {
        Cc2ofR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ic1ir(&self) -> Ic1irR {
        Ic1irR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ic2ir(&self) -> Ic2irR {
        Ic2irR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ic1if(&self) -> Ic1ifR {
        Ic1ifR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ic2if(&self) -> Ic2ifR {
        Ic2ifR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uif(&mut self) -> UifW<'_, SrSpec> {
        UifW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cc1if(&mut self) -> Cc1ifW<'_, SrSpec> {
        Cc1ifW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cc2if(&mut self) -> Cc2ifW<'_, SrSpec> {
        Cc2ifW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn comif(&mut self) -> ComifW<'_, SrSpec> {
        ComifW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tif(&mut self) -> TifW<'_, SrSpec> {
        TifW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bif(&mut self) -> BifW<'_, SrSpec> {
        BifW::new(self, 7)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cc1of(&mut self) -> Cc1ofW<'_, SrSpec> {
        Cc1ofW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cc2of(&mut self) -> Cc2ofW<'_, SrSpec> {
        Cc2ofW::new(self, 10)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ic1ir(&mut self) -> Ic1irW<'_, SrSpec> {
        Ic1irW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ic2ir(&mut self) -> Ic2irW<'_, SrSpec> {
        Ic2irW::new(self, 17)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ic1if(&mut self) -> Ic1ifW<'_, SrSpec> {
        Ic1ifW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ic2if(&mut self) -> Ic2ifW<'_, SrSpec> {
        Ic2ifW::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
