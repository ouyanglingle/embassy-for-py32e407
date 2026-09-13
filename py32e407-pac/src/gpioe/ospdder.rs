#[doc = "Register `OSPDDER` reader"]
pub type R = crate::R<OspdderSpec>;
#[doc = "Register `OSPDDER` writer"]
pub type W = crate::W<OspdderSpec>;
#[doc = "Field `OSPEED0` reader - GPIOE0 output speed"]
pub type Ospeed0R = crate::FieldReader;
#[doc = "Field `OSPEED0` writer - GPIOE0 output speed"]
pub type Ospeed0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED1` reader - GPIOE1 output speed"]
pub type Ospeed1R = crate::FieldReader;
#[doc = "Field `OSPEED1` writer - GPIOE1 output speed"]
pub type Ospeed1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED2` reader - GPIOE2 output speed"]
pub type Ospeed2R = crate::FieldReader;
#[doc = "Field `OSPEED2` writer - GPIOE2 output speed"]
pub type Ospeed2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED3` reader - GPIOE3 output speed"]
pub type Ospeed3R = crate::FieldReader;
#[doc = "Field `OSPEED3` writer - GPIOE3 output speed"]
pub type Ospeed3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED4` reader - GPIOE4 output speed"]
pub type Ospeed4R = crate::FieldReader;
#[doc = "Field `OSPEED4` writer - GPIOE4 output speed"]
pub type Ospeed4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED5` reader - GPIOE5 output speed"]
pub type Ospeed5R = crate::FieldReader;
#[doc = "Field `OSPEED5` writer - GPIOE5 output speed"]
pub type Ospeed5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED6` reader - GPIOE6 output speed"]
pub type Ospeed6R = crate::FieldReader;
#[doc = "Field `OSPEED6` writer - GPIOE6 output speed"]
pub type Ospeed6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED7` reader - GPIOE7 output speed"]
pub type Ospeed7R = crate::FieldReader;
#[doc = "Field `OSPEED7` writer - GPIOE7 output speed"]
pub type Ospeed7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED8` reader - GPIOE8 output speed"]
pub type Ospeed8R = crate::FieldReader;
#[doc = "Field `OSPEED8` writer - GPIOE8 output speed"]
pub type Ospeed8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED9` reader - GPIOE9 output speed"]
pub type Ospeed9R = crate::FieldReader;
#[doc = "Field `OSPEED9` writer - GPIOE9 output speed"]
pub type Ospeed9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED10` reader - GPIOE10 output speed"]
pub type Ospeed10R = crate::FieldReader;
#[doc = "Field `OSPEED10` writer - GPIOE10 output speed"]
pub type Ospeed10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED11` reader - GPIOE11 output speed"]
pub type Ospeed11R = crate::FieldReader;
#[doc = "Field `OSPEED11` writer - GPIOE11 output speed"]
pub type Ospeed11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED12` reader - GPIOE12 output speed"]
pub type Ospeed12R = crate::FieldReader;
#[doc = "Field `OSPEED12` writer - GPIOE12 output speed"]
pub type Ospeed12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED13` reader - GPIOE13 output speed"]
pub type Ospeed13R = crate::FieldReader;
#[doc = "Field `OSPEED13` writer - GPIOE13 output speed"]
pub type Ospeed13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED14` reader - GPIOE14 output speed"]
pub type Ospeed14R = crate::FieldReader;
#[doc = "Field `OSPEED14` writer - GPIOE14 output speed"]
pub type Ospeed14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED15` reader - GPIOE15 output speed"]
pub type Ospeed15R = crate::FieldReader;
#[doc = "Field `OSPEED15` writer - GPIOE15 output speed"]
pub type Ospeed15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - GPIOE0 output speed"]
    #[inline(always)]
    pub fn ospeed0(&self) -> Ospeed0R {
        Ospeed0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIOE1 output speed"]
    #[inline(always)]
    pub fn ospeed1(&self) -> Ospeed1R {
        Ospeed1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIOE2 output speed"]
    #[inline(always)]
    pub fn ospeed2(&self) -> Ospeed2R {
        Ospeed2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIOE3 output speed"]
    #[inline(always)]
    pub fn ospeed3(&self) -> Ospeed3R {
        Ospeed3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIOE4 output speed"]
    #[inline(always)]
    pub fn ospeed4(&self) -> Ospeed4R {
        Ospeed4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIOE5 output speed"]
    #[inline(always)]
    pub fn ospeed5(&self) -> Ospeed5R {
        Ospeed5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIOE6 output speed"]
    #[inline(always)]
    pub fn ospeed6(&self) -> Ospeed6R {
        Ospeed6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIOE7 output speed"]
    #[inline(always)]
    pub fn ospeed7(&self) -> Ospeed7R {
        Ospeed7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPIOE8 output speed"]
    #[inline(always)]
    pub fn ospeed8(&self) -> Ospeed8R {
        Ospeed8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPIOE9 output speed"]
    #[inline(always)]
    pub fn ospeed9(&self) -> Ospeed9R {
        Ospeed9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPIOE10 output speed"]
    #[inline(always)]
    pub fn ospeed10(&self) -> Ospeed10R {
        Ospeed10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - GPIOE11 output speed"]
    #[inline(always)]
    pub fn ospeed11(&self) -> Ospeed11R {
        Ospeed11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GPIOE12 output speed"]
    #[inline(always)]
    pub fn ospeed12(&self) -> Ospeed12R {
        Ospeed12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - GPIOE13 output speed"]
    #[inline(always)]
    pub fn ospeed13(&self) -> Ospeed13R {
        Ospeed13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - GPIOE14 output speed"]
    #[inline(always)]
    pub fn ospeed14(&self) -> Ospeed14R {
        Ospeed14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - GPIOE15 output speed"]
    #[inline(always)]
    pub fn ospeed15(&self) -> Ospeed15R {
        Ospeed15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIOE0 output speed"]
    #[inline(always)]
    pub fn ospeed0(&mut self) -> Ospeed0W<'_, OspdderSpec> {
        Ospeed0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIOE1 output speed"]
    #[inline(always)]
    pub fn ospeed1(&mut self) -> Ospeed1W<'_, OspdderSpec> {
        Ospeed1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIOE2 output speed"]
    #[inline(always)]
    pub fn ospeed2(&mut self) -> Ospeed2W<'_, OspdderSpec> {
        Ospeed2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIOE3 output speed"]
    #[inline(always)]
    pub fn ospeed3(&mut self) -> Ospeed3W<'_, OspdderSpec> {
        Ospeed3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIOE4 output speed"]
    #[inline(always)]
    pub fn ospeed4(&mut self) -> Ospeed4W<'_, OspdderSpec> {
        Ospeed4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIOE5 output speed"]
    #[inline(always)]
    pub fn ospeed5(&mut self) -> Ospeed5W<'_, OspdderSpec> {
        Ospeed5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIOE6 output speed"]
    #[inline(always)]
    pub fn ospeed6(&mut self) -> Ospeed6W<'_, OspdderSpec> {
        Ospeed6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIOE7 output speed"]
    #[inline(always)]
    pub fn ospeed7(&mut self) -> Ospeed7W<'_, OspdderSpec> {
        Ospeed7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - GPIOE8 output speed"]
    #[inline(always)]
    pub fn ospeed8(&mut self) -> Ospeed8W<'_, OspdderSpec> {
        Ospeed8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - GPIOE9 output speed"]
    #[inline(always)]
    pub fn ospeed9(&mut self) -> Ospeed9W<'_, OspdderSpec> {
        Ospeed9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - GPIOE10 output speed"]
    #[inline(always)]
    pub fn ospeed10(&mut self) -> Ospeed10W<'_, OspdderSpec> {
        Ospeed10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - GPIOE11 output speed"]
    #[inline(always)]
    pub fn ospeed11(&mut self) -> Ospeed11W<'_, OspdderSpec> {
        Ospeed11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - GPIOE12 output speed"]
    #[inline(always)]
    pub fn ospeed12(&mut self) -> Ospeed12W<'_, OspdderSpec> {
        Ospeed12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - GPIOE13 output speed"]
    #[inline(always)]
    pub fn ospeed13(&mut self) -> Ospeed13W<'_, OspdderSpec> {
        Ospeed13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - GPIOE14 output speed"]
    #[inline(always)]
    pub fn ospeed14(&mut self) -> Ospeed14W<'_, OspdderSpec> {
        Ospeed14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - GPIOE15 output speed"]
    #[inline(always)]
    pub fn ospeed15(&mut self) -> Ospeed15W<'_, OspdderSpec> {
        Ospeed15W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ospdder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospdder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OspdderSpec;
impl crate::RegisterSpec for OspdderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospdder::R`](R) reader structure"]
impl crate::Readable for OspdderSpec {}
#[doc = "`write(|w| ..)` method takes [`ospdder::W`](W) writer structure"]
impl crate::Writable for OspdderSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSPDDER to value 0"]
impl crate::Resettable for OspdderSpec {}
