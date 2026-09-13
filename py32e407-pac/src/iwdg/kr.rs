#[doc = "Register `KR` reader"]
pub type R = crate::R<KrSpec>;
#[doc = "Register `KR` writer"]
pub type W = crate::W<KrSpec>;
#[doc = "Field `KEY` reader - IWDG KEY"]
pub type KeyR = crate::FieldReader<u16>;
#[doc = "Field `KEY` writer - IWDG KEY"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IWDG KEY"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IWDG KEY"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, KrSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`kr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KrSpec;
impl crate::RegisterSpec for KrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`kr::R`](R) reader structure"]
impl crate::Readable for KrSpec {}
#[doc = "`write(|w| ..)` method takes [`kr::W`](W) writer structure"]
impl crate::Writable for KrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KR to value 0"]
impl crate::Resettable for KrSpec {}
