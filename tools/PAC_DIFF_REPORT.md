# PY32E407 PAC 与 C 版 HAL/LL 驱动的核对报告

- 生成时间：2026-09-06
- 被核对对象：`D:\embassy-for-py32e407\py32e407-pac`（svd2rust 0.37.1）
- 参照基准：`参考设计\PY32E407_Firmware_V0.7.0` 中的
  - `Drivers\CMSIS\Device\PUYA\PY32E407\Include\py32e407xE.h`（寄存器/位权威定义）
  - `Drivers\PY32E407_HAL_Driver\{Inc,Src}`（HAL 53 个模块 + LL 27 个模块）
- 核对脚本：`tools\compare_regs.py`（外设/寄存器）、`tools\compare_offsets.py`（地址偏移）、`tools\compare_fields.py`（位域）

## 1. 结论

| 维度 | 结果 |
| --- | --- |
| 外设级 | **无缺失**：C 侧 43 个外设结构体全部有对应 PAC 模块（PAC 共 53 个模块 / 84 个外设实例） |
| 寄存器级 | 原缺 5 个，已全部补齐 |
| 位域级 | 原缺 21 个，已全部补齐 |
| SVD 命名错误 | 2 处，已修正 |
| 地址偏移 | 除已知误报外无冲突 |
| HAL/LL 实际用到的寄存器 | 已 100% 覆盖（compare_regs.py 第 3 节为空） |

## 2. 已补齐的寄存器（5 个）

| 外设 | 寄存器 | 偏移 | 说明 |
| --- | --- | --- | --- |
| DBG | IDCODE | 0x00 | `HAL_GetDEVID()` / `HAL_GetREVID()` 依赖 |
| I2C1 | TIMEOUTR | 0x24 | 总线超时（TIMEOUTA/TIDLE/TIMEOUTEN/TIMEOUTB/TEXTEN） |
| SDIO | FIFODATA | 0x200 | SDIO FIFO 数据口 |
| ETHERNET_MAC | MACRWUFFR | 0x28 | 远程唤醒帧过滤 |
| ETHERNET_DMA | DMARIWT | 0x24 | DMA 接收中断看门狗 |

## 3. 已补齐的位域（21 个）

| 外设.寄存器 | 补上的位 |
| --- | --- |
| ADC1.SMPR1 | SMPPLUS[31] |
| ADC1.JSQR | JEXTEN[8:7]（注入通道外部触发使能） |
| FLASH.OPTR1 | IWDG_STOP[9]、IWDG_STDBY[10]、WWDG_SW[15]（选项字节） |
| I2C1.CR1 | WUPEN[2]、WKUP_DIV[17:16]、WKUP_CNT[19:18]（低功耗唤醒） |
| I2C1.CCR | FP[13] |
| I2C1.TRISE | THOLDDATA[11:7]、THOLDDATA_SEL[12] |
| PWR.CR1 | SRAM_RETV[12:11] |
| PWR.SR | MR_RDY[9] |
| RCC.AHB2ENR | ETHTXEN[16]、ETHRXEN[17] |
| RCC.APB2ENR | OPAEN[28] |
| RCC.CFGR | SRAM2PRE[23] |
| RCC.CR | HSIKERON[2] |
| USART1.CR2 | RTOEN[25]（接收超时使能） |
| USART1.SR | ABRRQ[12]（自动波特率请求） |
| ETHERNET_MAC.MACFFR | VTFE[16]（VLAN 标签过滤） |

## 4. 已修正的 SVD 命名错误（2 处）

| 位置 | 原（错） | 现（对） |
| --- | --- | --- |
| DAC.DOR2 的字段 | DACC1DOR | DACC2DOR |
| TIM1.CCR6 的字段 | CCR5 | CCR6 |

## 5. 剩余差异（确认是命名/组织方式不同，不是缺失）

| C 侧 | PAC 侧 | 说明 |
| --- | --- | --- |
| GPIO OSPEEDR / AFR[2] | `ospdder` / `afrl`+`afrh` | SVD 命名/拆分 |
| PWR SCR | `csr` | 同名不同写法 |
| RNG LFSR | `algr` | 同上 |
| SPI I2SCFGR | `i2s_cfgr` | I2S 支持存在 |
| USART DR | `rdr` + `tdr` | 收发分离 |
| LCDC CSR/DR/CMDR/WCFGR/RCFGR | `cr` + `lcd0..3_*` | LCDC 有 4 组通道，SVD 已展开 |
| ETH ETH_TypeDef（单一大结构） | `ethernet_mac/dma/mmc/ptp` 四个模块 | 拆分方式不同，偏移一致 |
| USB OTG GOTGCTL 等 | `fs_gotgctl` 等 | SVD 加了 `FS_` 前缀 |
| AES CTX_KEY[8] 等 | `ctx_key0..7` | 数组已展开 |
| TIM CCMR1/CCMR2 | `ccmr1_input`/`ccmr1_output` 等 | 输入输出模式拆分 |

## 6. 需要人工确认的高风险项：RTC 布局

C 头与 SVD 对 RTC 的描述**完全不同**，不是简单的命名差异：

| 偏移 | C（py32e407xE.h） | SVD / PAC |
| --- | --- | --- |
| 0x08 / 0x0C | PRLH / PRLL（各 32 位槽，16 位有效） | PRL0 / PRL1（8 位） |
| 0x10 | DIVH | PRL2（8 位） |
| 0x14 / 0x18 | DIVL / CNTH | DIV0 / DIV1 |
| 0x1C / 0x20 | CNTL / ALRH | DIV2 / CNT0 |
| 0x24 | ALRL | CNT1 |
| 0x28–0x3C | — | CNT2 / CNT3 / ALR0..ALR3 |

- HAL 的 `py32e407_hal_rtc.c` 用的是 `PRLH/PRLL/CNTH/CNTL/ALRH/ALRL`（F1 风格，16 位高低半字）。
- SVD 描述的是字节寻址型 RTC（PRL 24 位、CNT/ALR 32 位，每个字节占一个 32 位地址槽），与 Puya 部分系列的 RTC 一致。
- **两者必有一错**。用 Rust 写 RTC 驱动前，请先以 PY32E407 参考手册的 RTC 章节核对：
  - 若手册是字节型 → 以 PAC（SVD）为准，C 的 `hal_rtc.c` 不可直接照抄；
  - 若手册是 F1 型 → 需要改 SVD 的 RTC 段（`tools/patch_svd.py` 里加一条替换即可）。

## 7. 复现方式

```bash
python tools/patch_svd.py     # 生成 py32e407-pac/svd/PY32E407xx.svd（含补丁）
python tools/gen_pac.py       # 重新生成 PAC（svd2rust + form + fmt）
cd py32e407-pac && cargo build --features rt,critical-section
python tools/compare_regs.py  # 复核差异
python tools/compare_fields.py --detail
```

补丁表中的每个位位置都会与 `py32e407xE.h` 的 `XXX_REG_FIELD_Pos` 自动校验，
抄错会直接报错终止，不会静默写歪。
