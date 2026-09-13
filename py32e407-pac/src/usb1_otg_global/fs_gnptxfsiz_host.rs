#[doc = "Register `FS_GNPTXFSIZ_HOST` reader"]
pub type R = crate::R<FsGnptxfsizHostSpec>;
#[doc = "Register `FS_GNPTXFSIZ_HOST` writer"]
pub type W = crate::W<FsGnptxfsizHostSpec>;
#[doc = "Field `NPTXFSA` reader - Non-periodic transmit RAM start address"]
pub type NptxfsaR = crate::FieldReader<u16>;
#[doc = "Field `NPTXFSA` writer - Non-periodic transmit RAM start address"]
pub type NptxfsaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NPTXFD` reader - Non-periodic TxFIFO depth"]
pub type NptxfdR = crate::FieldReader<u16>;
#[doc = "Field `NPTXFD` writer - Non-periodic TxFIFO depth"]
pub type NptxfdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Non-periodic transmit RAM start address"]
    #[inline(always)]
    pub fn nptxfsa(&self) -> NptxfsaR {
        NptxfsaR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO depth"]
    #[inline(always)]
    pub fn nptxfd(&self) -> NptxfdR {
        NptxfdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Non-periodic transmit RAM start address"]
    #[inline(always)]
    pub fn nptxfsa(&mut self) -> NptxfsaW<'_, FsGnptxfsizHostSpec> {
        NptxfsaW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO depth"]
    #[inline(always)]
    pub fn nptxfd(&mut self) -> NptxfdW<'_, FsGnptxfsizHostSpec> {
        NptxfdW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_gnptxfsiz_host::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gnptxfsiz_host::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsGnptxfsizHostSpec;
impl crate::RegisterSpec for FsGnptxfsizHostSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_gnptxfsiz_host::R`](R) reader structure"]
impl crate::Readable for FsGnptxfsizHostSpec {}
#[doc = "`write(|w| ..)` method takes [`fs_gnptxfsiz_host::W`](W) writer structure"]
impl crate::Writable for FsGnptxfsizHostSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FS_GNPTXFSIZ_HOST to value 0"]
impl crate::Resettable for FsGnptxfsizHostSpec {}
