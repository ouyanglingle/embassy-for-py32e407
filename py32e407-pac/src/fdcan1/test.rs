#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Field `LBCK` reader - Loop Back Mode"]
pub type LbckR = crate::BitReader;
#[doc = "Field `LBCK` writer - Loop Back Mode"]
pub type LbckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX` reader - Control of Transmit Pin"]
pub type TxR = crate::FieldReader;
#[doc = "Field `TX` writer - Control of Transmit Pin"]
pub type TxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX` reader - Receive Pin"]
pub type RxR = crate::BitReader;
#[doc = "Field `RX` writer - Receive Pin"]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBNP` reader - Tx Buffer Number Prepared"]
pub type TxbnpR = crate::FieldReader;
#[doc = "Field `TXBNP` writer - Tx Buffer Number Prepared"]
pub type TxbnpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PVAL` reader - Prepared Valid"]
pub type PvalR = crate::BitReader;
#[doc = "Field `PVAL` writer - Prepared Valid"]
pub type PvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBNS` reader - Tx Buffer Number Started"]
pub type TxbnsR = crate::FieldReader;
#[doc = "Field `TXBNS` writer - Tx Buffer Number Started"]
pub type TxbnsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SVAL` reader - Started Valid"]
pub type SvalR = crate::BitReader;
#[doc = "Field `SVAL` writer - Started Valid"]
pub type SvalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Loop Back Mode"]
    #[inline(always)]
    pub fn lbck(&self) -> LbckR {
        LbckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive Pin"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Tx Buffer Number Prepared"]
    #[inline(always)]
    pub fn txbnp(&self) -> TxbnpR {
        TxbnpR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - Prepared Valid"]
    #[inline(always)]
    pub fn pval(&self) -> PvalR {
        PvalR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Tx Buffer Number Started"]
    #[inline(always)]
    pub fn txbns(&self) -> TxbnsR {
        TxbnsR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Started Valid"]
    #[inline(always)]
    pub fn sval(&self) -> SvalR {
        SvalR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Loop Back Mode"]
    #[inline(always)]
    pub fn lbck(&mut self) -> LbckW<'_, TestSpec> {
        LbckW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin"]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, TestSpec> {
        TxW::new(self, 5)
    }
    #[doc = "Bit 7 - Receive Pin"]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<'_, TestSpec> {
        RxW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Tx Buffer Number Prepared"]
    #[inline(always)]
    pub fn txbnp(&mut self) -> TxbnpW<'_, TestSpec> {
        TxbnpW::new(self, 8)
    }
    #[doc = "Bit 13 - Prepared Valid"]
    #[inline(always)]
    pub fn pval(&mut self) -> PvalW<'_, TestSpec> {
        PvalW::new(self, 13)
    }
    #[doc = "Bits 16:20 - Tx Buffer Number Started"]
    #[inline(always)]
    pub fn txbns(&mut self) -> TxbnsW<'_, TestSpec> {
        TxbnsW::new(self, 16)
    }
    #[doc = "Bit 21 - Started Valid"]
    #[inline(always)]
    pub fn sval(&mut self) -> SvalW<'_, TestSpec> {
        SvalW::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {}
