#[doc = "Register `FS_DAINT` reader"]
pub type R = crate::R<FsDaintSpec>;
#[doc = "Register `FS_DAINT` writer"]
pub type W = crate::W<FsDaintSpec>;
#[doc = "Field `IEPINT` reader - IN endpoint interrupt bits"]
pub type IepintR = crate::FieldReader<u16>;
#[doc = "Field `OEPINT` reader - OUT endpoint interrupt bits"]
pub type OepintR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint interrupt bits"]
    #[inline(always)]
    pub fn iepint(&self) -> IepintR {
        IepintR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepint(&self) -> OepintR {
        OepintR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_daint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_daint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsDaintSpec;
impl crate::RegisterSpec for FsDaintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_daint::R`](R) reader structure"]
impl crate::Readable for FsDaintSpec {}
#[doc = "`write(|w| ..)` method takes [`fs_daint::W`](W) writer structure"]
impl crate::Writable for FsDaintSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FS_DAINT to value 0"]
impl crate::Resettable for FsDaintSpec {}
