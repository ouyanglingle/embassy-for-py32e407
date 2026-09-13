#[doc = "Register `FS_GRXSTSR_DEVICE` reader"]
pub type R = crate::R<FsGrxstsrDeviceSpec>;
#[doc = "Register `FS_GRXSTSR_DEVICE` writer"]
pub type W = crate::W<FsGrxstsrDeviceSpec>;
#[doc = "Field `EPNUM` reader - Endpoint number"]
pub type EpnumR = crate::FieldReader;
#[doc = "Field `BCNT` reader - Byte count"]
pub type BcntR = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DpidR = crate::FieldReader;
#[doc = "Field `PKTSTS` reader - Packet status"]
pub type PktstsR = crate::FieldReader;
#[doc = "Field `FRMNUM` reader - Frame number"]
pub type FrmnumR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&self) -> EpnumR {
        EpnumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BcntR {
        BcntR::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Packet status"]
    #[inline(always)]
    pub fn pktsts(&self) -> PktstsR {
        PktstsR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24 - Frame number"]
    #[inline(always)]
    pub fn frmnum(&self) -> FrmnumR {
        FrmnumR::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_grxstsr_device::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_grxstsr_device::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsGrxstsrDeviceSpec;
impl crate::RegisterSpec for FsGrxstsrDeviceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_grxstsr_device::R`](R) reader structure"]
impl crate::Readable for FsGrxstsrDeviceSpec {}
#[doc = "`write(|w| ..)` method takes [`fs_grxstsr_device::W`](W) writer structure"]
impl crate::Writable for FsGrxstsrDeviceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FS_GRXSTSR_DEVICE to value 0"]
impl crate::Resettable for FsGrxstsrDeviceSpec {}
