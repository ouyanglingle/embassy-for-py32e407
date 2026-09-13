#[doc = "Register `MACVLANTR` reader"]
pub type R = crate::R<MacvlantrSpec>;
#[doc = "Register `MACVLANTR` writer"]
pub type W = crate::W<MacvlantrSpec>;
#[doc = "Field `VLANTI` reader - VLAN tag identifier (for receive frames)"]
pub type VlantiR = crate::FieldReader<u16>;
#[doc = "Field `VLANTI` writer - VLAN tag identifier (for receive frames)"]
pub type VlantiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VLANTC` reader - 12-bit VLAN tag comparison"]
pub type VlantcR = crate::BitReader;
#[doc = "Field `VLANTC` writer - 12-bit VLAN tag comparison"]
pub type VlantcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vlanti(&self) -> VlantiR {
        VlantiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vlantc(&self) -> VlantcR {
        VlantcR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vlanti(&mut self) -> VlantiW<'_, MacvlantrSpec> {
        VlantiW::new(self, 0)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vlantc(&mut self) -> VlantcW<'_, MacvlantrSpec> {
        VlantcW::new(self, 16)
    }
}
#[doc = "Ethernet MAC VLAN tag register (ETH_MACVLANTR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macvlantr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvlantr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacvlantrSpec;
impl crate::RegisterSpec for MacvlantrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvlantr::R`](R) reader structure"]
impl crate::Readable for MacvlantrSpec {}
#[doc = "`write(|w| ..)` method takes [`macvlantr::W`](W) writer structure"]
impl crate::Writable for MacvlantrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACVLANTR to value 0"]
impl crate::Resettable for MacvlantrSpec {}
