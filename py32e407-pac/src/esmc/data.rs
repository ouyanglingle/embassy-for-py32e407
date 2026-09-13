#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Register `DATA` writer"]
pub type W = crate::W<DataSpec>;
#[doc = "Field `DATA0` reader - data0"]
pub type Data0R = crate::FieldReader;
#[doc = "Field `DATA0` writer - data0"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA1` reader - data1"]
pub type Data1R = crate::FieldReader;
#[doc = "Field `DATA1` writer - data1"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA2` reader - data2"]
pub type Data2R = crate::FieldReader;
#[doc = "Field `DATA2` writer - data2"]
pub type Data2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA3` reader - data3"]
pub type Data3R = crate::FieldReader;
#[doc = "Field `DATA3` writer - data3"]
pub type Data3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data0"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - data1"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - data2"]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - data3"]
    #[inline(always)]
    pub fn data3(&self) -> Data3R {
        Data3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data0"]
    #[inline(always)]
    pub fn data0(&mut self) -> Data0W<'_, DataSpec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - data1"]
    #[inline(always)]
    pub fn data1(&mut self) -> Data1W<'_, DataSpec> {
        Data1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - data2"]
    #[inline(always)]
    pub fn data2(&mut self) -> Data2W<'_, DataSpec> {
        Data2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - data3"]
    #[inline(always)]
    pub fn data3(&mut self) -> Data3W<'_, DataSpec> {
        Data3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {}
