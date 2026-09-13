#[doc = "Register `SR2` reader"]
pub type R = crate::R<Sr2Spec>;
#[doc = "Register `SR2` writer"]
pub type W = crate::W<Sr2Spec>;
#[doc = "Field `MSL` reader - desc MSL"]
pub type MslR = crate::BitReader;
#[doc = "Field `BUSY` reader - desc BUSY"]
pub type BusyR = crate::BitReader;
#[doc = "Field `TRA` reader - desc TRA"]
pub type TraR = crate::BitReader;
#[doc = "Field `GENCALL` reader - desc GENCALL"]
pub type GencallR = crate::BitReader;
#[doc = "Field `SMBDEFAULT` reader - desc SMBDEFAULT"]
pub type SmbdefaultR = crate::BitReader;
#[doc = "Field `SMBHOST` reader - desc SMBHOST"]
pub type SmbhostR = crate::BitReader;
#[doc = "Field `DUALF` reader - desc DUALF"]
pub type DualfR = crate::BitReader;
#[doc = "Field `PEC` reader - desc PEC"]
pub type PecR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - desc MSL"]
    #[inline(always)]
    pub fn msl(&self) -> MslR {
        MslR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TRA"]
    #[inline(always)]
    pub fn tra(&self) -> TraR {
        TraR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc GENCALL"]
    #[inline(always)]
    pub fn gencall(&self) -> GencallR {
        GencallR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc SMBDEFAULT"]
    #[inline(always)]
    pub fn smbdefault(&self) -> SmbdefaultR {
        SmbdefaultR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SMBHOST"]
    #[inline(always)]
    pub fn smbhost(&self) -> SmbhostR {
        SmbhostR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DUALF"]
    #[inline(always)]
    pub fn dualf(&self) -> DualfR {
        DualfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - desc PEC"]
    #[inline(always)]
    pub fn pec(&self) -> PecR {
        PecR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {}
#[doc = "desc SR2\n\nYou can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr2Spec;
impl crate::RegisterSpec for Sr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for Sr2Spec {}
#[doc = "`write(|w| ..)` method takes [`sr2::W`](W) writer structure"]
impl crate::Writable for Sr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for Sr2Spec {}
