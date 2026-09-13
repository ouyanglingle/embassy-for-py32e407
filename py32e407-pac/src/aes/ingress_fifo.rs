#[doc = "Register `INGRESS_FIFO` reader"]
pub type R = crate::R<IngressFifoSpec>;
#[doc = "Register `INGRESS_FIFO` writer"]
pub type W = crate::W<IngressFifoSpec>;
#[doc = "Field `INGRESS_FIFO` reader - "]
pub type IngressFifoR = crate::FieldReader<u32>;
#[doc = "Field `INGRESS_FIFO` writer - "]
pub type IngressFifoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ingress_fifo(&self) -> IngressFifoR {
        IngressFifoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ingress_fifo(&mut self) -> IngressFifoW<'_, IngressFifoSpec> {
        IngressFifoW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ingress_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ingress_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IngressFifoSpec;
impl crate::RegisterSpec for IngressFifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ingress_fifo::R`](R) reader structure"]
impl crate::Readable for IngressFifoSpec {}
#[doc = "`write(|w| ..)` method takes [`ingress_fifo::W`](W) writer structure"]
impl crate::Writable for IngressFifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INGRESS_FIFO to value 0"]
impl crate::Resettable for IngressFifoSpec {}
