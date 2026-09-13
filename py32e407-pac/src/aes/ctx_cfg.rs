#[doc = "Register `CTX_CFG` reader"]
pub type R = crate::R<CtxCfgSpec>;
#[doc = "Register `CTX_CFG` writer"]
pub type W = crate::W<CtxCfgSpec>;
#[doc = "Field `CTX_RET` reader - "]
pub type CtxRetR = crate::BitReader;
#[doc = "Field `CTX_RET` writer - "]
pub type CtxRetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTX_STR` reader - "]
pub type CtxStrR = crate::BitReader;
#[doc = "Field `CTX_STR` writer - "]
pub type CtxStrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INV_KEY_RET` reader - "]
pub type InvKeyRetR = crate::BitReader;
#[doc = "Field `INV_KEY_RET` writer - "]
pub type InvKeyRetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INV_KEY_STR` reader - "]
pub type InvKeyStrR = crate::BitReader;
#[doc = "Field `INV_KEY_STR` writer - "]
pub type InvKeyStrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTX_INDEX` reader - "]
pub type CtxIndexR = crate::FieldReader<u16>;
#[doc = "Field `CTX_INDEX` writer - "]
pub type CtxIndexW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctx_ret(&self) -> CtxRetR {
        CtxRetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ctx_str(&self) -> CtxStrR {
        CtxStrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn inv_key_ret(&self) -> InvKeyRetR {
        InvKeyRetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn inv_key_str(&self) -> InvKeyStrR {
        InvKeyStrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:15"]
    #[inline(always)]
    pub fn ctx_index(&self) -> CtxIndexR {
        CtxIndexR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctx_ret(&mut self) -> CtxRetW<'_, CtxCfgSpec> {
        CtxRetW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ctx_str(&mut self) -> CtxStrW<'_, CtxCfgSpec> {
        CtxStrW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn inv_key_ret(&mut self) -> InvKeyRetW<'_, CtxCfgSpec> {
        InvKeyRetW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn inv_key_str(&mut self) -> InvKeyStrW<'_, CtxCfgSpec> {
        InvKeyStrW::new(self, 3)
    }
    #[doc = "Bits 4:15"]
    #[inline(always)]
    pub fn ctx_index(&mut self) -> CtxIndexW<'_, CtxCfgSpec> {
        CtxIndexW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxCfgSpec;
impl crate::RegisterSpec for CtxCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_cfg::R`](R) reader structure"]
impl crate::Readable for CtxCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ctx_cfg::W`](W) writer structure"]
impl crate::Writable for CtxCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTX_CFG to value 0"]
impl crate::Resettable for CtxCfgSpec {}
