#[doc = "Register `TXEFS` reader"]
pub type R = crate::R<TxefsSpec>;
#[doc = "Register `TXEFS` writer"]
pub type W = crate::W<TxefsSpec>;
#[doc = "Field `EFFL` reader - Event FIFO Fill Level"]
pub type EfflR = crate::FieldReader;
#[doc = "Field `EFGI` reader - Event FIFO Get Index"]
pub type EfgiR = crate::FieldReader;
#[doc = "Field `EFPI` reader - Event FIFO Put Index"]
pub type EfpiR = crate::FieldReader;
#[doc = "Field `EFF` reader - Event FIFO Full"]
pub type EffR = crate::BitReader;
#[doc = "Field `TEFL` reader - Tx Event FIFO Element Lost"]
pub type TeflR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Event FIFO Fill Level"]
    #[inline(always)]
    pub fn effl(&self) -> EfflR {
        EfflR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Event FIFO Get Index"]
    #[inline(always)]
    pub fn efgi(&self) -> EfgiR {
        EfgiR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Event FIFO Put Index"]
    #[inline(always)]
    pub fn efpi(&self) -> EfpiR {
        EfpiR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Event FIFO Full"]
    #[inline(always)]
    pub fn eff(&self) -> EffR {
        EffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TeflR {
        TeflR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`txefs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxefsSpec;
impl crate::RegisterSpec for TxefsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefs::R`](R) reader structure"]
impl crate::Readable for TxefsSpec {}
#[doc = "`write(|w| ..)` method takes [`txefs::W`](W) writer structure"]
impl crate::Writable for TxefsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXEFS to value 0"]
impl crate::Resettable for TxefsSpec {}
