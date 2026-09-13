#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    cr2: Cr2,
    tcr: Tcr,
    baud: Baud,
    sfcr: Sfcr,
    socr: Socr,
    dcr: Dcr,
    cr3: Cr3,
    addr24: Addr24,
    addr32: Addr32,
    data: Data,
    sr: Sr,
    sr1: Sr1,
    rxstat: Rxstat,
    _reserved14: [u8; 0x01],
    ifr1: Ifr1,
    ier1: Ier1,
    ifr2: Ifr2,
    ier2: Ier2,
    strr: Strr,
    _reserved19: [u8; 0x03],
    xsfcr: Xsfcr,
    xsocr: Xsocr,
    xdcr: Xdcr,
    xcr3: Xcr3,
    xmode: Xmode,
    xmode_we: XmodeWe,
    xssocr: Xssocr,
    xssocr_we: XssocrWe,
    xsfcr_we: XsfcrWe,
    xsocr_we: XsocrWe,
    xdcr_we: XdcrWe,
    xcr3_we: Xcr3We,
    bcr: Bcr,
    rxtout: Rxtout,
    xstrr: Xstrr,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x01 - "]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x02 - "]
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    #[doc = "0x03 - "]
    #[inline(always)]
    pub const fn baud(&self) -> &Baud {
        &self.baud
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn sfcr(&self) -> &Sfcr {
        &self.sfcr
    }
    #[doc = "0x05 - "]
    #[inline(always)]
    pub const fn socr(&self) -> &Socr {
        &self.socr
    }
    #[doc = "0x06 - "]
    #[inline(always)]
    pub const fn dcr(&self) -> &Dcr {
        &self.dcr
    }
    #[doc = "0x07 - "]
    #[inline(always)]
    pub const fn cr3(&self) -> &Cr3 {
        &self.cr3
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn addr24(&self) -> &Addr24 {
        &self.addr24
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn addr32(&self) -> &Addr32 {
        &self.addr32
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x15 - "]
    #[inline(always)]
    pub const fn sr1(&self) -> &Sr1 {
        &self.sr1
    }
    #[doc = "0x16 - "]
    #[inline(always)]
    pub const fn rxstat(&self) -> &Rxstat {
        &self.rxstat
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn ifr1(&self) -> &Ifr1 {
        &self.ifr1
    }
    #[doc = "0x19 - "]
    #[inline(always)]
    pub const fn ier1(&self) -> &Ier1 {
        &self.ier1
    }
    #[doc = "0x1a - "]
    #[inline(always)]
    pub const fn ifr2(&self) -> &Ifr2 {
        &self.ifr2
    }
    #[doc = "0x1b - "]
    #[inline(always)]
    pub const fn ier2(&self) -> &Ier2 {
        &self.ier2
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn strr(&self) -> &Strr {
        &self.strr
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn xsfcr(&self) -> &Xsfcr {
        &self.xsfcr
    }
    #[doc = "0x21 - "]
    #[inline(always)]
    pub const fn xsocr(&self) -> &Xsocr {
        &self.xsocr
    }
    #[doc = "0x22 - "]
    #[inline(always)]
    pub const fn xdcr(&self) -> &Xdcr {
        &self.xdcr
    }
    #[doc = "0x23 - "]
    #[inline(always)]
    pub const fn xcr3(&self) -> &Xcr3 {
        &self.xcr3
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn xmode(&self) -> &Xmode {
        &self.xmode
    }
    #[doc = "0x25 - "]
    #[inline(always)]
    pub const fn xmode_we(&self) -> &XmodeWe {
        &self.xmode_we
    }
    #[doc = "0x26 - "]
    #[inline(always)]
    pub const fn xssocr(&self) -> &Xssocr {
        &self.xssocr
    }
    #[doc = "0x27 - "]
    #[inline(always)]
    pub const fn xssocr_we(&self) -> &XssocrWe {
        &self.xssocr_we
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn xsfcr_we(&self) -> &XsfcrWe {
        &self.xsfcr_we
    }
    #[doc = "0x29 - "]
    #[inline(always)]
    pub const fn xsocr_we(&self) -> &XsocrWe {
        &self.xsocr_we
    }
    #[doc = "0x2a - "]
    #[inline(always)]
    pub const fn xdcr_we(&self) -> &XdcrWe {
        &self.xdcr_we
    }
    #[doc = "0x2b - "]
    #[inline(always)]
    pub const fn xcr3_we(&self) -> &Xcr3We {
        &self.xcr3_we
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn bcr(&self) -> &Bcr {
        &self.bcr
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn rxtout(&self) -> &Rxtout {
        &self.rxtout
    }
    #[doc = "0x31 - "]
    #[inline(always)]
    pub const fn xstrr(&self) -> &Xstrr {
        &self.xstrr
    }
}
#[doc = "CR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = ""]
pub mod cr;
#[doc = "CR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = ""]
pub mod cr2;
#[doc = "TCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`] module"]
#[doc(alias = "TCR")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
#[doc = ""]
pub mod tcr;
#[doc = "BAUD (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`baud::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud`] module"]
#[doc(alias = "BAUD")]
pub type Baud = crate::Reg<baud::BaudSpec>;
#[doc = ""]
pub mod baud;
#[doc = "SFCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfcr`] module"]
#[doc(alias = "SFCR")]
pub type Sfcr = crate::Reg<sfcr::SfcrSpec>;
#[doc = ""]
pub mod sfcr;
#[doc = "SOCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`socr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`socr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@socr`] module"]
#[doc(alias = "SOCR")]
pub type Socr = crate::Reg<socr::SocrSpec>;
#[doc = ""]
pub mod socr;
#[doc = "DCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`] module"]
#[doc(alias = "DCR")]
pub type Dcr = crate::Reg<dcr::DcrSpec>;
#[doc = ""]
pub mod dcr;
#[doc = "CR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`] module"]
#[doc(alias = "CR3")]
pub type Cr3 = crate::Reg<cr3::Cr3Spec>;
#[doc = ""]
pub mod cr3;
#[doc = "ADDR24 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`addr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr24`] module"]
#[doc(alias = "ADDR24")]
pub type Addr24 = crate::Reg<addr24::Addr24Spec>;
#[doc = ""]
pub mod addr24;
#[doc = "ADDR32 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`addr32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr32`] module"]
#[doc(alias = "ADDR32")]
pub type Addr32 = crate::Reg<addr32::Addr32Spec>;
#[doc = ""]
pub mod addr32;
#[doc = "DATA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = ""]
pub mod data;
#[doc = "SR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = ""]
pub mod sr;
#[doc = "SR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`] module"]
#[doc(alias = "SR1")]
pub type Sr1 = crate::Reg<sr1::Sr1Spec>;
#[doc = ""]
pub mod sr1;
#[doc = "RXSTAT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rxstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxstat`] module"]
#[doc(alias = "RXSTAT")]
pub type Rxstat = crate::Reg<rxstat::RxstatSpec>;
#[doc = ""]
pub mod rxstat;
#[doc = "IFR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ifr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifr1`] module"]
#[doc(alias = "IFR1")]
pub type Ifr1 = crate::Reg<ifr1::Ifr1Spec>;
#[doc = ""]
pub mod ifr1;
#[doc = "IER1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier1`] module"]
#[doc(alias = "IER1")]
pub type Ier1 = crate::Reg<ier1::Ier1Spec>;
#[doc = ""]
pub mod ier1;
#[doc = "IFR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ifr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifr2`] module"]
#[doc(alias = "IFR2")]
pub type Ifr2 = crate::Reg<ifr2::Ifr2Spec>;
#[doc = ""]
pub mod ifr2;
#[doc = "IER2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier2`] module"]
#[doc(alias = "IER2")]
pub type Ier2 = crate::Reg<ier2::Ier2Spec>;
#[doc = ""]
pub mod ier2;
#[doc = "STRR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`strr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`strr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@strr`] module"]
#[doc(alias = "STRR")]
pub type Strr = crate::Reg<strr::StrrSpec>;
#[doc = ""]
pub mod strr;
#[doc = "XSFCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xsfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xsfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xsfcr`] module"]
#[doc(alias = "XSFCR")]
pub type Xsfcr = crate::Reg<xsfcr::XsfcrSpec>;
#[doc = ""]
pub mod xsfcr;
#[doc = "XSOCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xsocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xsocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xsocr`] module"]
#[doc(alias = "XSOCR")]
pub type Xsocr = crate::Reg<xsocr::XsocrSpec>;
#[doc = ""]
pub mod xsocr;
#[doc = "XDCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xdcr`] module"]
#[doc(alias = "XDCR")]
pub type Xdcr = crate::Reg<xdcr::XdcrSpec>;
#[doc = ""]
pub mod xdcr;
#[doc = "XCR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcr3`] module"]
#[doc(alias = "XCR3")]
pub type Xcr3 = crate::Reg<xcr3::Xcr3Spec>;
#[doc = ""]
pub mod xcr3;
#[doc = "XMODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xmode`] module"]
#[doc(alias = "XMODE")]
pub type Xmode = crate::Reg<xmode::XmodeSpec>;
#[doc = ""]
pub mod xmode;
#[doc = "XMODE_WE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xmode_we::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xmode_we::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xmode_we`] module"]
#[doc(alias = "XMODE_WE")]
pub type XmodeWe = crate::Reg<xmode_we::XmodeWeSpec>;
#[doc = ""]
pub mod xmode_we;
#[doc = "XSSOCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xssocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xssocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xssocr`] module"]
#[doc(alias = "XSSOCR")]
pub type Xssocr = crate::Reg<xssocr::XssocrSpec>;
#[doc = ""]
pub mod xssocr;
#[doc = "XSSOCR_WE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xssocr_we::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xssocr_we::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xssocr_we`] module"]
#[doc(alias = "XSSOCR_WE")]
pub type XssocrWe = crate::Reg<xssocr_we::XssocrWeSpec>;
#[doc = ""]
pub mod xssocr_we;
#[doc = "XSFCR_WE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xsfcr_we::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xsfcr_we::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xsfcr_we`] module"]
#[doc(alias = "XSFCR_WE")]
pub type XsfcrWe = crate::Reg<xsfcr_we::XsfcrWeSpec>;
#[doc = ""]
pub mod xsfcr_we;
#[doc = "XSOCR_WE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xsocr_we::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xsocr_we::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xsocr_we`] module"]
#[doc(alias = "XSOCR_WE")]
pub type XsocrWe = crate::Reg<xsocr_we::XsocrWeSpec>;
#[doc = ""]
pub mod xsocr_we;
#[doc = "XDCR_WE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xdcr_we::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xdcr_we::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xdcr_we`] module"]
#[doc(alias = "XDCR_WE")]
pub type XdcrWe = crate::Reg<xdcr_we::XdcrWeSpec>;
#[doc = ""]
pub mod xdcr_we;
#[doc = "XCR3_WE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xcr3_we::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcr3_we::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcr3_we`] module"]
#[doc(alias = "XCR3_WE")]
pub type Xcr3We = crate::Reg<xcr3_we::Xcr3WeSpec>;
#[doc = ""]
pub mod xcr3_we;
#[doc = "BCR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`bcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr`] module"]
#[doc(alias = "BCR")]
pub type Bcr = crate::Reg<bcr::BcrSpec>;
#[doc = ""]
pub mod bcr;
#[doc = "RXTOUT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rxtout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtout`] module"]
#[doc(alias = "RXTOUT")]
pub type Rxtout = crate::Reg<rxtout::RxtoutSpec>;
#[doc = ""]
pub mod rxtout;
#[doc = "XSTRR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xstrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xstrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xstrr`] module"]
#[doc(alias = "XSTRR")]
pub type Xstrr = crate::Reg<xstrr::XstrrSpec>;
#[doc = ""]
pub mod xstrr;
