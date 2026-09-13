#[doc = "Register `LCD3_DR` reader"]
pub type R = crate::R<Lcd3DrSpec>;
#[doc = "Register `LCD3_DR` writer"]
pub type W = crate::W<Lcd3DrSpec>;
#[doc = "Field `D` reader - LCD3 data register"]
pub type DR = crate::FieldReader<u32>;
#[doc = "Field `D` writer - LCD3 data register"]
pub type DW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - LCD3 data register"]
    #[inline(always)]
    pub fn d(&self) -> DR {
        DR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - LCD3 data register"]
    #[inline(always)]
    pub fn d(&mut self) -> DW<'_, Lcd3DrSpec> {
        DW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd3_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd3_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcd3DrSpec;
impl crate::RegisterSpec for Lcd3DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd3_dr::R`](R) reader structure"]
impl crate::Readable for Lcd3DrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd3_dr::W`](W) writer structure"]
impl crate::Writable for Lcd3DrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD3_DR to value 0"]
impl crate::Resettable for Lcd3DrSpec {}
