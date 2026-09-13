#[doc = "Register `LCD1_DR` reader"]
pub type R = crate::R<Lcd1DrSpec>;
#[doc = "Register `LCD1_DR` writer"]
pub type W = crate::W<Lcd1DrSpec>;
#[doc = "Field `D` reader - LCD1 data register"]
pub type DR = crate::FieldReader<u32>;
#[doc = "Field `D` writer - LCD1 data register"]
pub type DW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - LCD1 data register"]
    #[inline(always)]
    pub fn d(&self) -> DR {
        DR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - LCD1 data register"]
    #[inline(always)]
    pub fn d(&mut self) -> DW<'_, Lcd1DrSpec> {
        DW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd1_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd1_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcd1DrSpec;
impl crate::RegisterSpec for Lcd1DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd1_dr::R`](R) reader structure"]
impl crate::Readable for Lcd1DrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd1_dr::W`](W) writer structure"]
impl crate::Writable for Lcd1DrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD1_DR to value 0"]
impl crate::Resettable for Lcd1DrSpec {}
