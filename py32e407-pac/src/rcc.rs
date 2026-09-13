#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    cfgr: Cfgr,
    cir: Cir,
    _reserved3: [u8; 0x1c],
    ahb1rstr: Ahb1rstr,
    ahb2rstr: Ahb2rstr,
    _reserved5: [u8; 0x08],
    apb1rstr1: Apb1rstr1,
    apb1rstr2: Apb1rstr2,
    apb2rstr: Apb2rstr,
    _reserved8: [u8; 0x04],
    ahb1enr: Ahb1enr,
    ahb2enr: Ahb2enr,
    _reserved10: [u8; 0x08],
    apb1enr1: Apb1enr1,
    apb1enr2: Apb1enr2,
    apb2enr: Apb2enr,
    _reserved13: [u8; 0x04],
    ccipr: Ccipr,
    _reserved14: [u8; 0x04],
    bdcr: Bdcr,
    csr: Csr,
    cfgr1: Cfgr1,
    cfgr2: Cfgr2,
    cfgr3: Cfgr3,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn cir(&self) -> &Cir {
        &self.cir
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn ahb1rstr(&self) -> &Ahb1rstr {
        &self.ahb1rstr
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn ahb2rstr(&self) -> &Ahb2rstr {
        &self.ahb2rstr
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn apb1rstr1(&self) -> &Apb1rstr1 {
        &self.apb1rstr1
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn apb1rstr2(&self) -> &Apb1rstr2 {
        &self.apb1rstr2
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &Apb2rstr {
        &self.apb2rstr
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &Ahb1enr {
        &self.ahb1enr
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn ahb2enr(&self) -> &Ahb2enr {
        &self.ahb2enr
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn apb1enr1(&self) -> &Apb1enr1 {
        &self.apb1enr1
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn apb1enr2(&self) -> &Apb1enr2 {
        &self.apb1enr2
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn apb2enr(&self) -> &Apb2enr {
        &self.apb2enr
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn ccipr(&self) -> &Ccipr {
        &self.ccipr
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn bdcr(&self) -> &Bdcr {
        &self.bdcr
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &Cfgr1 {
        &self.cfgr1
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &Cfgr2 {
        &self.cfgr2
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn cfgr3(&self) -> &Cfgr3 {
        &self.cfgr3
    }
}
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
#[doc = "CIR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir`] module"]
#[doc(alias = "CIR")]
pub type Cir = crate::Reg<cir::CirSpec>;
#[doc = ""]
pub mod cir;
#[doc = "AHB1RSTR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1rstr`] module"]
#[doc(alias = "AHB1RSTR")]
pub type Ahb1rstr = crate::Reg<ahb1rstr::Ahb1rstrSpec>;
#[doc = ""]
pub mod ahb1rstr;
#[doc = "AHB2RSTR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2rstr`] module"]
#[doc(alias = "AHB2RSTR")]
pub type Ahb2rstr = crate::Reg<ahb2rstr::Ahb2rstrSpec>;
#[doc = ""]
pub mod ahb2rstr;
#[doc = "APB1RSTR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rstr1`] module"]
#[doc(alias = "APB1RSTR1")]
pub type Apb1rstr1 = crate::Reg<apb1rstr1::Apb1rstr1Spec>;
#[doc = ""]
pub mod apb1rstr1;
#[doc = "APB1RSTR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rstr2`] module"]
#[doc(alias = "APB1RSTR2")]
pub type Apb1rstr2 = crate::Reg<apb1rstr2::Apb1rstr2Spec>;
#[doc = ""]
pub mod apb1rstr2;
#[doc = "APB2RSTR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rstr`] module"]
#[doc(alias = "APB2RSTR")]
pub type Apb2rstr = crate::Reg<apb2rstr::Apb2rstrSpec>;
#[doc = ""]
pub mod apb2rstr;
#[doc = "AHB1ENR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1enr`] module"]
#[doc(alias = "AHB1ENR")]
pub type Ahb1enr = crate::Reg<ahb1enr::Ahb1enrSpec>;
#[doc = ""]
pub mod ahb1enr;
#[doc = "AHB2ENR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2enr`] module"]
#[doc(alias = "AHB2ENR")]
pub type Ahb2enr = crate::Reg<ahb2enr::Ahb2enrSpec>;
#[doc = ""]
pub mod ahb2enr;
#[doc = "APB1ENR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`apb1enr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1enr1`] module"]
#[doc(alias = "APB1ENR1")]
pub type Apb1enr1 = crate::Reg<apb1enr1::Apb1enr1Spec>;
#[doc = ""]
pub mod apb1enr1;
#[doc = "APB1ENR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`apb1enr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1enr2`] module"]
#[doc(alias = "APB1ENR2")]
pub type Apb1enr2 = crate::Reg<apb1enr2::Apb1enr2Spec>;
#[doc = ""]
pub mod apb1enr2;
#[doc = "APB2ENR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2enr`] module"]
#[doc(alias = "APB2ENR")]
pub type Apb2enr = crate::Reg<apb2enr::Apb2enrSpec>;
#[doc = ""]
pub mod apb2enr;
#[doc = "CCIPR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr`] module"]
#[doc(alias = "CCIPR")]
pub type Ccipr = crate::Reg<ccipr::CciprSpec>;
#[doc = ""]
pub mod ccipr;
#[doc = "BDCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`] module"]
#[doc(alias = "BDCR")]
pub type Bdcr = crate::Reg<bdcr::BdcrSpec>;
#[doc = ""]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = ""]
pub mod csr;
#[doc = "CFGR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`] module"]
#[doc(alias = "CFGR1")]
pub type Cfgr1 = crate::Reg<cfgr1::Cfgr1Spec>;
#[doc = ""]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`] module"]
#[doc(alias = "CFGR2")]
pub type Cfgr2 = crate::Reg<cfgr2::Cfgr2Spec>;
#[doc = ""]
pub mod cfgr2;
#[doc = "CFGR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr3`] module"]
#[doc(alias = "CFGR3")]
pub type Cfgr3 = crate::Reg<cfgr3::Cfgr3Spec>;
#[doc = ""]
pub mod cfgr3;
