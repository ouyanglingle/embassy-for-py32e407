#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SDIORST` reader - desc SDIORST"]
pub type SdiorstR = crate::BitReader;
#[doc = "Field `SDIORST` writer - desc SDIORST"]
pub type SdiorstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFORST` reader - desc FIFORST"]
pub type FiforstR = crate::BitReader;
#[doc = "Field `FIFORST` writer - desc FIFORST"]
pub type FiforstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN` reader - desc INTEN"]
pub type IntenR = crate::BitReader;
#[doc = "Field `INTEN` writer - desc INTEN"]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - desc DMAEN"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - desc DMAEN"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READWAIT` reader - desc READWAIT"]
pub type ReadwaitR = crate::BitReader;
#[doc = "Field `READWAIT` writer - desc READWAIT"]
pub type ReadwaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOIRQRESP` reader - desc AUTOIRQRESP"]
pub type AutoirqrespR = crate::BitReader;
#[doc = "Field `AUTOIRQRESP` writer - desc AUTOIRQRESP"]
pub type AutoirqrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORTRD` reader - desc ABORTRD"]
pub type AbortrdR = crate::BitReader;
#[doc = "Field `ABORTRD` writer - desc ABORTRD"]
pub type AbortrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCSDEN` reader - desc CCSDEN"]
pub type CcsdenR = crate::BitReader;
#[doc = "Field `CCSDEN` writer - desc CCSDEN"]
pub type CcsdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOSTOPCCSD` reader - desc AUTOSTOPCCSD"]
pub type AutostopccsdR = crate::BitReader;
#[doc = "Field `AUTOSTOPCCSD` writer - desc AUTOSTOPCCSD"]
pub type AutostopccsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEATAINTEN` reader - desc CEATAINTEN"]
pub type CeataintenR = crate::BitReader;
#[doc = "Field `CEATAINTEN` writer - desc CEATAINTEN"]
pub type CeataintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODPUEN` reader - desc ODPUEN"]
pub type OdpuenR = crate::BitReader;
#[doc = "Field `ODPUEN` writer - desc ODPUEN"]
pub type OdpuenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SDIORST"]
    #[inline(always)]
    pub fn sdiorst(&self) -> SdiorstR {
        SdiorstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FIFORST"]
    #[inline(always)]
    pub fn fiforst(&self) -> FiforstR {
        FiforstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc INTEN"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc READWAIT"]
    #[inline(always)]
    pub fn readwait(&self) -> ReadwaitR {
        ReadwaitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc AUTOIRQRESP"]
    #[inline(always)]
    pub fn autoirqresp(&self) -> AutoirqrespR {
        AutoirqrespR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc ABORTRD"]
    #[inline(always)]
    pub fn abortrd(&self) -> AbortrdR {
        AbortrdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CCSDEN"]
    #[inline(always)]
    pub fn ccsden(&self) -> CcsdenR {
        CcsdenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc AUTOSTOPCCSD"]
    #[inline(always)]
    pub fn autostopccsd(&self) -> AutostopccsdR {
        AutostopccsdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc CEATAINTEN"]
    #[inline(always)]
    pub fn ceatainten(&self) -> CeataintenR {
        CeataintenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 24 - desc ODPUEN"]
    #[inline(always)]
    pub fn odpuen(&self) -> OdpuenR {
        OdpuenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SDIORST"]
    #[inline(always)]
    pub fn sdiorst(&mut self) -> SdiorstW<'_, CtrlSpec> {
        SdiorstW::new(self, 0)
    }
    #[doc = "Bit 1 - desc FIFORST"]
    #[inline(always)]
    pub fn fiforst(&mut self) -> FiforstW<'_, CtrlSpec> {
        FiforstW::new(self, 1)
    }
    #[doc = "Bit 4 - desc INTEN"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<'_, CtrlSpec> {
        IntenW::new(self, 4)
    }
    #[doc = "Bit 5 - desc DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, CtrlSpec> {
        DmaenW::new(self, 5)
    }
    #[doc = "Bit 6 - desc READWAIT"]
    #[inline(always)]
    pub fn readwait(&mut self) -> ReadwaitW<'_, CtrlSpec> {
        ReadwaitW::new(self, 6)
    }
    #[doc = "Bit 7 - desc AUTOIRQRESP"]
    #[inline(always)]
    pub fn autoirqresp(&mut self) -> AutoirqrespW<'_, CtrlSpec> {
        AutoirqrespW::new(self, 7)
    }
    #[doc = "Bit 8 - desc ABORTRD"]
    #[inline(always)]
    pub fn abortrd(&mut self) -> AbortrdW<'_, CtrlSpec> {
        AbortrdW::new(self, 8)
    }
    #[doc = "Bit 9 - desc CCSDEN"]
    #[inline(always)]
    pub fn ccsden(&mut self) -> CcsdenW<'_, CtrlSpec> {
        CcsdenW::new(self, 9)
    }
    #[doc = "Bit 10 - desc AUTOSTOPCCSD"]
    #[inline(always)]
    pub fn autostopccsd(&mut self) -> AutostopccsdW<'_, CtrlSpec> {
        AutostopccsdW::new(self, 10)
    }
    #[doc = "Bit 11 - desc CEATAINTEN"]
    #[inline(always)]
    pub fn ceatainten(&mut self) -> CeataintenW<'_, CtrlSpec> {
        CeataintenW::new(self, 11)
    }
    #[doc = "Bit 24 - desc ODPUEN"]
    #[inline(always)]
    pub fn odpuen(&mut self) -> OdpuenW<'_, CtrlSpec> {
        OdpuenW::new(self, 24)
    }
}
#[doc = "desc CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
