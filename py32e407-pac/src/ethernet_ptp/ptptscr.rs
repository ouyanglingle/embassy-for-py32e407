#[doc = "Register `PTPTSCR` reader"]
pub type R = crate::R<PtptscrSpec>;
#[doc = "Register `PTPTSCR` writer"]
pub type W = crate::W<PtptscrSpec>;
#[doc = "Field `TSE` reader - Time stamp enable"]
pub type TseR = crate::BitReader;
#[doc = "Field `TSE` writer - Time stamp enable"]
pub type TseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSFCU` reader - Time stamp fine or coarse update"]
pub type TsfcuR = crate::BitReader;
#[doc = "Field `TSFCU` writer - Time stamp fine or coarse update"]
pub type TsfcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSTI` reader - Time stamp system time initialize"]
pub type TsstiR = crate::BitReader;
#[doc = "Field `TSSTI` writer - Time stamp system time initialize"]
pub type TsstiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSSTU` reader - Time stamp system time update"]
pub type TsstuR = crate::BitReader;
#[doc = "Field `TSSTU` writer - Time stamp system time update"]
pub type TsstuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSITE` reader - Time stamp interrupt trigger enable"]
pub type TsiteR = crate::BitReader;
#[doc = "Field `TSITE` writer - Time stamp interrupt trigger enable"]
pub type TsiteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSARU` reader - Time stamp addend register update"]
pub type TsaruR = crate::BitReader;
#[doc = "Field `TSARU` writer - Time stamp addend register update"]
pub type TsaruW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Time stamp enable"]
    #[inline(always)]
    pub fn tse(&self) -> TseR {
        TseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time stamp fine or coarse update"]
    #[inline(always)]
    pub fn tsfcu(&self) -> TsfcuR {
        TsfcuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time stamp system time initialize"]
    #[inline(always)]
    pub fn tssti(&self) -> TsstiR {
        TsstiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time stamp system time update"]
    #[inline(always)]
    pub fn tsstu(&self) -> TsstuR {
        TsstuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Time stamp interrupt trigger enable"]
    #[inline(always)]
    pub fn tsite(&self) -> TsiteR {
        TsiteR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Time stamp addend register update"]
    #[inline(always)]
    pub fn tsaru(&self) -> TsaruR {
        TsaruR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time stamp enable"]
    #[inline(always)]
    pub fn tse(&mut self) -> TseW<'_, PtptscrSpec> {
        TseW::new(self, 0)
    }
    #[doc = "Bit 1 - Time stamp fine or coarse update"]
    #[inline(always)]
    pub fn tsfcu(&mut self) -> TsfcuW<'_, PtptscrSpec> {
        TsfcuW::new(self, 1)
    }
    #[doc = "Bit 2 - Time stamp system time initialize"]
    #[inline(always)]
    pub fn tssti(&mut self) -> TsstiW<'_, PtptscrSpec> {
        TsstiW::new(self, 2)
    }
    #[doc = "Bit 3 - Time stamp system time update"]
    #[inline(always)]
    pub fn tsstu(&mut self) -> TsstuW<'_, PtptscrSpec> {
        TsstuW::new(self, 3)
    }
    #[doc = "Bit 4 - Time stamp interrupt trigger enable"]
    #[inline(always)]
    pub fn tsite(&mut self) -> TsiteW<'_, PtptscrSpec> {
        TsiteW::new(self, 4)
    }
    #[doc = "Bit 5 - Time stamp addend register update"]
    #[inline(always)]
    pub fn tsaru(&mut self) -> TsaruW<'_, PtptscrSpec> {
        TsaruW::new(self, 5)
    }
}
#[doc = "Ethernet PTP time stamp control register (ETH_PTPTSCR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtptscrSpec;
impl crate::RegisterSpec for PtptscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptscr::R`](R) reader structure"]
impl crate::Readable for PtptscrSpec {}
#[doc = "`write(|w| ..)` method takes [`ptptscr::W`](W) writer structure"]
impl crate::Writable for PtptscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PTPTSCR to value 0"]
impl crate::Resettable for PtptscrSpec {}
