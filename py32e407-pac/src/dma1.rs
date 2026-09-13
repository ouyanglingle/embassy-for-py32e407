#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    en: En,
    _reserved1: [u8; 0x04],
    ch_en: ChEn,
    _reserved2: [u8; 0x04],
    ch_ctrll0: ChCtrll0,
    ch_ctrlh0: ChCtrlh0,
    ch_cfgl0: ChCfgl0,
    ch_cfgh0: ChCfgh0,
    sa0: Sa0,
    _reserved7: [u8; 0x04],
    da0: Da0,
    _reserved8: [u8; 0x3c],
    ch_ctrll1: ChCtrll1,
    ch_ctrlh1: ChCtrlh1,
    ch_cfgl1: ChCfgl1,
    ch_cfgh1: ChCfgh1,
    sa1: Sa1,
    _reserved13: [u8; 0x04],
    da1: Da1,
    _reserved14: [u8; 0x3c],
    ch_ctrll2: ChCtrll2,
    ch_ctrlh2: ChCtrlh2,
    ch_cfgl2: ChCfgl2,
    ch_cfgh2: ChCfgh2,
    sa2: Sa2,
    _reserved19: [u8; 0x04],
    da2: Da2,
    _reserved20: [u8; 0x3c],
    ch_ctrll3: ChCtrll3,
    ch_ctrlh3: ChCtrlh3,
    ch_cfgl3: ChCfgl3,
    ch_cfgh3: ChCfgh3,
    sa3: Sa3,
    _reserved25: [u8; 0x04],
    da3: Da3,
    _reserved26: [u8; 0x3c],
    ch_ctrll4: ChCtrll4,
    ch_ctrlh4: ChCtrlh4,
    ch_cfgl4: ChCfgl4,
    ch_cfgh4: ChCfgh4,
    sa4: Sa4,
    _reserved31: [u8; 0x04],
    da4: Da4,
    _reserved32: [u8; 0x3c],
    ch_ctrll5: ChCtrll5,
    ch_ctrlh5: ChCtrlh5,
    ch_cfgl5: ChCfgl5,
    ch_cfgh5: ChCfgh5,
    sa5: Sa5,
    _reserved37: [u8; 0x04],
    da5: Da5,
    _reserved38: [u8; 0xbc],
    rawhalfblock: Rawhalfblock,
    _reserved39: [u8; 0x04],
    statushalfblock: Statushalfblock,
    _reserved40: [u8; 0x04],
    maskhalfblock: Maskhalfblock,
    _reserved41: [u8; 0x04],
    clearhalfblock: Clearhalfblock,
    _reserved42: [u8; 0x04],
    rawtfr: Rawtfr,
    _reserved43: [u8; 0x04],
    rawblock: Rawblock,
    _reserved44: [u8; 0x04],
    rawsrctran: Rawsrctran,
    _reserved45: [u8; 0x04],
    rawdsttran: Rawdsttran,
    _reserved46: [u8; 0x04],
    rawerr: Rawerr,
    _reserved47: [u8; 0x04],
    statustfr: Statustfr,
    _reserved48: [u8; 0x04],
    statusblock: Statusblock,
    _reserved49: [u8; 0x04],
    statussrctran: Statussrctran,
    _reserved50: [u8; 0x04],
    statusdsttran: Statusdsttran,
    _reserved51: [u8; 0x04],
    statuserr: Statuserr,
    _reserved52: [u8; 0x04],
    masktfr: Masktfr,
    _reserved53: [u8; 0x04],
    maskblock: Maskblock,
    _reserved54: [u8; 0x04],
    masksrctran: Masksrctran,
    _reserved55: [u8; 0x04],
    maskdsttran: Maskdsttran,
    _reserved56: [u8; 0x04],
    maskerr: Maskerr,
    _reserved57: [u8; 0x04],
    cleartfr: Cleartfr,
    _reserved58: [u8; 0x04],
    clearblock: Clearblock,
    _reserved59: [u8; 0x04],
    clearsrctran: Clearsrctran,
    _reserved60: [u8; 0x04],
    cleardsttran: Cleardsttran,
    _reserved61: [u8; 0x04],
    clearerr: Clearerr,
    _reserved62: [u8; 0x04],
    statusint: Statusint,
    _reserved63: [u8; 0x04],
    reqsrc: Reqsrc,
    _reserved64: [u8; 0x04],
    reqdst: Reqdst,
    _reserved65: [u8; 0x04],
    sglrqsrc: Sglrqsrc,
    _reserved66: [u8; 0x04],
    sglrqdst: Sglrqdst,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn ch_en(&self) -> &ChEn {
        &self.ch_en
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn ch_ctrll0(&self) -> &ChCtrll0 {
        &self.ch_ctrll0
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn ch_ctrlh0(&self) -> &ChCtrlh0 {
        &self.ch_ctrlh0
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn ch_cfgl0(&self) -> &ChCfgl0 {
        &self.ch_cfgl0
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn ch_cfgh0(&self) -> &ChCfgh0 {
        &self.ch_cfgh0
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn sa0(&self) -> &Sa0 {
        &self.sa0
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn da0(&self) -> &Da0 {
        &self.da0
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn ch_ctrll1(&self) -> &ChCtrll1 {
        &self.ch_ctrll1
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn ch_ctrlh1(&self) -> &ChCtrlh1 {
        &self.ch_ctrlh1
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn ch_cfgl1(&self) -> &ChCfgl1 {
        &self.ch_cfgl1
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn ch_cfgh1(&self) -> &ChCfgh1 {
        &self.ch_cfgh1
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn sa1(&self) -> &Sa1 {
        &self.sa1
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn da1(&self) -> &Da1 {
        &self.da1
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn ch_ctrll2(&self) -> &ChCtrll2 {
        &self.ch_ctrll2
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn ch_ctrlh2(&self) -> &ChCtrlh2 {
        &self.ch_ctrlh2
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn ch_cfgl2(&self) -> &ChCfgl2 {
        &self.ch_cfgl2
    }
    #[doc = "0xcc - "]
    #[inline(always)]
    pub const fn ch_cfgh2(&self) -> &ChCfgh2 {
        &self.ch_cfgh2
    }
    #[doc = "0xd0 - "]
    #[inline(always)]
    pub const fn sa2(&self) -> &Sa2 {
        &self.sa2
    }
    #[doc = "0xd8 - "]
    #[inline(always)]
    pub const fn da2(&self) -> &Da2 {
        &self.da2
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn ch_ctrll3(&self) -> &ChCtrll3 {
        &self.ch_ctrll3
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn ch_ctrlh3(&self) -> &ChCtrlh3 {
        &self.ch_ctrlh3
    }
    #[doc = "0x120 - "]
    #[inline(always)]
    pub const fn ch_cfgl3(&self) -> &ChCfgl3 {
        &self.ch_cfgl3
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn ch_cfgh3(&self) -> &ChCfgh3 {
        &self.ch_cfgh3
    }
    #[doc = "0x128 - "]
    #[inline(always)]
    pub const fn sa3(&self) -> &Sa3 {
        &self.sa3
    }
    #[doc = "0x130 - "]
    #[inline(always)]
    pub const fn da3(&self) -> &Da3 {
        &self.da3
    }
    #[doc = "0x170 - "]
    #[inline(always)]
    pub const fn ch_ctrll4(&self) -> &ChCtrll4 {
        &self.ch_ctrll4
    }
    #[doc = "0x174 - "]
    #[inline(always)]
    pub const fn ch_ctrlh4(&self) -> &ChCtrlh4 {
        &self.ch_ctrlh4
    }
    #[doc = "0x178 - "]
    #[inline(always)]
    pub const fn ch_cfgl4(&self) -> &ChCfgl4 {
        &self.ch_cfgl4
    }
    #[doc = "0x17c - "]
    #[inline(always)]
    pub const fn ch_cfgh4(&self) -> &ChCfgh4 {
        &self.ch_cfgh4
    }
    #[doc = "0x180 - "]
    #[inline(always)]
    pub const fn sa4(&self) -> &Sa4 {
        &self.sa4
    }
    #[doc = "0x188 - "]
    #[inline(always)]
    pub const fn da4(&self) -> &Da4 {
        &self.da4
    }
    #[doc = "0x1c8 - "]
    #[inline(always)]
    pub const fn ch_ctrll5(&self) -> &ChCtrll5 {
        &self.ch_ctrll5
    }
    #[doc = "0x1cc - "]
    #[inline(always)]
    pub const fn ch_ctrlh5(&self) -> &ChCtrlh5 {
        &self.ch_ctrlh5
    }
    #[doc = "0x1d0 - "]
    #[inline(always)]
    pub const fn ch_cfgl5(&self) -> &ChCfgl5 {
        &self.ch_cfgl5
    }
    #[doc = "0x1d4 - "]
    #[inline(always)]
    pub const fn ch_cfgh5(&self) -> &ChCfgh5 {
        &self.ch_cfgh5
    }
    #[doc = "0x1d8 - "]
    #[inline(always)]
    pub const fn sa5(&self) -> &Sa5 {
        &self.sa5
    }
    #[doc = "0x1e0 - "]
    #[inline(always)]
    pub const fn da5(&self) -> &Da5 {
        &self.da5
    }
    #[doc = "0x2a0 - "]
    #[inline(always)]
    pub const fn rawhalfblock(&self) -> &Rawhalfblock {
        &self.rawhalfblock
    }
    #[doc = "0x2a8 - "]
    #[inline(always)]
    pub const fn statushalfblock(&self) -> &Statushalfblock {
        &self.statushalfblock
    }
    #[doc = "0x2b0 - "]
    #[inline(always)]
    pub const fn maskhalfblock(&self) -> &Maskhalfblock {
        &self.maskhalfblock
    }
    #[doc = "0x2b8 - "]
    #[inline(always)]
    pub const fn clearhalfblock(&self) -> &Clearhalfblock {
        &self.clearhalfblock
    }
    #[doc = "0x2c0 - "]
    #[inline(always)]
    pub const fn rawtfr(&self) -> &Rawtfr {
        &self.rawtfr
    }
    #[doc = "0x2c8 - "]
    #[inline(always)]
    pub const fn rawblock(&self) -> &Rawblock {
        &self.rawblock
    }
    #[doc = "0x2d0 - "]
    #[inline(always)]
    pub const fn rawsrctran(&self) -> &Rawsrctran {
        &self.rawsrctran
    }
    #[doc = "0x2d8 - "]
    #[inline(always)]
    pub const fn rawdsttran(&self) -> &Rawdsttran {
        &self.rawdsttran
    }
    #[doc = "0x2e0 - "]
    #[inline(always)]
    pub const fn rawerr(&self) -> &Rawerr {
        &self.rawerr
    }
    #[doc = "0x2e8 - "]
    #[inline(always)]
    pub const fn statustfr(&self) -> &Statustfr {
        &self.statustfr
    }
    #[doc = "0x2f0 - "]
    #[inline(always)]
    pub const fn statusblock(&self) -> &Statusblock {
        &self.statusblock
    }
    #[doc = "0x2f8 - "]
    #[inline(always)]
    pub const fn statussrctran(&self) -> &Statussrctran {
        &self.statussrctran
    }
    #[doc = "0x300 - "]
    #[inline(always)]
    pub const fn statusdsttran(&self) -> &Statusdsttran {
        &self.statusdsttran
    }
    #[doc = "0x308 - "]
    #[inline(always)]
    pub const fn statuserr(&self) -> &Statuserr {
        &self.statuserr
    }
    #[doc = "0x310 - "]
    #[inline(always)]
    pub const fn masktfr(&self) -> &Masktfr {
        &self.masktfr
    }
    #[doc = "0x318 - "]
    #[inline(always)]
    pub const fn maskblock(&self) -> &Maskblock {
        &self.maskblock
    }
    #[doc = "0x320 - "]
    #[inline(always)]
    pub const fn masksrctran(&self) -> &Masksrctran {
        &self.masksrctran
    }
    #[doc = "0x328 - "]
    #[inline(always)]
    pub const fn maskdsttran(&self) -> &Maskdsttran {
        &self.maskdsttran
    }
    #[doc = "0x330 - "]
    #[inline(always)]
    pub const fn maskerr(&self) -> &Maskerr {
        &self.maskerr
    }
    #[doc = "0x338 - "]
    #[inline(always)]
    pub const fn cleartfr(&self) -> &Cleartfr {
        &self.cleartfr
    }
    #[doc = "0x340 - "]
    #[inline(always)]
    pub const fn clearblock(&self) -> &Clearblock {
        &self.clearblock
    }
    #[doc = "0x348 - "]
    #[inline(always)]
    pub const fn clearsrctran(&self) -> &Clearsrctran {
        &self.clearsrctran
    }
    #[doc = "0x350 - "]
    #[inline(always)]
    pub const fn cleardsttran(&self) -> &Cleardsttran {
        &self.cleardsttran
    }
    #[doc = "0x358 - "]
    #[inline(always)]
    pub const fn clearerr(&self) -> &Clearerr {
        &self.clearerr
    }
    #[doc = "0x360 - "]
    #[inline(always)]
    pub const fn statusint(&self) -> &Statusint {
        &self.statusint
    }
    #[doc = "0x368 - "]
    #[inline(always)]
    pub const fn reqsrc(&self) -> &Reqsrc {
        &self.reqsrc
    }
    #[doc = "0x370 - "]
    #[inline(always)]
    pub const fn reqdst(&self) -> &Reqdst {
        &self.reqdst
    }
    #[doc = "0x378 - "]
    #[inline(always)]
    pub const fn sglrqsrc(&self) -> &Sglrqsrc {
        &self.sglrqsrc
    }
    #[doc = "0x380 - "]
    #[inline(always)]
    pub const fn sglrqdst(&self) -> &Sglrqdst {
        &self.sglrqdst
    }
}
#[doc = "EN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`] module"]
#[doc(alias = "EN")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = ""]
pub mod en;
#[doc = "CH_EN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_en`] module"]
#[doc(alias = "CH_EN")]
pub type ChEn = crate::Reg<ch_en::ChEnSpec>;
#[doc = ""]
pub mod ch_en;
#[doc = "CH_CTRLL0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctrll0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctrll0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ctrll0`] module"]
#[doc(alias = "CH_CTRLL0")]
pub type ChCtrll0 = crate::Reg<ch_ctrll0::ChCtrll0Spec>;
#[doc = ""]
pub mod ch_ctrll0;
#[doc = "CH_CTRLH0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctrlh0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctrlh0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ctrlh0`] module"]
#[doc(alias = "CH_CTRLH0")]
pub type ChCtrlh0 = crate::Reg<ch_ctrlh0::ChCtrlh0Spec>;
#[doc = ""]
pub mod ch_ctrlh0;
#[doc = "CH_CFGL0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfgl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfgl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_cfgl0`] module"]
#[doc(alias = "CH_CFGL0")]
pub type ChCfgl0 = crate::Reg<ch_cfgl0::ChCfgl0Spec>;
#[doc = ""]
pub mod ch_cfgl0;
#[doc = "CH_CFGH0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfgh0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfgh0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_cfgh0`] module"]
#[doc(alias = "CH_CFGH0")]
pub type ChCfgh0 = crate::Reg<ch_cfgh0::ChCfgh0Spec>;
#[doc = ""]
pub mod ch_cfgh0;
#[doc = "SA0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sa0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa0`] module"]
#[doc(alias = "SA0")]
pub type Sa0 = crate::Reg<sa0::Sa0Spec>;
#[doc = ""]
pub mod sa0;
#[doc = "DA0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`da0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`da0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@da0`] module"]
#[doc(alias = "DA0")]
pub type Da0 = crate::Reg<da0::Da0Spec>;
#[doc = ""]
pub mod da0;
#[doc = "CH_CTRLL1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctrll1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctrll1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ctrll1`] module"]
#[doc(alias = "CH_CTRLL1")]
pub type ChCtrll1 = crate::Reg<ch_ctrll1::ChCtrll1Spec>;
#[doc = ""]
pub mod ch_ctrll1;
#[doc = "CH_CTRLH1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctrlh1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctrlh1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ctrlh1`] module"]
#[doc(alias = "CH_CTRLH1")]
pub type ChCtrlh1 = crate::Reg<ch_ctrlh1::ChCtrlh1Spec>;
#[doc = ""]
pub mod ch_ctrlh1;
#[doc = "CH_CFGL1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfgl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfgl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_cfgl1`] module"]
#[doc(alias = "CH_CFGL1")]
pub type ChCfgl1 = crate::Reg<ch_cfgl1::ChCfgl1Spec>;
#[doc = ""]
pub mod ch_cfgl1;
#[doc = "CH_CFGH1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfgh1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfgh1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_cfgh1`] module"]
#[doc(alias = "CH_CFGH1")]
pub type ChCfgh1 = crate::Reg<ch_cfgh1::ChCfgh1Spec>;
#[doc = ""]
pub mod ch_cfgh1;
#[doc = "SA1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sa1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa1`] module"]
#[doc(alias = "SA1")]
pub type Sa1 = crate::Reg<sa1::Sa1Spec>;
#[doc = ""]
pub mod sa1;
#[doc = "DA1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`da1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`da1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@da1`] module"]
#[doc(alias = "DA1")]
pub type Da1 = crate::Reg<da1::Da1Spec>;
#[doc = ""]
pub mod da1;
#[doc = "CH_CTRLL2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctrll2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctrll2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ctrll2`] module"]
#[doc(alias = "CH_CTRLL2")]
pub type ChCtrll2 = crate::Reg<ch_ctrll2::ChCtrll2Spec>;
#[doc = ""]
pub mod ch_ctrll2;
#[doc = "CH_CTRLH2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctrlh2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctrlh2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ctrlh2`] module"]
#[doc(alias = "CH_CTRLH2")]
pub type ChCtrlh2 = crate::Reg<ch_ctrlh2::ChCtrlh2Spec>;
#[doc = ""]
pub mod ch_ctrlh2;
#[doc = "CH_CFGL2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfgl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfgl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_cfgl2`] module"]
#[doc(alias = "CH_CFGL2")]
pub type ChCfgl2 = crate::Reg<ch_cfgl2::ChCfgl2Spec>;
#[doc = ""]
pub mod ch_cfgl2;
#[doc = "CH_CFGH2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfgh2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfgh2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_cfgh2`] module"]
#[doc(alias = "CH_CFGH2")]
pub type ChCfgh2 = crate::Reg<ch_cfgh2::ChCfgh2Spec>;
#[doc = ""]
pub mod ch_cfgh2;
#[doc = "SA2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sa2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa2`] module"]
#[doc(alias = "SA2")]
pub type Sa2 = crate::Reg<sa2::Sa2Spec>;
#[doc = ""]
pub mod sa2;
#[doc = "DA2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`da2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`da2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@da2`] module"]
#[doc(alias = "DA2")]
pub type Da2 = crate::Reg<da2::Da2Spec>;
#[doc = ""]
pub mod da2;
#[doc = "CH_CTRLL3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctrll3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctrll3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ctrll3`] module"]
#[doc(alias = "CH_CTRLL3")]
pub type ChCtrll3 = crate::Reg<ch_ctrll3::ChCtrll3Spec>;
#[doc = ""]
pub mod ch_ctrll3;
#[doc = "CH_CTRLH3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctrlh3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctrlh3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ctrlh3`] module"]
#[doc(alias = "CH_CTRLH3")]
pub type ChCtrlh3 = crate::Reg<ch_ctrlh3::ChCtrlh3Spec>;
#[doc = ""]
pub mod ch_ctrlh3;
#[doc = "CH_CFGL3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfgl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfgl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_cfgl3`] module"]
#[doc(alias = "CH_CFGL3")]
pub type ChCfgl3 = crate::Reg<ch_cfgl3::ChCfgl3Spec>;
#[doc = ""]
pub mod ch_cfgl3;
#[doc = "CH_CFGH3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfgh3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfgh3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_cfgh3`] module"]
#[doc(alias = "CH_CFGH3")]
pub type ChCfgh3 = crate::Reg<ch_cfgh3::ChCfgh3Spec>;
#[doc = ""]
pub mod ch_cfgh3;
#[doc = "SA3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sa3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa3`] module"]
#[doc(alias = "SA3")]
pub type Sa3 = crate::Reg<sa3::Sa3Spec>;
#[doc = ""]
pub mod sa3;
#[doc = "DA3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`da3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`da3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@da3`] module"]
#[doc(alias = "DA3")]
pub type Da3 = crate::Reg<da3::Da3Spec>;
#[doc = ""]
pub mod da3;
#[doc = "CH_CTRLL4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctrll4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctrll4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ctrll4`] module"]
#[doc(alias = "CH_CTRLL4")]
pub type ChCtrll4 = crate::Reg<ch_ctrll4::ChCtrll4Spec>;
#[doc = ""]
pub mod ch_ctrll4;
#[doc = "CH_CTRLH4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctrlh4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctrlh4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ctrlh4`] module"]
#[doc(alias = "CH_CTRLH4")]
pub type ChCtrlh4 = crate::Reg<ch_ctrlh4::ChCtrlh4Spec>;
#[doc = ""]
pub mod ch_ctrlh4;
#[doc = "CH_CFGL4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfgl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfgl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_cfgl4`] module"]
#[doc(alias = "CH_CFGL4")]
pub type ChCfgl4 = crate::Reg<ch_cfgl4::ChCfgl4Spec>;
#[doc = ""]
pub mod ch_cfgl4;
#[doc = "CH_CFGH4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfgh4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfgh4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_cfgh4`] module"]
#[doc(alias = "CH_CFGH4")]
pub type ChCfgh4 = crate::Reg<ch_cfgh4::ChCfgh4Spec>;
#[doc = ""]
pub mod ch_cfgh4;
#[doc = "SA4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sa4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa4`] module"]
#[doc(alias = "SA4")]
pub type Sa4 = crate::Reg<sa4::Sa4Spec>;
#[doc = ""]
pub mod sa4;
#[doc = "DA4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`da4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`da4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@da4`] module"]
#[doc(alias = "DA4")]
pub type Da4 = crate::Reg<da4::Da4Spec>;
#[doc = ""]
pub mod da4;
#[doc = "CH_CTRLL5 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctrll5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctrll5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ctrll5`] module"]
#[doc(alias = "CH_CTRLL5")]
pub type ChCtrll5 = crate::Reg<ch_ctrll5::ChCtrll5Spec>;
#[doc = ""]
pub mod ch_ctrll5;
#[doc = "CH_CTRLH5 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctrlh5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctrlh5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ctrlh5`] module"]
#[doc(alias = "CH_CTRLH5")]
pub type ChCtrlh5 = crate::Reg<ch_ctrlh5::ChCtrlh5Spec>;
#[doc = ""]
pub mod ch_ctrlh5;
#[doc = "CH_CFGL5 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfgl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfgl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_cfgl5`] module"]
#[doc(alias = "CH_CFGL5")]
pub type ChCfgl5 = crate::Reg<ch_cfgl5::ChCfgl5Spec>;
#[doc = ""]
pub mod ch_cfgl5;
#[doc = "CH_CFGH5 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ch_cfgh5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_cfgh5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_cfgh5`] module"]
#[doc(alias = "CH_CFGH5")]
pub type ChCfgh5 = crate::Reg<ch_cfgh5::ChCfgh5Spec>;
#[doc = ""]
pub mod ch_cfgh5;
#[doc = "SA5 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sa5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa5`] module"]
#[doc(alias = "SA5")]
pub type Sa5 = crate::Reg<sa5::Sa5Spec>;
#[doc = ""]
pub mod sa5;
#[doc = "DA5 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`da5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`da5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@da5`] module"]
#[doc(alias = "DA5")]
pub type Da5 = crate::Reg<da5::Da5Spec>;
#[doc = ""]
pub mod da5;
#[doc = "RAWHALFBLOCK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rawhalfblock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawhalfblock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawhalfblock`] module"]
#[doc(alias = "RAWHALFBLOCK")]
pub type Rawhalfblock = crate::Reg<rawhalfblock::RawhalfblockSpec>;
#[doc = ""]
pub mod rawhalfblock;
#[doc = "STATUSHALFBLOCK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`statushalfblock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statushalfblock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statushalfblock`] module"]
#[doc(alias = "STATUSHALFBLOCK")]
pub type Statushalfblock = crate::Reg<statushalfblock::StatushalfblockSpec>;
#[doc = ""]
pub mod statushalfblock;
#[doc = "MASKHALFBLOCK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`maskhalfblock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskhalfblock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskhalfblock`] module"]
#[doc(alias = "MASKHALFBLOCK")]
pub type Maskhalfblock = crate::Reg<maskhalfblock::MaskhalfblockSpec>;
#[doc = ""]
pub mod maskhalfblock;
#[doc = "CLEARHALFBLOCK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clearhalfblock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearhalfblock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearhalfblock`] module"]
#[doc(alias = "CLEARHALFBLOCK")]
pub type Clearhalfblock = crate::Reg<clearhalfblock::ClearhalfblockSpec>;
#[doc = ""]
pub mod clearhalfblock;
#[doc = "RAWTFR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rawtfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawtfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawtfr`] module"]
#[doc(alias = "RAWTFR")]
pub type Rawtfr = crate::Reg<rawtfr::RawtfrSpec>;
#[doc = ""]
pub mod rawtfr;
#[doc = "RAWBLOCK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rawblock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawblock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawblock`] module"]
#[doc(alias = "RAWBLOCK")]
pub type Rawblock = crate::Reg<rawblock::RawblockSpec>;
#[doc = ""]
pub mod rawblock;
#[doc = "RAWSRCTRAN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rawsrctran::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawsrctran::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawsrctran`] module"]
#[doc(alias = "RAWSRCTRAN")]
pub type Rawsrctran = crate::Reg<rawsrctran::RawsrctranSpec>;
#[doc = ""]
pub mod rawsrctran;
#[doc = "RAWDSTTRAN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rawdsttran::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawdsttran::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawdsttran`] module"]
#[doc(alias = "RAWDSTTRAN")]
pub type Rawdsttran = crate::Reg<rawdsttran::RawdsttranSpec>;
#[doc = ""]
pub mod rawdsttran;
#[doc = "RAWERR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rawerr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawerr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawerr`] module"]
#[doc(alias = "RAWERR")]
pub type Rawerr = crate::Reg<rawerr::RawerrSpec>;
#[doc = ""]
pub mod rawerr;
#[doc = "STATUSTFR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`statustfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statustfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statustfr`] module"]
#[doc(alias = "STATUSTFR")]
pub type Statustfr = crate::Reg<statustfr::StatustfrSpec>;
#[doc = ""]
pub mod statustfr;
#[doc = "STATUSBLOCK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`statusblock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusblock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusblock`] module"]
#[doc(alias = "STATUSBLOCK")]
pub type Statusblock = crate::Reg<statusblock::StatusblockSpec>;
#[doc = ""]
pub mod statusblock;
#[doc = "STATUSSRCTRAN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`statussrctran::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statussrctran::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statussrctran`] module"]
#[doc(alias = "STATUSSRCTRAN")]
pub type Statussrctran = crate::Reg<statussrctran::StatussrctranSpec>;
#[doc = ""]
pub mod statussrctran;
#[doc = "STATUSDSTTRAN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`statusdsttran::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusdsttran::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusdsttran`] module"]
#[doc(alias = "STATUSDSTTRAN")]
pub type Statusdsttran = crate::Reg<statusdsttran::StatusdsttranSpec>;
#[doc = ""]
pub mod statusdsttran;
#[doc = "STATUSERR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`statuserr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statuserr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statuserr`] module"]
#[doc(alias = "STATUSERR")]
pub type Statuserr = crate::Reg<statuserr::StatuserrSpec>;
#[doc = ""]
pub mod statuserr;
#[doc = "MASKTFR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`masktfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masktfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@masktfr`] module"]
#[doc(alias = "MASKTFR")]
pub type Masktfr = crate::Reg<masktfr::MasktfrSpec>;
#[doc = ""]
pub mod masktfr;
#[doc = "MASKBLOCK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`maskblock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskblock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskblock`] module"]
#[doc(alias = "MASKBLOCK")]
pub type Maskblock = crate::Reg<maskblock::MaskblockSpec>;
#[doc = ""]
pub mod maskblock;
#[doc = "MASKSRCTRAN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`masksrctran::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masksrctran::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@masksrctran`] module"]
#[doc(alias = "MASKSRCTRAN")]
pub type Masksrctran = crate::Reg<masksrctran::MasksrctranSpec>;
#[doc = ""]
pub mod masksrctran;
#[doc = "MASKDSTTRAN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`maskdsttran::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskdsttran::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskdsttran`] module"]
#[doc(alias = "MASKDSTTRAN")]
pub type Maskdsttran = crate::Reg<maskdsttran::MaskdsttranSpec>;
#[doc = ""]
pub mod maskdsttran;
#[doc = "MASKERR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`maskerr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskerr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskerr`] module"]
#[doc(alias = "MASKERR")]
pub type Maskerr = crate::Reg<maskerr::MaskerrSpec>;
#[doc = ""]
pub mod maskerr;
#[doc = "CLEARTFR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cleartfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cleartfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cleartfr`] module"]
#[doc(alias = "CLEARTFR")]
pub type Cleartfr = crate::Reg<cleartfr::CleartfrSpec>;
#[doc = ""]
pub mod cleartfr;
#[doc = "CLEARBLOCK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clearblock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearblock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearblock`] module"]
#[doc(alias = "CLEARBLOCK")]
pub type Clearblock = crate::Reg<clearblock::ClearblockSpec>;
#[doc = ""]
pub mod clearblock;
#[doc = "CLEARSRCTRAN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clearsrctran::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearsrctran::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearsrctran`] module"]
#[doc(alias = "CLEARSRCTRAN")]
pub type Clearsrctran = crate::Reg<clearsrctran::ClearsrctranSpec>;
#[doc = ""]
pub mod clearsrctran;
#[doc = "CLEARDSTTRAN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cleardsttran::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cleardsttran::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cleardsttran`] module"]
#[doc(alias = "CLEARDSTTRAN")]
pub type Cleardsttran = crate::Reg<cleardsttran::CleardsttranSpec>;
#[doc = ""]
pub mod cleardsttran;
#[doc = "CLEARERR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clearerr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearerr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearerr`] module"]
#[doc(alias = "CLEARERR")]
pub type Clearerr = crate::Reg<clearerr::ClearerrSpec>;
#[doc = ""]
pub mod clearerr;
#[doc = "STATUSINT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`statusint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusint`] module"]
#[doc(alias = "STATUSINT")]
pub type Statusint = crate::Reg<statusint::StatusintSpec>;
#[doc = ""]
pub mod statusint;
#[doc = "REQSRC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`reqsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqsrc`] module"]
#[doc(alias = "REQSRC")]
pub type Reqsrc = crate::Reg<reqsrc::ReqsrcSpec>;
#[doc = ""]
pub mod reqsrc;
#[doc = "REQDST (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`reqdst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqdst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqdst`] module"]
#[doc(alias = "REQDST")]
pub type Reqdst = crate::Reg<reqdst::ReqdstSpec>;
#[doc = ""]
pub mod reqdst;
#[doc = "SGLRQSRC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sglrqsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sglrqsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sglrqsrc`] module"]
#[doc(alias = "SGLRQSRC")]
pub type Sglrqsrc = crate::Reg<sglrqsrc::SglrqsrcSpec>;
#[doc = ""]
pub mod sglrqsrc;
#[doc = "SGLRQDST (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sglrqdst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sglrqdst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sglrqdst`] module"]
#[doc(alias = "SGLRQDST")]
pub type Sglrqdst = crate::Reg<sglrqdst::SglrqdstSpec>;
#[doc = ""]
pub mod sglrqdst;
