#[doc = "Register `LCD0_WCFGR` reader"]
pub type R = crate::R<Lcd0WcfgrSpec>;
#[doc = "Register `LCD0_WCFGR` writer"]
pub type W = crate::W<Lcd0WcfgrSpec>;
#[doc = "Field `ADDSET` reader - LCD0 write ADDSET"]
pub type AddsetR = crate::FieldReader;
#[doc = "Field `ADDSET` writer - LCD0 write ADDSET"]
pub type AddsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAST` reader - LCD0 write DATAST"]
pub type DatastR = crate::FieldReader;
#[doc = "Field `DATAST` writer - LCD0 write DATAST"]
pub type DatastW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATAHLD` reader - LCD0 write DATAHLD"]
pub type DatahldR = crate::FieldReader;
#[doc = "Field `DATAHLD` writer - LCD0 write DATAHLD"]
pub type DatahldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - LCD0 write ADDSET"]
    #[inline(always)]
    pub fn addset(&self) -> AddsetR {
        AddsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - LCD0 write DATAST"]
    #[inline(always)]
    pub fn datast(&self) -> DatastR {
        DatastR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 30:31 - LCD0 write DATAHLD"]
    #[inline(always)]
    pub fn datahld(&self) -> DatahldR {
        DatahldR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - LCD0 write ADDSET"]
    #[inline(always)]
    pub fn addset(&mut self) -> AddsetW<'_, Lcd0WcfgrSpec> {
        AddsetW::new(self, 0)
    }
    #[doc = "Bits 8:15 - LCD0 write DATAST"]
    #[inline(always)]
    pub fn datast(&mut self) -> DatastW<'_, Lcd0WcfgrSpec> {
        DatastW::new(self, 8)
    }
    #[doc = "Bits 30:31 - LCD0 write DATAHLD"]
    #[inline(always)]
    pub fn datahld(&mut self) -> DatahldW<'_, Lcd0WcfgrSpec> {
        DatahldW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd0_wcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd0_wcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcd0WcfgrSpec;
impl crate::RegisterSpec for Lcd0WcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd0_wcfgr::R`](R) reader structure"]
impl crate::Readable for Lcd0WcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd0_wcfgr::W`](W) writer structure"]
impl crate::Writable for Lcd0WcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD0_WCFGR to value 0"]
impl crate::Resettable for Lcd0WcfgrSpec {}
