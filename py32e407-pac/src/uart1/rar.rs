#[doc = "Register `RAR` reader"]
pub type R = crate::R<RarSpec>;
#[doc = "Register `RAR` writer"]
pub type W = crate::W<RarSpec>;
#[doc = "Field `RAR` reader - Receive address matching register"]
pub type RarR = crate::FieldReader;
#[doc = "Field `RAR` writer - Receive address matching register"]
pub type RarW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive address matching register"]
    #[inline(always)]
    pub fn rar(&self) -> RarR {
        RarR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive address matching register"]
    #[inline(always)]
    pub fn rar(&mut self) -> RarW<'_, RarSpec> {
        RarW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RarSpec;
impl crate::RegisterSpec for RarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rar::R`](R) reader structure"]
impl crate::Readable for RarSpec {}
#[doc = "`write(|w| ..)` method takes [`rar::W`](W) writer structure"]
impl crate::Writable for RarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAR to value 0"]
impl crate::Resettable for RarSpec {}
