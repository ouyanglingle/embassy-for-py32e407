#[doc = "Register `RSSR` reader"]
pub type R = crate::R<RssrSpec>;
#[doc = "Register `RSSR` writer"]
pub type W = crate::W<RssrSpec>;
#[doc = "Field `RSS0` reader - "]
pub type Rss0R = crate::BitReader;
#[doc = "Field `RSS1` reader - "]
pub type Rss1R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rss0(&self) -> Rss0R {
        Rss0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rss1(&self) -> Rss1R {
        Rss1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssrSpec;
impl crate::RegisterSpec for RssrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rssr::R`](R) reader structure"]
impl crate::Readable for RssrSpec {}
#[doc = "`write(|w| ..)` method takes [`rssr::W`](W) writer structure"]
impl crate::Writable for RssrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSSR to value 0"]
impl crate::Resettable for RssrSpec {}
