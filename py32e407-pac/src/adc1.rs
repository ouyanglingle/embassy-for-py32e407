#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: Isr,
    ier: Ier,
    cr: Cr,
    cfgr: Cfgr,
    cfgr2: Cfgr2,
    smpr1: Smpr1,
    smpr2: Smpr2,
    _reserved7: [u8; 0x04],
    tr1: Tr1,
    tr2: Tr2,
    tr3: Tr3,
    _reserved10: [u8; 0x04],
    sqr1: Sqr1,
    sqr2: Sqr2,
    sqr3: Sqr3,
    sqr4: Sqr4,
    dr: Dr,
    _reserved15: [u8; 0x08],
    jsqr: Jsqr,
    _reserved16: [u8; 0x10],
    ofr1: Ofr1,
    ofr2: Ofr2,
    ofr3: Ofr3,
    ofr4: Ofr4,
    _reserved20: [u8; 0x10],
    jdr1: Jdr1,
    jdr2: Jdr2,
    jdr3: Jdr3,
    jdr4: Jdr4,
    _reserved24: [u8; 0x10],
    awd2cr: Awd2cr,
    awd3cr: Awd3cr,
    _reserved26: [u8; 0x08],
    difsel: Difsel,
    calfact: Calfact,
    _reserved28: [u8; 0x08],
    gcomp: Gcomp,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &Cfgr2 {
        &self.cfgr2
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn smpr1(&self) -> &Smpr1 {
        &self.smpr1
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn smpr2(&self) -> &Smpr2 {
        &self.smpr2
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn tr1(&self) -> &Tr1 {
        &self.tr1
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn tr2(&self) -> &Tr2 {
        &self.tr2
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn tr3(&self) -> &Tr3 {
        &self.tr3
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn sqr1(&self) -> &Sqr1 {
        &self.sqr1
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn sqr2(&self) -> &Sqr2 {
        &self.sqr2
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn sqr3(&self) -> &Sqr3 {
        &self.sqr3
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn sqr4(&self) -> &Sqr4 {
        &self.sqr4
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn jsqr(&self) -> &Jsqr {
        &self.jsqr
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn ofr1(&self) -> &Ofr1 {
        &self.ofr1
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn ofr2(&self) -> &Ofr2 {
        &self.ofr2
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn ofr3(&self) -> &Ofr3 {
        &self.ofr3
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn ofr4(&self) -> &Ofr4 {
        &self.ofr4
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn jdr1(&self) -> &Jdr1 {
        &self.jdr1
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn jdr2(&self) -> &Jdr2 {
        &self.jdr2
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn jdr3(&self) -> &Jdr3 {
        &self.jdr3
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn jdr4(&self) -> &Jdr4 {
        &self.jdr4
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn awd2cr(&self) -> &Awd2cr {
        &self.awd2cr
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn awd3cr(&self) -> &Awd3cr {
        &self.awd3cr
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn difsel(&self) -> &Difsel {
        &self.difsel
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn calfact(&self) -> &Calfact {
        &self.calfact
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn gcomp(&self) -> &Gcomp {
        &self.gcomp
    }
}
#[doc = "ISR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = ""]
pub mod isr;
#[doc = "IER (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = ""]
pub mod ier;
#[doc = "CR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = ""]
pub mod cr;
#[doc = "CFGR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = ""]
pub mod cfgr;
#[doc = "CFGR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`] module"]
#[doc(alias = "CFGR2")]
pub type Cfgr2 = crate::Reg<cfgr2::Cfgr2Spec>;
#[doc = ""]
pub mod cfgr2;
#[doc = "SMPR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr1`] module"]
#[doc(alias = "SMPR1")]
pub type Smpr1 = crate::Reg<smpr1::Smpr1Spec>;
#[doc = ""]
pub mod smpr1;
#[doc = "SMPR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr2`] module"]
#[doc(alias = "SMPR2")]
pub type Smpr2 = crate::Reg<smpr2::Smpr2Spec>;
#[doc = ""]
pub mod smpr2;
#[doc = "TR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr1`] module"]
#[doc(alias = "TR1")]
pub type Tr1 = crate::Reg<tr1::Tr1Spec>;
#[doc = ""]
pub mod tr1;
#[doc = "TR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr2`] module"]
#[doc(alias = "TR2")]
pub type Tr2 = crate::Reg<tr2::Tr2Spec>;
#[doc = ""]
pub mod tr2;
#[doc = "TR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr3`] module"]
#[doc(alias = "TR3")]
pub type Tr3 = crate::Reg<tr3::Tr3Spec>;
#[doc = ""]
pub mod tr3;
#[doc = "SQR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sqr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr1`] module"]
#[doc(alias = "SQR1")]
pub type Sqr1 = crate::Reg<sqr1::Sqr1Spec>;
#[doc = ""]
pub mod sqr1;
#[doc = "SQR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sqr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr2`] module"]
#[doc(alias = "SQR2")]
pub type Sqr2 = crate::Reg<sqr2::Sqr2Spec>;
#[doc = ""]
pub mod sqr2;
#[doc = "SQR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sqr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr3`] module"]
#[doc(alias = "SQR3")]
pub type Sqr3 = crate::Reg<sqr3::Sqr3Spec>;
#[doc = ""]
pub mod sqr3;
#[doc = "SQR4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sqr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr4`] module"]
#[doc(alias = "SQR4")]
pub type Sqr4 = crate::Reg<sqr4::Sqr4Spec>;
#[doc = ""]
pub mod sqr4;
#[doc = "DR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = ""]
pub mod dr;
#[doc = "JSQR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`jsqr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jsqr`] module"]
#[doc(alias = "JSQR")]
pub type Jsqr = crate::Reg<jsqr::JsqrSpec>;
#[doc = ""]
pub mod jsqr;
#[doc = "OFR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ofr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr1`] module"]
#[doc(alias = "OFR1")]
pub type Ofr1 = crate::Reg<ofr1::Ofr1Spec>;
#[doc = ""]
pub mod ofr1;
#[doc = "OFR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ofr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr2`] module"]
#[doc(alias = "OFR2")]
pub type Ofr2 = crate::Reg<ofr2::Ofr2Spec>;
#[doc = ""]
pub mod ofr2;
#[doc = "OFR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ofr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr3`] module"]
#[doc(alias = "OFR3")]
pub type Ofr3 = crate::Reg<ofr3::Ofr3Spec>;
#[doc = ""]
pub mod ofr3;
#[doc = "OFR4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ofr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr4`] module"]
#[doc(alias = "OFR4")]
pub type Ofr4 = crate::Reg<ofr4::Ofr4Spec>;
#[doc = ""]
pub mod ofr4;
#[doc = "JDR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`jdr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr1`] module"]
#[doc(alias = "JDR1")]
pub type Jdr1 = crate::Reg<jdr1::Jdr1Spec>;
#[doc = ""]
pub mod jdr1;
#[doc = "JDR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`jdr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr2`] module"]
#[doc(alias = "JDR2")]
pub type Jdr2 = crate::Reg<jdr2::Jdr2Spec>;
#[doc = ""]
pub mod jdr2;
#[doc = "JDR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`jdr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr3`] module"]
#[doc(alias = "JDR3")]
pub type Jdr3 = crate::Reg<jdr3::Jdr3Spec>;
#[doc = ""]
pub mod jdr3;
#[doc = "JDR4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`jdr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr4`] module"]
#[doc(alias = "JDR4")]
pub type Jdr4 = crate::Reg<jdr4::Jdr4Spec>;
#[doc = ""]
pub mod jdr4;
#[doc = "AWD2CR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`awd2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd2cr`] module"]
#[doc(alias = "AWD2CR")]
pub type Awd2cr = crate::Reg<awd2cr::Awd2crSpec>;
#[doc = ""]
pub mod awd2cr;
#[doc = "AWD3CR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`awd3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd3cr`] module"]
#[doc(alias = "AWD3CR")]
pub type Awd3cr = crate::Reg<awd3cr::Awd3crSpec>;
#[doc = ""]
pub mod awd3cr;
#[doc = "DIFSEL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`difsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@difsel`] module"]
#[doc(alias = "DIFSEL")]
pub type Difsel = crate::Reg<difsel::DifselSpec>;
#[doc = ""]
pub mod difsel;
#[doc = "CALFACT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`calfact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calfact`] module"]
#[doc(alias = "CALFACT")]
pub type Calfact = crate::Reg<calfact::CalfactSpec>;
#[doc = ""]
pub mod calfact;
#[doc = "GCOMP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gcomp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcomp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcomp`] module"]
#[doc(alias = "GCOMP")]
pub type Gcomp = crate::Reg<gcomp::GcompSpec>;
#[doc = ""]
pub mod gcomp;
