# py32e407-pac

Puya（普冉）**PY32E407** 系列 MCU（Cortex-M4）的 Peripheral Access Crate，由  
[`svd2rust` v0.37.1](https://github.com/rust-embedded/svd2rust) 从官方 SVD 生成。

- 源文件：`参考设计/PY32E407_Firmware_V0.7.0/Pack/KEIL/CMSIS/SVD/PY32E407xx.svd`

> 官方 SVD 并不完整，本仓库使用的 `svd/PY32E407xx.svd` 是打过补丁的版本：  
> 对照 CMSIS 头文件 `py32e407xE.h` 与 HAL/LL 驱动，补了 5 个寄存器  
> （DBG.IDCODE、I2C1.TIMEOUTR、SDIO.FIFODATA、ETHERNET_MAC.MACRWUFFR、  
> ETHERNET_DMA.DMARIWT）、21 个位域，并修正了 2 处字段命名错误。  
> 详见 `../tools/PAC_DIFF_REPORT.md`。

- 生成命令（已脚本化，见 `../tools/gen_pac.py`）：

```bash
python ../tools/patch_svd.py   # 打补丁，产出 svd/PY32E407xx.svd
python ../tools/gen_pac.py     # svd2rust -> form -> 重建 lib.rs 骨架 -> cargo fmt
cargo build --features rt,critical-section
```

`form` 拆分后按标准流程删除 `src/lib.rs` 会连 crate 骨架（Interrupt、Peripherals、  
外设别名）一起删掉，所以 `gen_pac.py` 会先从完整 lib.rs 提取骨架再还原。

## Feature

| Feature                  | 说明                                                    |
| ------------------------ | ----------------------------------------------------- |
| `rt`（默认关闭）               | 生成中断向量表 `__INTERRUPTS`，依赖 `cortex-m-rt`，需要 `memory.x` |
| `critical-section`（默认关闭） | 提供 `Peripherals::take()` 的 `critical-section` 保护版本写个  |

## 使用

在应用的 `Cargo.toml` 中：

```toml
[dependencies]
py32e407-pac = { path = "../py32e407-pac", features = ["rt", "critical-section"] }
cortex-m = "0.7.7"
cortex-m-rt = "0.7.5"
```

```rust
let p = py32e407_pac::Peripherals::take().unwrap();
p.rcc.ahbenr().modify(|_, w| w.gpioaen().set_bit());
```

> 注意：链接应用时还需要芯片的 `memory.x`（Flash / RAM 起始地址与长度），  
> 本 crate 只提供外设寄存器定义，不包含内存布局文件。
