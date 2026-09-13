#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crh: Crh,
    crl: Crl,
    prl0: Prl0,
    _reserved3: [u8; 0x03],
    prl1: Prl1,
    _reserved4: [u8; 0x03],
    prl2: Prl2,
    _reserved5: [u8; 0x03],
    div0: Div0,
    _reserved6: [u8; 0x03],
    div1: Div1,
    _reserved7: [u8; 0x03],
    div2: Div2,
    _reserved8: [u8; 0x03],
    cnt0: Cnt0,
    _reserved9: [u8; 0x03],
    cnt1: Cnt1,
    _reserved10: [u8; 0x03],
    cnt2: Cnt2,
    _reserved11: [u8; 0x03],
    cnt3: Cnt3,
    _reserved12: [u8; 0x03],
    alr0: Alr0,
    _reserved13: [u8; 0x03],
    alr1: Alr1,
    _reserved14: [u8; 0x03],
    alr2: Alr2,
    _reserved15: [u8; 0x03],
    alr3: Alr3,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn crh(&self) -> &Crh {
        &self.crh
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn crl(&self) -> &Crl {
        &self.crl
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn prl0(&self) -> &Prl0 {
        &self.prl0
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn prl1(&self) -> &Prl1 {
        &self.prl1
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn prl2(&self) -> &Prl2 {
        &self.prl2
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn div0(&self) -> &Div0 {
        &self.div0
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn div1(&self) -> &Div1 {
        &self.div1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn div2(&self) -> &Div2 {
        &self.div2
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn cnt0(&self) -> &Cnt0 {
        &self.cnt0
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn cnt1(&self) -> &Cnt1 {
        &self.cnt1
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn cnt2(&self) -> &Cnt2 {
        &self.cnt2
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn cnt3(&self) -> &Cnt3 {
        &self.cnt3
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn alr0(&self) -> &Alr0 {
        &self.alr0
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn alr1(&self) -> &Alr1 {
        &self.alr1
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn alr2(&self) -> &Alr2 {
        &self.alr2
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn alr3(&self) -> &Alr3 {
        &self.alr3
    }
}
#[doc = "CRH (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crh`] module"]
#[doc(alias = "CRH")]
pub type Crh = crate::Reg<crh::CrhSpec>;
#[doc = ""]
pub mod crh;
#[doc = "CRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crl`] module"]
#[doc(alias = "CRL")]
pub type Crl = crate::Reg<crl::CrlSpec>;
#[doc = ""]
pub mod crl;
#[doc = "PRL0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`prl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prl0`] module"]
#[doc(alias = "PRL0")]
pub type Prl0 = crate::Reg<prl0::Prl0Spec>;
#[doc = ""]
pub mod prl0;
#[doc = "PRL1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`prl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prl1`] module"]
#[doc(alias = "PRL1")]
pub type Prl1 = crate::Reg<prl1::Prl1Spec>;
#[doc = ""]
pub mod prl1;
#[doc = "PRL2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`prl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prl2`] module"]
#[doc(alias = "PRL2")]
pub type Prl2 = crate::Reg<prl2::Prl2Spec>;
#[doc = ""]
pub mod prl2;
#[doc = "DIV0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`div0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div0`] module"]
#[doc(alias = "DIV0")]
pub type Div0 = crate::Reg<div0::Div0Spec>;
#[doc = ""]
pub mod div0;
#[doc = "DIV1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`div1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div1`] module"]
#[doc(alias = "DIV1")]
pub type Div1 = crate::Reg<div1::Div1Spec>;
#[doc = ""]
pub mod div1;
#[doc = "DIV2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`div2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div2`] module"]
#[doc(alias = "DIV2")]
pub type Div2 = crate::Reg<div2::Div2Spec>;
#[doc = ""]
pub mod div2;
#[doc = "CNT0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt0`] module"]
#[doc(alias = "CNT0")]
pub type Cnt0 = crate::Reg<cnt0::Cnt0Spec>;
#[doc = ""]
pub mod cnt0;
#[doc = "CNT1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt1`] module"]
#[doc(alias = "CNT1")]
pub type Cnt1 = crate::Reg<cnt1::Cnt1Spec>;
#[doc = ""]
pub mod cnt1;
#[doc = "CNT2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cnt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt2`] module"]
#[doc(alias = "CNT2")]
pub type Cnt2 = crate::Reg<cnt2::Cnt2Spec>;
#[doc = ""]
pub mod cnt2;
#[doc = "CNT3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cnt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt3`] module"]
#[doc(alias = "CNT3")]
pub type Cnt3 = crate::Reg<cnt3::Cnt3Spec>;
#[doc = ""]
pub mod cnt3;
#[doc = "ALR0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`alr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alr0`] module"]
#[doc(alias = "ALR0")]
pub type Alr0 = crate::Reg<alr0::Alr0Spec>;
#[doc = ""]
pub mod alr0;
#[doc = "ALR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`alr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alr1`] module"]
#[doc(alias = "ALR1")]
pub type Alr1 = crate::Reg<alr1::Alr1Spec>;
#[doc = ""]
pub mod alr1;
#[doc = "ALR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`alr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alr2`] module"]
#[doc(alias = "ALR2")]
pub type Alr2 = crate::Reg<alr2::Alr2Spec>;
#[doc = ""]
pub mod alr2;
#[doc = "ALR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`alr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alr3`] module"]
#[doc(alias = "ALR3")]
pub type Alr3 = crate::Reg<alr3::Alr3Spec>;
#[doc = ""]
pub mod alr3;
