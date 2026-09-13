#[doc = "Register `MSG_AAD_BYTES` reader"]
pub type R = crate::R<MsgAadBytesSpec>;
#[doc = "Register `MSG_AAD_BYTES` writer"]
pub type W = crate::W<MsgAadBytesSpec>;
#[doc = "Field `MSG_LEN` reader - "]
pub type MsgLenR = crate::FieldReader<u16>;
#[doc = "Field `MSG_LEN` writer - "]
pub type MsgLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADD_LEN` reader - "]
pub type AddLenR = crate::FieldReader<u16>;
#[doc = "Field `ADD_LEN` writer - "]
pub type AddLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn msg_len(&self) -> MsgLenR {
        MsgLenR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn add_len(&self) -> AddLenR {
        AddLenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn msg_len(&mut self) -> MsgLenW<'_, MsgAadBytesSpec> {
        MsgLenW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn add_len(&mut self) -> AddLenW<'_, MsgAadBytesSpec> {
        AddLenW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`msg_aad_bytes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msg_aad_bytes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsgAadBytesSpec;
impl crate::RegisterSpec for MsgAadBytesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msg_aad_bytes::R`](R) reader structure"]
impl crate::Readable for MsgAadBytesSpec {}
#[doc = "`write(|w| ..)` method takes [`msg_aad_bytes::W`](W) writer structure"]
impl crate::Writable for MsgAadBytesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSG_AAD_BYTES to value 0"]
impl crate::Resettable for MsgAadBytesSpec {}
