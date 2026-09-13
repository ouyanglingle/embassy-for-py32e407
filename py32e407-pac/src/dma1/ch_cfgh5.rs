#[doc = "Register `CH_CFGH5` reader"]
pub type R = crate::R<ChCfgh5Spec>;
#[doc = "Register `CH_CFGH5` writer"]
pub type W = crate::W<ChCfgh5Spec>;
#[doc = "Field `FIFO_MODE` reader - FIFO Mode Select"]
pub type FifoModeR = crate::BitReader;
#[doc = "Field `FIFO_MODE` writer - FIFO Mode Select"]
pub type FifoModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROTCTL` reader - Protection Control"]
pub type ProtctlR = crate::FieldReader;
#[doc = "Field `PROTCTL` writer - Protection Control"]
pub type ProtctlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRC_PER` reader - Source hardware interface"]
pub type SrcPerR = crate::FieldReader;
#[doc = "Field `SRC_PER` writer - Source hardware interface"]
pub type SrcPerW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DST_PER` reader - Destination hardware interface"]
pub type DstPerR = crate::FieldReader;
#[doc = "Field `DST_PER` writer - Destination hardware interface"]
pub type DstPerW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 1 - FIFO Mode Select"]
    #[inline(always)]
    pub fn fifo_mode(&self) -> FifoModeR {
        FifoModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Protection Control"]
    #[inline(always)]
    pub fn protctl(&self) -> ProtctlR {
        ProtctlR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Source hardware interface"]
    #[inline(always)]
    pub fn src_per(&self) -> SrcPerR {
        SrcPerR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Destination hardware interface"]
    #[inline(always)]
    pub fn dst_per(&self) -> DstPerR {
        DstPerR::new(((self.bits >> 11) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - FIFO Mode Select"]
    #[inline(always)]
    pub fn fifo_mode(&mut self) -> FifoModeW<'_, ChCfgh5Spec> {
        FifoModeW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Protection Control"]
    #[inline(always)]
    pub fn protctl(&mut self) -> ProtctlW<'_, ChCfgh5Spec> {
        ProtctlW::new(self, 2)
    }
    #[doc = "Bits 7:9 - Source hardware interface"]
    #[inline(always)]
    pub fn src_per(&mut self) -> SrcPerW<'_, ChCfgh5Spec> {
        SrcPerW::new(self, 7)
    }
    #[doc = "Bits 11:13 - Destination hardware interface"]
    #[inline(always)]
    pub fn dst_per(&mut self) -> DstPerW<'_, ChCfgh5Spec> {
        DstPerW::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfgh5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfgh5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChCfgh5Spec;
impl crate::RegisterSpec for ChCfgh5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_cfgh5::R`](R) reader structure"]
impl crate::Readable for ChCfgh5Spec {}
#[doc = "`write(|w| ..)` method takes [`ch_cfgh5::W`](W) writer structure"]
impl crate::Writable for ChCfgh5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_CFGH5 to value 0"]
impl crate::Resettable for ChCfgh5Spec {}
