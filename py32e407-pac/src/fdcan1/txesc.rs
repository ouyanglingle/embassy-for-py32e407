#[doc = "Register `TXESC` reader"]
pub type R = crate::R<TxescSpec>;
#[doc = "Register `TXESC` writer"]
pub type W = crate::W<TxescSpec>;
#[doc = "Field `TBDS` reader - Tx Buffer Data Field Size"]
pub type TbdsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size"]
    #[inline(always)]
    pub fn tbds(&self) -> TbdsR {
        TbdsR::new((self.bits & 7) as u8)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`txesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxescSpec;
impl crate::RegisterSpec for TxescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txesc::R`](R) reader structure"]
impl crate::Readable for TxescSpec {}
#[doc = "`write(|w| ..)` method takes [`txesc::W`](W) writer structure"]
impl crate::Writable for TxescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXESC to value 0"]
impl crate::Resettable for TxescSpec {}
