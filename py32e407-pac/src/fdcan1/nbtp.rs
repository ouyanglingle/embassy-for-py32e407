#[doc = "Register `NBTP` reader"]
pub type R = crate::R<NbtpSpec>;
#[doc = "Register `NBTP` writer"]
pub type W = crate::W<NbtpSpec>;
#[doc = "Field `NTSEG2` reader - Nominal Time segment after sample point"]
pub type Ntseg2R = crate::FieldReader;
#[doc = "Field `NTSEG1` reader - Nominal Time segment before sample point"]
pub type Ntseg1R = crate::FieldReader;
#[doc = "Field `NBRP` reader - Nominal Bit Rate Prescaler"]
pub type NbrpR = crate::FieldReader<u16>;
#[doc = "Field `NSJW` reader - Nominal (Re)Synchronization Jump Width"]
pub type NsjwR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Nominal Time segment after sample point"]
    #[inline(always)]
    pub fn ntseg2(&self) -> Ntseg2R {
        Ntseg2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point"]
    #[inline(always)]
    pub fn ntseg1(&self) -> Ntseg1R {
        Ntseg1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - Nominal Bit Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(&self) -> NbrpR {
        NbrpR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - Nominal (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(&self) -> NsjwR {
        NsjwR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`nbtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nbtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NbtpSpec;
impl crate::RegisterSpec for NbtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nbtp::R`](R) reader structure"]
impl crate::Readable for NbtpSpec {}
#[doc = "`write(|w| ..)` method takes [`nbtp::W`](W) writer structure"]
impl crate::Writable for NbtpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NBTP to value 0"]
impl crate::Resettable for NbtpSpec {}
