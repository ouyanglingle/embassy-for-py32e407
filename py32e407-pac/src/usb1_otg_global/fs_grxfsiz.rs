#[doc = "Register `FS_GRXFSIZ` reader"]
pub type R = crate::R<FsGrxfsizSpec>;
#[doc = "Register `FS_GRXFSIZ` writer"]
pub type W = crate::W<FsGrxfsizSpec>;
#[doc = "Field `RXFD` reader - RxFIFO depth"]
pub type RxfdR = crate::FieldReader<u16>;
#[doc = "Field `RXFD` writer - RxFIFO depth"]
pub type RxfdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    pub fn rxfd(&self) -> RxfdR {
        RxfdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    pub fn rxfd(&mut self) -> RxfdW<'_, FsGrxfsizSpec> {
        RxfdW::new(self, 0)
    }
}
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_grxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_grxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsGrxfsizSpec;
impl crate::RegisterSpec for FsGrxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_grxfsiz::R`](R) reader structure"]
impl crate::Readable for FsGrxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`fs_grxfsiz::W`](W) writer structure"]
impl crate::Writable for FsGrxfsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FS_GRXFSIZ to value 0"]
impl crate::Resettable for FsGrxfsizSpec {}
