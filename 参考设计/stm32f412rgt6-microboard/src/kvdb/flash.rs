//! Flash 操作 trait 定义。
//!
//! 对底层 NOR Flash（如 QSPI NOR Flash）的抽象接口。
//! NOR Flash 的特性：
//! - 按字节读取
//! - 只能将 bit 从 1 编程为 0（不能从 0 变回 1）
//! - 擦除以扇区/块为单位，擦除后所有字节恢复为 0xFF
//!
//! Trait 采用层次结构，避免关联类型歧义：
//! `FlashRead` → `FlashWrite` → `FlashErase`

use embassy_stm32::qspi::Qspi;

/// 从 Flash 读取数据。
pub trait FlashRead {
    /// Flash 操作错误类型。
    type Error;

    /// 从 `addr` 地址读取 `buf.len()` 字节到 `buf`。
    fn read(&mut self, addr: u32, buf: &mut [u8]) -> Result<(), Self::Error>;
}

/// 向 Flash 写入数据。
///
/// # 注意
/// NOR Flash 只能将 bit 从 1 变为 0。写入前目标区域必须处于擦除状态（全 0xFF）。
pub trait FlashWrite: FlashRead {
    /// 从 `addr` 地址开始写入 `buf` 中的字节。
    fn write(&mut self, addr: u32, buf: &[u8]) -> Result<(), Self::Error>;
}

/// 擦除 Flash 扇区。
pub trait FlashErase: FlashWrite {
    /// 擦除从 `addr` 开始的一个扇区（大小由实现决定）。
    /// 擦除后该扇区所有字节恢复为 0xFF。
    fn erase(&mut self, addr: u32) -> Result<(), Self::Error>;
}

use embassy_stm32::{
    peripherals,
    qspi::{
        TransferConfig,
        enums::{DummyCycles, QspiWidth},
    },
};


/// 实现自己的Flash
pub struct MyFlash {
    pub qspi: Qspi<'static, peripherals::QUADSPI, embassy_stm32::mode::Blocking>,
}

impl MyFlash {
    pub fn new(qspi: Qspi<'static, peripherals::QUADSPI, embassy_stm32::mode::Blocking>) -> Self {
        Self { qspi }
    }

    fn wait_ready(&mut self) {
        let mut status = [0u8; 1];
        loop {
            self.qspi.blocking_read(
                &mut status,
                TransferConfig {
                    iwidth: QspiWidth::SING,
                    awidth: QspiWidth::NONE,
                    dwidth: QspiWidth::SING,
                    instruction: 0x05,
                    address: None,
                    dummy: DummyCycles::_0,
                },
            );
            if status[0] & 0x01 == 0 {
                return;
            }
        }
    }

    fn write_enable(&mut self) {
        self.qspi.blocking_command(TransferConfig {
            iwidth: QspiWidth::SING,
            awidth: QspiWidth::NONE,
            dwidth: QspiWidth::NONE,
            instruction: 0x06,
            address: None,
            dummy: DummyCycles::_0,
        });
    }
}

impl FlashRead for MyFlash {
    type Error = ();

    fn read(&mut self, addr: u32, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.qspi.blocking_read(
            buf,
            TransferConfig {
                iwidth: QspiWidth::SING,
                awidth: QspiWidth::SING,
                dwidth: QspiWidth::SING,
                instruction: 0x03,
                address: Some(addr),
                dummy: DummyCycles::_0,
            },
        );
        Ok(())
    }
}

impl FlashWrite for MyFlash {
    fn write(&mut self, addr: u32, buf: &[u8]) -> Result<(), Self::Error> {
        self.write_enable();
        self.qspi.blocking_write(
            buf,
            TransferConfig {
                iwidth: QspiWidth::SING,
                awidth: QspiWidth::SING,
                dwidth: QspiWidth::SING,
                instruction: 0x02,
                address: Some(addr),
                dummy: DummyCycles::_0,
            },
        );
        self.wait_ready();
        Ok(())
    }
}

impl FlashErase for MyFlash {
    fn erase(&mut self, addr: u32) -> Result<(), Self::Error> {
        self.write_enable();
        self.qspi.blocking_command(TransferConfig {
            iwidth: QspiWidth::SING,
            awidth: QspiWidth::SING,
            dwidth: QspiWidth::NONE,
            instruction: 0x20,
            address: Some(addr),
            dummy: DummyCycles::_0,
        });
        self.wait_ready();
        Ok(())
    }
}
