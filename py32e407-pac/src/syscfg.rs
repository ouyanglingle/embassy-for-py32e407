#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfgr1: Cfgr1,
    cfgr2: Cfgr2,
    cfgr3: Cfgr3,
    cfgr4: Cfgr4,
    cfgr5: Cfgr5,
    exticr1: Exticr1,
    exticr2: Exticr2,
    exticr3: Exticr3,
    exticr4: Exticr4,
    pa_ens: PaEns,
    pb_ens: PbEns,
    pc_ens: PcEns,
    pd_ens: PdEns,
    pe_ens: PeEns,
    pf_ens: PfEns,
    _reserved15: [u8; 0x04],
    pa_ana2enr: PaAna2enr,
    pb_ana2enr: PbAna2enr,
    pc_ana2enr: PcAna2enr,
    pd_ana2enr: PdAna2enr,
    pe_ana2enr: PeAna2enr,
    pf_ana2enr: PfAna2enr,
    _reserved21: [u8; 0x18],
    scsr: Scsr,
    swpr: Swpr,
    skr: Skr,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &Cfgr1 {
        &self.cfgr1
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &Cfgr2 {
        &self.cfgr2
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn cfgr3(&self) -> &Cfgr3 {
        &self.cfgr3
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn cfgr4(&self) -> &Cfgr4 {
        &self.cfgr4
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn cfgr5(&self) -> &Cfgr5 {
        &self.cfgr5
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn exticr1(&self) -> &Exticr1 {
        &self.exticr1
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn exticr2(&self) -> &Exticr2 {
        &self.exticr2
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn exticr3(&self) -> &Exticr3 {
        &self.exticr3
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn exticr4(&self) -> &Exticr4 {
        &self.exticr4
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn pa_ens(&self) -> &PaEns {
        &self.pa_ens
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn pb_ens(&self) -> &PbEns {
        &self.pb_ens
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn pc_ens(&self) -> &PcEns {
        &self.pc_ens
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn pd_ens(&self) -> &PdEns {
        &self.pd_ens
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn pe_ens(&self) -> &PeEns {
        &self.pe_ens
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn pf_ens(&self) -> &PfEns {
        &self.pf_ens
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn pa_ana2enr(&self) -> &PaAna2enr {
        &self.pa_ana2enr
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn pb_ana2enr(&self) -> &PbAna2enr {
        &self.pb_ana2enr
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn pc_ana2enr(&self) -> &PcAna2enr {
        &self.pc_ana2enr
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn pd_ana2enr(&self) -> &PdAna2enr {
        &self.pd_ana2enr
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn pe_ana2enr(&self) -> &PeAna2enr {
        &self.pe_ana2enr
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn pf_ana2enr(&self) -> &PfAna2enr {
        &self.pf_ana2enr
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn scsr(&self) -> &Scsr {
        &self.scsr
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn swpr(&self) -> &Swpr {
        &self.swpr
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn skr(&self) -> &Skr {
        &self.skr
    }
}
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
#[doc = "CFGR4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr4`] module"]
#[doc(alias = "CFGR4")]
pub type Cfgr4 = crate::Reg<cfgr4::Cfgr4Spec>;
#[doc = ""]
pub mod cfgr4;
#[doc = "CFGR5 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr5`] module"]
#[doc(alias = "CFGR5")]
pub type Cfgr5 = crate::Reg<cfgr5::Cfgr5Spec>;
#[doc = ""]
pub mod cfgr5;
#[doc = "EXTICR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`exticr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr1`] module"]
#[doc(alias = "EXTICR1")]
pub type Exticr1 = crate::Reg<exticr1::Exticr1Spec>;
#[doc = ""]
pub mod exticr1;
#[doc = "EXTICR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`exticr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr2`] module"]
#[doc(alias = "EXTICR2")]
pub type Exticr2 = crate::Reg<exticr2::Exticr2Spec>;
#[doc = ""]
pub mod exticr2;
#[doc = "EXTICR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`exticr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr3`] module"]
#[doc(alias = "EXTICR3")]
pub type Exticr3 = crate::Reg<exticr3::Exticr3Spec>;
#[doc = ""]
pub mod exticr3;
#[doc = "EXTICR4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`exticr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr4`] module"]
#[doc(alias = "EXTICR4")]
pub type Exticr4 = crate::Reg<exticr4::Exticr4Spec>;
#[doc = ""]
pub mod exticr4;
#[doc = "PA_ENS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pa_ens::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_ens::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_ens`] module"]
#[doc(alias = "PA_ENS")]
pub type PaEns = crate::Reg<pa_ens::PaEnsSpec>;
#[doc = ""]
pub mod pa_ens;
#[doc = "PB_ENS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pb_ens::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_ens::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_ens`] module"]
#[doc(alias = "PB_ENS")]
pub type PbEns = crate::Reg<pb_ens::PbEnsSpec>;
#[doc = ""]
pub mod pb_ens;
#[doc = "PC_ENS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pc_ens::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_ens::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_ens`] module"]
#[doc(alias = "PC_ENS")]
pub type PcEns = crate::Reg<pc_ens::PcEnsSpec>;
#[doc = ""]
pub mod pc_ens;
#[doc = "PD_ENS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pd_ens::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_ens::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_ens`] module"]
#[doc(alias = "PD_ENS")]
pub type PdEns = crate::Reg<pd_ens::PdEnsSpec>;
#[doc = ""]
pub mod pd_ens;
#[doc = "PE_ENS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pe_ens::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_ens::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_ens`] module"]
#[doc(alias = "PE_ENS")]
pub type PeEns = crate::Reg<pe_ens::PeEnsSpec>;
#[doc = ""]
pub mod pe_ens;
#[doc = "PF_ENS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pf_ens::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_ens::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_ens`] module"]
#[doc(alias = "PF_ENS")]
pub type PfEns = crate::Reg<pf_ens::PfEnsSpec>;
#[doc = ""]
pub mod pf_ens;
#[doc = "PA_ANA2ENR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pa_ana2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_ana2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_ana2enr`] module"]
#[doc(alias = "PA_ANA2ENR")]
pub type PaAna2enr = crate::Reg<pa_ana2enr::PaAna2enrSpec>;
#[doc = ""]
pub mod pa_ana2enr;
#[doc = "PB_ANA2ENR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pb_ana2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_ana2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_ana2enr`] module"]
#[doc(alias = "PB_ANA2ENR")]
pub type PbAna2enr = crate::Reg<pb_ana2enr::PbAna2enrSpec>;
#[doc = ""]
pub mod pb_ana2enr;
#[doc = "PC_ANA2ENR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pc_ana2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_ana2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_ana2enr`] module"]
#[doc(alias = "PC_ANA2ENR")]
pub type PcAna2enr = crate::Reg<pc_ana2enr::PcAna2enrSpec>;
#[doc = ""]
pub mod pc_ana2enr;
#[doc = "PD_ANA2ENR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pd_ana2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_ana2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_ana2enr`] module"]
#[doc(alias = "PD_ANA2ENR")]
pub type PdAna2enr = crate::Reg<pd_ana2enr::PdAna2enrSpec>;
#[doc = ""]
pub mod pd_ana2enr;
#[doc = "PE_ANA2ENR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pe_ana2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_ana2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_ana2enr`] module"]
#[doc(alias = "PE_ANA2ENR")]
pub type PeAna2enr = crate::Reg<pe_ana2enr::PeAna2enrSpec>;
#[doc = ""]
pub mod pe_ana2enr;
#[doc = "PF_ANA2ENR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pf_ana2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_ana2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_ana2enr`] module"]
#[doc(alias = "PF_ANA2ENR")]
pub type PfAna2enr = crate::Reg<pf_ana2enr::PfAna2enrSpec>;
#[doc = ""]
pub mod pf_ana2enr;
#[doc = "SCSR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`scsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsr`] module"]
#[doc(alias = "SCSR")]
pub type Scsr = crate::Reg<scsr::ScsrSpec>;
#[doc = ""]
pub mod scsr;
#[doc = "SWPR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`swpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swpr`] module"]
#[doc(alias = "SWPR")]
pub type Swpr = crate::Reg<swpr::SwprSpec>;
#[doc = ""]
pub mod swpr;
#[doc = "SKR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`skr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`skr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@skr`] module"]
#[doc(alias = "SKR")]
pub type Skr = crate::Reg<skr::SkrSpec>;
#[doc = ""]
pub mod skr;
