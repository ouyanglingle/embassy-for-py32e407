#[doc = "Register `XSSOCR_WE` reader"]
pub type R = crate::R<XssocrWeSpec>;
#[doc = "Register `XSSOCR_WE` writer"]
pub type W = crate::W<XssocrWeSpec>;
#[doc = "Field `SSXO` reader - xip write Slave select enable"]
pub type SsxoR = crate::FieldReader;
#[doc = "Field `SSXO` writer - xip write Slave select enable"]
pub type SsxoW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SS_CLR_RQ` reader - xip write Slave select clear"]
pub type SsClrRqR = crate::BitReader;
#[doc = "Field `SS_CLR_RQ` writer - xip write Slave select clear"]
pub type SsClrRqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - xip write Slave select enable"]
    #[inline(always)]
    pub fn ssxo(&self) -> SsxoR {
        SsxoR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - xip write Slave select clear"]
    #[inline(always)]
    pub fn ss_clr_rq(&self) -> SsClrRqR {
        SsClrRqR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - xip write Slave select enable"]
    #[inline(always)]
    pub fn ssxo(&mut self) -> SsxoW<'_, XssocrWeSpec> {
        SsxoW::new(self, 0)
    }
    #[doc = "Bit 6 - xip write Slave select clear"]
    #[inline(always)]
    pub fn ss_clr_rq(&mut self) -> SsClrRqW<'_, XssocrWeSpec> {
        SsClrRqW::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xssocr_we::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xssocr_we::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XssocrWeSpec;
impl crate::RegisterSpec for XssocrWeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xssocr_we::R`](R) reader structure"]
impl crate::Readable for XssocrWeSpec {}
#[doc = "`write(|w| ..)` method takes [`xssocr_we::W`](W) writer structure"]
impl crate::Writable for XssocrWeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XSSOCR_WE to value 0"]
impl crate::Resettable for XssocrWeSpec {}
