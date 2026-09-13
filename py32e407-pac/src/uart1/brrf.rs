#[doc = "Register `BRRF` reader"]
pub type R = crate::R<BrrfSpec>;
#[doc = "Register `BRRF` writer"]
pub type W = crate::W<BrrfSpec>;
#[doc = "Field `BRRF` reader - Baud rate fractional register"]
pub type BrrfR = crate::FieldReader;
#[doc = "Field `BRRF` writer - Baud rate fractional register"]
pub type BrrfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Baud rate fractional register"]
    #[inline(always)]
    pub fn brrf(&self) -> BrrfR {
        BrrfR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Baud rate fractional register"]
    #[inline(always)]
    pub fn brrf(&mut self) -> BrrfW<'_, BrrfSpec> {
        BrrfW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`brrf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brrf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrfSpec;
impl crate::RegisterSpec for BrrfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brrf::R`](R) reader structure"]
impl crate::Readable for BrrfSpec {}
#[doc = "`write(|w| ..)` method takes [`brrf::W`](W) writer structure"]
impl crate::Writable for BrrfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRRF to value 0"]
impl crate::Resettable for BrrfSpec {}
