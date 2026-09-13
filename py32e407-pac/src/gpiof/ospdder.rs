#[doc = "Register `OSPDDER` reader"]
pub type R = crate::R<OspdderSpec>;
#[doc = "Register `OSPDDER` writer"]
pub type W = crate::W<OspdderSpec>;
#[doc = "Field `OSPEED0` reader - GPIOF0 output speed"]
pub type Ospeed0R = crate::FieldReader;
#[doc = "Field `OSPEED0` writer - GPIOF0 output speed"]
pub type Ospeed0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED1` reader - GPIOF1 output speed"]
pub type Ospeed1R = crate::FieldReader;
#[doc = "Field `OSPEED1` writer - GPIOF1 output speed"]
pub type Ospeed1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED2` reader - GPIOF2 output speed"]
pub type Ospeed2R = crate::FieldReader;
#[doc = "Field `OSPEED2` writer - GPIOF2 output speed"]
pub type Ospeed2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED3` reader - GPIOF3 output speed"]
pub type Ospeed3R = crate::FieldReader;
#[doc = "Field `OSPEED3` writer - GPIOF3 output speed"]
pub type Ospeed3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED4` reader - GPIOF4 output speed"]
pub type Ospeed4R = crate::FieldReader;
#[doc = "Field `OSPEED4` writer - GPIOF4 output speed"]
pub type Ospeed4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED5` reader - GPIOF5 output speed"]
pub type Ospeed5R = crate::FieldReader;
#[doc = "Field `OSPEED5` writer - GPIOF5 output speed"]
pub type Ospeed5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - GPIOF0 output speed"]
    #[inline(always)]
    pub fn ospeed0(&self) -> Ospeed0R {
        Ospeed0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIOF1 output speed"]
    #[inline(always)]
    pub fn ospeed1(&self) -> Ospeed1R {
        Ospeed1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIOF2 output speed"]
    #[inline(always)]
    pub fn ospeed2(&self) -> Ospeed2R {
        Ospeed2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIOF3 output speed"]
    #[inline(always)]
    pub fn ospeed3(&self) -> Ospeed3R {
        Ospeed3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIOF4 output speed"]
    #[inline(always)]
    pub fn ospeed4(&self) -> Ospeed4R {
        Ospeed4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIOF5 output speed"]
    #[inline(always)]
    pub fn ospeed5(&self) -> Ospeed5R {
        Ospeed5R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIOF0 output speed"]
    #[inline(always)]
    pub fn ospeed0(&mut self) -> Ospeed0W<'_, OspdderSpec> {
        Ospeed0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIOF1 output speed"]
    #[inline(always)]
    pub fn ospeed1(&mut self) -> Ospeed1W<'_, OspdderSpec> {
        Ospeed1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIOF2 output speed"]
    #[inline(always)]
    pub fn ospeed2(&mut self) -> Ospeed2W<'_, OspdderSpec> {
        Ospeed2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIOF3 output speed"]
    #[inline(always)]
    pub fn ospeed3(&mut self) -> Ospeed3W<'_, OspdderSpec> {
        Ospeed3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIOF4 output speed"]
    #[inline(always)]
    pub fn ospeed4(&mut self) -> Ospeed4W<'_, OspdderSpec> {
        Ospeed4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIOF5 output speed"]
    #[inline(always)]
    pub fn ospeed5(&mut self) -> Ospeed5W<'_, OspdderSpec> {
        Ospeed5W::new(self, 10)
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
