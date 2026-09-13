#[doc = "Register `MSG_TOTAL_BYTES` reader"]
pub type R = crate::R<MsgTotalBytesSpec>;
#[doc = "Register `MSG_TOTAL_BYTES` writer"]
pub type W = crate::W<MsgTotalBytesSpec>;
#[doc = "Field `TOTAL_BYTES` reader - "]
pub type TotalBytesR = crate::FieldReader<u32>;
#[doc = "Field `TOTAL_BYTES` writer - "]
pub type TotalBytesW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn total_bytes(&self) -> TotalBytesR {
        TotalBytesR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn total_bytes(&mut self) -> TotalBytesW<'_, MsgTotalBytesSpec> {
        TotalBytesW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`msg_total_bytes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msg_total_bytes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsgTotalBytesSpec;
impl crate::RegisterSpec for MsgTotalBytesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msg_total_bytes::R`](R) reader structure"]
impl crate::Readable for MsgTotalBytesSpec {}
#[doc = "`write(|w| ..)` method takes [`msg_total_bytes::W`](W) writer structure"]
impl crate::Writable for MsgTotalBytesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSG_TOTAL_BYTES to value 0"]
impl crate::Resettable for MsgTotalBytesSpec {}
