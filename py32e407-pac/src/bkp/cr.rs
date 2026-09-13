#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `TPE` reader - TAMP enable"]
pub type TpeR = crate::BitReader;
#[doc = "Field `TPE` writer - TAMP enable"]
pub type TpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPAL` reader - TAMPER pin interrupt enable"]
pub type TpalR = crate::BitReader;
#[doc = "Field `TPAL` writer - TAMPER pin interrupt enable"]
pub type TpalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TAMP enable"]
    #[inline(always)]
    pub fn tpe(&self) -> TpeR {
        TpeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMPER pin interrupt enable"]
    #[inline(always)]
    pub fn tpal(&self) -> TpalR {
        TpalR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP enable"]
    #[inline(always)]
    pub fn tpe(&mut self) -> TpeW<'_, CrSpec> {
        TpeW::new(self, 0)
    }
    #[doc = "Bit 1 - TAMPER pin interrupt enable"]
    #[inline(always)]
    pub fn tpal(&mut self) -> TpalW<'_, CrSpec> {
        TpalW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
