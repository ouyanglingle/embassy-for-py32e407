//! 嵌入式键值数据库模块。
//!
//! 特性：
//! - `no_std`，无动态内存分配（基于 `heapless`）
//! - 磨损均衡：扇区轮转
//! - 掉电安全：先写数据后写标志位 + CRC16 校验
//! - 日志结构存储

pub mod db;
pub mod flash;

pub use db::Kvdb;
pub use flash::{FlashErase, FlashRead, FlashWrite, MyFlash};
