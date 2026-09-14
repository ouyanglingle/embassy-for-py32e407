# KVDB 使用指南

嵌入式 `no_std` 键值数据库，基于QSPI NOR Flash，含磨损均衡与掉电安全，适用于键数量有限、写入或者更新频率不高的嵌入式场景

## 目录

- [快速开始](#快速开始)
- [为 Flash 实现 Trait](#为-flash-实现-trait)
- [创建数据库实例](#创建数据库实例)
- [CRUD 操作](#crud-操作)
- [参数选择](#参数选择)
- [掉电安全机制](#掉电安全机制)
- [磨损均衡](#磨损均衡)
- [CRC 校验](#crc-校验)
- [调试与诊断](#调试与诊断)
- [与 flashdb-rs 对比](#与-flashdb-rs-对比)

---

## 快速开始

```rust
#![no_std]
#![no_main]

use miniboard::kvdb::{FlashErase, FlashRead, FlashWrite, Kvdb};

// 1. 为你的 Flash 驱动实现 trait（见下一节）

// 2. 定义数据库类型
type MyKvdb = Kvdb<MyFlash, 4096, 8, 32, 256, 64>;
//                  ^^^^^^^  ^^^^  ^  ^^  ^^^  ^^
//                  Flash    扇区  扇 最  最   最大
//                  驱动     大小  区 大  大   键数
//                                数 键  值
//                                   长  长

// 3. 初始化
let mut db = MyKvdb::new(flash, 0x0000_0000).unwrap();

// 4. 读写
db.set(b"counter", &42u32.to_le_bytes()).unwrap();

let mut buf = [0u8; 4];
let n = db.get(b"counter", &mut buf).unwrap();
// buf[..n] == 42u32.to_le_bytes()

db.delete(b"counter").unwrap();
```

---

## 为 Flash 实现 Trait

三个 trait 采用层次结构：`FlashRead` → `FlashWrite` → `FlashErase`。

### 以 STM32F412 + GD25Q128 QSPI Flash 为例

GD25Q128 参数：
- 容量：128 Mbit / 16 MiB
- 扇区大小：4096 字节（4KB）
- 页大小：256 字节
- 命令集：Read(0x03), Page Program(0x02), Sector Erase(0x20), Write Enable(0x06), Read Status(0x05)

```rust
use embassy_stm32::qspi::{Qspi, TransferConfig, enums::*};
use embassy_stm32::{peripherals, mode::Blocking};
use miniboard::kvdb::{FlashRead, FlashWrite, FlashErase};

// ========== 封装 Flash 驱动 ==========

pub struct MyFlash {
    pub qspi: Qspi<'static, peripherals::QUADSPI, Blocking>,
}

impl MyFlash {
    /// 等待 Flash 内部操作完成（轮询状态寄存器 BUSY 位）
    fn wait_ready(&mut self) -> Result<(), FlashError> {
        let mut status = [0u8; 1];
        loop {
            self.qspi.blocking_read(
                &mut status,
                TransferConfig {
                    iwidth: QspiWidth::SING,
                    awidth: QspiWidth::NONE,
                    dwidth: QspiWidth::SING,
                    instruction: 0x05,   // Read Status Register
                    address: None,
                    dummy: DummyCycles::_0,
                },
            )?;
            if status[0] & 0x01 == 0 {
                // BUSY 位清零
                return Ok(());
            }
        }
    }

    /// 发送 Write Enable 命令（写/擦前必须）
    fn write_enable(&mut self) -> Result<(), FlashError> {
        self.qspi.blocking_command(TransferConfig {
            iwidth: QspiWidth::SING,
            awidth: QspiWidth::NONE,
            dwidth: QspiWidth::NONE,
            instruction: 0x06,   // Write Enable
            address: None,
            dummy: DummyCycles::_0,
        })
    }
}

// ========== 实现 Trait ==========

/// 统一错误类型（将 QSPI 错误映射为我们的类型）
#[derive(Debug)]
pub enum FlashError {
    Qspi(embassy_stm32::qspi::Error),
}

impl From<embassy_stm32::qspi::Error> for FlashError {
    fn from(e: embassy_stm32::qspi::Error) -> Self {
        FlashError::Qspi(e)
    }
}

impl FlashRead for MyFlash {
    type Error = FlashError;

    fn read(&mut self, addr: u32, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.qspi.blocking_read(
            buf,
            TransferConfig {
                iwidth: QspiWidth::SING,
                awidth: QspiWidth::SING,
                dwidth: QspiWidth::SING,
                instruction: 0x03,   // Read Data
                address: Some(addr),
                dummy: DummyCycles::_0,
            },
        )?;
        Ok(())
    }
}

impl FlashWrite for MyFlash {
    fn write(&mut self, addr: u32, buf: &[u8]) -> Result<(), Self::Error> {
        // ⚠️ 确保写入不超过页边界 (256 字节)
        // 实际使用时建议分页写入，此处简化为单次写
        self.write_enable()?;
        self.qspi.blocking_write(
            buf,
            TransferConfig {
                iwidth: QspiWidth::SING,
                awidth: QspiWidth::SING,
                dwidth: QspiWidth::SING,
                instruction: 0x02,   // Page Program
                address: Some(addr),
                dummy: DummyCycles::_0,
            },
        )?;
        self.wait_ready()?;
        Ok(())
    }
}

impl FlashErase for MyFlash {
    fn erase(&mut self, addr: u32) -> Result<(), Self::Error> {
        self.write_enable()?;
        self.qspi.blocking_command(TransferConfig {
            iwidth: QspiWidth::SING,
            awidth: QspiWidth::SING,
            dwidth: QspiWidth::NONE,
            instruction: 0x20,   // Sector Erase (4KB)
            address: Some(addr),
            dummy: DummyCycles::_0,
        })?;
        // 扇区擦除可能需要 ~45ms（典型值），等待完成
        self.wait_ready()?;
        Ok(())
    }
}
```

### 注意事项

1. **页对齐**：Flash Page Program 通常限制一页内（如 256 字节）。KVDB 内部每次写入不超过约 `6 + MAX_KEY_LEN + MAX_VALUE_LEN` 字节（默认 ~294B），如果超过页大小需自行拆分写入。

2. **Write Enable**：每次写/擦前必须发送 `0x06` 命令，Flash 硬件会自动清除 WE 位。

3. **等待就绪**：写/擦后必须轮询状态寄存器 BUSY 位，确认操作完成。

4. **QSPI 模式**：示例使用 SINGLE 宽度。如果你的 Flash 支持 QUAD 模式，可改为 `QspiWidth::QUAD` 提升速度（注意指令码也可能变化，如 QPP 用 `0x32` 或 `0x38`）。

---

## 创建数据库实例

### 方式一：从已有数据恢复（常规启动）

```rust
let mut db = MyKvdb::new(flash, 0x0000_0000)?;
```

`new()` 会扫描所有扇区，重建内存索引。如果 Flash 上没有有效数据（全 `0xFF`），会自动初始化扇区 0。

### 方式二：格式化（清空所有数据，全新开局）

```rust
let mut db = MyKvdb::format(flash, 0x0000_0000)?;
```

⚠️ `format()` 会擦除 `SECTOR_COUNT * SECTOR_SIZE` 字节的 Flash 区域，**不可逆**。

### 地址空间规划

GD25Q128 为 16 MiB（地址范围 `0x000000..0xFFFFFF`）。建议：

```
0x000000..0x0FFFFF  (前 1 MiB)   → KVDB 存储区
0x100000..0xFFFFFF  (后 15 MiB)  → 固件升级 / 日志 / 其他用途
```

若 KVDB 使用 8 个 4KB 扇区，仅占 32 KB，余量充足。

---

## CRUD 操作

### 写入 / 更新

```rust
// 写入字符串
db.set(b"name", b"STM32F412").unwrap();

// 写入整数（小端字节序）
db.set(b"boot_count", &42u32.to_le_bytes()).unwrap();

// 写入结构体（需确保 repr(C) 或手动序列化）
#[repr(C)]
struct Config { baud: u32, parity: u8 }
let cfg = Config { baud: 115200, parity: 0 };
db.set(b"config", unsafe {
    core::slice::from_raw_parts(&cfg as *const _ as *const u8, core::mem::size_of::<Config>())
}).unwrap();

// 更新已有键（追加新条目，旧条目在 Flash 上保留但被索引忽略）
db.set(b"boot_count", &100u32.to_le_bytes()).unwrap();
```

### 读取

```rust
let mut buf = [0u8; 4];
let n = db.get(b"boot_count", &mut buf).unwrap();
assert_eq!(n, 4);
let count = u32::from_le_bytes(buf[..4].try_into().unwrap());
```

### 带 CRC 校验的读取

```rust
// get_verified() 在读取后还会验证 CRC16，数据被 bit-flip 损坏时会返回 CrcMismatch
let mut buf = [0u8; 4];
let n = db.get_verified(b"boot_count", &mut buf).unwrap();
```

`get_verified()` 适合高可靠性读取场景（如关键配置校验）。相比 `get()` 多一次 CRC 读取和比对，性能稍低但保证返回数据的完整性。

### 检查存在

```rust
if db.contains(b"config") {
    // ...
}
```

### 删除

```rust
db.delete(b"temp_key").unwrap();
```

删除会写入一条**墓碑记录**（tombstone），键从索引中移除。下次恢复时墓碑会使旧条目被忽略。

### 状态查询

```rust
db.len()        // 当前存储的键数量
db.is_empty()   // 是否为空
db.capacity()   // 索引最大容量 (MAX_KEYS)
db.free_space() // 活动扇区剩余可写字节数
```

---

## 参数选择

`Kvdb<F, SECTOR_SIZE, SECTOR_COUNT, MAX_KEY_LEN, MAX_VALUE_LEN, MAX_KEYS>` 的六个泛型参数需要根据实际需求调整：

| 参数 | 说明 | 典型值 | 影响 |
|------|------|--------|------|
| `SECTOR_SIZE` | Flash 扇区大小（字节） | `4096` | 必须匹配 Flash 芯片规格 |
| `SECTOR_COUNT` | 扇区数量 | `4..16` | 越多磨损越均匀，占 Flash 越多 |
| `MAX_KEY_LEN` | 最大键长度（字节） | `16..32` | 影响 RAM 索引每条目大小 |
| `MAX_VALUE_LEN` | 最大值长度（字节） | `64..256` | 影响栈上读写缓冲区大小 |
| `MAX_KEYS` | 索引最大条目数 | `32..128` | 影响 RAM 占用 |

### RAM 占用估算

每个索引条目：`MAX_KEY_LEN + 8` 字节
```
MAX_KEY_LEN=32, MAX_KEYS=64  →  64 × 40 = 2560 字节
MAX_KEY_LEN=16, MAX_KEYS=32  →  32 × 24 =  768 字节
```

加上 `Kvdb` 结构体本身（~40 字节），总计 RAM 开销很小。

### Flash 占用

```
总占用 = SECTOR_COUNT × SECTOR_SIZE
       = 8 × 4096 = 32 KB  （示例配置）
```

---

## 掉电安全机制

### 条目级安全

条目写入顺序：
1. 写 klen、vlen、key、value、CRC（flag 仍为 `0xFF`）
2. **最后**写 flag 字节（`0x00` = 有效 / `0xAA` = 墓碑）

```
时间线：
  [写 body+CRC]  ← 若此时掉电 → flag=0xFF，恢复时视为空闲空间 ✓
  [写 flag]      ← 若此时掉电 → 条目完整但不一定在索引中，恢复时重建索引 ✓
```

**旧值不会被破坏**：更新键时新条目追加写入而非原地修改，掉电后索引恢复仍指向旧条目。

### 扇区级安全

扇区旋转流程：
1. 擦除新扇区
2. **先**写新扇区头部（status=`Active`）
3. **后**关旧扇区（status: `Active`→`Closed`）

```
掉电场景：
  步骤 2 后掉电 → 两个 Active 扇区 → 恢复时选 sequence 更大者（新扇区） ✓
  步骤 1 后掉电 → 新扇区部分是垃圾 → 恢复时 CRC 校验失败 → 视为擦除状态 ✓
```

### 残留数据处理

若在条目写入过程中掉电（flag 已写入但 body 不完整），扇区中会留下**部分写入残留**（非全 `0xFF`）。`find_write_pos()` 检测到残留后会返回扇区末尾，使下一次写入自动触发扇区旋转（`rotate_sector()`），避免直接在脏数据上覆写。

`close_sector()` 在写入前会先读取当前 status，若扇区已关闭则直接跳过，减少不必要的 Flash 写操作。

### CRC 双重校验

- **扇区头部 CRC**：确保扇区元数据正确
- **条目 CRC**：确保 key/value 未被 bit-flip 破坏

若 CRC 不匹配，该条目/扇区被跳过，不会导致系统崩溃。

---

## 磨损均衡

算法：**简单环形轮转**（Simple Ring Rotation）

```
扇区状态流转：
  Erased → Active → Closed → (被选中) → 擦除 → Active → ...

轮转示意（SECTOR_COUNT=4）：
  启动: [A0] [E1] [E2] [E3]         A=Active, C=Closed, E=Erased
  写满: [C0] [A1] [E2] [E3]  ← 旋转到扇区1
  写满: [C0] [C1] [A2] [E3]  ← 旋转到扇区2
  写满: [C0] [C1] [C2] [A3]  ← 旋转到扇区3
  写满: [A0] [C1] [C2] [C3]  ← 回绕到扇区0（擦除后复用）
```

**特点**：
- 实现简单，无复杂 GC 逻辑
- 擦除次数在所有扇区间均匀分布
- 旧条目物理保留（被新索引覆盖），无需后台整理

**权衡**：
- 不执行压实（compaction），旧条目占用 Flash 空间
- 适用于键数量有限、更新频率中等的嵌入式场景

---

## CRC 校验

### 算法参数

| 项目 | 值 |
|------|-----|
| 算法 | CRC-16/CCITT-FALSE |
| 多项式 | `0x1021` (x¹⁶ + x¹² + x⁵ + 1) |
| 初始值 | `0xFFFF` |
| 实现方式 | 软件查表法（256 项 × 2 字节 = 512 字节 ROM） |

### 覆盖范围

```
扇区头部（16 字节）:
  [magic:4] [seq:4] [status:1] [reserved:5] [crc16:2]
   └────────── CRC 计算范围 ──────────┘

条目（变长）:
  [flag:1] [klen:1] [vlen:2] [key:N] [value:M] [crc16:2]
           └───────────── CRC 计算范围 ──────────┘
```

> 注意：flag 字节**不参与 CRC 计算**，因为它是最后写入的（掉电安全机制的关键）。

### 读取时 CRC 验证

普通 `get()` 为性能考虑跳过 CRC 验证。如需完整性保证，使用 `get_verified()`：

```rust
db.get_verified(b"config", &mut buf)?;
// 若 Flash 上数据已被 bit-flip 损坏，返回 KvdbError::CrcMismatch
```

该方法在读取 value 后额外读取 Flash 上的 CRC 值并重新计算比对，适合关键数据的读取。

### 为什么不使用 STM32F4 硬件 CRC？

STM32F412 的 CRC 外设计算的是 **CRC-32**（多项式 `0x04C11DB7`），与我们使用的 **CRC-16** 算法不同：

| | 软件 CRC-16（当前） | 硬件 CRC-32（STM32F4） |
|--|---------------------|------------------------|
| ROM 占用 | 512 字节（查表） | 0（使用外设） |
| Flash 开销/条目 | 2 字节 | 4 字节 |
| 计算速度 | ~10 周期/字节 | 1 周期/字（DMA 可用） |
| 可移植性 | 跨平台 | 绑定 STM32 |

对于 KVDB 场景，**Flash I/O 是瓶颈**（QSPI 传输延迟远大于 CRC 计算），软件 CRC-16 完全够用，且节省 50% 的 Flash 校验存储空间。

若确实需要硬件加速，可改为 CRC-32 并修改 `db.rs` 中的：
- `CRC16_TABLE` → 使用 `embassy_stm32::crc::Crc` 外设驱动
- 扇区头部 CRC 字段从 2 字节扩为 4 字节
- 条目 CRC 字段从 2 字节扩为 4 字节

但需注意：**一旦 on-flash 格式变化，旧数据将无法读取**。

---

## 调试与诊断

### 开启 defmt 日志（可选）

如需调试输出，可在 `Cargo.toml` 添加：

```toml
[features]
default = []
defmt-log = []
```

然后在代码中使用 `defmt::trace!` 输出内部状态。当前版本为了保持零依赖未内置日志。

### 常见问题

**Q: 写入后立即掉电，数据会丢失吗？**
A: 正在写入的条目会丢失（flag 未提交），但之前的所有数据完好。这是设计预期。

**Q: 索引满了怎么办？**
A: `set()` 返回 `KvdbError::TooManyKeys`。增大 `MAX_KEYS` 或减少键数量。

**Q: 活动扇区满了怎么办？**
A: 自动旋转到下一个扇区。如果所有扇区都满了（极端情况），返回 `KvdbError::NoSpace`。

**Q: 能存储多少个键？**
A: 受两个限制：① `MAX_KEYS`（RAM 索引容量）；② 扇区总空间。每个条目约 6+N+M 字节，例如 8 个 4KB 扇区扣除头部约可存 500+ 个小条目（键 8B + 值 8B）。

**Q: 如何选择 SECTOR_COUNT？**
A: 受 Flash 芯片擦除寿命（通常 10 万次）和写入频率影响。例如每天写 1000 次，4 个扇区可分摊为每扇区每天 250 次，寿命约 400 天。8 个扇区可延长到 800 天。建议至少 4 个。
