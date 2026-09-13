#[doc = "Register `FIFOTH` reader"]
pub type R = crate::R<FifothSpec>;
#[doc = "Register `FIFOTH` writer"]
pub type W = crate::W<FifothSpec>;
#[doc = "Field `TXWMARK` reader - desc TXWMARK"]
pub type TxwmarkR = crate::FieldReader<u16>;
#[doc = "Field `TXWMARK` writer - desc TXWMARK"]
pub type TxwmarkW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RXWMARK` reader - desc RXWMARK"]
pub type RxwmarkR = crate::FieldReader<u16>;
#[doc = "Field `RXWMARK` writer - desc RXWMARK"]
pub type RxwmarkW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - desc TXWMARK"]
    #[inline(always)]
    pub fn txwmark(&self) -> TxwmarkR {
        TxwmarkR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - desc RXWMARK"]
    #[inline(always)]
    pub fn rxwmark(&self) -> RxwmarkR {
        RxwmarkR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc TXWMARK"]
    #[inline(always)]
    pub fn txwmark(&mut self) -> TxwmarkW<'_, FifothSpec> {
        TxwmarkW::new(self, 0)
    }
    #[doc = "Bits 16:27 - desc RXWMARK"]
    #[inline(always)]
    pub fn rxwmark(&mut self) -> RxwmarkW<'_, FifothSpec> {
        RxwmarkW::new(self, 16)
    }
}
#[doc = "desc FIFOTH\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifothSpec;
impl crate::RegisterSpec for FifothSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoth::R`](R) reader structure"]
impl crate::Readable for FifothSpec {}
#[doc = "`write(|w| ..)` method takes [`fifoth::W`](W) writer structure"]
impl crate::Writable for FifothSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFOTH to value 0"]
impl crate::Resettable for FifothSpec {}
