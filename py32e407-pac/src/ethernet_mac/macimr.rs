#[doc = "Register `MACIMR` reader"]
pub type R = crate::R<MacimrSpec>;
#[doc = "Register `MACIMR` writer"]
pub type W = crate::W<MacimrSpec>;
#[doc = "Field `PMTIM` reader - PMT interrupt mask"]
pub type PmtimR = crate::BitReader;
#[doc = "Field `PMTIM` writer - PMT interrupt mask"]
pub type PmtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTIM` reader - Time stamp trigger interrupt mask"]
pub type TstimR = crate::BitReader;
#[doc = "Field `TSTIM` writer - Time stamp trigger interrupt mask"]
pub type TstimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - PMT interrupt mask"]
    #[inline(always)]
    pub fn pmtim(&self) -> PmtimR {
        PmtimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Time stamp trigger interrupt mask"]
    #[inline(always)]
    pub fn tstim(&self) -> TstimR {
        TstimR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMT interrupt mask"]
    #[inline(always)]
    pub fn pmtim(&mut self) -> PmtimW<'_, MacimrSpec> {
        PmtimW::new(self, 3)
    }
    #[doc = "Bit 9 - Time stamp trigger interrupt mask"]
    #[inline(always)]
    pub fn tstim(&mut self) -> TstimW<'_, MacimrSpec> {
        TstimW::new(self, 9)
    }
}
#[doc = "Ethernet MAC interrupt mask register (ETH_MACIMR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacimrSpec;
impl crate::RegisterSpec for MacimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macimr::R`](R) reader structure"]
impl crate::Readable for MacimrSpec {}
#[doc = "`write(|w| ..)` method takes [`macimr::W`](W) writer structure"]
impl crate::Writable for MacimrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACIMR to value 0"]
impl crate::Resettable for MacimrSpec {}
