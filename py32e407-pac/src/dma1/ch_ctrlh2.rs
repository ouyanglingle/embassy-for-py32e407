#[doc = "Register `CH_CTRLH2` reader"]
pub type R = crate::R<ChCtrlh2Spec>;
#[doc = "Register `CH_CTRLH2` writer"]
pub type W = crate::W<ChCtrlh2Spec>;
#[doc = "Field `BLOCK_TS` reader - Block Transfer Size."]
pub type BlockTsR = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_TS` writer - Block Transfer Size."]
pub type BlockTsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Block Transfer Size."]
    #[inline(always)]
    pub fn block_ts(&self) -> BlockTsR {
        BlockTsR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Block Transfer Size."]
    #[inline(always)]
    pub fn block_ts(&mut self) -> BlockTsW<'_, ChCtrlh2Spec> {
        BlockTsW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctrlh2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctrlh2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChCtrlh2Spec;
impl crate::RegisterSpec for ChCtrlh2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_ctrlh2::R`](R) reader structure"]
impl crate::Readable for ChCtrlh2Spec {}
#[doc = "`write(|w| ..)` method takes [`ch_ctrlh2::W`](W) writer structure"]
impl crate::Writable for ChCtrlh2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_CTRLH2 to value 0"]
impl crate::Resettable for ChCtrlh2Spec {}
