#[doc = "Register `DLEN` reader"]
pub type R = crate::R<DlenSpec>;
#[doc = "Register `DLEN` writer"]
pub type W = crate::W<DlenSpec>;
#[doc = "Field `DATALENGTH` reader - desc DATALENGTH"]
pub type DatalengthR = crate::FieldReader<u32>;
#[doc = "Field `DATALENGTH` writer - desc DATALENGTH"]
pub type DatalengthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc DATALENGTH"]
    #[inline(always)]
    pub fn datalength(&self) -> DatalengthR {
        DatalengthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc DATALENGTH"]
    #[inline(always)]
    pub fn datalength(&mut self) -> DatalengthW<'_, DlenSpec> {
        DatalengthW::new(self, 0)
    }
}
#[doc = "desc DLEN\n\nYou can [`read`](crate::Reg::read) this register and get [`dlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlenSpec;
impl crate::RegisterSpec for DlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlen::R`](R) reader structure"]
impl crate::Readable for DlenSpec {}
#[doc = "`write(|w| ..)` method takes [`dlen::W`](W) writer structure"]
impl crate::Writable for DlenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DLEN to value 0"]
impl crate::Resettable for DlenSpec {}
