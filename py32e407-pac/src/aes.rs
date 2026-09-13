#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    msg_cfg: MsgCfg,
    ctx_cfg: CtxCfg,
    msg_total_bytes: MsgTotalBytes,
    msg_aad_bytes: MsgAadBytes,
    gcm_aad_info: GcmAadInfo,
    _reserved5: [u8; 0x0c],
    ctx_key0: CtxKey0,
    ctx_key1: CtxKey1,
    ctx_key2: CtxKey2,
    ctx_key3: CtxKey3,
    ctx_key4: CtxKey4,
    ctx_key5: CtxKey5,
    ctx_key6: CtxKey6,
    ctx_key7: CtxKey7,
    ctx_cbc_key0: CtxCbcKey0,
    ctx_cbc_key1: CtxCbcKey1,
    ctx_cbc_key2: CtxCbcKey2,
    ctx_cbc_key3: CtxCbcKey3,
    ctx_ctr0: CtxCtr0,
    ctx_ctr1: CtxCtr1,
    ctx_ctr2: CtxCtr2,
    ctx_ctr3: CtxCtr3,
    ctx_iv0: CtxIv0,
    ctx_iv1: CtxIv1,
    ctx_iv2: CtxIv2,
    ctx_iv3: CtxIv3,
    ctx_mac0: CtxMac0,
    ctx_mac1: CtxMac1,
    ctx_mac2: CtxMac2,
    ctx_mac3: CtxMac3,
    ingress_fifo: IngressFifo,
    ing_dbcfg: IngDbcfg,
    dma_eng_len: DmaEngLen,
    eng_dbcfg: EngDbcfg,
    dma_ing_len: DmaIngLen,
    ingfifif_status: IngfififStatus,
    engress_fifo: EngressFifo,
    engfifo_status: EngfifoStatus,
    ing_dma_done: IngDmaDone,
    eng_dma_done: EngDmaDone,
    done_status: DoneStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn msg_cfg(&self) -> &MsgCfg {
        &self.msg_cfg
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn ctx_cfg(&self) -> &CtxCfg {
        &self.ctx_cfg
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn msg_total_bytes(&self) -> &MsgTotalBytes {
        &self.msg_total_bytes
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn msg_aad_bytes(&self) -> &MsgAadBytes {
        &self.msg_aad_bytes
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn gcm_aad_info(&self) -> &GcmAadInfo {
        &self.gcm_aad_info
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn ctx_key0(&self) -> &CtxKey0 {
        &self.ctx_key0
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn ctx_key1(&self) -> &CtxKey1 {
        &self.ctx_key1
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn ctx_key2(&self) -> &CtxKey2 {
        &self.ctx_key2
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn ctx_key3(&self) -> &CtxKey3 {
        &self.ctx_key3
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn ctx_key4(&self) -> &CtxKey4 {
        &self.ctx_key4
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn ctx_key5(&self) -> &CtxKey5 {
        &self.ctx_key5
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn ctx_key6(&self) -> &CtxKey6 {
        &self.ctx_key6
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn ctx_key7(&self) -> &CtxKey7 {
        &self.ctx_key7
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn ctx_cbc_key0(&self) -> &CtxCbcKey0 {
        &self.ctx_cbc_key0
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn ctx_cbc_key1(&self) -> &CtxCbcKey1 {
        &self.ctx_cbc_key1
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn ctx_cbc_key2(&self) -> &CtxCbcKey2 {
        &self.ctx_cbc_key2
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn ctx_cbc_key3(&self) -> &CtxCbcKey3 {
        &self.ctx_cbc_key3
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn ctx_ctr0(&self) -> &CtxCtr0 {
        &self.ctx_ctr0
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn ctx_ctr1(&self) -> &CtxCtr1 {
        &self.ctx_ctr1
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn ctx_ctr2(&self) -> &CtxCtr2 {
        &self.ctx_ctr2
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn ctx_ctr3(&self) -> &CtxCtr3 {
        &self.ctx_ctr3
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn ctx_iv0(&self) -> &CtxIv0 {
        &self.ctx_iv0
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn ctx_iv1(&self) -> &CtxIv1 {
        &self.ctx_iv1
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn ctx_iv2(&self) -> &CtxIv2 {
        &self.ctx_iv2
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn ctx_iv3(&self) -> &CtxIv3 {
        &self.ctx_iv3
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn ctx_mac0(&self) -> &CtxMac0 {
        &self.ctx_mac0
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn ctx_mac1(&self) -> &CtxMac1 {
        &self.ctx_mac1
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn ctx_mac2(&self) -> &CtxMac2 {
        &self.ctx_mac2
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn ctx_mac3(&self) -> &CtxMac3 {
        &self.ctx_mac3
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn ingress_fifo(&self) -> &IngressFifo {
        &self.ingress_fifo
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn ing_dbcfg(&self) -> &IngDbcfg {
        &self.ing_dbcfg
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn dma_eng_len(&self) -> &DmaEngLen {
        &self.dma_eng_len
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn eng_dbcfg(&self) -> &EngDbcfg {
        &self.eng_dbcfg
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn dma_ing_len(&self) -> &DmaIngLen {
        &self.dma_ing_len
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn ingfifif_status(&self) -> &IngfififStatus {
        &self.ingfifif_status
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn engress_fifo(&self) -> &EngressFifo {
        &self.engress_fifo
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn engfifo_status(&self) -> &EngfifoStatus {
        &self.engfifo_status
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn ing_dma_done(&self) -> &IngDmaDone {
        &self.ing_dma_done
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn eng_dma_done(&self) -> &EngDmaDone {
        &self.eng_dma_done
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn done_status(&self) -> &DoneStatus {
        &self.done_status
    }
}
#[doc = "MSG_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`msg_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msg_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msg_cfg`] module"]
#[doc(alias = "MSG_CFG")]
pub type MsgCfg = crate::Reg<msg_cfg::MsgCfgSpec>;
#[doc = ""]
pub mod msg_cfg;
#[doc = "CTX_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_cfg`] module"]
#[doc(alias = "CTX_CFG")]
pub type CtxCfg = crate::Reg<ctx_cfg::CtxCfgSpec>;
#[doc = ""]
pub mod ctx_cfg;
#[doc = "MSG_TOTAL_BYTES (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`msg_total_bytes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msg_total_bytes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msg_total_bytes`] module"]
#[doc(alias = "MSG_TOTAL_BYTES")]
pub type MsgTotalBytes = crate::Reg<msg_total_bytes::MsgTotalBytesSpec>;
#[doc = ""]
pub mod msg_total_bytes;
#[doc = "MSG_AAD_BYTES (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`msg_aad_bytes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msg_aad_bytes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msg_aad_bytes`] module"]
#[doc(alias = "MSG_AAD_BYTES")]
pub type MsgAadBytes = crate::Reg<msg_aad_bytes::MsgAadBytesSpec>;
#[doc = ""]
pub mod msg_aad_bytes;
#[doc = "GCM_AAD_INFO (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gcm_aad_info::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcm_aad_info::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcm_aad_info`] module"]
#[doc(alias = "GCM_AAD_INFO")]
pub type GcmAadInfo = crate::Reg<gcm_aad_info::GcmAadInfoSpec>;
#[doc = ""]
pub mod gcm_aad_info;
#[doc = "CTX_KEY0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_key0`] module"]
#[doc(alias = "CTX_KEY0")]
pub type CtxKey0 = crate::Reg<ctx_key0::CtxKey0Spec>;
#[doc = ""]
pub mod ctx_key0;
#[doc = "CTX_KEY1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_key1`] module"]
#[doc(alias = "CTX_KEY1")]
pub type CtxKey1 = crate::Reg<ctx_key1::CtxKey1Spec>;
#[doc = ""]
pub mod ctx_key1;
#[doc = "CTX_KEY2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_key2`] module"]
#[doc(alias = "CTX_KEY2")]
pub type CtxKey2 = crate::Reg<ctx_key2::CtxKey2Spec>;
#[doc = ""]
pub mod ctx_key2;
#[doc = "CTX_KEY3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_key3`] module"]
#[doc(alias = "CTX_KEY3")]
pub type CtxKey3 = crate::Reg<ctx_key3::CtxKey3Spec>;
#[doc = ""]
pub mod ctx_key3;
#[doc = "CTX_KEY4 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_key4`] module"]
#[doc(alias = "CTX_KEY4")]
pub type CtxKey4 = crate::Reg<ctx_key4::CtxKey4Spec>;
#[doc = ""]
pub mod ctx_key4;
#[doc = "CTX_KEY5 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_key5`] module"]
#[doc(alias = "CTX_KEY5")]
pub type CtxKey5 = crate::Reg<ctx_key5::CtxKey5Spec>;
#[doc = ""]
pub mod ctx_key5;
#[doc = "CTX_KEY6 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_key6`] module"]
#[doc(alias = "CTX_KEY6")]
pub type CtxKey6 = crate::Reg<ctx_key6::CtxKey6Spec>;
#[doc = ""]
pub mod ctx_key6;
#[doc = "CTX_KEY7 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_key7`] module"]
#[doc(alias = "CTX_KEY7")]
pub type CtxKey7 = crate::Reg<ctx_key7::CtxKey7Spec>;
#[doc = ""]
pub mod ctx_key7;
#[doc = "CTX_CBC_KEY0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_cbc_key0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_cbc_key0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_cbc_key0`] module"]
#[doc(alias = "CTX_CBC_KEY0")]
pub type CtxCbcKey0 = crate::Reg<ctx_cbc_key0::CtxCbcKey0Spec>;
#[doc = ""]
pub mod ctx_cbc_key0;
#[doc = "CTX_CBC_KEY1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_cbc_key1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_cbc_key1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_cbc_key1`] module"]
#[doc(alias = "CTX_CBC_KEY1")]
pub type CtxCbcKey1 = crate::Reg<ctx_cbc_key1::CtxCbcKey1Spec>;
#[doc = ""]
pub mod ctx_cbc_key1;
#[doc = "CTX_CBC_KEY2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_cbc_key2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_cbc_key2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_cbc_key2`] module"]
#[doc(alias = "CTX_CBC_KEY2")]
pub type CtxCbcKey2 = crate::Reg<ctx_cbc_key2::CtxCbcKey2Spec>;
#[doc = ""]
pub mod ctx_cbc_key2;
#[doc = "CTX_CBC_KEY3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_cbc_key3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_cbc_key3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_cbc_key3`] module"]
#[doc(alias = "CTX_CBC_KEY3")]
pub type CtxCbcKey3 = crate::Reg<ctx_cbc_key3::CtxCbcKey3Spec>;
#[doc = ""]
pub mod ctx_cbc_key3;
#[doc = "CTX_CTR0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_ctr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_ctr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_ctr0`] module"]
#[doc(alias = "CTX_CTR0")]
pub type CtxCtr0 = crate::Reg<ctx_ctr0::CtxCtr0Spec>;
#[doc = ""]
pub mod ctx_ctr0;
#[doc = "CTX_CTR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_ctr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_ctr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_ctr1`] module"]
#[doc(alias = "CTX_CTR1")]
pub type CtxCtr1 = crate::Reg<ctx_ctr1::CtxCtr1Spec>;
#[doc = ""]
pub mod ctx_ctr1;
#[doc = "CTX_CTR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_ctr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_ctr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_ctr2`] module"]
#[doc(alias = "CTX_CTR2")]
pub type CtxCtr2 = crate::Reg<ctx_ctr2::CtxCtr2Spec>;
#[doc = ""]
pub mod ctx_ctr2;
#[doc = "CTX_CTR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_ctr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_ctr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_ctr3`] module"]
#[doc(alias = "CTX_CTR3")]
pub type CtxCtr3 = crate::Reg<ctx_ctr3::CtxCtr3Spec>;
#[doc = ""]
pub mod ctx_ctr3;
#[doc = "CTX_IV0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_iv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_iv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_iv0`] module"]
#[doc(alias = "CTX_IV0")]
pub type CtxIv0 = crate::Reg<ctx_iv0::CtxIv0Spec>;
#[doc = ""]
pub mod ctx_iv0;
#[doc = "CTX_IV1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_iv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_iv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_iv1`] module"]
#[doc(alias = "CTX_IV1")]
pub type CtxIv1 = crate::Reg<ctx_iv1::CtxIv1Spec>;
#[doc = ""]
pub mod ctx_iv1;
#[doc = "CTX_IV2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_iv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_iv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_iv2`] module"]
#[doc(alias = "CTX_IV2")]
pub type CtxIv2 = crate::Reg<ctx_iv2::CtxIv2Spec>;
#[doc = ""]
pub mod ctx_iv2;
#[doc = "CTX_IV3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_iv3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_iv3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_iv3`] module"]
#[doc(alias = "CTX_IV3")]
pub type CtxIv3 = crate::Reg<ctx_iv3::CtxIv3Spec>;
#[doc = ""]
pub mod ctx_iv3;
#[doc = "CTX_MAC0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_mac0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_mac0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_mac0`] module"]
#[doc(alias = "CTX_MAC0")]
pub type CtxMac0 = crate::Reg<ctx_mac0::CtxMac0Spec>;
#[doc = ""]
pub mod ctx_mac0;
#[doc = "CTX_MAC1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_mac1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_mac1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_mac1`] module"]
#[doc(alias = "CTX_MAC1")]
pub type CtxMac1 = crate::Reg<ctx_mac1::CtxMac1Spec>;
#[doc = ""]
pub mod ctx_mac1;
#[doc = "CTX_MAC2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_mac2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_mac2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_mac2`] module"]
#[doc(alias = "CTX_MAC2")]
pub type CtxMac2 = crate::Reg<ctx_mac2::CtxMac2Spec>;
#[doc = ""]
pub mod ctx_mac2;
#[doc = "CTX_MAC3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_mac3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_mac3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_mac3`] module"]
#[doc(alias = "CTX_MAC3")]
pub type CtxMac3 = crate::Reg<ctx_mac3::CtxMac3Spec>;
#[doc = ""]
pub mod ctx_mac3;
#[doc = "INGRESS_FIFO (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ingress_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ingress_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ingress_fifo`] module"]
#[doc(alias = "INGRESS_FIFO")]
pub type IngressFifo = crate::Reg<ingress_fifo::IngressFifoSpec>;
#[doc = ""]
pub mod ingress_fifo;
#[doc = "ING_DBCFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ing_dbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ing_dbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ing_dbcfg`] module"]
#[doc(alias = "ING_DBCFG")]
pub type IngDbcfg = crate::Reg<ing_dbcfg::IngDbcfgSpec>;
#[doc = ""]
pub mod ing_dbcfg;
#[doc = "DMA_ENG_LEN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_eng_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_eng_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_eng_len`] module"]
#[doc(alias = "DMA_ENG_LEN")]
pub type DmaEngLen = crate::Reg<dma_eng_len::DmaEngLenSpec>;
#[doc = ""]
pub mod dma_eng_len;
#[doc = "ENG_DBCFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`eng_dbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eng_dbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eng_dbcfg`] module"]
#[doc(alias = "ENG_DBCFG")]
pub type EngDbcfg = crate::Reg<eng_dbcfg::EngDbcfgSpec>;
#[doc = ""]
pub mod eng_dbcfg;
#[doc = "DMA_ING_LEN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ing_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ing_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ing_len`] module"]
#[doc(alias = "DMA_ING_LEN")]
pub type DmaIngLen = crate::Reg<dma_ing_len::DmaIngLenSpec>;
#[doc = ""]
pub mod dma_ing_len;
#[doc = "INGFIFIF_STATUS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ingfifif_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ingfifif_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ingfifif_status`] module"]
#[doc(alias = "INGFIFIF_STATUS")]
pub type IngfififStatus = crate::Reg<ingfifif_status::IngfififStatusSpec>;
#[doc = ""]
pub mod ingfifif_status;
#[doc = "ENGRESS_FIFO (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`engress_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`engress_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@engress_fifo`] module"]
#[doc(alias = "ENGRESS_FIFO")]
pub type EngressFifo = crate::Reg<engress_fifo::EngressFifoSpec>;
#[doc = ""]
pub mod engress_fifo;
#[doc = "ENGFIFO_STATUS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`engfifo_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`engfifo_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@engfifo_status`] module"]
#[doc(alias = "ENGFIFO_STATUS")]
pub type EngfifoStatus = crate::Reg<engfifo_status::EngfifoStatusSpec>;
#[doc = ""]
pub mod engfifo_status;
#[doc = "ING_DMA_DONE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ing_dma_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ing_dma_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ing_dma_done`] module"]
#[doc(alias = "ING_DMA_DONE")]
pub type IngDmaDone = crate::Reg<ing_dma_done::IngDmaDoneSpec>;
#[doc = ""]
pub mod ing_dma_done;
#[doc = "ENG_DMA_DONE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`eng_dma_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eng_dma_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eng_dma_done`] module"]
#[doc(alias = "ENG_DMA_DONE")]
pub type EngDmaDone = crate::Reg<eng_dma_done::EngDmaDoneSpec>;
#[doc = ""]
pub mod eng_dma_done;
#[doc = "DONE_STATUS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`done_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`done_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@done_status`] module"]
#[doc(alias = "DONE_STATUS")]
pub type DoneStatus = crate::Reg<done_status::DoneStatusSpec>;
#[doc = ""]
pub mod done_status;
