#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: Cr1,
    cr2: Cr2,
    sr: Sr,
    dr: Dr,
    crcpr: Crcpr,
    rxcrcr: Rxcrcr,
    txcrcr: Txcrcr,
    i2s_cfgr: I2sCfgr,
    i2spr: I2spr,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn crcpr(&self) -> &Crcpr {
        &self.crcpr
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn rxcrcr(&self) -> &Rxcrcr {
        &self.rxcrcr
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn txcrcr(&self) -> &Txcrcr {
        &self.txcrcr
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn i2s_cfgr(&self) -> &I2sCfgr {
        &self.i2s_cfgr
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn i2spr(&self) -> &I2spr {
        &self.i2spr
    }
}
#[doc = "CR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = ""]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = ""]
pub mod cr2;
#[doc = "SR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = ""]
pub mod sr;
#[doc = "DR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = ""]
pub mod dr;
#[doc = "CRCPR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`crcpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpr`] module"]
#[doc(alias = "CRCPR")]
pub type Crcpr = crate::Reg<crcpr::CrcprSpec>;
#[doc = ""]
pub mod crcpr;
#[doc = "RXCRCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rxcrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxcrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcrcr`] module"]
#[doc(alias = "RXCRCR")]
pub type Rxcrcr = crate::Reg<rxcrcr::RxcrcrSpec>;
#[doc = ""]
pub mod rxcrcr;
#[doc = "TXCRCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`txcrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txcrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcrcr`] module"]
#[doc(alias = "TXCRCR")]
pub type Txcrcr = crate::Reg<txcrcr::TxcrcrSpec>;
#[doc = ""]
pub mod txcrcr;
#[doc = "I2S_CFGR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_cfgr`] module"]
#[doc(alias = "I2S_CFGR")]
pub type I2sCfgr = crate::Reg<i2s_cfgr::I2sCfgrSpec>;
#[doc = ""]
pub mod i2s_cfgr;
#[doc = "I2SPR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`i2spr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2spr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2spr`] module"]
#[doc(alias = "I2SPR")]
pub type I2spr = crate::Reg<i2spr::I2sprSpec>;
#[doc = ""]
pub mod i2spr;
