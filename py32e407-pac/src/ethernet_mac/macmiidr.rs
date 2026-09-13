#[doc = "Register `MACMIIDR` reader"]
pub type R = crate::R<MacmiidrSpec>;
#[doc = "Register `MACMIIDR` writer"]
pub type W = crate::W<MacmiidrSpec>;
#[doc = "Field `MD` reader - MII data"]
pub type MdR = crate::FieldReader<u16>;
#[doc = "Field `MD` writer - MII data"]
pub type MdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MII data"]
    #[inline(always)]
    pub fn md(&self) -> MdR {
        MdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII data"]
    #[inline(always)]
    pub fn md(&mut self) -> MdW<'_, MacmiidrSpec> {
        MdW::new(self, 0)
    }
}
#[doc = "Ethernet MAC MII data register (ETH_MACMIIDR)\n\nYou can [`read`](crate::Reg::read) this register and get [`macmiidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmiidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacmiidrSpec;
impl crate::RegisterSpec for MacmiidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmiidr::R`](R) reader structure"]
impl crate::Readable for MacmiidrSpec {}
#[doc = "`write(|w| ..)` method takes [`macmiidr::W`](W) writer structure"]
impl crate::Writable for MacmiidrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACMIIDR to value 0"]
impl crate::Resettable for MacmiidrSpec {}
