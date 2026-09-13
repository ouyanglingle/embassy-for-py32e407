#[doc = "Register `MSG_CFG` reader"]
pub type R = crate::R<MsgCfgSpec>;
#[doc = "Register `MSG_CFG` writer"]
pub type W = crate::W<MsgCfgSpec>;
#[doc = "Field `AES_GO` reader - "]
pub type AesGoR = crate::BitReader;
#[doc = "Field `AES_GO` writer - "]
pub type AesGoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSG_END` reader - "]
pub type MsgEndR = crate::BitReader;
#[doc = "Field `MSG_END` writer - "]
pub type MsgEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSG_BEGIN` reader - "]
pub type MsgBeginR = crate::BitReader;
#[doc = "Field `MSG_BEGIN` writer - "]
pub type MsgBeginW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALGODIR` reader - "]
pub type AlgodirR = crate::BitReader;
#[doc = "Field `ALGODIR` writer - "]
pub type AlgodirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALG_MODE` reader - "]
pub type AlgModeR = crate::FieldReader;
#[doc = "Field `ALG_MODE` writer - "]
pub type AlgModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MAC_LEN` reader - "]
pub type MacLenR = crate::FieldReader;
#[doc = "Field `MAC_LEN` writer - "]
pub type MacLenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `KEY_SIZE` reader - "]
pub type KeySizeR = crate::FieldReader;
#[doc = "Field `KEY_SIZE` writer - "]
pub type KeySizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aes_go(&self) -> AesGoR {
        AesGoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn msg_end(&self) -> MsgEndR {
        MsgEndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn msg_begin(&self) -> MsgBeginR {
        MsgBeginR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn algodir(&self) -> AlgodirR {
        AlgodirR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn alg_mode(&self) -> AlgModeR {
        AlgModeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn mac_len(&self) -> MacLenR {
        MacLenR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn key_size(&self) -> KeySizeR {
        KeySizeR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aes_go(&mut self) -> AesGoW<'_, MsgCfgSpec> {
        AesGoW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn msg_end(&mut self) -> MsgEndW<'_, MsgCfgSpec> {
        MsgEndW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn msg_begin(&mut self) -> MsgBeginW<'_, MsgCfgSpec> {
        MsgBeginW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn algodir(&mut self) -> AlgodirW<'_, MsgCfgSpec> {
        AlgodirW::new(self, 3)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn alg_mode(&mut self) -> AlgModeW<'_, MsgCfgSpec> {
        AlgModeW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn mac_len(&mut self) -> MacLenW<'_, MsgCfgSpec> {
        MacLenW::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn key_size(&mut self) -> KeySizeW<'_, MsgCfgSpec> {
        KeySizeW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`msg_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msg_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsgCfgSpec;
impl crate::RegisterSpec for MsgCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msg_cfg::R`](R) reader structure"]
impl crate::Readable for MsgCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`msg_cfg::W`](W) writer structure"]
impl crate::Writable for MsgCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSG_CFG to value 0"]
impl crate::Resettable for MsgCfgSpec {}
