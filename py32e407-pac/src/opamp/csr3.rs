#[doc = "Register `CSR3` reader"]
pub type R = crate::R<Csr3Spec>;
#[doc = "Register `CSR3` writer"]
pub type W = crate::W<Csr3Spec>;
#[doc = "Field `OPAEN` reader - OPA Enable"]
pub type OpaenR = crate::BitReader;
#[doc = "Field `OPAEN` writer - OPA Enable"]
pub type OpaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VP_SEL` reader - Non reverse input selection"]
pub type VpSelR = crate::FieldReader;
#[doc = "Field `VP_SEL` writer - Non reverse input selection"]
pub type VpSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VM_SEL` reader - Reverse input selection"]
pub type VmSelR = crate::FieldReader;
#[doc = "Field `VM_SEL` writer - Reverse input selection"]
pub type VmSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPAHSM` reader - OPA high-speed mode"]
pub type OpahsmR = crate::BitReader;
#[doc = "Field `OPAHSM` writer - OPA high-speed mode"]
pub type OpahsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAINTOEN` reader - OPA internal output enable"]
pub type OpaintoenR = crate::BitReader;
#[doc = "Field `OPAINTOEN` writer - OPA internal output enable"]
pub type OpaintoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGA_GAIN` reader - OPA configurable amplifier gain value"]
pub type PgaGainR = crate::FieldReader;
#[doc = "Field `PGA_GAIN` writer - OPA configurable amplifier gain value"]
pub type PgaGainW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OPA3_VBSEL` reader - OPA3 Bias selection"]
pub type Opa3VbselR = crate::BitReader;
#[doc = "Field `OPA3_VBSEL` writer - OPA3 Bias selection"]
pub type Opa3VbselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - OPAMP3_CSR register Lock"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - OPAMP3_CSR register Lock"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OPA Enable"]
    #[inline(always)]
    pub fn opaen(&self) -> OpaenR {
        OpaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Non reverse input selection"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VpSelR {
        VpSelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Reverse input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VmSelR {
        VmSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - OPA high-speed mode"]
    #[inline(always)]
    pub fn opahsm(&self) -> OpahsmR {
        OpahsmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OPA internal output enable"]
    #[inline(always)]
    pub fn opaintoen(&self) -> OpaintoenR {
        OpaintoenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 14:18 - OPA configurable amplifier gain value"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PgaGainR {
        PgaGainR::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - OPA3 Bias selection"]
    #[inline(always)]
    pub fn opa3_vbsel(&self) -> Opa3VbselR {
        Opa3VbselR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - OPAMP3_CSR register Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPA Enable"]
    #[inline(always)]
    pub fn opaen(&mut self) -> OpaenW<'_, Csr3Spec> {
        OpaenW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Non reverse input selection"]
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VpSelW<'_, Csr3Spec> {
        VpSelW::new(self, 2)
    }
    #[doc = "Bits 5:6 - Reverse input selection"]
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VmSelW<'_, Csr3Spec> {
        VmSelW::new(self, 5)
    }
    #[doc = "Bit 7 - OPA high-speed mode"]
    #[inline(always)]
    pub fn opahsm(&mut self) -> OpahsmW<'_, Csr3Spec> {
        OpahsmW::new(self, 7)
    }
    #[doc = "Bit 8 - OPA internal output enable"]
    #[inline(always)]
    pub fn opaintoen(&mut self) -> OpaintoenW<'_, Csr3Spec> {
        OpaintoenW::new(self, 8)
    }
    #[doc = "Bits 14:18 - OPA configurable amplifier gain value"]
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PgaGainW<'_, Csr3Spec> {
        PgaGainW::new(self, 14)
    }
    #[doc = "Bit 29 - OPA3 Bias selection"]
    #[inline(always)]
    pub fn opa3_vbsel(&mut self) -> Opa3VbselW<'_, Csr3Spec> {
        Opa3VbselW::new(self, 29)
    }
    #[doc = "Bit 31 - OPAMP3_CSR register Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, Csr3Spec> {
        LockW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`csr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr3Spec;
impl crate::RegisterSpec for Csr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr3::R`](R) reader structure"]
impl crate::Readable for Csr3Spec {}
#[doc = "`write(|w| ..)` method takes [`csr3::W`](W) writer structure"]
impl crate::Writable for Csr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR3 to value 0"]
impl crate::Resettable for Csr3Spec {}
