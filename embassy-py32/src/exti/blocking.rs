//! Blocking-mode EXTI utilities（对齐 embassy-stm32 的 `exti/blocking.rs`）
//!
//! 本模块提供手工处理 EXTI 中断时用的工具类型。主阻塞驱动是
//! [`ExtiInput`](super::ExtiInput)。

use super::{ExtiInput, low_level};

/// 预计算的位掩码：一次清掉同一中断向量下多条线的挂起位。
///
/// 用途同 embassy-stm32 的同名类型——PY32E407 把多条线合并进一个向量：
/// `EXTI9_5`（线 5–9，IRQ 23）与 `EXTI15_10`（线 10–15，IRQ 40）由 RM 13.1.3 给出，
/// 而 `EXTI0`–`EXTI4` 各占一个独立向量（IRQ 6–10）。对于共向量的那一组，
/// 可以在中断服务函数里用预先算好的掩码**一次整写**清掉全部挂起位，
/// 而不是逐条 `clear_pending()`（每条都要一次读-改-写）。
///
/// # 使用前提（与 embassy-stm32 的说明一致）
///
/// * 传入的输入**必须**都属于同一个 IRQ 组（例如全部是 10–15 线的输入）；
/// * 混入别的组的引脚会连带清掉其它向量的挂起位，导致**漏中断**；
/// * 线号到向量的对应关系查 RM 13.1.3。
pub struct ExtiGroupMask(u32);

impl ExtiGroupMask {
    /// 由同一 IRQ 组内的若干 [`ExtiInput`] 计算掩码。
    ///
    /// 掩码算一次即可复用，适合放在向量处理函数里。
    pub fn new(inputs: &[&ExtiInput<'_>]) -> Self {
        let mut mask = 0;
        for input in inputs {
            mask |= input.pin_mask();
        }
        Self(mask)
    }

    /// 清掉掩码内的全部挂起位（一次写 `PR`）。
    pub fn clear(&self) {
        low_level::clear_exti_pending_mask(self.0);
    }
}
