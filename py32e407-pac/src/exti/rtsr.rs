#[doc = "Register `RTSR` reader"]
pub type R = crate::R<RtsrSpec>;
#[doc = "Register `RTSR` writer"]
pub type W = crate::W<RtsrSpec>;
#[doc = "Field `RTSR0` reader - "]
pub type Rtsr0R = crate::BitReader;
#[doc = "Field `RTSR0` writer - "]
pub type Rtsr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR1` reader - "]
pub type Rtsr1R = crate::BitReader;
#[doc = "Field `RTSR1` writer - "]
pub type Rtsr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR2` reader - "]
pub type Rtsr2R = crate::BitReader;
#[doc = "Field `RTSR2` writer - "]
pub type Rtsr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR3` reader - "]
pub type Rtsr3R = crate::BitReader;
#[doc = "Field `RTSR3` writer - "]
pub type Rtsr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR4` reader - "]
pub type Rtsr4R = crate::BitReader;
#[doc = "Field `RTSR4` writer - "]
pub type Rtsr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR5` reader - "]
pub type Rtsr5R = crate::BitReader;
#[doc = "Field `RTSR5` writer - "]
pub type Rtsr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR6` reader - "]
pub type Rtsr6R = crate::BitReader;
#[doc = "Field `RTSR6` writer - "]
pub type Rtsr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR7` reader - "]
pub type Rtsr7R = crate::BitReader;
#[doc = "Field `RTSR7` writer - "]
pub type Rtsr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR8` reader - "]
pub type Rtsr8R = crate::BitReader;
#[doc = "Field `RTSR8` writer - "]
pub type Rtsr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR9` reader - "]
pub type Rtsr9R = crate::BitReader;
#[doc = "Field `RTSR9` writer - "]
pub type Rtsr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR10` reader - "]
pub type Rtsr10R = crate::BitReader;
#[doc = "Field `RTSR10` writer - "]
pub type Rtsr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR11` reader - "]
pub type Rtsr11R = crate::BitReader;
#[doc = "Field `RTSR11` writer - "]
pub type Rtsr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR12` reader - "]
pub type Rtsr12R = crate::BitReader;
#[doc = "Field `RTSR12` writer - "]
pub type Rtsr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR13` reader - "]
pub type Rtsr13R = crate::BitReader;
#[doc = "Field `RTSR13` writer - "]
pub type Rtsr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR14` reader - "]
pub type Rtsr14R = crate::BitReader;
#[doc = "Field `RTSR14` writer - "]
pub type Rtsr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR15` reader - "]
pub type Rtsr15R = crate::BitReader;
#[doc = "Field `RTSR15` writer - "]
pub type Rtsr15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR16` reader - "]
pub type Rtsr16R = crate::BitReader;
#[doc = "Field `RTSR16` writer - "]
pub type Rtsr16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR17` reader - "]
pub type Rtsr17R = crate::BitReader;
#[doc = "Field `RTSR17` writer - "]
pub type Rtsr17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR18` reader - "]
pub type Rtsr18R = crate::BitReader;
#[doc = "Field `RTSR18` writer - "]
pub type Rtsr18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR19` reader - "]
pub type Rtsr19R = crate::BitReader;
#[doc = "Field `RTSR19` writer - "]
pub type Rtsr19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR20` reader - "]
pub type Rtsr20R = crate::BitReader;
#[doc = "Field `RTSR20` writer - "]
pub type Rtsr20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSR21` reader - "]
pub type Rtsr21R = crate::BitReader;
#[doc = "Field `RTSR21` writer - "]
pub type Rtsr21W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtsr0(&self) -> Rtsr0R {
        Rtsr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtsr1(&self) -> Rtsr1R {
        Rtsr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtsr2(&self) -> Rtsr2R {
        Rtsr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtsr3(&self) -> Rtsr3R {
        Rtsr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtsr4(&self) -> Rtsr4R {
        Rtsr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtsr5(&self) -> Rtsr5R {
        Rtsr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtsr6(&self) -> Rtsr6R {
        Rtsr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtsr7(&self) -> Rtsr7R {
        Rtsr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtsr8(&self) -> Rtsr8R {
        Rtsr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rtsr9(&self) -> Rtsr9R {
        Rtsr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rtsr10(&self) -> Rtsr10R {
        Rtsr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rtsr11(&self) -> Rtsr11R {
        Rtsr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtsr12(&self) -> Rtsr12R {
        Rtsr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rtsr13(&self) -> Rtsr13R {
        Rtsr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rtsr14(&self) -> Rtsr14R {
        Rtsr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtsr15(&self) -> Rtsr15R {
        Rtsr15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rtsr16(&self) -> Rtsr16R {
        Rtsr16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rtsr17(&self) -> Rtsr17R {
        Rtsr17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rtsr18(&self) -> Rtsr18R {
        Rtsr18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rtsr19(&self) -> Rtsr19R {
        Rtsr19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rtsr20(&self) -> Rtsr20R {
        Rtsr20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rtsr21(&self) -> Rtsr21R {
        Rtsr21R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtsr0(&mut self) -> Rtsr0W<'_, RtsrSpec> {
        Rtsr0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtsr1(&mut self) -> Rtsr1W<'_, RtsrSpec> {
        Rtsr1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtsr2(&mut self) -> Rtsr2W<'_, RtsrSpec> {
        Rtsr2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtsr3(&mut self) -> Rtsr3W<'_, RtsrSpec> {
        Rtsr3W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtsr4(&mut self) -> Rtsr4W<'_, RtsrSpec> {
        Rtsr4W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtsr5(&mut self) -> Rtsr5W<'_, RtsrSpec> {
        Rtsr5W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtsr6(&mut self) -> Rtsr6W<'_, RtsrSpec> {
        Rtsr6W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtsr7(&mut self) -> Rtsr7W<'_, RtsrSpec> {
        Rtsr7W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtsr8(&mut self) -> Rtsr8W<'_, RtsrSpec> {
        Rtsr8W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rtsr9(&mut self) -> Rtsr9W<'_, RtsrSpec> {
        Rtsr9W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rtsr10(&mut self) -> Rtsr10W<'_, RtsrSpec> {
        Rtsr10W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rtsr11(&mut self) -> Rtsr11W<'_, RtsrSpec> {
        Rtsr11W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtsr12(&mut self) -> Rtsr12W<'_, RtsrSpec> {
        Rtsr12W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rtsr13(&mut self) -> Rtsr13W<'_, RtsrSpec> {
        Rtsr13W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rtsr14(&mut self) -> Rtsr14W<'_, RtsrSpec> {
        Rtsr14W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtsr15(&mut self) -> Rtsr15W<'_, RtsrSpec> {
        Rtsr15W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rtsr16(&mut self) -> Rtsr16W<'_, RtsrSpec> {
        Rtsr16W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rtsr17(&mut self) -> Rtsr17W<'_, RtsrSpec> {
        Rtsr17W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rtsr18(&mut self) -> Rtsr18W<'_, RtsrSpec> {
        Rtsr18W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rtsr19(&mut self) -> Rtsr19W<'_, RtsrSpec> {
        Rtsr19W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rtsr20(&mut self) -> Rtsr20W<'_, RtsrSpec> {
        Rtsr20W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rtsr21(&mut self) -> Rtsr21W<'_, RtsrSpec> {
        Rtsr21W::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtsrSpec;
impl crate::RegisterSpec for RtsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr::R`](R) reader structure"]
impl crate::Readable for RtsrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtsr::W`](W) writer structure"]
impl crate::Writable for RtsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTSR to value 0"]
impl crate::Resettable for RtsrSpec {}
