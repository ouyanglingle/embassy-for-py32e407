#[doc = "Register `ADDR24` reader"]
pub type R = crate::R<Addr24Spec>;
#[doc = "Register `ADDR24` writer"]
pub type W = crate::W<Addr24Spec>;
#[doc = "Field `INS1` reader - command1"]
pub type Ins1R = crate::FieldReader;
#[doc = "Field `INS1` writer - command1"]
pub type Ins1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDR0` reader - address0"]
pub type Addr0R = crate::FieldReader;
#[doc = "Field `ADDR0` writer - address0"]
pub type Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDR1` reader - address1"]
pub type Addr1R = crate::FieldReader;
#[doc = "Field `ADDR1` writer - address1"]
pub type Addr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDR2` reader - address2"]
pub type Addr2R = crate::FieldReader;
#[doc = "Field `ADDR2` writer - address2"]
pub type Addr2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - command1"]
    #[inline(always)]
    pub fn ins1(&self) -> Ins1R {
        Ins1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - address0"]
    #[inline(always)]
    pub fn addr0(&self) -> Addr0R {
        Addr0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - address1"]
    #[inline(always)]
    pub fn addr1(&self) -> Addr1R {
        Addr1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - address2"]
    #[inline(always)]
    pub fn addr2(&self) -> Addr2R {
        Addr2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - command1"]
    #[inline(always)]
    pub fn ins1(&mut self) -> Ins1W<'_, Addr24Spec> {
        Ins1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - address0"]
    #[inline(always)]
    pub fn addr0(&mut self) -> Addr0W<'_, Addr24Spec> {
        Addr0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - address1"]
    #[inline(always)]
    pub fn addr1(&mut self) -> Addr1W<'_, Addr24Spec> {
        Addr1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - address2"]
    #[inline(always)]
    pub fn addr2(&mut self) -> Addr2W<'_, Addr24Spec> {
        Addr2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`addr24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr24Spec;
impl crate::RegisterSpec for Addr24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr24::R`](R) reader structure"]
impl crate::Readable for Addr24Spec {}
#[doc = "`write(|w| ..)` method takes [`addr24::W`](W) writer structure"]
impl crate::Writable for Addr24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR24 to value 0"]
impl crate::Resettable for Addr24Spec {}
