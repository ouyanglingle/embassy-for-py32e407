#[doc = "Register `FR` reader"]
pub type R = crate::R<FrSpec>;
#[doc = "Register `FR` writer"]
pub type W = crate::W<FrSpec>;
#[doc = "Field `FLTEN1` reader - Comparator 1 digital filter configuration"]
pub type Flten1R = crate::BitReader;
#[doc = "Field `FLTEN1` writer - Comparator 1 digital filter configuration"]
pub type Flten1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTCNT1` reader - Comparator 1 sampling filter counter"]
pub type Fltcnt1R = crate::FieldReader<u16>;
#[doc = "Field `FLTCNT1` writer - Comparator 1 sampling filter counter"]
pub type Fltcnt1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Comparator 1 digital filter configuration"]
    #[inline(always)]
    pub fn flten1(&self) -> Flten1R {
        Flten1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - Comparator 1 sampling filter counter"]
    #[inline(always)]
    pub fn fltcnt1(&self) -> Fltcnt1R {
        Fltcnt1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 digital filter configuration"]
    #[inline(always)]
    pub fn flten1(&mut self) -> Flten1W<'_, FrSpec> {
        Flten1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Comparator 1 sampling filter counter"]
    #[inline(always)]
    pub fn fltcnt1(&mut self) -> Fltcnt1W<'_, FrSpec> {
        Fltcnt1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`fr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrSpec;
impl crate::RegisterSpec for FrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fr::R`](R) reader structure"]
impl crate::Readable for FrSpec {}
#[doc = "`write(|w| ..)` method takes [`fr::W`](W) writer structure"]
impl crate::Writable for FrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FrSpec {}
