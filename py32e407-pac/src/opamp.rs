#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr1: Csr1,
    csr2: Csr2,
    csr3: Csr3,
    tcmr1: Tcmr1,
    tcmr2: Tcmr2,
    tcmr3: Tcmr3,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn csr1(&self) -> &Csr1 {
        &self.csr1
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn csr2(&self) -> &Csr2 {
        &self.csr2
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn csr3(&self) -> &Csr3 {
        &self.csr3
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn tcmr1(&self) -> &Tcmr1 {
        &self.tcmr1
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn tcmr2(&self) -> &Tcmr2 {
        &self.tcmr2
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn tcmr3(&self) -> &Tcmr3 {
        &self.tcmr3
    }
}
#[doc = "CSR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`csr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr1`] module"]
#[doc(alias = "CSR1")]
pub type Csr1 = crate::Reg<csr1::Csr1Spec>;
#[doc = ""]
pub mod csr1;
#[doc = "CSR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`csr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr2`] module"]
#[doc(alias = "CSR2")]
pub type Csr2 = crate::Reg<csr2::Csr2Spec>;
#[doc = ""]
pub mod csr2;
#[doc = "CSR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`csr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr3`] module"]
#[doc(alias = "CSR3")]
pub type Csr3 = crate::Reg<csr3::Csr3Spec>;
#[doc = ""]
pub mod csr3;
#[doc = "TCMR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`tcmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcmr1`] module"]
#[doc(alias = "TCMR1")]
pub type Tcmr1 = crate::Reg<tcmr1::Tcmr1Spec>;
#[doc = ""]
pub mod tcmr1;
#[doc = "TCMR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`tcmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcmr2`] module"]
#[doc(alias = "TCMR2")]
pub type Tcmr2 = crate::Reg<tcmr2::Tcmr2Spec>;
#[doc = ""]
pub mod tcmr2;
#[doc = "TCMR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`tcmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcmr3`] module"]
#[doc(alias = "TCMR3")]
pub type Tcmr3 = crate::Reg<tcmr3::Tcmr3Spec>;
#[doc = ""]
pub mod tcmr3;
