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
#[doc = "Field `CC3IF` reader - "]
pub type Cc3ifR = crate::BitReader;
#[doc = "Field `CC3IF` writer - "]
pub type Cc3ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IF` reader - "]
pub type Cc4ifR = crate::BitReader;
#[doc = "Field `CC4IF` writer - "]
pub type Cc4ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIF` reader - "]
pub type TifR = crate::BitReader;
#[doc = "Field `TIF` writer - "]
pub type TifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OF` reader - "]
pub type Cc1ofR = crate::BitReader;
#[doc = "Field `CC1OF` writer - "]
pub type Cc1ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2OF` reader - "]
pub type Cc2ofR = crate::BitReader;
#[doc = "Field `CC2OF` writer - "]
pub type Cc2ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3OF` reader - "]
pub type Cc3ofR = crate::BitReader;
#[doc = "Field `CC3OF` writer - "]
pub type Cc3ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4OF` reader - "]
pub type Cc4ofR = crate::BitReader;
#[doc = "Field `CC4OF` writer - "]
pub type Cc4ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC1IR` reader - "]
pub type Ic1irR = crate::BitReader;
#[doc = "Field `IC1IR` writer - "]
pub type Ic1irW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC2IR` reader - "]
pub type Ic2irR = crate::BitReader;
#[doc = "Field `IC2IR` writer - "]
pub type Ic2irW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDXF` reader - "]
pub type IdxfR = crate::BitReader;
#[doc = "Field `IDXF` writer - "]
pub type IdxfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRF` reader - "]
pub type DirfR = crate::BitReader;
#[doc = "Field `DIRF` writer - "]
pub type DirfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IERRF` reader - "]
pub type IerrfR = crate::BitReader;
#[doc = "Field `IERRF` writer - "]
pub type IerrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERRF` reader - "]
pub type TerrfR = crate::BitReader;
#[doc = "Field `TERRF` writer - "]
pub type TerrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC3IR` reader - "]
pub type Ic3irR = crate::BitReader;
#[doc = "Field `IC3IR` writer - "]
pub type Ic3irW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC4IR` reader - "]
pub type Ic4irR = crate::BitReader;
#[doc = "Field `IC4IR` writer - "]
pub type Ic4irW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC1IF` reader - "]
pub type Ic1ifR = crate::BitReader;
#[doc = "Field `IC1IF` writer - "]
pub type Ic1ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC2IF` reader - "]
pub type Ic2ifR = crate::BitReader;
#[doc = "Field `IC2IF` writer - "]
pub type Ic2ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC3IF` reader - "]
pub type Ic3ifR = crate::BitReader;
#[doc = "Field `IC3IF` writer - "]
pub type Ic3ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC4IF` reader - "]
pub type Ic4ifR = crate::BitReader;
#[doc = "Field `IC4IF` writer - "]
pub type Ic4ifW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cc3if(&self) -> Cc3ifR {
        Cc3ifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cc4if(&self) -> Cc4ifR {
        Cc4ifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tif(&self) -> TifR {
        TifR::new(((self.bits >> 6) & 1) != 0)
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
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cc3of(&self) -> Cc3ofR {
        Cc3ofR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cc4of(&self) -> Cc4ofR {
        Cc4ofR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ic1ir(&self) -> Ic1irR {
        Ic1irR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ic2ir(&self) -> Ic2irR {
        Ic2irR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn idxf(&self) -> IdxfR {
        IdxfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dirf(&self) -> DirfR {
        DirfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ierrf(&self) -> IerrfR {
        IerrfR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn terrf(&self) -> TerrfR {
        TerrfR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ic3ir(&self) -> Ic3irR {
        Ic3irR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ic4ir(&self) -> Ic4irR {
        Ic4irR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ic1if(&self) -> Ic1ifR {
        Ic1ifR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ic2if(&self) -> Ic2ifR {
        Ic2ifR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ic3if(&self) -> Ic3ifR {
        Ic3ifR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ic4if(&self) -> Ic4ifR {
        Ic4ifR::new(((self.bits >> 29) & 1) != 0)
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cc3if(&mut self) -> Cc3ifW<'_, SrSpec> {
        Cc3ifW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cc4if(&mut self) -> Cc4ifW<'_, SrSpec> {
        Cc4ifW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tif(&mut self) -> TifW<'_, SrSpec> {
        TifW::new(self, 6)
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
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cc3of(&mut self) -> Cc3ofW<'_, SrSpec> {
        Cc3ofW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cc4of(&mut self) -> Cc4ofW<'_, SrSpec> {
        Cc4ofW::new(self, 12)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ic1ir(&mut self) -> Ic1irW<'_, SrSpec> {
        Ic1irW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ic2ir(&mut self) -> Ic2irW<'_, SrSpec> {
        Ic2irW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn idxf(&mut self) -> IdxfW<'_, SrSpec> {
        IdxfW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dirf(&mut self) -> DirfW<'_, SrSpec> {
        DirfW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ierrf(&mut self) -> IerrfW<'_, SrSpec> {
        IerrfW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn terrf(&mut self) -> TerrfW<'_, SrSpec> {
        TerrfW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ic3ir(&mut self) -> Ic3irW<'_, SrSpec> {
        Ic3irW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ic4ir(&mut self) -> Ic4irW<'_, SrSpec> {
        Ic4irW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ic1if(&mut self) -> Ic1ifW<'_, SrSpec> {
        Ic1ifW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ic2if(&mut self) -> Ic2ifW<'_, SrSpec> {
        Ic2ifW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ic3if(&mut self) -> Ic3ifW<'_, SrSpec> {
        Ic3ifW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ic4if(&mut self) -> Ic4ifW<'_, SrSpec> {
        Ic4ifW::new(self, 29)
    }
}
#[doc = "desc SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
