#[doc = "Register `FS_DAINTMSK` reader"]
pub type R = crate::R<FsDaintmskSpec>;
#[doc = "Register `FS_DAINTMSK` writer"]
pub type W = crate::W<FsDaintmskSpec>;
#[doc = "Field `IEPM` reader - IN EP interrupt mask bits"]
pub type IepmR = crate::FieldReader<u16>;
#[doc = "Field `IEPM` writer - IN EP interrupt mask bits"]
pub type IepmW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OEPINT` reader - OUT endpoint interrupt bits"]
pub type OepintR = crate::FieldReader<u16>;
#[doc = "Field `OEPINT` writer - OUT endpoint interrupt bits"]
pub type OepintW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    pub fn iepm(&self) -> IepmR {
        IepmR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepint(&self) -> OepintR {
        OepintR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    pub fn iepm(&mut self) -> IepmW<'_, FsDaintmskSpec> {
        IepmW::new(self, 0)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepint(&mut self) -> OepintW<'_, FsDaintmskSpec> {
        OepintW::new(self, 16)
    }
}
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_daintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_daintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsDaintmskSpec;
impl crate::RegisterSpec for FsDaintmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_daintmsk::R`](R) reader structure"]
impl crate::Readable for FsDaintmskSpec {}
#[doc = "`write(|w| ..)` method takes [`fs_daintmsk::W`](W) writer structure"]
impl crate::Writable for FsDaintmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FS_DAINTMSK to value 0"]
impl crate::Resettable for FsDaintmskSpec {}
