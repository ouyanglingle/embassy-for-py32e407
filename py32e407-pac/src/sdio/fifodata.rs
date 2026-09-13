#[doc = "Register `FIFODATA` reader"]
pub type R = crate::R<FifodataSpec>;
#[doc = "Register `FIFODATA` writer"]
pub type W = crate::W<FifodataSpec>;
#[doc = "Field `FIFODATA` reader - FIFODATA"]
pub type FifodataR = crate::FieldReader<u32>;
#[doc = "Field `FIFODATA` writer - FIFODATA"]
pub type FifodataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFODATA"]
    #[inline(always)]
    pub fn fifodata(&self) -> FifodataR {
        FifodataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFODATA"]
    #[inline(always)]
    pub fn fifodata(&mut self) -> FifodataW<'_, FifodataSpec> {
        FifodataW::new(self, 0)
    }
}
#[doc = "SDIO FIFO data register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifodata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifodata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifodataSpec;
impl crate::RegisterSpec for FifodataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifodata::R`](R) reader structure"]
impl crate::Readable for FifodataSpec {}
#[doc = "`write(|w| ..)` method takes [`fifodata::W`](W) writer structure"]
impl crate::Writable for FifodataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFODATA to value 0"]
impl crate::Resettable for FifodataSpec {}
