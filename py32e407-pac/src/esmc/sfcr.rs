#[doc = "Register `SFCR` reader"]
pub type R = crate::R<SfcrSpec>;
#[doc = "Register `SFCR` writer"]
pub type W = crate::W<SfcrSpec>;
#[doc = "Field `SFCR0` reader - Command Only"]
pub type Sfcr0R = crate::FieldReader;
#[doc = "Field `SFCR0` writer - Command Only"]
pub type Sfcr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Command Only"]
    #[inline(always)]
    pub fn sfcr0(&self) -> Sfcr0R {
        Sfcr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command Only"]
    #[inline(always)]
    pub fn sfcr0(&mut self) -> Sfcr0W<'_, SfcrSpec> {
        Sfcr0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfcrSpec;
impl crate::RegisterSpec for SfcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sfcr::R`](R) reader structure"]
impl crate::Readable for SfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfcr::W`](W) writer structure"]
impl crate::Writable for SfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFCR to value 0"]
impl crate::Resettable for SfcrSpec {}
