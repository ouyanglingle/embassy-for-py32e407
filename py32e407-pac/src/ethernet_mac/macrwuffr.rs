#[doc = "Register `MACRWUFFR` reader"]
pub type R = crate::R<MacrwuffrSpec>;
#[doc = "Register `MACRWUFFR` writer"]
pub type W = crate::W<MacrwuffrSpec>;
#[doc = "Field `D` reader - D"]
pub type DR = crate::FieldReader<u32>;
#[doc = "Field `D` writer - D"]
pub type DW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - D"]
    #[inline(always)]
    pub fn d(&self) -> DR {
        DR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - D"]
    #[inline(always)]
    pub fn d(&mut self) -> DW<'_, MacrwuffrSpec> {
        DW::new(self, 0)
    }
}
#[doc = "ETH remote wakeup frame filter register\n\nYou can [`read`](crate::Reg::read) this register and get [`macrwuffr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrwuffr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacrwuffrSpec;
impl crate::RegisterSpec for MacrwuffrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macrwuffr::R`](R) reader structure"]
impl crate::Readable for MacrwuffrSpec {}
#[doc = "`write(|w| ..)` method takes [`macrwuffr::W`](W) writer structure"]
impl crate::Writable for MacrwuffrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACRWUFFR to value 0"]
impl crate::Resettable for MacrwuffrSpec {}
