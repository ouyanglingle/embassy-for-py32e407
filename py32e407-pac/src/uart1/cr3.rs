#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `M_E` reader - Extension for DLS"]
pub type MER = crate::BitReader;
#[doc = "Field `M_E` writer - Extension for DLS"]
pub type MEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_MATCH` reader - Address Match Mode"]
pub type AddrMatchR = crate::BitReader;
#[doc = "Field `ADDR_MATCH` writer - Address Match Mode"]
pub type AddrMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_ADDR` reader - Send address control bit"]
pub type SendAddrR = crate::BitReader;
#[doc = "Field `SEND_ADDR` writer - Send address control bit"]
pub type SendAddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_MODE` reader - Transmit mode control bit"]
pub type TxModeR = crate::BitReader;
#[doc = "Field `TX_MODE` writer - Transmit mode control bit"]
pub type TxModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAR` reader - DMA receive enable"]
pub type DmarR = crate::BitReader;
#[doc = "Field `DMAR` writer - DMA receive enable"]
pub type DmarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAT` reader - DMA transmit enable"]
pub type DmatR = crate::BitReader;
#[doc = "Field `DMAT` writer - DMA transmit enable"]
pub type DmatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_SOFT_ACK` reader - DMA software ack"]
pub type DmaSoftAckR = crate::BitReader;
#[doc = "Field `DMA_SOFT_ACK` writer - DMA software ack"]
pub type DmaSoftAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Extension for DLS"]
    #[inline(always)]
    pub fn m_e(&self) -> MER {
        MER::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address Match Mode"]
    #[inline(always)]
    pub fn addr_match(&self) -> AddrMatchR {
        AddrMatchR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Send address control bit"]
    #[inline(always)]
    pub fn send_addr(&self) -> SendAddrR {
        SendAddrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit mode control bit"]
    #[inline(always)]
    pub fn tx_mode(&self) -> TxModeR {
        TxModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA receive enable"]
    #[inline(always)]
    pub fn dmar(&self) -> DmarR {
        DmarR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA transmit enable"]
    #[inline(always)]
    pub fn dmat(&self) -> DmatR {
        DmatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA software ack"]
    #[inline(always)]
    pub fn dma_soft_ack(&self) -> DmaSoftAckR {
        DmaSoftAckR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Extension for DLS"]
    #[inline(always)]
    pub fn m_e(&mut self) -> MEW<'_, Cr3Spec> {
        MEW::new(self, 0)
    }
    #[doc = "Bit 1 - Address Match Mode"]
    #[inline(always)]
    pub fn addr_match(&mut self) -> AddrMatchW<'_, Cr3Spec> {
        AddrMatchW::new(self, 1)
    }
    #[doc = "Bit 2 - Send address control bit"]
    #[inline(always)]
    pub fn send_addr(&mut self) -> SendAddrW<'_, Cr3Spec> {
        SendAddrW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit mode control bit"]
    #[inline(always)]
    pub fn tx_mode(&mut self) -> TxModeW<'_, Cr3Spec> {
        TxModeW::new(self, 3)
    }
    #[doc = "Bit 4 - DMA receive enable"]
    #[inline(always)]
    pub fn dmar(&mut self) -> DmarW<'_, Cr3Spec> {
        DmarW::new(self, 4)
    }
    #[doc = "Bit 5 - DMA transmit enable"]
    #[inline(always)]
    pub fn dmat(&mut self) -> DmatW<'_, Cr3Spec> {
        DmatW::new(self, 5)
    }
    #[doc = "Bit 6 - DMA software ack"]
    #[inline(always)]
    pub fn dma_soft_ack(&mut self) -> DmaSoftAckW<'_, Cr3Spec> {
        DmaSoftAckW::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for Cr3Spec {}
