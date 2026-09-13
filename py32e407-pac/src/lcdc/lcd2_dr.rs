#[doc = "Register `LCD2_DR` reader"]
pub type R = crate::R<Lcd2DrSpec>;
#[doc = "Register `LCD2_DR` writer"]
pub type W = crate::W<Lcd2DrSpec>;
#[doc = "Field `D` reader - LCD2 data register"]
pub type DR = crate::FieldReader<u32>;
#[doc = "Field `D` writer - LCD2 data register"]
pub type DW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - LCD2 data register"]
    #[inline(always)]
    pub fn d(&self) -> DR {
        DR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - LCD2 data register"]
    #[inline(always)]
    pub fn d(&mut self) -> DW<'_, Lcd2DrSpec> {
        DW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd2_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd2_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcd2DrSpec;
impl crate::RegisterSpec for Lcd2DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd2_dr::R`](R) reader structure"]
impl crate::Readable for Lcd2DrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd2_dr::W`](W) writer structure"]
impl crate::Writable for Lcd2DrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD2_DR to value 0"]
impl crate::Resettable for Lcd2DrSpec {}
