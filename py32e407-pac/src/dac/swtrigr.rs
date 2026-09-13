#[doc = "Register `SWTRIGR` reader"]
pub type R = crate::R<SwtrigrSpec>;
#[doc = "Register `SWTRIGR` writer"]
pub type W = crate::W<SwtrigrSpec>;
#[doc = "Field `SWTRIG1` reader - "]
pub type Swtrig1R = crate::BitReader;
#[doc = "Field `SWTRIG1` writer - "]
pub type Swtrig1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG2` reader - "]
pub type Swtrig2R = crate::BitReader;
#[doc = "Field `SWTRIG2` writer - "]
pub type Swtrig2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swtrig1(&self) -> Swtrig1R {
        Swtrig1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swtrig2(&self) -> Swtrig2R {
        Swtrig2R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swtrig1(&mut self) -> Swtrig1W<'_, SwtrigrSpec> {
        Swtrig1W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swtrig2(&mut self) -> Swtrig2W<'_, SwtrigrSpec> {
        Swtrig2W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`swtrigr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtrigrSpec;
impl crate::RegisterSpec for SwtrigrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swtrigr::R`](R) reader structure"]
impl crate::Readable for SwtrigrSpec {}
#[doc = "`write(|w| ..)` method takes [`swtrigr::W`](W) writer structure"]
impl crate::Writable for SwtrigrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWTRIGR to value 0"]
impl crate::Resettable for SwtrigrSpec {}
