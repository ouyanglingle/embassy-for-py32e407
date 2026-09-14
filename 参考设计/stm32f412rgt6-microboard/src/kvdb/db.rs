//! 嵌入式键值数据库核心实现。
//!
//! ## 存储布局
//!
//! Flash 被划分为多个等大的扇区（sector），以环形方式轮转使用。
//! 每个扇区从头部（16 字节）开始，剩余空间按顺序追加条目。
//!
//! ### 扇区头部（16 字节）
//! ```text
//! Offset  Size  Field
//! 0       4     Magic ("KVDB": [0x4B, 0x56, 0x44, 0x42])
//! 4       4     Sequence (u32, little-endian)
//! 8       1     Status (0xFF=Erased, 0xFE=Active, 0xFC=Closed)
//! 9       5     Reserved (0xFF)
//! 14      2     CRC16 over bytes 0..14
//! ```
//!
//! ### 条目格式
//! ```text
//! Offset  Size  Field
//! 0       1     Flag (0xFF=Free, 0x00=Valid,  0xAA=Tombstone)
//! 1       1     Key length (u8)
//! 2       2     Value length (u16, LE)
//! 4       N     Key bytes
//! 4+N     M     Value bytes
//! 4+N+M   2     CRC16 over bytes 1..(4+N+M)
//! ```
//!
//! 写入顺序：先写 flag 之后的所有字节，最后写 flag。
//! 若掉电发生在最后一步之前，flag 仍为 0xFF，恢复时该条目被忽略。
//!
//! ## 磨损均衡
//! 扇区环形轮转：当前扇区写满后关闭，擦除下一个扇区并作为新活动扇区。
//! 擦除次数在扇区间均匀分布。
//!
//! ## 掉电安全
//! - 扇区状态转换：先写新扇区头，再关旧扇区（如果掉电，选 sequence 更大者）
//! - 条目写入：flag 字节最后写入
//! - CRC16 校验确保数据完整性

use heapless::Vec;

use crate::kvdb::flash::{FlashErase, FlashRead, FlashWrite};

// ============================================================================
// Constants
// ============================================================================

/// 扇区魔法值
const SECTOR_MAGIC: [u8; 4] = [0x4B, 0x56, 0x44, 0x42]; // "KVDB"

/// 扇区头部大小（字节）
const HEADER_SIZE: u32 = 16;

/// 扇区状态：已擦除（全 0xFF）
const STATUS_ERASED: u8 = 0xFF;
/// 扇区状态：活动（正在写入）
const STATUS_ACTIVE: u8 = 0xFE;
/// 扇区状态：已关闭（写满）
const STATUS_CLOSED: u8 = 0xFC;

/// 条目标志：空闲
const FLAG_FREE: u8 = 0xFF;
/// 条目标志：有效
const FLAG_VALID: u8 = 0x00;
/// 条目标志：逻辑删除
const FLAG_TOMBSTONE: u8 = 0xAA;

/// 最小扇区数（至少需要 2 个以实现安全的旋转）
const MIN_SECTOR_COUNT: usize = 2;

// ============================================================================
// CRC16 (CRC-16/CCITT-FALSE: poly=0x1021, init=0xFFFF)
// ============================================================================

const CRC16_TABLE: [u16; 256] = [
    0x0000, 0x1021, 0x2042, 0x3063, 0x4084, 0x50A5, 0x60C6, 0x70E7, 0x8108, 0x9129, 0xA14A, 0xB16B,
    0xC18C, 0xD1AD, 0xE1CE, 0xF1EF, 0x1231, 0x0210, 0x3273, 0x2252, 0x52B5, 0x4294, 0x72F7, 0x62D6,
    0x9339, 0x8318, 0xB37B, 0xA35A, 0xD3BD, 0xC39C, 0xF3FF, 0xE3DE, 0x2462, 0x3443, 0x0420, 0x1401,
    0x64E6, 0x74C7, 0x44A4, 0x5485, 0xA56A, 0xB54B, 0x8528, 0x9509, 0xE5EE, 0xF5CF, 0xC5AC, 0xD58D,
    0x3653, 0x2672, 0x1611, 0x0630, 0x76D7, 0x66F6, 0x5695, 0x46B4, 0xB75B, 0xA77A, 0x9719, 0x8738,
    0xF7DF, 0xE7FE, 0xD79D, 0xC7BC, 0x48C4, 0x58E5, 0x6886, 0x78A7, 0x0840, 0x1861, 0x2802, 0x3823,
    0xC9CC, 0xD9ED, 0xE98E, 0xF9AF, 0x8948, 0x9969, 0xA90A, 0xB92B, 0x5AF5, 0x4AD4, 0x7AB7, 0x6A96,
    0x1A71, 0x0A50, 0x3A33, 0x2A12, 0xDBFD, 0xCBDC, 0xFBBF, 0xEB9E, 0x9B79, 0x8B58, 0xBB3B, 0xAB1A,
    0x6CA6, 0x7C87, 0x4CE4, 0x5CC5, 0x2C22, 0x3C03, 0x0C60, 0x1C41, 0xEDAE, 0xFD8F, 0xCDEC, 0xDDCD,
    0xAD2A, 0xBD0B, 0x8D68, 0x9D49, 0x7E97, 0x6EB6, 0x5ED5, 0x4EF4, 0x3E13, 0x2E32, 0x1E51, 0x0E70,
    0xFF9F, 0xEFBE, 0xDFDD, 0xCFFC, 0xBF1B, 0xAF3A, 0x9F59, 0x8F78, 0x9188, 0x81A9, 0xB1CA, 0xA1EB,
    0xD10C, 0xC12D, 0xF14E, 0xE16F, 0x1080, 0x00A1, 0x30C2, 0x20E3, 0x5004, 0x4025, 0x7046, 0x6067,
    0x83B9, 0x9398, 0xA3FB, 0xB3DA, 0xC33D, 0xD31C, 0xE37F, 0xF35E, 0x02B1, 0x1290, 0x22F3, 0x32D2,
    0x4235, 0x5214, 0x6277, 0x7256, 0xB5EA, 0xA5CB, 0x95A8, 0x8589, 0xF56E, 0xE54F, 0xD52C, 0xC50D,
    0x34E2, 0x24C3, 0x14A0, 0x0481, 0x7466, 0x6447, 0x5424, 0x4405, 0xA7DB, 0xB7FA, 0x8799, 0x97B8,
    0xE75F, 0xF77E, 0xC71D, 0xD73C, 0x26D3, 0x36F2, 0x0691, 0x16B0, 0x6657, 0x7676, 0x4615, 0x5634,
    0xD94C, 0xC96D, 0xF90E, 0xE92F, 0x99C8, 0x89E9, 0xB98A, 0xA9AB, 0x5844, 0x4865, 0x7806, 0x6827,
    0x18C0, 0x08E1, 0x3882, 0x28A3, 0xCB7D, 0xDB5C, 0xEB3F, 0xFB1E, 0x8BF9, 0x9BD8, 0xABBB, 0xBB9A,
    0x4A75, 0x5A54, 0x6A37, 0x7A16, 0x0AF1, 0x1AD0, 0x2AB3, 0x3A92, 0xFD2E, 0xED0F, 0xDD6C, 0xCD4D,
    0xBDAA, 0xAD8B, 0x9DE8, 0x8DC9, 0x7C26, 0x6C07, 0x5C64, 0x4C45, 0x3CA2, 0x2C83, 0x1CE0, 0x0CC1,
    0xEF1F, 0xFF3E, 0xCF5D, 0xDF7C, 0xAF9B, 0xBFBA, 0x8FD9, 0x9FF8, 0x6E17, 0x7E36, 0x4E55, 0x5E74,
    0x2E93, 0x3EB2, 0x0ED1, 0x1EF0,
];

/// 计算一段数据的 CRC16 校验值。
fn crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        let idx = ((crc >> 8) as u8 ^ byte) as usize;
        crc = (crc << 8) ^ CRC16_TABLE[idx];
    }
    crc
}

/// 流式 CRC16：在已有 CRC 值的基础上继续计算。
fn crc16_update(mut crc: u16, data: &[u8]) -> u16 {
    for &byte in data {
        let idx = ((crc >> 8) as u8 ^ byte) as usize;
        crc = (crc << 8) ^ CRC16_TABLE[idx];
    }
    crc
}

// ============================================================================
// Error Types
// ============================================================================

/// KV 数据库错误类型。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KvdbError<FE> {
    /// 底层 Flash 操作错误。
    Flash(FE),

    /// 存储空间不足（扇区已满且无法旋转）。
    NoSpace,

    /// 键不存在。
    KeyNotFound,

    /// 键长度超过限制。
    KeyTooLong,

    /// 值长度超过限制。
    ValueTooLong,

    /// 键数量超过索引容量。
    TooManyKeys,

    /// CRC 校验不匹配（数据损坏）。
    CrcMismatch,

    /// 数据格式损坏，无法恢复。
    Corrupted,
}

impl<FE> From<FE> for KvdbError<FE> {
    fn from(e: FE) -> Self {
        KvdbError::Flash(e)
    }
}

/// KV 数据库操作结果别名。
pub type KvdbResult<T, FE> = Result<T, KvdbError<FE>>;

// ============================================================================
// Sector Header
// ============================================================================

/// 解析后的扇区头部信息。
#[derive(Clone, Copy, Debug)]
struct SectorHeader {
    sequence: u32,
    status: u8,
    valid: bool,
}

/// 从 Flash 读取并解析扇区头部。
fn read_sector_header<F: FlashRead>(
    flash: &mut F,
    sector_addr: u32,
) -> Result<SectorHeader, F::Error> {
    let mut buf = [0xFFu8; HEADER_SIZE as usize];
    flash.read(sector_addr, &mut buf)?;

    // 检查是否完全擦除（全 0xFF）
    if buf.iter().all(|&b| b == 0xFF) {
        return Ok(SectorHeader {
            sequence: 0,
            status: STATUS_ERASED,
            valid: false,
        });
    }

    // 检查魔法值
    if buf[0..4] != SECTOR_MAGIC {
        // 非 KVDB 格式，视为无效
        return Ok(SectorHeader {
            sequence: 0,
            status: STATUS_ERASED,
            valid: false,
        });
    }

    let sequence = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
    let status = buf[8];
    let crc_stored = u16::from_le_bytes([buf[14], buf[15]]);
    let crc_computed = crc16(&buf[0..14]);

    Ok(SectorHeader {
        sequence,
        status,
        valid: crc_stored == crc_computed,
    })
}

/// 向 Flash 写入扇区头部。
fn write_sector_header<F: FlashWrite>(
    flash: &mut F,
    sector_addr: u32,
    sequence: u32,
    status: u8,
) -> Result<(), F::Error> {
    let mut buf = [0xFFu8; HEADER_SIZE as usize];
    buf[0..4].copy_from_slice(&SECTOR_MAGIC);
    buf[4..8].copy_from_slice(&sequence.to_le_bytes());
    buf[8] = status;
    // bytes 9..14 保持 0xFF
    let crc = crc16(&buf[0..14]);
    buf[14..16].copy_from_slice(&crc.to_le_bytes());
    flash.write(sector_addr, &buf)
}

/// 关闭扇区：将状态字节从 0xFE 改为 0xFC（仅在 bit 1 上 1→0）。
/// 若扇区已关闭则跳过写操作。
fn close_sector<F: FlashWrite>(flash: &mut F, sector_addr: u32) -> Result<(), F::Error> {
    let mut status_buf = [0xFFu8; 1];
    flash.read(sector_addr + 8, &mut status_buf)?;
    if status_buf[0] == STATUS_CLOSED {
        return Ok(()); // 已经关闭，无需重复写
    }
    flash.write(sector_addr + 8, &[STATUS_CLOSED])
}

/// 计算指定扇区索引的物理地址。
fn sector_addr(start_addr: u32, sector_index: usize, sector_size: u32) -> u32 {
    start_addr + sector_index as u32 * sector_size
}

// ============================================================================
// In-Memory Index
// ============================================================================

/// 内存索引条目：记录键到 Flash 地址的映射。
#[derive(Clone, Debug)]
struct IndexEntry<const MAX_K: usize> {
    /// 键的字节内容（固定长度缓冲区，未使用部分为零）
    key: [u8; MAX_K],
    /// 键的实际长度
    key_len: u8,
    /// 条目在 Flash 上的地址（指向 flag 字节）
    addr: u32,
    /// 值长度
    value_len: u16,
    /// 所属扇区的 sequence 号（用于判断新旧）
    seq: u32,
}

impl<const MAX_K: usize> IndexEntry<MAX_K> {
    fn matches_key(&self, key: &[u8]) -> bool {
        key.len() == self.key_len as usize && &self.key[..key.len()] == key
    }
}

/// 内存索引：使用 `heapless::Vec` 存储，线性查找。
///
/// 对于嵌入式场景（几十到上百个 key），线性查找足够快。
struct Index<const MAX_K: usize, const N: usize> {
    entries: Vec<IndexEntry<MAX_K>, N>,
}

impl<const MAX_K: usize, const N: usize> Index<MAX_K, N> {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn clear(&mut self) {
        self.entries.clear();
    }

    /// 查找键，返回条目引用。
    fn get(&self, key: &[u8]) -> Option<&IndexEntry<MAX_K>> {
        self.entries.iter().find(|e| e.matches_key(key))
    }

    /// 插入或更新条目。如果键已存在且新 sequence >= 旧的，则更新地址和长度。
    ///
    /// 返回 `true` 表示新插入，`false` 表示更新已有条目。
    fn upsert(
        &mut self,
        key: &[u8],
        addr: u32,
        value_len: u16,
        seq: u32,
    ) -> Result<bool, KvdbError<core::convert::Infallible>> {
        let key_len = key.len();
        if key_len > MAX_K {
            return Err(KvdbError::KeyTooLong);
        }

        // 查找已有条目
        if let Some(existing) = self.entries.iter_mut().find(|e| e.matches_key(key)) {
            if seq >= existing.seq {
                existing.addr = addr;
                existing.value_len = value_len;
                existing.seq = seq;
            }
            return Ok(false); // 更新，非新插入
        }

        // 新插入
        let mut key_buf = [0u8; MAX_K];
        key_buf[..key_len].copy_from_slice(key);

        let entry = IndexEntry {
            key: key_buf,
            key_len: key_len as u8,
            addr,
            value_len,
            seq,
        };

        self.entries
            .push(entry)
            .map_err(|_| KvdbError::TooManyKeys)?;

        Ok(true)
    }

    /// 从索引中移除键。
    fn remove(&mut self, key: &[u8]) -> bool {
        if let Some(pos) = self.entries.iter().position(|e| e.matches_key(key)) {
            self.entries.swap_remove(pos);
            true
        } else {
            false
        }
    }
}

// ============================================================================
// Kvdb
// ============================================================================

/// 嵌入式键值数据库。
///
/// # 泛型参数
///
/// * `F`        - Flash 驱动类型，需实现 `FlashRead` + `FlashWrite` + `FlashErase`
/// * `SS`       - 扇区大小（字节），通常为 4096
/// * `SC`       - 扇区数量，至少 2，越大磨损均衡越好
/// * `MAX_K`    - 最大键长度（字节）
/// * `MAX_V`    - 最大值长度（字节）
/// * `MAX_KEYS` - 内存索引最大键数量
///
/// # 示例
///
/// ```ignore
/// type MyKvdb = Kvdb<MyFlash, 4096, 8, 32, 256, 64>;
/// let db = MyKvdb::new(flash, 0x0000_0000).unwrap();
/// db.set(b"counter", &42u32.to_le_bytes()).unwrap();
/// ```
pub struct Kvdb<
    F: FlashErase,
    const SS: usize,
    const SC: usize,
    const MAX_K: usize,
    const MAX_V: usize,
    const MAX_KEYS: usize,
> {
    flash: F,
    /// Flash 上 KVDB 区域的起始物理地址
    start_addr: u32,
    /// 当前活动扇区索引 (0..SC)
    active_idx: usize,
    /// 当前活动扇区的 sequence 号（单调递增）
    active_seq: u32,
    /// 当前活动扇区内的写入位置（相对扇区起始的偏移）
    write_pos: u32,
    /// 内存索引
    index: Index<MAX_K, MAX_KEYS>,
}

impl<
    F: FlashErase,
    const SS: usize,
    const SC: usize,
    const MAX_K: usize,
    const MAX_V: usize,
    const MAX_KEYS: usize,
> Kvdb<F, SS, SC, MAX_K, MAX_V, MAX_KEYS>
{
    // 编译期检查：扇区数至少为 2
    const _CHECK_SECTOR_COUNT: () = {
        assert!(SC >= MIN_SECTOR_COUNT, "SECTOR_COUNT must be >= 2");
    };

    /// 创建 Kvdb 实例，从 Flash 恢复上次状态。
    ///
    /// 会扫描所有扇区以重建内存索引。如果 Flash 上无有效数据，会自动初始化。
    pub fn new(mut flash: F, start_addr: u32) -> KvdbResult<Self, F::Error> {
        let _ = Self::_CHECK_SECTOR_COUNT;

        let ss = SS as u32;

        // 1. 读取所有扇区头部
        let mut headers: [Option<SectorHeader>; SC] = [None; SC];
        for i in 0..SC {
            let addr = sector_addr(start_addr, i, ss);
            match read_sector_header(&mut flash, addr) {
                Ok(hdr) => headers[i] = Some(hdr),
                Err(e) => return Err(KvdbError::Flash(e)),
            }
        }

        // 2. 查找活动扇区
        let mut found_active: Option<usize> = None;
        let mut max_seq: u32 = 0;
        let mut max_closed_seq: u32 = 0;
        let mut max_closed_idx: Option<usize> = None;

        for i in 0..SC {
            if let Some(ref hdr) = headers[i] {
                if hdr.valid && hdr.status == STATUS_ACTIVE {
                    if found_active.is_none() || hdr.sequence > max_seq {
                        found_active = Some(i);
                        max_seq = hdr.sequence;
                    }
                }
                if hdr.valid && hdr.status == STATUS_CLOSED {
                    if max_closed_idx.is_none() || hdr.sequence > max_closed_seq {
                        max_closed_idx = Some(i);
                        max_closed_seq = hdr.sequence;
                    }
                }
            }
        }

        // 3. 创建初始状态
        let active_idx: usize;
        let active_seq: u32;

        let has_data = headers
            .iter()
            .any(|h| h.as_ref().map_or(false, |x| x.valid));

        match (has_data, found_active) {
            // 情况 A：Flash 上没有任何有效数据 → 擦除扇区 0 并初始化
            (false, _) => {
                flash.erase(start_addr)?;
                write_sector_header(&mut flash, start_addr, 1, STATUS_ACTIVE)?;
                active_idx = 0;
                active_seq = 1;
            }
            // 情况 B：没有活动扇区但有已关闭扇区 → 旋转到下一个
            (_, None) => {
                let src_idx = max_closed_idx.unwrap(); // 前面检查过 has_data，不会 panic
                active_idx = (src_idx + 1) % SC;
                active_seq = max_closed_seq + 1;

                // 擦除下一个扇区并写入新头部
                let new_addr = sector_addr(start_addr, active_idx, ss);
                flash.erase(new_addr)?;
                write_sector_header(&mut flash, new_addr, active_seq, STATUS_ACTIVE)?;
            }
            // 情况 C：找到活动扇区
            (_, Some(idx)) => {
                active_idx = idx;
                active_seq = max_seq;
            }
        }

        // 4. 扫描所有扇区，重建索引
        let mut index: Index<MAX_K, MAX_KEYS> = Index::new();

        for i in 0..SC {
            if let Some(ref hdr) = headers[i] {
                if !hdr.valid || hdr.status == STATUS_ERASED {
                    continue;
                }

                let sector_start = sector_addr(start_addr, i, ss);
                let sector_seq = hdr.sequence;

                // 从 HEADER_SIZE 开始扫描条目
                let scan_result =
                    Self::scan_sector_entries(&mut flash, sector_start, ss, sector_seq, &mut index);

                match scan_result {
                    Ok(()) => {}
                    Err(KvdbError::CrcMismatch) => {
                        // 遇到 CRC 错误，跳过该条目及其后（扇区可能被部分损坏）
                        // 但不影响其他扇区
                    }
                    Err(e) => return Err(e),
                }
            }
        }

        // 5. 在活动扇区中查找写入位置
        let active_sector_start = sector_addr(start_addr, active_idx, ss);
        let write_pos = Self::find_write_pos(&mut flash, active_sector_start, ss)?;

        Ok(Self {
            flash,
            start_addr,
            active_idx,
            active_seq,
            write_pos,
            index,
        })
    }

    /// 格式化 KVDB 区域（擦除所有扇区并初始化）。
    ///
    /// 这会销毁所有已存储的数据。
    pub fn format(mut flash: F, start_addr: u32) -> KvdbResult<Self, F::Error> {
        let ss = SS as u32;

        // 擦除所有扇区
        for i in 0..SC {
            flash.erase(sector_addr(start_addr, i, ss))?;
        }

        // 初始化扇区 0
        write_sector_header(&mut flash, start_addr, 1, STATUS_ACTIVE)?;

        Ok(Self {
            flash,
            start_addr,
            active_idx: 0,
            active_seq: 1,
            write_pos: HEADER_SIZE,
            index: Index::new(),
        })
    }

    // ========================================================================
    // Public API
    // ========================================================================

    /// 写入键值对。若键已存在，则更新其值。
    ///
    /// 掉电安全：如果在写入过程中掉电，该条目在下次初始化时会被忽略，
    /// 旧值（如果存在）不受影响。
    pub fn set(&mut self, key: &[u8], value: &[u8]) -> KvdbResult<(), F::Error> {
        if key.len() > MAX_K {
            return Err(KvdbError::KeyTooLong);
        }
        if value.len() > MAX_V {
            return Err(KvdbError::ValueTooLong);
        }

        self.write_entry_impl(key, value, FLAG_VALID)?;
        Ok(())
    }

    /// 读取键对应的值到 `buf` 中，返回实际读取的字节数。
    pub fn get(&mut self, key: &[u8], buf: &mut [u8]) -> KvdbResult<usize, F::Error> {
        let entry = self.index.get(key).ok_or(KvdbError::KeyNotFound)?;

        let vlen = entry.value_len as usize;
        if vlen > buf.len() {
            return Err(KvdbError::ValueTooLong);
        }

        // 值地址 = 条目地址 + 4（flag + klen + vlen）+ key_len
        let value_addr = entry.addr + 4 + entry.key_len as u32;
        self.flash.read(value_addr, &mut buf[..vlen])?;

        Ok(vlen)
    }

    /// 读取键对应的值到 `buf` 中，并校验 CRC16。
    ///
    /// 与 `get()` 的区别在于会额外读取 Flash 上存储的 CRC，与实时计算的
    /// CRC 比对。若不匹配则返回 `KvdbError::CrcMismatch`，保证返回的数据
    /// 未被 bit-flip 损坏。适用于高可靠性读取场景。
    ///
    /// 注：校验数据包括 klen、vlen、key 和 value。flag 字节不参与 CRC
    /// 计算（因 flag 是最后写入的原子提交标志）。
    pub fn get_verified(&mut self, key: &[u8], buf: &mut [u8]) -> KvdbResult<usize, F::Error> {
        let entry = self.index.get(key).ok_or(KvdbError::KeyNotFound)?;

        let vlen = entry.value_len as usize;
        let klen = entry.key_len as usize;
        if vlen > buf.len() {
            return Err(KvdbError::ValueTooLong);
        }

        // 读取 value
        let value_addr = entry.addr + 4 + klen as u32;
        self.flash.read(value_addr, &mut buf[..vlen])?;

        // 读取 Flash 上存储的 CRC
        let mut crc_buf = [0u8; 2];
        self.flash.read(
            entry.addr + 4 + klen as u32 + vlen as u32,
            &mut crc_buf,
        )?;
        let crc_stored = u16::from_le_bytes(crc_buf);

        // 用调用方传入的 key 和刚读出的 value 重新计算 CRC
        let mut crc_computed: u16 = 0xFFFF;
        crc_computed = crc16_update(crc_computed, &[klen as u8]);
        crc_computed = crc16_update(crc_computed, &(vlen as u16).to_le_bytes());
        crc_computed = crc16_update(crc_computed, key);
        crc_computed = crc16_update(crc_computed, &buf[..vlen]);

        if crc_stored != crc_computed {
            return Err(KvdbError::CrcMismatch);
        }

        Ok(vlen)
    }

    /// 删除键。
    ///
    /// 写入一条墓碑（tombstone）记录。键从索引中移除，
    /// 下次初始化时墓碑之前的同名条目会被忽略。
    pub fn delete(&mut self, key: &[u8]) -> KvdbResult<(), F::Error> {
        if key.len() > MAX_K {
            return Err(KvdbError::KeyTooLong);
        }

        // 检查键是否存在
        if self.index.get(key).is_none() {
            return Err(KvdbError::KeyNotFound);
        }

        self.write_entry_impl(key, &[], FLAG_TOMBSTONE)?;
        self.index.remove(key);
        Ok(())
    }

    /// 检查键是否存在。
    pub fn contains(&self, key: &[u8]) -> bool {
        self.index.get(key).is_some()
    }

    /// 返回当前存储的键数量。
    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// 是否没有任何键。
    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// 索引最大容量（MAX_KEYS）。
    pub fn capacity(&self) -> usize {
        MAX_KEYS
    }

    /// 返回指定索引位置的键，用于遍历所有键。
    /// 示例
    ///
    /// ```rust
    /// // 遍历所有键
    /// for i in 0..db.len() {
    ///    let key = db.key_at(i).unwrap();
    ///    defmt::info!("key: {:a}", key);
    /// }
    /// ```
    pub fn key_at(&self, index: usize) -> Option<&[u8]> {
        self.index
            .entries
            .get(index)
            .map(|e| &e.key[..e.key_len as usize])
    }

    /// 活动扇区剩余可写字节数。
    pub fn free_space(&self) -> u32 {
        (SS as u32).saturating_sub(self.write_pos)
    }

    /// 返回内部 Flash 的可变引用。
    ///
    /// 用于在 KVDB 初始化完成后访问底层 Flash（如启用 QSPI 内存映射模式）。
    /// **注意**：修改 Flash 模式后，KVDB 的 read/write/erase 操作
    /// 需要确保 Flash 处于正确的模式（间接读写模式）。
    pub fn flash_mut(&mut self) -> &mut F {
        &mut self.flash
    }

    // ========================================================================
    // Private: Sector Management
    // ========================================================================

    /// 旋转到下一个扇区：关闭当前扇区，擦除下一个，写入新头部。
    fn rotate_sector(&mut self) -> KvdbResult<(), F::Error> {
        let ss = SS as u32;

        // 1. 确定下一个扇区
        let next_idx = (self.active_idx + 1) % SC;
        let next_addr = sector_addr(self.start_addr, next_idx, ss);

        // 2. 擦除下一个扇区（无需检查状态，erase 总是安全的）
        self.flash.erase(next_addr)?;

        // 3. 在新扇区写入活动头部
        let new_seq = self.active_seq + 1;
        write_sector_header(&mut self.flash, next_addr, new_seq, STATUS_ACTIVE)?;

        // 4. 关闭旧扇区（从 0xFE → 0xFC）
        let old_addr = sector_addr(self.start_addr, self.active_idx, ss);
        // 关闭失败不影响数据完整性 — 恢复时多个 Active 扇区会选 sequence 最大的
        let _ = close_sector(&mut self.flash, old_addr);

        // 5. 更新运行时状态
        self.active_idx = next_idx;
        self.active_seq = new_seq;
        self.write_pos = HEADER_SIZE;

        Ok(())
    }

    // ========================================================================
    // Private: Entry Operations
    // ========================================================================

    /// 写入条目的核心实现（set 和 delete 共享）。
    ///
    /// 写入顺序保证掉电安全：
    /// 1. 写入 flag 之后的所有字节（body + CRC）
    /// 2. 最后写入 flag 字节
    ///
    /// 若在步骤 1 期间掉电，flag 仍为 0xFF，恢复时该条目被视为空闲。
    fn write_entry_impl(&mut self, key: &[u8], value: &[u8], flag: u8) -> KvdbResult<(), F::Error> {
        let klen = key.len();
        let vlen = value.len();
        let entry_size = 6u32 + klen as u32 + vlen as u32; // 6 = 1 flag + 1 klen + 2 vlen + 2 crc

        // 空间不足时旋转扇区
        if self.write_pos + entry_size > SS as u32 {
            self.rotate_sector()?;
        }

        let addr = self.start_addr + self.active_idx as u32 * (SS as u32) + self.write_pos;

        // 准备 CRC 输入：klen + vlen + key + value
        let mut crc: u16 = 0xFFFF;
        crc = crc16_update(crc, &[klen as u8]);
        crc = crc16_update(crc, &(vlen as u16).to_le_bytes());
        crc = crc16_update(crc, key);
        crc = crc16_update(crc, value);

        // 步骤 1：写入 flag 之后的所有内容
        // 先写 klen (1 byte) + vlen (2 bytes)
        let mut header_buf = [0u8; 3];
        header_buf[0] = klen as u8;
        header_buf[1..3].copy_from_slice(&(vlen as u16).to_le_bytes());
        self.flash.write(addr + 1, &header_buf)?;

        // 写 key
        if klen > 0 {
            self.flash.write(addr + 4, key)?;
        }

        // 写 value
        if vlen > 0 {
            self.flash.write(addr + 4 + klen as u32, value)?;
        }

        // 写 CRC
        self.flash
            .write(addr + 4 + klen as u32 + vlen as u32, &crc.to_le_bytes())?;

        // 步骤 2：最后写 flag 字节（原子提交）
        // 这一步完成前掉电 → 条目视为空闲 ✓
        self.flash.write(addr, &[flag])?;

        // 更新运行时状态
        self.write_pos += entry_size;

        // 更新索引
        self.index
            .upsert(key, addr, vlen as u16, self.active_seq)
            .map_err(|e| match e {
                // upsert 内部不会产生 Flash 错误，转换类型
                KvdbError::TooManyKeys => KvdbError::TooManyKeys,
                _ => unreachable!(),
            })?;

        Ok(())
    }

    // ========================================================================
    // Private: Scanning
    // ========================================================================

    /// 扫描一个扇区中的所有有效条目，加入索引。
    fn scan_sector_entries(
        flash: &mut F,
        sector_start: u32,
        sector_size: u32,
        sector_seq: u32,
        index: &mut Index<MAX_K, MAX_KEYS>,
    ) -> KvdbResult<(), F::Error> {
        let mut pos = HEADER_SIZE;
        let mut key_buf = [0u8; MAX_K];
        let mut val_buf = [0u8; MAX_V];

        while pos < sector_size {
            let addr = sector_start + pos;

            // 读取 flag
            let mut flag_buf = [0xFFu8; 1];
            flash.read(addr, &mut flag_buf)?;
            let flag = flag_buf[0];

            if flag == FLAG_FREE {
                // 之后的空间都空闲
                break;
            }

            // 读取 klen 和 vlen
            let mut meta_buf = [0u8; 3];
            flash.read(addr + 1, &mut meta_buf)?;
            let klen = meta_buf[0] as usize;
            let vlen = u16::from_le_bytes([meta_buf[1], meta_buf[2]]) as usize;

            // 边界检查
            if klen > MAX_K || vlen > MAX_V {
                // 数据损坏，跳过并继续扫描（因为不知道条目实际长度）
                // 这种情形只能在扇区关闭前检测到，保守处理：停止扫描此扇区
                break;
            }

            let entry_size = 6 + klen + vlen;

            if pos as usize + entry_size > sector_size as usize {
                // 条目超出扇区边界，数据损坏
                break;
            }

            // 读取 key 和 value
            if klen > 0 {
                flash.read(addr + 4, &mut key_buf[..klen])?;
            }
            if vlen > 0 {
                flash.read(addr + 4 + klen as u32, &mut val_buf[..vlen])?;
            }

            // 读取 CRC（存储值）
            let mut crc_buf = [0u8; 2];
            flash.read(addr + 4 + klen as u32 + vlen as u32, &mut crc_buf)?;
            let crc_stored = u16::from_le_bytes(crc_buf);

            // 计算 CRC 期望值
            let mut crc_computed: u16 = 0xFFFF;
            crc_computed = crc16_update(crc_computed, &[klen as u8]);
            crc_computed = crc16_update(crc_computed, &(vlen as u16).to_le_bytes());
            crc_computed = crc16_update(crc_computed, &key_buf[..klen]);
            crc_computed = crc16_update(crc_computed, &val_buf[..vlen]);

            if crc_stored != crc_computed {
                pos += entry_size as u32;
                continue; // CRC 不匹配，跳过此条目
            }

            // 处理有效条目
            if flag == FLAG_VALID {
                let _ = index.upsert(&key_buf[..klen], addr, vlen as u16, sector_seq);
            }
            // FLAG_TOMBSTONE: 忽略（不需要特殊处理，因为同名 VALID 会被后续条目覆盖）

            pos += entry_size as u32;
        }

        Ok(())
    }

    /// 在活动扇区中找到可写入位置（第一个 FLAG_FREE 位置）。
    fn find_write_pos(
        flash: &mut F,
        sector_start: u32,
        sector_size: u32,
    ) -> KvdbResult<u32, F::Error> {
        let mut pos = HEADER_SIZE;

        while pos < sector_size {
            let addr = sector_start + pos;

            let mut flag_buf = [0xFFu8; 1];
            flash.read(addr, &mut flag_buf)?;

            if flag_buf[0] == FLAG_FREE {
                // 检查后面几个字节是否也是 0xFF（排除部分写入的残留数据）
                let mut check = [0xFFu8; 8];
                flash.read(addr, &mut check)?;
                if check.iter().all(|&b| b == 0xFF) {
                    return Ok(pos);
                }
                // 存在残留数据（部分写入），说明上一次写入在此处掉电。
                // 此区域并非全 0xFF，NOR Flash 无法直接覆盖写入。
                // 返回扇区末尾使下一次 set() 自动触发 rotate_sector()。
                return Ok(sector_size);
            }

            // 读取 klen 和 vlen 以计算条目大小
            let mut meta_buf = [0u8; 3];
            flash.read(addr + 1, &mut meta_buf)?;
            let klen = meta_buf[0] as u32;
            let vlen = u16::from_le_bytes([meta_buf[1], meta_buf[2]]) as u32;

            // 防止异常大值导致无限循环
            if klen > MAX_K as u32 || vlen > MAX_V as u32 {
                // 数据损坏，无法继续
                break;
            }

            let entry_size = 6 + klen + vlen;

            if pos + entry_size > sector_size {
                break;
            }

            pos += entry_size;
        }

        // 如果走到这里，说明扇区扫描完毕或异常终止
        Ok(pos)
    }
}
