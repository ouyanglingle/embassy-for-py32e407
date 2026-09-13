#[doc = "Register `FTSR` reader"]
pub type R = crate::R<FtsrSpec>;
#[doc = "Register `FTSR` writer"]
pub type W = crate::W<FtsrSpec>;
#[doc = "Field `FTSR0` reader - "]
pub type Ftsr0R = crate::BitReader;
#[doc = "Field `FTSR0` writer - "]
pub type Ftsr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR1` reader - "]
pub type Ftsr1R = crate::BitReader;
#[doc = "Field `FTSR1` writer - "]
pub type Ftsr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR2` reader - "]
pub type Ftsr2R = crate::BitReader;
#[doc = "Field `FTSR2` writer - "]
pub type Ftsr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR3` reader - "]
pub type Ftsr3R = crate::BitReader;
#[doc = "Field `FTSR3` writer - "]
pub type Ftsr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR4` reader - "]
pub type Ftsr4R = crate::BitReader;
#[doc = "Field `FTSR4` writer - "]
pub type Ftsr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR5` reader - "]
pub type Ftsr5R = crate::BitReader;
#[doc = "Field `FTSR5` writer - "]
pub type Ftsr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR6` reader - "]
pub type Ftsr6R = crate::BitReader;
#[doc = "Field `FTSR6` writer - "]
pub type Ftsr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR7` reader - "]
pub type Ftsr7R = crate::BitReader;
#[doc = "Field `FTSR7` writer - "]
pub type Ftsr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR8` reader - "]
pub type Ftsr8R = crate::BitReader;
#[doc = "Field `FTSR8` writer - "]
pub type Ftsr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR9` reader - "]
pub type Ftsr9R = crate::BitReader;
#[doc = "Field `FTSR9` writer - "]
pub type Ftsr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR10` reader - "]
pub type Ftsr10R = crate::BitReader;
#[doc = "Field `FTSR10` writer - "]
pub type Ftsr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR11` reader - "]
pub type Ftsr11R = crate::BitReader;
#[doc = "Field `FTSR11` writer - "]
pub type Ftsr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR12` reader - "]
pub type Ftsr12R = crate::BitReader;
#[doc = "Field `FTSR12` writer - "]
pub type Ftsr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR13` reader - "]
pub type Ftsr13R = crate::BitReader;
#[doc = "Field `FTSR13` writer - "]
pub type Ftsr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR14` reader - "]
pub type Ftsr14R = crate::BitReader;
#[doc = "Field `FTSR14` writer - "]
pub type Ftsr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR15` reader - "]
pub type Ftsr15R = crate::BitReader;
#[doc = "Field `FTSR15` writer - "]
pub type Ftsr15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR16` reader - "]
pub type Ftsr16R = crate::BitReader;
#[doc = "Field `FTSR16` writer - "]
pub type Ftsr16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR17` reader - "]
pub type Ftsr17R = crate::BitReader;
#[doc = "Field `FTSR17` writer - "]
pub type Ftsr17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR18` reader - "]
pub type Ftsr18R = crate::BitReader;
#[doc = "Field `FTSR18` writer - "]
pub type Ftsr18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR19` reader - "]
pub type Ftsr19R = crate::BitReader;
#[doc = "Field `FTSR19` writer - "]
pub type Ftsr19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR20` reader - "]
pub type Ftsr20R = crate::BitReader;
#[doc = "Field `FTSR20` writer - "]
pub type Ftsr20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTSR21` reader - "]
pub type Ftsr21R = crate::BitReader;
#[doc = "Field `FTSR21` writer - "]
pub type Ftsr21W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ftsr0(&self) -> Ftsr0R {
        Ftsr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ftsr1(&self) -> Ftsr1R {
        Ftsr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ftsr2(&self) -> Ftsr2R {
        Ftsr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ftsr3(&self) -> Ftsr3R {
        Ftsr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ftsr4(&self) -> Ftsr4R {
        Ftsr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ftsr5(&self) -> Ftsr5R {
        Ftsr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ftsr6(&self) -> Ftsr6R {
        Ftsr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ftsr7(&self) -> Ftsr7R {
        Ftsr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ftsr8(&self) -> Ftsr8R {
        Ftsr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ftsr9(&self) -> Ftsr9R {
        Ftsr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ftsr10(&self) -> Ftsr10R {
        Ftsr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ftsr11(&self) -> Ftsr11R {
        Ftsr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ftsr12(&self) -> Ftsr12R {
        Ftsr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ftsr13(&self) -> Ftsr13R {
        Ftsr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ftsr14(&self) -> Ftsr14R {
        Ftsr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ftsr15(&self) -> Ftsr15R {
        Ftsr15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ftsr16(&self) -> Ftsr16R {
        Ftsr16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ftsr17(&self) -> Ftsr17R {
        Ftsr17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ftsr18(&self) -> Ftsr18R {
        Ftsr18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ftsr19(&self) -> Ftsr19R {
        Ftsr19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ftsr20(&self) -> Ftsr20R {
        Ftsr20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ftsr21(&self) -> Ftsr21R {
        Ftsr21R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ftsr0(&mut self) -> Ftsr0W<'_, FtsrSpec> {
        Ftsr0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ftsr1(&mut self) -> Ftsr1W<'_, FtsrSpec> {
        Ftsr1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ftsr2(&mut self) -> Ftsr2W<'_, FtsrSpec> {
        Ftsr2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ftsr3(&mut self) -> Ftsr3W<'_, FtsrSpec> {
        Ftsr3W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ftsr4(&mut self) -> Ftsr4W<'_, FtsrSpec> {
        Ftsr4W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ftsr5(&mut self) -> Ftsr5W<'_, FtsrSpec> {
        Ftsr5W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ftsr6(&mut self) -> Ftsr6W<'_, FtsrSpec> {
        Ftsr6W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ftsr7(&mut self) -> Ftsr7W<'_, FtsrSpec> {
        Ftsr7W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ftsr8(&mut self) -> Ftsr8W<'_, FtsrSpec> {
        Ftsr8W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ftsr9(&mut self) -> Ftsr9W<'_, FtsrSpec> {
        Ftsr9W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ftsr10(&mut self) -> Ftsr10W<'_, FtsrSpec> {
        Ftsr10W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ftsr11(&mut self) -> Ftsr11W<'_, FtsrSpec> {
        Ftsr11W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ftsr12(&mut self) -> Ftsr12W<'_, FtsrSpec> {
        Ftsr12W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ftsr13(&mut self) -> Ftsr13W<'_, FtsrSpec> {
        Ftsr13W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ftsr14(&mut self) -> Ftsr14W<'_, FtsrSpec> {
        Ftsr14W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ftsr15(&mut self) -> Ftsr15W<'_, FtsrSpec> {
        Ftsr15W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ftsr16(&mut self) -> Ftsr16W<'_, FtsrSpec> {
        Ftsr16W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ftsr17(&mut self) -> Ftsr17W<'_, FtsrSpec> {
        Ftsr17W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ftsr18(&mut self) -> Ftsr18W<'_, FtsrSpec> {
        Ftsr18W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ftsr19(&mut self) -> Ftsr19W<'_, FtsrSpec> {
        Ftsr19W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ftsr20(&mut self) -> Ftsr20W<'_, FtsrSpec> {
        Ftsr20W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ftsr21(&mut self) -> Ftsr21W<'_, FtsrSpec> {
        Ftsr21W::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtsrSpec;
impl crate::RegisterSpec for FtsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr::R`](R) reader structure"]
impl crate::Readable for FtsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ftsr::W`](W) writer structure"]
impl crate::Writable for FtsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FTSR to value 0"]
impl crate::Resettable for FtsrSpec {}
