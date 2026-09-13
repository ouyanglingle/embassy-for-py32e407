#[doc = "Register `DONE_STATUS` reader"]
pub type R = crate::R<DoneStatusSpec>;
#[doc = "Register `DONE_STATUS` writer"]
pub type W = crate::W<DoneStatusSpec>;
#[doc = "Field `AES_DONE` reader - "]
pub type AesDoneR = crate::BitReader;
#[doc = "Field `AES_DONE` writer - "]
pub type AesDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAC_VAILD` reader - "]
pub type MacVaildR = crate::BitReader;
#[doc = "Field `MAC_VAILD` writer - "]
pub type MacVaildW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aes_done(&self) -> AesDoneR {
        AesDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mac_vaild(&self) -> MacVaildR {
        MacVaildR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aes_done(&mut self) -> AesDoneW<'_, DoneStatusSpec> {
        AesDoneW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mac_vaild(&mut self) -> MacVaildW<'_, DoneStatusSpec> {
        MacVaildW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`done_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`done_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoneStatusSpec;
impl crate::RegisterSpec for DoneStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`done_status::R`](R) reader structure"]
impl crate::Readable for DoneStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`done_status::W`](W) writer structure"]
impl crate::Writable for DoneStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DONE_STATUS to value 0"]
impl crate::Resettable for DoneStatusSpec {}
