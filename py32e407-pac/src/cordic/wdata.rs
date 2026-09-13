#[doc = "Register `WDATA` reader"]
pub type R = crate::R<WdataSpec>;
#[doc = "Register `WDATA` writer"]
pub type W = crate::W<WdataSpec>;
#[doc = "Field `ARG` reader - Function input arguments"]
pub type ArgR = crate::FieldReader<u32>;
#[doc = "Field `ARG` writer - Function input arguments"]
pub type ArgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Function input arguments"]
    #[inline(always)]
    pub fn arg(&self) -> ArgR {
        ArgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Function input arguments"]
    #[inline(always)]
    pub fn arg(&mut self) -> ArgW<'_, WdataSpec> {
        ArgW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdataSpec;
impl crate::RegisterSpec for WdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdata::R`](R) reader structure"]
impl crate::Readable for WdataSpec {}
#[doc = "`write(|w| ..)` method takes [`wdata::W`](W) writer structure"]
impl crate::Writable for WdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDATA to value 0"]
impl crate::Resettable for WdataSpec {}
