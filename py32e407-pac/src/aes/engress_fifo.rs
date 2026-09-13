#[doc = "Register `ENGRESS_FIFO` reader"]
pub type R = crate::R<EngressFifoSpec>;
#[doc = "Register `ENGRESS_FIFO` writer"]
pub type W = crate::W<EngressFifoSpec>;
#[doc = "Field `INGRESS_FIFO` reader - "]
pub type IngressFifoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ingress_fifo(&self) -> IngressFifoR {
        IngressFifoR::new(self.bits)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`engress_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`engress_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EngressFifoSpec;
impl crate::RegisterSpec for EngressFifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`engress_fifo::R`](R) reader structure"]
impl crate::Readable for EngressFifoSpec {}
#[doc = "`write(|w| ..)` method takes [`engress_fifo::W`](W) writer structure"]
impl crate::Writable for EngressFifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENGRESS_FIFO to value 0"]
impl crate::Resettable for EngressFifoSpec {}
