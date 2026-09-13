#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    acr: Acr,
    _reserved1: [u8; 0x04],
    keyr: Keyr,
    optkeyr: Optkeyr,
    sr: Sr,
    cr: Cr,
    eccr: Eccr,
    _reserved6: [u8; 0x04],
    optr1: Optr1,
    optr2: Optr2,
    _reserved8: [u8; 0x04],
    wrpr: Wrpr,
    pcrop0sr: Pcrop0sr,
    pcrop0er: Pcrop0er,
    pcrop1sr: Pcrop1sr,
    pcrop1er: Pcrop1er,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn keyr(&self) -> &Keyr {
        &self.keyr
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn optkeyr(&self) -> &Optkeyr {
        &self.optkeyr
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn eccr(&self) -> &Eccr {
        &self.eccr
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn optr1(&self) -> &Optr1 {
        &self.optr1
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn optr2(&self) -> &Optr2 {
        &self.optr2
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn wrpr(&self) -> &Wrpr {
        &self.wrpr
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn pcrop0sr(&self) -> &Pcrop0sr {
        &self.pcrop0sr
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn pcrop0er(&self) -> &Pcrop0er {
        &self.pcrop0er
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn pcrop1sr(&self) -> &Pcrop1sr {
        &self.pcrop1sr
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn pcrop1er(&self) -> &Pcrop1er {
        &self.pcrop1er
    }
}
#[doc = "ACR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`] module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = ""]
pub mod acr;
#[doc = "KEYR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`keyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr`] module"]
#[doc(alias = "KEYR")]
pub type Keyr = crate::Reg<keyr::KeyrSpec>;
#[doc = ""]
pub mod keyr;
#[doc = "OPTKEYR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`optkeyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr`] module"]
#[doc(alias = "OPTKEYR")]
pub type Optkeyr = crate::Reg<optkeyr::OptkeyrSpec>;
#[doc = ""]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = ""]
pub mod sr;
#[doc = "CR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = ""]
pub mod cr;
#[doc = "ECCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`eccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccr`] module"]
#[doc(alias = "ECCR")]
pub type Eccr = crate::Reg<eccr::EccrSpec>;
#[doc = ""]
pub mod eccr;
#[doc = "OPTR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`optr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optr1`] module"]
#[doc(alias = "OPTR1")]
pub type Optr1 = crate::Reg<optr1::Optr1Spec>;
#[doc = ""]
pub mod optr1;
#[doc = "OPTR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`optr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optr2`] module"]
#[doc(alias = "OPTR2")]
pub type Optr2 = crate::Reg<optr2::Optr2Spec>;
#[doc = ""]
pub mod optr2;
#[doc = "WRPR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`wrpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpr`] module"]
#[doc(alias = "WRPR")]
pub type Wrpr = crate::Reg<wrpr::WrprSpec>;
#[doc = ""]
pub mod wrpr;
#[doc = "PCROP0SR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop0sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop0sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop0sr`] module"]
#[doc(alias = "PCROP0SR")]
pub type Pcrop0sr = crate::Reg<pcrop0sr::Pcrop0srSpec>;
#[doc = ""]
pub mod pcrop0sr;
#[doc = "PCROP0ER (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop0er::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop0er::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop0er`] module"]
#[doc(alias = "PCROP0ER")]
pub type Pcrop0er = crate::Reg<pcrop0er::Pcrop0erSpec>;
#[doc = ""]
pub mod pcrop0er;
#[doc = "PCROP1SR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop1sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop1sr`] module"]
#[doc(alias = "PCROP1SR")]
pub type Pcrop1sr = crate::Reg<pcrop1sr::Pcrop1srSpec>;
#[doc = ""]
pub mod pcrop1sr;
#[doc = "PCROP1ER (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pcrop1er::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1er::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcrop1er`] module"]
#[doc(alias = "PCROP1ER")]
pub type Pcrop1er = crate::Reg<pcrop1er::Pcrop1erSpec>;
#[doc = ""]
pub mod pcrop1er;
