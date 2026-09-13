#[doc = "Register `RXTOUT` reader"]
pub type R = crate::R<RxtoutSpec>;
#[doc = "Register `RXTOUT` writer"]
pub type W = crate::W<RxtoutSpec>;
#[doc = "Field `RXTOUT` reader - rx timeout"]
pub type RxtoutR = crate::FieldReader;
#[doc = "Field `RXTOUT` writer - rx timeout"]
pub type RxtoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - rx timeout"]
    #[inline(always)]
    pub fn rxtout(&self) -> RxtoutR {
        RxtoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - rx timeout"]
    #[inline(always)]
    pub fn rxtout(&mut self) -> RxtoutW<'_, RxtoutSpec> {
        RxtoutW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxtoutSpec;
impl crate::RegisterSpec for RxtoutSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxtout::R`](R) reader structure"]
impl crate::Readable for RxtoutSpec {}
#[doc = "`write(|w| ..)` method takes [`rxtout::W`](W) writer structure"]
impl crate::Writable for RxtoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXTOUT to value 0"]
impl crate::Resettable for RxtoutSpec {}
