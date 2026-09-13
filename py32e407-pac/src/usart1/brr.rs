#[doc = "Register `BRR` reader"]
pub type R = crate::R<BrrSpec>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `DIV_F` reader - DIV_Fraction"]
pub type DivFR = crate::FieldReader;
#[doc = "Field `DIV_F` writer - DIV_Fraction"]
pub type DivFW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIV_M` reader - DIV_Mantissa"]
pub type DivMR = crate::FieldReader<u16>;
#[doc = "Field `DIV_M` writer - DIV_Mantissa"]
pub type DivMW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - DIV_Fraction"]
    #[inline(always)]
    pub fn div_f(&self) -> DivFR {
        DivFR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - DIV_Mantissa"]
    #[inline(always)]
    pub fn div_m(&self) -> DivMR {
        DivMR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - DIV_Fraction"]
    #[inline(always)]
    pub fn div_f(&mut self) -> DivFW<'_, BrrSpec> {
        DivFW::new(self, 0)
    }
    #[doc = "Bits 4:15 - DIV_Mantissa"]
    #[inline(always)]
    pub fn div_m(&mut self) -> DivMW<'_, BrrSpec> {
        DivMW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BrrSpec {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BrrSpec {}
