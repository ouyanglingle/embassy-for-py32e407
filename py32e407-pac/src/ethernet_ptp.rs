#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ptptscr: Ptptscr,
    ptpssir: Ptpssir,
    ptptshr: Ptptshr,
    ptptslr: Ptptslr,
    ptptshur: Ptptshur,
    ptptslur: Ptptslur,
    ptptsar: Ptptsar,
    ptptthr: Ptptthr,
    ptpttlr: Ptpttlr,
}
impl RegisterBlock {
    #[doc = "0x00 - Ethernet PTP time stamp control register (ETH_PTPTSCR)"]
    #[inline(always)]
    pub const fn ptptscr(&self) -> &Ptptscr {
        &self.ptptscr
    }
    #[doc = "0x04 - Ethernet PTP subsecond increment register"]
    #[inline(always)]
    pub const fn ptpssir(&self) -> &Ptpssir {
        &self.ptpssir
    }
    #[doc = "0x08 - Ethernet PTP time stamp high register"]
    #[inline(always)]
    pub const fn ptptshr(&self) -> &Ptptshr {
        &self.ptptshr
    }
    #[doc = "0x0c - Ethernet PTP time stamp low register (ETH_PTPTSLR)"]
    #[inline(always)]
    pub const fn ptptslr(&self) -> &Ptptslr {
        &self.ptptslr
    }
    #[doc = "0x10 - Ethernet PTP time stamp high update register"]
    #[inline(always)]
    pub const fn ptptshur(&self) -> &Ptptshur {
        &self.ptptshur
    }
    #[doc = "0x14 - Ethernet PTP time stamp low update register (ETH_PTPTSLUR)"]
    #[inline(always)]
    pub const fn ptptslur(&self) -> &Ptptslur {
        &self.ptptslur
    }
    #[doc = "0x18 - Ethernet PTP time stamp addend register"]
    #[inline(always)]
    pub const fn ptptsar(&self) -> &Ptptsar {
        &self.ptptsar
    }
    #[doc = "0x1c - Ethernet PTP target time high register"]
    #[inline(always)]
    pub const fn ptptthr(&self) -> &Ptptthr {
        &self.ptptthr
    }
    #[doc = "0x20 - Ethernet PTP target time low register"]
    #[inline(always)]
    pub const fn ptpttlr(&self) -> &Ptpttlr {
        &self.ptpttlr
    }
}
#[doc = "PTPTSCR (rw) register accessor: Ethernet PTP time stamp control register (ETH_PTPTSCR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptscr`] module"]
#[doc(alias = "PTPTSCR")]
pub type Ptptscr = crate::Reg<ptptscr::PtptscrSpec>;
#[doc = "Ethernet PTP time stamp control register (ETH_PTPTSCR)"]
pub mod ptptscr;
#[doc = "PTPSSIR (rw) register accessor: Ethernet PTP subsecond increment register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptpssir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptpssir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptpssir`] module"]
#[doc(alias = "PTPSSIR")]
pub type Ptpssir = crate::Reg<ptpssir::PtpssirSpec>;
#[doc = "Ethernet PTP subsecond increment register"]
pub mod ptpssir;
#[doc = "PTPTSHR (rw) register accessor: Ethernet PTP time stamp high register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptshr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptshr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptshr`] module"]
#[doc(alias = "PTPTSHR")]
pub type Ptptshr = crate::Reg<ptptshr::PtptshrSpec>;
#[doc = "Ethernet PTP time stamp high register"]
pub mod ptptshr;
#[doc = "PTPTSLR (rw) register accessor: Ethernet PTP time stamp low register (ETH_PTPTSLR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptslr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptslr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptslr`] module"]
#[doc(alias = "PTPTSLR")]
pub type Ptptslr = crate::Reg<ptptslr::PtptslrSpec>;
#[doc = "Ethernet PTP time stamp low register (ETH_PTPTSLR)"]
pub mod ptptslr;
#[doc = "PTPTSHUR (rw) register accessor: Ethernet PTP time stamp high update register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptshur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptshur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptshur`] module"]
#[doc(alias = "PTPTSHUR")]
pub type Ptptshur = crate::Reg<ptptshur::PtptshurSpec>;
#[doc = "Ethernet PTP time stamp high update register"]
pub mod ptptshur;
#[doc = "PTPTSLUR (rw) register accessor: Ethernet PTP time stamp low update register (ETH_PTPTSLUR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptslur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptslur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptslur`] module"]
#[doc(alias = "PTPTSLUR")]
pub type Ptptslur = crate::Reg<ptptslur::PtptslurSpec>;
#[doc = "Ethernet PTP time stamp low update register (ETH_PTPTSLUR)"]
pub mod ptptslur;
#[doc = "PTPTSAR (rw) register accessor: Ethernet PTP time stamp addend register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptsar`] module"]
#[doc(alias = "PTPTSAR")]
pub type Ptptsar = crate::Reg<ptptsar::PtptsarSpec>;
#[doc = "Ethernet PTP time stamp addend register"]
pub mod ptptsar;
#[doc = "PTPTTHR (rw) register accessor: Ethernet PTP target time high register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptthr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptthr`] module"]
#[doc(alias = "PTPTTHR")]
pub type Ptptthr = crate::Reg<ptptthr::PtptthrSpec>;
#[doc = "Ethernet PTP target time high register"]
pub mod ptptthr;
#[doc = "PTPTTLR (rw) register accessor: Ethernet PTP target time low register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptpttlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptpttlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptpttlr`] module"]
#[doc(alias = "PTPTTLR")]
pub type Ptpttlr = crate::Reg<ptpttlr::PtpttlrSpec>;
#[doc = "Ethernet PTP target time low register"]
pub mod ptpttlr;
