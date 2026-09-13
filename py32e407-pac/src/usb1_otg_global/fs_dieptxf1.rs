#[doc = "Register `FS_DIEPTXF1` reader"]
pub type R = crate::R<FsDieptxf1Spec>;
#[doc = "Register `FS_DIEPTXF1` writer"]
pub type W = crate::W<FsDieptxf1Spec>;
#[doc = "Field `INEPTXSA` reader - IN endpoint FIFO2 transmit RAM start address"]
pub type IneptxsaR = crate::FieldReader<u16>;
#[doc = "Field `INEPTXSA` writer - IN endpoint FIFO2 transmit RAM start address"]
pub type IneptxsaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPTXFD` reader - IN endpoint TxFIFO depth"]
pub type IneptxfdR = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFD` writer - IN endpoint TxFIFO depth"]
pub type IneptxfdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFO2 transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxsa(&self) -> IneptxsaR {
        IneptxsaR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(&self) -> IneptxfdR {
        IneptxfdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint FIFO2 transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxsa(&mut self) -> IneptxsaW<'_, FsDieptxf1Spec> {
        IneptxsaW::new(self, 0)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(&mut self) -> IneptxfdW<'_, FsDieptxf1Spec> {
        IneptxfdW::new(self, 16)
    }
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_dieptxf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_dieptxf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsDieptxf1Spec;
impl crate::RegisterSpec for FsDieptxf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_dieptxf1::R`](R) reader structure"]
impl crate::Readable for FsDieptxf1Spec {}
#[doc = "`write(|w| ..)` method takes [`fs_dieptxf1::W`](W) writer structure"]
impl crate::Writable for FsDieptxf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FS_DIEPTXF1 to value 0"]
impl crate::Resettable for FsDieptxf1Spec {}
