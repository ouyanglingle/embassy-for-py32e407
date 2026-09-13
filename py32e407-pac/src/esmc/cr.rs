#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `SOFTRST` reader - Software Reset"]
pub type SoftrstR = crate::BitReader;
#[doc = "Field `SOFTRST` writer - Software Reset"]
pub type SoftrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `2QSPI` reader - Dual Flash Mode"]
pub type _2qspiR = crate::BitReader;
#[doc = "Field `2QSPI` writer - Dual Flash Mode"]
pub type _2qspiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - dma en"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - dma en"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIE` reader - globle interrupt enable"]
pub type GieR = crate::BitReader;
#[doc = "Field `GIE` writer - globle interrupt enable"]
pub type GieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIPEN` reader - xip mode en"]
pub type XipenR = crate::BitReader;
#[doc = "Field `XIPEN` writer - xip mode en"]
pub type XipenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIEN` reader - spi en"]
pub type SpienR = crate::BitReader;
#[doc = "Field `SPIEN` writer - spi en"]
pub type SpienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn softrst(&self) -> SoftrstR {
        SoftrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Dual Flash Mode"]
    #[inline(always)]
    pub fn _2qspi(&self) -> _2qspiR {
        _2qspiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - dma en"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - globle interrupt enable"]
    #[inline(always)]
    pub fn gie(&self) -> GieR {
        GieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - xip mode en"]
    #[inline(always)]
    pub fn xipen(&self) -> XipenR {
        XipenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - spi en"]
    #[inline(always)]
    pub fn spien(&self) -> SpienR {
        SpienR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn softrst(&mut self) -> SoftrstW<'_, CrSpec> {
        SoftrstW::new(self, 0)
    }
    #[doc = "Bit 2 - Dual Flash Mode"]
    #[inline(always)]
    pub fn _2qspi(&mut self) -> _2qspiW<'_, CrSpec> {
        _2qspiW::new(self, 2)
    }
    #[doc = "Bit 3 - dma en"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, CrSpec> {
        DmaenW::new(self, 3)
    }
    #[doc = "Bit 4 - globle interrupt enable"]
    #[inline(always)]
    pub fn gie(&mut self) -> GieW<'_, CrSpec> {
        GieW::new(self, 4)
    }
    #[doc = "Bit 6 - xip mode en"]
    #[inline(always)]
    pub fn xipen(&mut self) -> XipenW<'_, CrSpec> {
        XipenW::new(self, 6)
    }
    #[doc = "Bit 7 - spi en"]
    #[inline(always)]
    pub fn spien(&mut self) -> SpienW<'_, CrSpec> {
        SpienW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
