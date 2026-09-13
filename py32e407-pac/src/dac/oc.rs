#[doc = "Register `OC` reader"]
pub type R = crate::R<OcSpec>;
#[doc = "Register `OC` writer"]
pub type W = crate::W<OcSpec>;
#[doc = "Field `INTEREN1` reader - "]
pub type Interen1R = crate::BitReader;
#[doc = "Field `INTEREN1` writer - "]
pub type Interen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEREN2` reader - "]
pub type Interen2R = crate::BitReader;
#[doc = "Field `INTEREN2` writer - "]
pub type Interen2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn interen1(&self) -> Interen1R {
        Interen1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn interen2(&self) -> Interen2R {
        Interen2R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn interen1(&mut self) -> Interen1W<'_, OcSpec> {
        Interen1W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn interen2(&mut self) -> Interen2W<'_, OcSpec> {
        Interen2W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`oc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcSpec;
impl crate::RegisterSpec for OcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oc::R`](R) reader structure"]
impl crate::Readable for OcSpec {}
#[doc = "`write(|w| ..)` method takes [`oc::W`](W) writer structure"]
impl crate::Writable for OcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OC to value 0"]
impl crate::Resettable for OcSpec {}
