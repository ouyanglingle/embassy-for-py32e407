#[doc = "Register `LCD3_RCFGR` reader"]
pub type R = crate::R<Lcd3RcfgrSpec>;
#[doc = "Register `LCD3_RCFGR` writer"]
pub type W = crate::W<Lcd3RcfgrSpec>;
#[doc = "Field `ADDSET` reader - LCD3 read ADDSET"]
pub type AddsetR = crate::FieldReader;
#[doc = "Field `ADDSET` writer - LCD3 read ADDSET"]
pub type AddsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAST` reader - LCD3 read DATAST"]
pub type DatastR = crate::FieldReader;
#[doc = "Field `DATAST` writer - LCD3 read DATAST"]
pub type DatastW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATAHLD` reader - LCD3 read DATAHLD"]
pub type DatahldR = crate::FieldReader;
#[doc = "Field `DATAHLD` writer - LCD3 read DATAHLD"]
pub type DatahldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - LCD3 read ADDSET"]
    #[inline(always)]
    pub fn addset(&self) -> AddsetR {
        AddsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - LCD3 read DATAST"]
    #[inline(always)]
    pub fn datast(&self) -> DatastR {
        DatastR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 30:31 - LCD3 read DATAHLD"]
    #[inline(always)]
    pub fn datahld(&self) -> DatahldR {
        DatahldR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - LCD3 read ADDSET"]
    #[inline(always)]
    pub fn addset(&mut self) -> AddsetW<'_, Lcd3RcfgrSpec> {
        AddsetW::new(self, 0)
    }
    #[doc = "Bits 8:15 - LCD3 read DATAST"]
    #[inline(always)]
    pub fn datast(&mut self) -> DatastW<'_, Lcd3RcfgrSpec> {
        DatastW::new(self, 8)
    }
    #[doc = "Bits 30:31 - LCD3 read DATAHLD"]
    #[inline(always)]
    pub fn datahld(&mut self) -> DatahldW<'_, Lcd3RcfgrSpec> {
        DatahldW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd3_rcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd3_rcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcd3RcfgrSpec;
impl crate::RegisterSpec for Lcd3RcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd3_rcfgr::R`](R) reader structure"]
impl crate::Readable for Lcd3RcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd3_rcfgr::W`](W) writer structure"]
impl crate::Writable for Lcd3RcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD3_RCFGR to value 0"]
impl crate::Resettable for Lcd3RcfgrSpec {}
