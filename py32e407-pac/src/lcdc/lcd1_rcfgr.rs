#[doc = "Register `LCD1_RCFGR` reader"]
pub type R = crate::R<Lcd1RcfgrSpec>;
#[doc = "Register `LCD1_RCFGR` writer"]
pub type W = crate::W<Lcd1RcfgrSpec>;
#[doc = "Field `ADDSET` reader - LCD1 read ADDSET"]
pub type AddsetR = crate::FieldReader;
#[doc = "Field `ADDSET` writer - LCD1 read ADDSET"]
pub type AddsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAST` reader - LCD1 read DATAST"]
pub type DatastR = crate::FieldReader;
#[doc = "Field `DATAST` writer - LCD1 read DATAST"]
pub type DatastW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATAHLD` reader - LCD1 read DATAHLD"]
pub type DatahldR = crate::FieldReader;
#[doc = "Field `DATAHLD` writer - LCD1 read DATAHLD"]
pub type DatahldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - LCD1 read ADDSET"]
    #[inline(always)]
    pub fn addset(&self) -> AddsetR {
        AddsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - LCD1 read DATAST"]
    #[inline(always)]
    pub fn datast(&self) -> DatastR {
        DatastR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 30:31 - LCD1 read DATAHLD"]
    #[inline(always)]
    pub fn datahld(&self) -> DatahldR {
        DatahldR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - LCD1 read ADDSET"]
    #[inline(always)]
    pub fn addset(&mut self) -> AddsetW<'_, Lcd1RcfgrSpec> {
        AddsetW::new(self, 0)
    }
    #[doc = "Bits 8:15 - LCD1 read DATAST"]
    #[inline(always)]
    pub fn datast(&mut self) -> DatastW<'_, Lcd1RcfgrSpec> {
        DatastW::new(self, 8)
    }
    #[doc = "Bits 30:31 - LCD1 read DATAHLD"]
    #[inline(always)]
    pub fn datahld(&mut self) -> DatahldW<'_, Lcd1RcfgrSpec> {
        DatahldW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd1_rcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd1_rcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcd1RcfgrSpec;
impl crate::RegisterSpec for Lcd1RcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd1_rcfgr::R`](R) reader structure"]
impl crate::Readable for Lcd1RcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd1_rcfgr::W`](W) writer structure"]
impl crate::Writable for Lcd1RcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD1_RCFGR to value 0"]
impl crate::Resettable for Lcd1RcfgrSpec {}
