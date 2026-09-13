#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    csr: Csr,
    rtccr: Rtccr,
    _reserved3: [u8; 0x74],
    dr0: Dr0,
    dr1: Dr1,
    dr2: Dr2,
    dr3: Dr3,
    dr4: Dr4,
    dr5: Dr5,
    dr6: Dr6,
    dr7: Dr7,
    dr8: Dr8,
    dr9: Dr9,
    dr10: Dr10,
    dr11: Dr11,
    dr12: Dr12,
    dr13: Dr13,
    dr14: Dr14,
    dr15: Dr15,
    dr16: Dr16,
    dr17: Dr17,
    dr18: Dr18,
    dr19: Dr19,
    dr20: Dr20,
    dr21: Dr21,
    dr22: Dr22,
    dr23: Dr23,
    dr24: Dr24,
    dr25: Dr25,
    dr26: Dr26,
    dr27: Dr27,
    dr28: Dr28,
    dr29: Dr29,
    dr30: Dr30,
    dr31: Dr31,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn rtccr(&self) -> &Rtccr {
        &self.rtccr
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn dr0(&self) -> &Dr0 {
        &self.dr0
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn dr1(&self) -> &Dr1 {
        &self.dr1
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn dr2(&self) -> &Dr2 {
        &self.dr2
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn dr3(&self) -> &Dr3 {
        &self.dr3
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn dr4(&self) -> &Dr4 {
        &self.dr4
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn dr5(&self) -> &Dr5 {
        &self.dr5
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn dr6(&self) -> &Dr6 {
        &self.dr6
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn dr7(&self) -> &Dr7 {
        &self.dr7
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn dr8(&self) -> &Dr8 {
        &self.dr8
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn dr9(&self) -> &Dr9 {
        &self.dr9
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn dr10(&self) -> &Dr10 {
        &self.dr10
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn dr11(&self) -> &Dr11 {
        &self.dr11
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn dr12(&self) -> &Dr12 {
        &self.dr12
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn dr13(&self) -> &Dr13 {
        &self.dr13
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn dr14(&self) -> &Dr14 {
        &self.dr14
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn dr15(&self) -> &Dr15 {
        &self.dr15
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn dr16(&self) -> &Dr16 {
        &self.dr16
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn dr17(&self) -> &Dr17 {
        &self.dr17
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn dr18(&self) -> &Dr18 {
        &self.dr18
    }
    #[doc = "0xcc - "]
    #[inline(always)]
    pub const fn dr19(&self) -> &Dr19 {
        &self.dr19
    }
    #[doc = "0xd0 - "]
    #[inline(always)]
    pub const fn dr20(&self) -> &Dr20 {
        &self.dr20
    }
    #[doc = "0xd4 - "]
    #[inline(always)]
    pub const fn dr21(&self) -> &Dr21 {
        &self.dr21
    }
    #[doc = "0xd8 - "]
    #[inline(always)]
    pub const fn dr22(&self) -> &Dr22 {
        &self.dr22
    }
    #[doc = "0xdc - "]
    #[inline(always)]
    pub const fn dr23(&self) -> &Dr23 {
        &self.dr23
    }
    #[doc = "0xe0 - "]
    #[inline(always)]
    pub const fn dr24(&self) -> &Dr24 {
        &self.dr24
    }
    #[doc = "0xe4 - "]
    #[inline(always)]
    pub const fn dr25(&self) -> &Dr25 {
        &self.dr25
    }
    #[doc = "0xe8 - "]
    #[inline(always)]
    pub const fn dr26(&self) -> &Dr26 {
        &self.dr26
    }
    #[doc = "0xec - "]
    #[inline(always)]
    pub const fn dr27(&self) -> &Dr27 {
        &self.dr27
    }
    #[doc = "0xf0 - "]
    #[inline(always)]
    pub const fn dr28(&self) -> &Dr28 {
        &self.dr28
    }
    #[doc = "0xf4 - "]
    #[inline(always)]
    pub const fn dr29(&self) -> &Dr29 {
        &self.dr29
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn dr30(&self) -> &Dr30 {
        &self.dr30
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn dr31(&self) -> &Dr31 {
        &self.dr31
    }
}
#[doc = "CR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = ""]
pub mod cr;
#[doc = "CSR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = ""]
pub mod csr;
#[doc = "RTCCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rtccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccr`] module"]
#[doc(alias = "RTCCR")]
pub type Rtccr = crate::Reg<rtccr::RtccrSpec>;
#[doc = ""]
pub mod rtccr;
#[doc = "DR0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr0`] module"]
#[doc(alias = "DR0")]
pub type Dr0 = crate::Reg<dr0::Dr0Spec>;
#[doc = ""]
pub mod dr0;
#[doc = "DR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr1`] module"]
#[doc(alias = "DR1")]
pub type Dr1 = crate::Reg<dr1::Dr1Spec>;
#[doc = ""]
pub mod dr1;
#[doc = "DR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr2`] module"]
#[doc(alias = "DR2")]
pub type Dr2 = crate::Reg<dr2::Dr2Spec>;
#[doc = ""]
pub mod dr2;
#[doc = "DR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr3`] module"]
#[doc(alias = "DR3")]
pub type Dr3 = crate::Reg<dr3::Dr3Spec>;
#[doc = ""]
pub mod dr3;
#[doc = "DR4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr4`] module"]
#[doc(alias = "DR4")]
pub type Dr4 = crate::Reg<dr4::Dr4Spec>;
#[doc = ""]
pub mod dr4;
#[doc = "DR5 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr5`] module"]
#[doc(alias = "DR5")]
pub type Dr5 = crate::Reg<dr5::Dr5Spec>;
#[doc = ""]
pub mod dr5;
#[doc = "DR6 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr6`] module"]
#[doc(alias = "DR6")]
pub type Dr6 = crate::Reg<dr6::Dr6Spec>;
#[doc = ""]
pub mod dr6;
#[doc = "DR7 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr7`] module"]
#[doc(alias = "DR7")]
pub type Dr7 = crate::Reg<dr7::Dr7Spec>;
#[doc = ""]
pub mod dr7;
#[doc = "DR8 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr8`] module"]
#[doc(alias = "DR8")]
pub type Dr8 = crate::Reg<dr8::Dr8Spec>;
#[doc = ""]
pub mod dr8;
#[doc = "DR9 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr9`] module"]
#[doc(alias = "DR9")]
pub type Dr9 = crate::Reg<dr9::Dr9Spec>;
#[doc = ""]
pub mod dr9;
#[doc = "DR10 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr10`] module"]
#[doc(alias = "DR10")]
pub type Dr10 = crate::Reg<dr10::Dr10Spec>;
#[doc = ""]
pub mod dr10;
#[doc = "DR11 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr11`] module"]
#[doc(alias = "DR11")]
pub type Dr11 = crate::Reg<dr11::Dr11Spec>;
#[doc = ""]
pub mod dr11;
#[doc = "DR12 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr12`] module"]
#[doc(alias = "DR12")]
pub type Dr12 = crate::Reg<dr12::Dr12Spec>;
#[doc = ""]
pub mod dr12;
#[doc = "DR13 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr13`] module"]
#[doc(alias = "DR13")]
pub type Dr13 = crate::Reg<dr13::Dr13Spec>;
#[doc = ""]
pub mod dr13;
#[doc = "DR14 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr14`] module"]
#[doc(alias = "DR14")]
pub type Dr14 = crate::Reg<dr14::Dr14Spec>;
#[doc = ""]
pub mod dr14;
#[doc = "DR15 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr15`] module"]
#[doc(alias = "DR15")]
pub type Dr15 = crate::Reg<dr15::Dr15Spec>;
#[doc = ""]
pub mod dr15;
#[doc = "DR16 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr16`] module"]
#[doc(alias = "DR16")]
pub type Dr16 = crate::Reg<dr16::Dr16Spec>;
#[doc = ""]
pub mod dr16;
#[doc = "DR17 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr17`] module"]
#[doc(alias = "DR17")]
pub type Dr17 = crate::Reg<dr17::Dr17Spec>;
#[doc = ""]
pub mod dr17;
#[doc = "DR18 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr18`] module"]
#[doc(alias = "DR18")]
pub type Dr18 = crate::Reg<dr18::Dr18Spec>;
#[doc = ""]
pub mod dr18;
#[doc = "DR19 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr19`] module"]
#[doc(alias = "DR19")]
pub type Dr19 = crate::Reg<dr19::Dr19Spec>;
#[doc = ""]
pub mod dr19;
#[doc = "DR20 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr20`] module"]
#[doc(alias = "DR20")]
pub type Dr20 = crate::Reg<dr20::Dr20Spec>;
#[doc = ""]
pub mod dr20;
#[doc = "DR21 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr21`] module"]
#[doc(alias = "DR21")]
pub type Dr21 = crate::Reg<dr21::Dr21Spec>;
#[doc = ""]
pub mod dr21;
#[doc = "DR22 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr22`] module"]
#[doc(alias = "DR22")]
pub type Dr22 = crate::Reg<dr22::Dr22Spec>;
#[doc = ""]
pub mod dr22;
#[doc = "DR23 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr23`] module"]
#[doc(alias = "DR23")]
pub type Dr23 = crate::Reg<dr23::Dr23Spec>;
#[doc = ""]
pub mod dr23;
#[doc = "DR24 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr24`] module"]
#[doc(alias = "DR24")]
pub type Dr24 = crate::Reg<dr24::Dr24Spec>;
#[doc = ""]
pub mod dr24;
#[doc = "DR25 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr25`] module"]
#[doc(alias = "DR25")]
pub type Dr25 = crate::Reg<dr25::Dr25Spec>;
#[doc = ""]
pub mod dr25;
#[doc = "DR26 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr26`] module"]
#[doc(alias = "DR26")]
pub type Dr26 = crate::Reg<dr26::Dr26Spec>;
#[doc = ""]
pub mod dr26;
#[doc = "DR27 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr27`] module"]
#[doc(alias = "DR27")]
pub type Dr27 = crate::Reg<dr27::Dr27Spec>;
#[doc = ""]
pub mod dr27;
#[doc = "DR28 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr28`] module"]
#[doc(alias = "DR28")]
pub type Dr28 = crate::Reg<dr28::Dr28Spec>;
#[doc = ""]
pub mod dr28;
#[doc = "DR29 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr29`] module"]
#[doc(alias = "DR29")]
pub type Dr29 = crate::Reg<dr29::Dr29Spec>;
#[doc = ""]
pub mod dr29;
#[doc = "DR30 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr30`] module"]
#[doc(alias = "DR30")]
pub type Dr30 = crate::Reg<dr30::Dr30Spec>;
#[doc = ""]
pub mod dr30;
#[doc = "DR31 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr31`] module"]
#[doc(alias = "DR31")]
pub type Dr31 = crate::Reg<dr31::Dr31Spec>;
#[doc = ""]
pub mod dr31;
