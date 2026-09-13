#[doc = "Register `BCR` reader"]
pub type R = crate::R<BcrSpec>;
#[doc = "Register `BCR` writer"]
pub type W = crate::W<BcrSpec>;
#[doc = "Field `BCR` reader - bit control register"]
pub type BcrR = crate::FieldReader<u32>;
#[doc = "Field `BCR` writer - bit control register"]
pub type BcrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bit control register"]
    #[inline(always)]
    pub fn bcr(&self) -> BcrR {
        BcrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bit control register"]
    #[inline(always)]
    pub fn bcr(&mut self) -> BcrW<'_, BcrSpec> {
        BcrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`bcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcrSpec;
impl crate::RegisterSpec for BcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcr::R`](R) reader structure"]
impl crate::Readable for BcrSpec {}
#[doc = "`write(|w| ..)` method takes [`bcr::W`](W) writer structure"]
impl crate::Writable for BcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BcrSpec {}
