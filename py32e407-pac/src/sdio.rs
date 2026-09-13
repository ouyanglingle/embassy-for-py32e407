#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    power: Power,
    clkcr: Clkcr,
    arg: Arg,
    cmd: Cmd,
    respcmd: Respcmd,
    resp0: Resp0,
    resp1: Resp1,
    resp2: Resp2,
    resp3: Resp3,
    tmout: Tmout,
    blksiz: Blksiz,
    dlen: Dlen,
    ctrl: Ctrl,
    status: Status,
    intsts: Intsts,
    intmask: Intmask,
    fifoth: Fifoth,
    tcbcnt: Tcbcnt,
    tbbcnt: Tbbcnt,
    _reserved19: [u8; 0x01b4],
    fifodata: Fifodata,
}
impl RegisterBlock {
    #[doc = "0x00 - desc POWER"]
    #[inline(always)]
    pub const fn power(&self) -> &Power {
        &self.power
    }
    #[doc = "0x04 - desc CLKCR"]
    #[inline(always)]
    pub const fn clkcr(&self) -> &Clkcr {
        &self.clkcr
    }
    #[doc = "0x08 - desc ARG"]
    #[inline(always)]
    pub const fn arg(&self) -> &Arg {
        &self.arg
    }
    #[doc = "0x0c - desc CMD"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x10 - desc RESPCMD"]
    #[inline(always)]
    pub const fn respcmd(&self) -> &Respcmd {
        &self.respcmd
    }
    #[doc = "0x14 - desc RESP0"]
    #[inline(always)]
    pub const fn resp0(&self) -> &Resp0 {
        &self.resp0
    }
    #[doc = "0x18 - desc RESP1"]
    #[inline(always)]
    pub const fn resp1(&self) -> &Resp1 {
        &self.resp1
    }
    #[doc = "0x1c - desc RESP2"]
    #[inline(always)]
    pub const fn resp2(&self) -> &Resp2 {
        &self.resp2
    }
    #[doc = "0x20 - desc RESP3"]
    #[inline(always)]
    pub const fn resp3(&self) -> &Resp3 {
        &self.resp3
    }
    #[doc = "0x24 - desc TMOUT"]
    #[inline(always)]
    pub const fn tmout(&self) -> &Tmout {
        &self.tmout
    }
    #[doc = "0x28 - desc BLKSIZ"]
    #[inline(always)]
    pub const fn blksiz(&self) -> &Blksiz {
        &self.blksiz
    }
    #[doc = "0x2c - desc DLEN"]
    #[inline(always)]
    pub const fn dlen(&self) -> &Dlen {
        &self.dlen
    }
    #[doc = "0x30 - desc CTRL"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x34 - desc STATUS"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x38 - desc INTSTS"]
    #[inline(always)]
    pub const fn intsts(&self) -> &Intsts {
        &self.intsts
    }
    #[doc = "0x3c - desc INTMASK"]
    #[inline(always)]
    pub const fn intmask(&self) -> &Intmask {
        &self.intmask
    }
    #[doc = "0x40 - desc FIFOTH"]
    #[inline(always)]
    pub const fn fifoth(&self) -> &Fifoth {
        &self.fifoth
    }
    #[doc = "0x44 - desc TCBCNT"]
    #[inline(always)]
    pub const fn tcbcnt(&self) -> &Tcbcnt {
        &self.tcbcnt
    }
    #[doc = "0x48 - desc TBBCNT"]
    #[inline(always)]
    pub const fn tbbcnt(&self) -> &Tbbcnt {
        &self.tbbcnt
    }
    #[doc = "0x200 - SDIO FIFO data register"]
    #[inline(always)]
    pub const fn fifodata(&self) -> &Fifodata {
        &self.fifodata
    }
}
#[doc = "POWER (rw) register accessor: desc POWER\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`] module"]
#[doc(alias = "POWER")]
pub type Power = crate::Reg<power::PowerSpec>;
#[doc = "desc POWER"]
pub mod power;
#[doc = "CLKCR (rw) register accessor: desc CLKCR\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcr`] module"]
#[doc(alias = "CLKCR")]
pub type Clkcr = crate::Reg<clkcr::ClkcrSpec>;
#[doc = "desc CLKCR"]
pub mod clkcr;
#[doc = "ARG (rw) register accessor: desc ARG\n\nYou can [`read`](crate::Reg::read) this register and get [`arg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg`] module"]
#[doc(alias = "ARG")]
pub type Arg = crate::Reg<arg::ArgSpec>;
#[doc = "desc ARG"]
pub mod arg;
#[doc = "CMD (rw) register accessor: desc CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "desc CMD"]
pub mod cmd;
#[doc = "RESPCMD (rw) register accessor: desc RESPCMD\n\nYou can [`read`](crate::Reg::read) this register and get [`respcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`respcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@respcmd`] module"]
#[doc(alias = "RESPCMD")]
pub type Respcmd = crate::Reg<respcmd::RespcmdSpec>;
#[doc = "desc RESPCMD"]
pub mod respcmd;
#[doc = "RESP0 (rw) register accessor: desc RESP0\n\nYou can [`read`](crate::Reg::read) this register and get [`resp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp0`] module"]
#[doc(alias = "RESP0")]
pub type Resp0 = crate::Reg<resp0::Resp0Spec>;
#[doc = "desc RESP0"]
pub mod resp0;
#[doc = "RESP1 (rw) register accessor: desc RESP1\n\nYou can [`read`](crate::Reg::read) this register and get [`resp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp1`] module"]
#[doc(alias = "RESP1")]
pub type Resp1 = crate::Reg<resp1::Resp1Spec>;
#[doc = "desc RESP1"]
pub mod resp1;
#[doc = "RESP2 (rw) register accessor: desc RESP2\n\nYou can [`read`](crate::Reg::read) this register and get [`resp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp2`] module"]
#[doc(alias = "RESP2")]
pub type Resp2 = crate::Reg<resp2::Resp2Spec>;
#[doc = "desc RESP2"]
pub mod resp2;
#[doc = "RESP3 (rw) register accessor: desc RESP3\n\nYou can [`read`](crate::Reg::read) this register and get [`resp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp3`] module"]
#[doc(alias = "RESP3")]
pub type Resp3 = crate::Reg<resp3::Resp3Spec>;
#[doc = "desc RESP3"]
pub mod resp3;
#[doc = "TMOUT (rw) register accessor: desc TMOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`tmout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmout`] module"]
#[doc(alias = "TMOUT")]
pub type Tmout = crate::Reg<tmout::TmoutSpec>;
#[doc = "desc TMOUT"]
pub mod tmout;
#[doc = "BLKSIZ (rw) register accessor: desc BLKSIZ\n\nYou can [`read`](crate::Reg::read) this register and get [`blksiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blksiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blksiz`] module"]
#[doc(alias = "BLKSIZ")]
pub type Blksiz = crate::Reg<blksiz::BlksizSpec>;
#[doc = "desc BLKSIZ"]
pub mod blksiz;
#[doc = "DLEN (rw) register accessor: desc DLEN\n\nYou can [`read`](crate::Reg::read) this register and get [`dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlen`] module"]
#[doc(alias = "DLEN")]
pub type Dlen = crate::Reg<dlen::DlenSpec>;
#[doc = "desc DLEN"]
pub mod dlen;
#[doc = "CTRL (rw) register accessor: desc CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "desc CTRL"]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: desc STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "desc STATUS"]
pub mod status;
#[doc = "INTSTS (rw) register accessor: desc INTSTS\n\nYou can [`read`](crate::Reg::read) this register and get [`intsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsts`] module"]
#[doc(alias = "INTSTS")]
pub type Intsts = crate::Reg<intsts::IntstsSpec>;
#[doc = "desc INTSTS"]
pub mod intsts;
#[doc = "INTMASK (rw) register accessor: desc INTMASK\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmask`] module"]
#[doc(alias = "INTMASK")]
pub type Intmask = crate::Reg<intmask::IntmaskSpec>;
#[doc = "desc INTMASK"]
pub mod intmask;
#[doc = "FIFOTH (rw) register accessor: desc FIFOTH\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoth`] module"]
#[doc(alias = "FIFOTH")]
pub type Fifoth = crate::Reg<fifoth::FifothSpec>;
#[doc = "desc FIFOTH"]
pub mod fifoth;
#[doc = "TCBCNT (rw) register accessor: desc TCBCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`tcbcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcbcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcbcnt`] module"]
#[doc(alias = "TCBCNT")]
pub type Tcbcnt = crate::Reg<tcbcnt::TcbcntSpec>;
#[doc = "desc TCBCNT"]
pub mod tcbcnt;
#[doc = "TBBCNT (rw) register accessor: desc TBBCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`tbbcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbbcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbbcnt`] module"]
#[doc(alias = "TBBCNT")]
pub type Tbbcnt = crate::Reg<tbbcnt::TbbcntSpec>;
#[doc = "desc TBBCNT"]
pub mod tbbcnt;
#[doc = "FIFODATA (rw) register accessor: SDIO FIFO data register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifodata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifodata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifodata`] module"]
#[doc(alias = "FIFODATA")]
pub type Fifodata = crate::Reg<fifodata::FifodataSpec>;
#[doc = "SDIO FIFO data register"]
pub mod fifodata;
