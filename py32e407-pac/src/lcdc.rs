#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    lcd0_dr: Lcd0Dr,
    lcd0_cmdr: Lcd0Cmdr,
    lcd0_wcfgr: Lcd0Wcfgr,
    lcd0_rcfgr: Lcd0Rcfgr,
    lcd1_dr: Lcd1Dr,
    lcd1_cmdr: Lcd1Cmdr,
    lcd1_wcfgr: Lcd1Wcfgr,
    lcd1_rcfgr: Lcd1Rcfgr,
    lcd2_dr: Lcd2Dr,
    lcd2_cmdr: Lcd2Cmdr,
    lcd2_wcfgr: Lcd2Wcfgr,
    lcd2_rcfgr: Lcd2Rcfgr,
    lcd3_dr: Lcd3Dr,
    lcd3_cmdr: Lcd3Cmdr,
    lcd3_wcfgr: Lcd3Wcfgr,
    lcd3_rcfgr: Lcd3Rcfgr,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn lcd0_dr(&self) -> &Lcd0Dr {
        &self.lcd0_dr
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn lcd0_cmdr(&self) -> &Lcd0Cmdr {
        &self.lcd0_cmdr
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn lcd0_wcfgr(&self) -> &Lcd0Wcfgr {
        &self.lcd0_wcfgr
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn lcd0_rcfgr(&self) -> &Lcd0Rcfgr {
        &self.lcd0_rcfgr
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn lcd1_dr(&self) -> &Lcd1Dr {
        &self.lcd1_dr
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn lcd1_cmdr(&self) -> &Lcd1Cmdr {
        &self.lcd1_cmdr
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn lcd1_wcfgr(&self) -> &Lcd1Wcfgr {
        &self.lcd1_wcfgr
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn lcd1_rcfgr(&self) -> &Lcd1Rcfgr {
        &self.lcd1_rcfgr
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn lcd2_dr(&self) -> &Lcd2Dr {
        &self.lcd2_dr
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn lcd2_cmdr(&self) -> &Lcd2Cmdr {
        &self.lcd2_cmdr
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn lcd2_wcfgr(&self) -> &Lcd2Wcfgr {
        &self.lcd2_wcfgr
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn lcd2_rcfgr(&self) -> &Lcd2Rcfgr {
        &self.lcd2_rcfgr
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn lcd3_dr(&self) -> &Lcd3Dr {
        &self.lcd3_dr
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn lcd3_cmdr(&self) -> &Lcd3Cmdr {
        &self.lcd3_cmdr
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn lcd3_wcfgr(&self) -> &Lcd3Wcfgr {
        &self.lcd3_wcfgr
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn lcd3_rcfgr(&self) -> &Lcd3Rcfgr {
        &self.lcd3_rcfgr
    }
}
#[doc = "CR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = ""]
pub mod cr;
#[doc = "LCD0_DR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd0_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd0_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd0_dr`] module"]
#[doc(alias = "LCD0_DR")]
pub type Lcd0Dr = crate::Reg<lcd0_dr::Lcd0DrSpec>;
#[doc = ""]
pub mod lcd0_dr;
#[doc = "LCD0_CMDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd0_cmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd0_cmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd0_cmdr`] module"]
#[doc(alias = "LCD0_CMDR")]
pub type Lcd0Cmdr = crate::Reg<lcd0_cmdr::Lcd0CmdrSpec>;
#[doc = ""]
pub mod lcd0_cmdr;
#[doc = "LCD0_WCFGR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd0_wcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd0_wcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd0_wcfgr`] module"]
#[doc(alias = "LCD0_WCFGR")]
pub type Lcd0Wcfgr = crate::Reg<lcd0_wcfgr::Lcd0WcfgrSpec>;
#[doc = ""]
pub mod lcd0_wcfgr;
#[doc = "LCD0_RCFGR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd0_rcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd0_rcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd0_rcfgr`] module"]
#[doc(alias = "LCD0_RCFGR")]
pub type Lcd0Rcfgr = crate::Reg<lcd0_rcfgr::Lcd0RcfgrSpec>;
#[doc = ""]
pub mod lcd0_rcfgr;
#[doc = "LCD1_DR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd1_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd1_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd1_dr`] module"]
#[doc(alias = "LCD1_DR")]
pub type Lcd1Dr = crate::Reg<lcd1_dr::Lcd1DrSpec>;
#[doc = ""]
pub mod lcd1_dr;
#[doc = "LCD1_CMDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd1_cmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd1_cmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd1_cmdr`] module"]
#[doc(alias = "LCD1_CMDR")]
pub type Lcd1Cmdr = crate::Reg<lcd1_cmdr::Lcd1CmdrSpec>;
#[doc = ""]
pub mod lcd1_cmdr;
#[doc = "LCD1_WCFGR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd1_wcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd1_wcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd1_wcfgr`] module"]
#[doc(alias = "LCD1_WCFGR")]
pub type Lcd1Wcfgr = crate::Reg<lcd1_wcfgr::Lcd1WcfgrSpec>;
#[doc = ""]
pub mod lcd1_wcfgr;
#[doc = "LCD1_RCFGR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd1_rcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd1_rcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd1_rcfgr`] module"]
#[doc(alias = "LCD1_RCFGR")]
pub type Lcd1Rcfgr = crate::Reg<lcd1_rcfgr::Lcd1RcfgrSpec>;
#[doc = ""]
pub mod lcd1_rcfgr;
#[doc = "LCD2_DR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd2_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd2_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd2_dr`] module"]
#[doc(alias = "LCD2_DR")]
pub type Lcd2Dr = crate::Reg<lcd2_dr::Lcd2DrSpec>;
#[doc = ""]
pub mod lcd2_dr;
#[doc = "LCD2_CMDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd2_cmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd2_cmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd2_cmdr`] module"]
#[doc(alias = "LCD2_CMDR")]
pub type Lcd2Cmdr = crate::Reg<lcd2_cmdr::Lcd2CmdrSpec>;
#[doc = ""]
pub mod lcd2_cmdr;
#[doc = "LCD2_WCFGR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd2_wcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd2_wcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd2_wcfgr`] module"]
#[doc(alias = "LCD2_WCFGR")]
pub type Lcd2Wcfgr = crate::Reg<lcd2_wcfgr::Lcd2WcfgrSpec>;
#[doc = ""]
pub mod lcd2_wcfgr;
#[doc = "LCD2_RCFGR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd2_rcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd2_rcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd2_rcfgr`] module"]
#[doc(alias = "LCD2_RCFGR")]
pub type Lcd2Rcfgr = crate::Reg<lcd2_rcfgr::Lcd2RcfgrSpec>;
#[doc = ""]
pub mod lcd2_rcfgr;
#[doc = "LCD3_DR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd3_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd3_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd3_dr`] module"]
#[doc(alias = "LCD3_DR")]
pub type Lcd3Dr = crate::Reg<lcd3_dr::Lcd3DrSpec>;
#[doc = ""]
pub mod lcd3_dr;
#[doc = "LCD3_CMDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd3_cmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd3_cmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd3_cmdr`] module"]
#[doc(alias = "LCD3_CMDR")]
pub type Lcd3Cmdr = crate::Reg<lcd3_cmdr::Lcd3CmdrSpec>;
#[doc = ""]
pub mod lcd3_cmdr;
#[doc = "LCD3_WCFGR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd3_wcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd3_wcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd3_wcfgr`] module"]
#[doc(alias = "LCD3_WCFGR")]
pub type Lcd3Wcfgr = crate::Reg<lcd3_wcfgr::Lcd3WcfgrSpec>;
#[doc = ""]
pub mod lcd3_wcfgr;
#[doc = "LCD3_RCFGR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd3_rcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd3_rcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd3_rcfgr`] module"]
#[doc(alias = "LCD3_RCFGR")]
pub type Lcd3Rcfgr = crate::Reg<lcd3_rcfgr::Lcd3RcfgrSpec>;
#[doc = ""]
pub mod lcd3_rcfgr;
