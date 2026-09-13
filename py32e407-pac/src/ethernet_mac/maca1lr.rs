#[doc = "Register `MACA1LR` reader"]
pub type R = crate::R<Maca1lrSpec>;
#[doc = "Register `MACA1LR` writer"]
pub type W = crate::W<Maca1lrSpec>;
#[doc = "Field `MACA1L` reader - MAC address1 low"]
pub type Maca1lR = crate::FieldReader<u32>;
#[doc = "Field `MACA1L` writer - MAC address1 low"]
pub type Maca1lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC address1 low"]
    #[inline(always)]
    pub fn maca1l(&self) -> Maca1lR {
        Maca1lR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC address1 low"]
    #[inline(always)]
    pub fn maca1l(&mut self) -> Maca1lW<'_, Maca1lrSpec> {
        Maca1lW::new(self, 0)
    }
}
#[doc = "Ethernet MAC address1 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca1lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maca1lrSpec;
impl crate::RegisterSpec for Maca1lrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca1lr::R`](R) reader structure"]
impl crate::Readable for Maca1lrSpec {}
#[doc = "`write(|w| ..)` method takes [`maca1lr::W`](W) writer structure"]
impl crate::Writable for Maca1lrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACA1LR to value 0"]
impl crate::Resettable for Maca1lrSpec {}
