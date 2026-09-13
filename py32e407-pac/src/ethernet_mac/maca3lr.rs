#[doc = "Register `MACA3LR` reader"]
pub type R = crate::R<Maca3lrSpec>;
#[doc = "Register `MACA3LR` writer"]
pub type W = crate::W<Maca3lrSpec>;
#[doc = "Field `MBCA3L` reader - MAC address3 low"]
pub type Mbca3lR = crate::FieldReader<u32>;
#[doc = "Field `MBCA3L` writer - MAC address3 low"]
pub type Mbca3lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC address3 low"]
    #[inline(always)]
    pub fn mbca3l(&self) -> Mbca3lR {
        Mbca3lR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC address3 low"]
    #[inline(always)]
    pub fn mbca3l(&mut self) -> Mbca3lW<'_, Maca3lrSpec> {
        Mbca3lW::new(self, 0)
    }
}
#[doc = "Ethernet MAC address 3 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`maca3lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maca3lrSpec;
impl crate::RegisterSpec for Maca3lrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca3lr::R`](R) reader structure"]
impl crate::Readable for Maca3lrSpec {}
#[doc = "`write(|w| ..)` method takes [`maca3lr::W`](W) writer structure"]
impl crate::Writable for Maca3lrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACA3LR to value 0"]
impl crate::Resettable for Maca3lrSpec {}
