#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `PG` reader - Page Program"]
pub type PgR = crate::BitReader;
#[doc = "Field `PG` writer - Page Program"]
pub type PgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER` reader - Page Erase"]
pub type PerR = crate::BitReader;
#[doc = "Field `PER` writer - Page Erase"]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER0` reader - Bank 0 Mass Erase"]
pub type Mer0R = crate::BitReader;
#[doc = "Field `MER0` writer - Bank 0 Mass Erase"]
pub type Mer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER1` reader - Bank 1 Mass Erase"]
pub type Mer1R = crate::BitReader;
#[doc = "Field `MER1` writer - Bank 1 Mass Erase"]
pub type Mer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SER` reader - Sector Erase"]
pub type SerR = crate::BitReader;
#[doc = "Field `SER` writer - Sector Erase"]
pub type SerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BER` reader - Block Erase"]
pub type BerR = crate::BitReader;
#[doc = "Field `BER` writer - Block Erase"]
pub type BerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTSTRT` reader - Option bytes Programming Start"]
pub type OptstrtR = crate::BitReader;
#[doc = "Field `OPTSTRT` writer - Option bytes Programming Start"]
pub type OptstrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSTRT` reader - Programming Start"]
pub type PgstrtR = crate::BitReader;
#[doc = "Field `PGSTRT` writer - Programming Start"]
pub type PgstrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPIE` reader - End of operation interrupt enable"]
pub type EopieR = crate::BitReader;
#[doc = "Field `EOPIE` writer - End of operation interrupt enable"]
pub type EopieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDERRIE` reader - "]
pub type RderrieR = crate::BitReader;
#[doc = "Field `RDERRIE` writer - "]
pub type RderrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBL_LAUNCH` reader - Force the option bytes loading"]
pub type OblLaunchR = crate::BitReader;
#[doc = "Field `OBL_LAUNCH` writer - Force the option bytes loading"]
pub type OblLaunchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTLOCK` reader - Option Lock"]
pub type OptlockR = crate::BitReader;
#[doc = "Field `OPTLOCK` writer - Option Lock"]
pub type OptlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - Lock"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Page Program"]
    #[inline(always)]
    pub fn pg(&self) -> PgR {
        PgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page Erase"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 0 Mass Erase"]
    #[inline(always)]
    pub fn mer0(&self) -> Mer0R {
        Mer0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 1 Mass Erase"]
    #[inline(always)]
    pub fn mer1(&self) -> Mer1R {
        Mer1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Sector Erase"]
    #[inline(always)]
    pub fn ser(&self) -> SerR {
        SerR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Block Erase"]
    #[inline(always)]
    pub fn ber(&self) -> BerR {
        BerR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - Option bytes Programming Start"]
    #[inline(always)]
    pub fn optstrt(&self) -> OptstrtR {
        OptstrtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Programming Start"]
    #[inline(always)]
    pub fn pgstrt(&self) -> PgstrtR {
        PgstrtR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EopieR {
        EopieR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rderrie(&self) -> RderrieR {
        RderrieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force the option bytes loading"]
    #[inline(always)]
    pub fn obl_launch(&self) -> OblLaunchR {
        OblLaunchR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Option Lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OptlockR {
        OptlockR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Page Program"]
    #[inline(always)]
    pub fn pg(&mut self) -> PgW<'_, CrSpec> {
        PgW::new(self, 0)
    }
    #[doc = "Bit 1 - Page Erase"]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<'_, CrSpec> {
        PerW::new(self, 1)
    }
    #[doc = "Bit 2 - Bank 0 Mass Erase"]
    #[inline(always)]
    pub fn mer0(&mut self) -> Mer0W<'_, CrSpec> {
        Mer0W::new(self, 2)
    }
    #[doc = "Bit 3 - Bank 1 Mass Erase"]
    #[inline(always)]
    pub fn mer1(&mut self) -> Mer1W<'_, CrSpec> {
        Mer1W::new(self, 3)
    }
    #[doc = "Bit 11 - Sector Erase"]
    #[inline(always)]
    pub fn ser(&mut self) -> SerW<'_, CrSpec> {
        SerW::new(self, 11)
    }
    #[doc = "Bit 12 - Block Erase"]
    #[inline(always)]
    pub fn ber(&mut self) -> BerW<'_, CrSpec> {
        BerW::new(self, 12)
    }
    #[doc = "Bit 17 - Option bytes Programming Start"]
    #[inline(always)]
    pub fn optstrt(&mut self) -> OptstrtW<'_, CrSpec> {
        OptstrtW::new(self, 17)
    }
    #[doc = "Bit 19 - Programming Start"]
    #[inline(always)]
    pub fn pgstrt(&mut self) -> PgstrtW<'_, CrSpec> {
        PgstrtW::new(self, 19)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&mut self) -> EopieW<'_, CrSpec> {
        EopieW::new(self, 24)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, CrSpec> {
        ErrieW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rderrie(&mut self) -> RderrieW<'_, CrSpec> {
        RderrieW::new(self, 26)
    }
    #[doc = "Bit 27 - Force the option bytes loading"]
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OblLaunchW<'_, CrSpec> {
        OblLaunchW::new(self, 27)
    }
    #[doc = "Bit 30 - Option Lock"]
    #[inline(always)]
    pub fn optlock(&mut self) -> OptlockW<'_, CrSpec> {
        OptlockW::new(self, 30)
    }
    #[doc = "Bit 31 - Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, CrSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
