#[doc = "Register `CH_CTRLH0` reader"]
pub type R = crate::R<ChCtrlh0Spec>;
#[doc = "Register `CH_CTRLH0` writer"]
pub type W = crate::W<ChCtrlh0Spec>;
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
    pub fn block_ts(&mut self) -> BlockTsW<'_, ChCtrlh0Spec> {
        BlockTsW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctrlh0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctrlh0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChCtrlh0Spec;
impl crate::RegisterSpec for ChCtrlh0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_ctrlh0::R`](R) reader structure"]
impl crate::Readable for ChCtrlh0Spec {}
#[doc = "`write(|w| ..)` method takes [`ch_ctrlh0::W`](W) writer structure"]
impl crate::Writable for ChCtrlh0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_CTRLH0 to value 0"]
impl crate::Resettable for ChCtrlh0Spec {}
