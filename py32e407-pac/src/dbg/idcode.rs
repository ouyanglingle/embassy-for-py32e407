#[doc = "Register `IDCODE` reader"]
pub type R = crate::R<IdcodeSpec>;
#[doc = "Register `IDCODE` writer"]
pub type W = crate::W<IdcodeSpec>;
#[doc = "Field `REV_ID` reader - REV_ID"]
pub type RevIdR = crate::FieldReader<u32>;
#[doc = "Field `REV_ID` writer - REV_ID"]
pub type RevIdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REV_ID"]
    #[inline(always)]
    pub fn rev_id(&self) -> RevIdR {
        RevIdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REV_ID"]
    #[inline(always)]
    pub fn rev_id(&mut self) -> RevIdW<'_, IdcodeSpec> {
        RevIdW::new(self, 0)
    }
}
#[doc = "DBGMCU IDCODE\n\nYou can [`read`](crate::Reg::read) this register and get [`idcode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idcode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdcodeSpec;
impl crate::RegisterSpec for IdcodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idcode::R`](R) reader structure"]
impl crate::Readable for IdcodeSpec {}
#[doc = "`write(|w| ..)` method takes [`idcode::W`](W) writer structure"]
impl crate::Writable for IdcodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDCODE to value 0"]
impl crate::Resettable for IdcodeSpec {}
