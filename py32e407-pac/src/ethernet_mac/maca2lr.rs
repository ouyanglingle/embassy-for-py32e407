#[doc = "Register `MACA2LR` reader"]
pub type R = crate::R<Maca2lrSpec>;
#[doc = "Register `MACA2LR` writer"]
pub type W = crate::W<Maca2lrSpec>;
#[doc = "Field `MACA2L` reader - MAC address2 low"]
pub type Maca2lR = crate::FieldReader<u32>;
#[doc = "Field `MACA2L` writer - MAC address2 low"]
pub type Maca2lW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - MAC address2 low"]
    #[inline(always)]
    pub fn maca2l(&self) -> Maca2lR {
        Maca2lR::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - MAC address2 low"]
    #[inline(always)]
    pub fn maca2l(&mut self) -> Maca2lW<'_, Maca2lrSpec> {
        Maca2lW::new(self, 0)
    }
}
#[doc = "Ethernet MAC address 2 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca2lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maca2lrSpec;
impl crate::RegisterSpec for Maca2lrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca2lr::R`](R) reader structure"]
impl crate::Readable for Maca2lrSpec {}
#[doc = "`write(|w| ..)` method takes [`maca2lr::W`](W) writer structure"]
impl crate::Writable for Maca2lrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACA2LR to value 0"]
impl crate::Resettable for Maca2lrSpec {}
