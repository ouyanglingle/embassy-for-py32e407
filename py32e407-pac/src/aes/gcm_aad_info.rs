#[doc = "Register `GCM_AAD_INFO` reader"]
pub type R = crate::R<GcmAadInfoSpec>;
#[doc = "Register `GCM_AAD_INFO` writer"]
pub type W = crate::W<GcmAadInfoSpec>;
#[doc = "Field `ADD_LEN_TOT` reader - "]
pub type AddLenTotR = crate::FieldReader<u32>;
#[doc = "Field `ADD_LEN_TOT` writer - "]
pub type AddLenTotW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn add_len_tot(&self) -> AddLenTotR {
        AddLenTotR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn add_len_tot(&mut self) -> AddLenTotW<'_, GcmAadInfoSpec> {
        AddLenTotW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gcm_aad_info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcm_aad_info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcmAadInfoSpec;
impl crate::RegisterSpec for GcmAadInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcm_aad_info::R`](R) reader structure"]
impl crate::Readable for GcmAadInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`gcm_aad_info::W`](W) writer structure"]
impl crate::Writable for GcmAadInfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCM_AAD_INFO to value 0"]
impl crate::Resettable for GcmAadInfoSpec {}
