//! MotionFX_initialize 反汇编分析
//!
//! 编译自 libMotionFX.a (Cortex-M4F fpv4-sp-d16-hard, v2.7.0)
//! 目标 MCU: STM32F405RG (Cortex-M4F, DEVID=0x463)
//! 结论: 该库不支持 STM32F405RG，初始化函数会进入死循环。
//!
//! 反汇编输出: libMotionFX_disasm.txt (完整 541KB)

/*
┌─────────────────────────────────────────────────────────────────────────────┐
│                        MotionFX_initialize 完整反汇编                       │
│                  (地址 0x0 是 .o 文件内部的相对地址)                         │
└─────────────────────────────────────────────────────────────────────────────┘

00000000 <MotionFX_initialize>:

    ── 步骤 1: 读取并修改 NVIC 的某个控制寄存器 ──────────────────────────
   0:  ldr  r2, [pc, #304]    ; r2 = 0xE0002000  (SCB->ACTLR, 辅助控制寄存器)
   2:  ldr  r3, [r2, #0]      ; r3 = 读取 ACTLR 当前值
   4:  movw ip, #0xE000E000   ; ip = 系统控制块基址 (SCB)
   8:  bic  r3, r3, #1        ; r3 = r3 & ~1  (清除 DISMCYCINT 位)
   c:  push {r4, r5, lr}      ; 保存寄存器
   e:  str  r3, [r2, #0]      ; 写回 ACTLR，启用多周期中断

    ── 步骤 2: 读取 CPUID 寄存器，检测 Cortex-M 架构版本 ───────────────
  10:  ldr.w r3, [ip, #3328]  ; r3 = *(0xE000ED00) = CPUID 寄存器
  14:  movw r1, #0xFFF0       ; r1 = 掩码 (只保留高 20 位)
  18:  ands r3, r1            ; r3 = CPUID & 0xFFF0
  1a:  movw r2, #0xC240       ; r2 = 0xC240 (Cortex-M4 的 CPUID 值)
  1e:  cmp  r3, r2            ; 是 Cortex-M4?
  20:  sub  sp, #20           ; 分配栈空间
  22:  mov  r4, r0            ; r4 = state 指针参数
  24:  beq  4e                ; 是 M4 → 跳转到 4e (继续初始化)

    ── 不是 M4? 检查是不是 Cortex-M7 ───────────────────────────────────
  26:  ldr.w r3, [ip, #3328]  ; 重新读取 CPUID
  2a:  movw r2, #0xC270       ; r2 = 0xC270 (Cortex-M7 的 CPUID 值)
  2e:  ands r1, r3            ; r1 = CPUID & 0xFFF0
  30:  cmp  r1, r2            ; 是 Cortex-M7?
  32:  beq  4e                ; 是 M7 → 跳转到 4e (继续初始化)

    ── 既不是 M4 也不是 M7 → 进入 "MCU 不支持" 的死循环路径 ──────────
  34:  ldr  r2, [pc, #256]    ; r2 = 0x40023000 (RCC 寄存器基址)
  36:  movs r3, #1
  38:  str  r3, [r2, #8]      ; RCC->AHB1ENR 置位 (启用某个时钟)
  3a:  ldr  r3, [r2, #8]      ; 读回确认
  3c:  cmp  r3, #0
  3e:  bne  3a                ; 自旋等待时钟就绪 (第一次死循环点)

  40:  ldr  r3, [pc, #248]    ; r3 = 0xF407A5C2 (魔数 1)
  42:  str  r3, [r2, #0]      ; 写入某个寄存器
  44:  ldr  r2, [r2, #0]      ; 读回
  46:  ldr  r3, [pc, #248]    ; r3 = 0xB5E8B5CD (魔数 2)
  48:  cmp  r2, r3            ; 读回值匹配魔数?
  4a:  beq  a6                ; 匹配 → 跳转到 a6 (继续初始化)
  4c:  b    4c                ; ★ 不匹配 → 死循环！(第二次死循环点)
  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  注意: 地址 34-4c 这段代码的意图是:
      对于"未知"MCU，尝试读取某外设 ID 寄存器并比对魔数。
      由于 F405 的这个寄存器返回 0，比对失败，跳转到 4c 死循环。

    ── 步骤 3 (M4/M7 路径): 读取 DBGMCU_IDCODE，检测具体 MCU 型号 ────
  4e:  ldr  r3, [pc, #244]    ; r3 = 0xE0042000 (DBGMCU_IDCODE 地址)
  50:  ldr  r3, [r3, #0]      ; r3 = DBGMCU_IDCODE 值
  52:  cmp  r3, #0            ; IDCODE 为 0?
  54:  bne  34                ; ★ 不为 0 → 跳转到 34 (走外设 ID 校验)
                              ; 但 F405 的 DBGMCU 时钟默认不开启!
                              ; RCC->APB2ENR 的 DBGMCU 位(位0)没使能时，
                              ; 读取 0xE0042000 返回 0
                              ; → 所以这里 cmp r3, #0 → EQ → 继续往下走

  56:  ldr  r2, [pc, #240]    ; r2 = 0x5C001000 (某个外设 ID 寄存器地址)
  58:  ldr  r3, [r2, #0]      ; r3 = 读取外设 ID
  5a:  ubfx r3, r3, #0, #12  ; r3 = DEVID (低 12 位)
  5e:  cmp.w r3, #0x450       ; DEVID == 0x450? (STM32H7)
  62:  beq  112               ; 是 H7 → 跳转到 112 (走 H7 初始化)

  64:  ldr  r3, [r2, #0]      ; 重新读取
  66:  movw r1, #0x483        ; DEVID == 0x483? (STM32F427/437)
  6e:  cmp  r3, r1
  70:  beq  112               ; 是 F427 → 跳转

  72:  ldr  r3, [r2, #0]      ; 重新读取
  74:  movw r2, #0x485        ; DEVID == 0x485? (STM32F469/479)
  7c:  cmp  r3, r2
  7e:  beq  112               ; 是 F469 → 跳转

  80:  ldr  r3, [pc, #200]    ; r3 = 0x50081000 (另一个外设 ID 寄存器)
  82:  ldr  r3, [r3, #0]      ; 读取
  84:  ubfx r3, r3, #0, #12  ; DEVID 低 12 位
  88:  cmp.w r3, #0x500       ; DEVID == 0x500?
  8c:  bne  34                ; ★ 不是 → 跳转到 34 (走 "不支持" 路径)
                              ;
                              ; ★ STM32F405RG 的 DEVID = 0x463 ★
                              ; 以上检查了 0x450, 0x483, 0x485, 0x500
                              ; 0x463 不匹配任何一个 → 跳转到 34
                              ; → 进入死循环 (地址 4c)

    ── 步骤 4 (MCU 匹配成功后的初始化) ──────────────────────────────
  8e:  ldr  r2, [pc, #192]    ; r2 = 0x4C004000
  90:  movs r3, #1
  92:  str  r3, [r2, #8]      ; 启用某外设时钟
  94:  ldr  r3, [r2, #8]      ; 等待就绪
  96:  cmp  r3, #0
  98:  bne  94                 ; 自旋等待

  9a:  ldr  r3, [pc, #160]    ; r3 = 0xF407A5C2 (魔数 1)
  9c:  str  r3, [r2, #0]
  9e:  ldr  r2, [r2, #0]
  a0:  ldr  r3, [pc, #156]    ; r3 = 0xB5E8B5CD (魔数 2)
  a2:  cmp  r2, r3
  a4:  bne  4c                 ; 校验失败 → 死循环

    ── 步骤 5: 初始化内部状态 ──────────────────────────────────────
  a6:  ldr  r3, [pc, #172]    ; r3 = 0x00756E65 = "une\0" (ASCII)
  a8:  ldr  r2, [pc, #172]    ; r2 = 0x00000000 (目标地址)
  aa:  str  r3, [sp, #12]     ; 栈上存字符串
  ac:  movs r5, #1
  ae:  mov  r0, r4            ; r0 = state 指针
  b0:  strd r3, r3, [sp, #4]  ; 栈上存 8 字节
  b4:  strb.w r5, [r2, #1052] ; 标志位置 1 (表示已初始化)
  b8:  bl  MFX_emptyAttitude  ; 清空姿态
  bc:  add  r1, sp, #4        ; 参数: 栈上数据
  be:  add.w r0, r4, #58      ; 参数: state[58]
  c2:  strb.w r5, [r4, #57]   ; state[57] = 1
  c6:  bl  updateOrientation   ; 初始化方向 1
  ca:  add  r1, sp, #12
  cc:  add.w r0, r4, #67      ; state[67]
  d0:  bl  updateOrientation   ; 初始化方向 2
  d4:  add  r1, sp, #8
  d6:  add.w r0, r4, #76      ; state[76]
  da:  bl  updateOrientation   ; 初始化方向 3

  de:  ldr  r3, [pc, #124]    ; r3 = 0x3F2AC083 (浮点常数 ~0.667)
  e0:  ldr  r1, [pc, #124]    ; r1 = 0x3F666666 (浮点常数 ~0.9)
  e2:  str  r3, [r4, #52]     ; state[52..55] = 常数
  e4:  ldr  r3, [pc, #124]    ; r3 = 0x3A378034 (浮点常数 ~0.00065)
  e6:  ldr  r2, [pc, #128]    ; r2 = 0x3A51B717 (浮点常数 ~0.00080)
  e8:  str  r1, [r4, #40]     ; state[40..43] = 0.9
  ea:  str  r3, [r4, #28]     ; state[28..31] = 0.00065
  ec:  ldr  r1, [pc, #124]    ; r1 = 0x3B378034 (浮点常数 ~0.0028)
  ee:  str  r2, [r4, #24]     ; state[24..27] = 0.00080
  f0:  movs r3, #0
  f2:  mov.w r0, #0x3FC00000  ; = 1.5 (浮点)
  f6:  movw r2, #258          ; = 0x102
  fa:  strb.w r3, [r4, #56]   ; state[56] = 0
  fe:  movw r3, #257          ; = 0x101
 102:  str  r0, [r4, #44]     ; state[44..47] = 1.5
 104:  str  r1, [r4, #32]     ; state[32..35] = 0.0028
 106:  strh r2, [r4, #4]      ; state[4..5] = 0x102
 108:  strh.w r3, [r4, #13]   ; state[13..14] = 0x101
 10c:  strb.w r5, [r4, #15]   ; state[15] = 1
 10e:  add  sp, #20
 110:  pop  {r4, r5, pc}      ; ★ 返回 (初始化成功)

    ── H7/F427/F469 的专用初始化路径 ─────────────────────────────
 112:  ldr  r2, [pc, #92]     ; r2 = 0x58024000 (H7/F4 系外设)
 114:  movs r3, #1
 116:  str.w r3, [r2, #3080]  ; 0xC08 偏移 = 使能时钟
 11a:  ldr.w r3, [r2, #3080]
 11e:  cmp  r3, #0
 120:  bne  11a               ; 自旋等待

 122:  ldr  r3, [pc, #24]     ; r3 = 0xF407A5C2
 124:  str.w r3, [r2, #3072]  ; 0xC00 偏移 = 写入魔数
 128:  ldr.w r2, [r2, #3072]
 12c:  ldr  r3, [pc, #16]     ; r3 = 0xB5E8B5CD
 12e:  cmp  r2, r3
 130:  beq  a6                ; 魔数匹配 → 跳转到 a6 (正常初始化)
 132:  b    4c                ; 不匹配 → 死循环

    ── 文字池 (Literal Pool) ──────────────────────────────────────────
 134:  .word 0xE0002000   // SCB->ACTLR 地址 (NVIC 辅助控制)
 138:  .word 0x40023000   // RCC 基址 (STM32F4 系列)
 13c:  .word 0xF407A5C2   // 魔数 1 (用于验证外设 ID)
 140:  .word 0xB5E8B5CD   // 魔数 2
 144:  .word 0xE0042000   // DBGMCU_IDCODE 地址
 148:  .word 0x5C001000   // 外设 ID 寄存器 1 (用于读取 DEVID)
 14c:  .word 0x50081000   // 外设 ID 寄存器 2
 150:  .word 0x4C004000   // 另一外设基址
 154:  .word 0x00756E65   // "une\0" (ASCII)
 158:  .word 0x00000000   // 目标地址
 15c:  .word 0x3F2AC083   // float ~0.667
 160:  .word 0x3F666666   // float ~0.9
 164:  .word 0x3A378034   // float ~0.00065
 168:  .word 0x3A51B717   // float ~0.00080
 16c:  .word 0x3B378034   // float ~0.0028
 170:  .word 0x58024000   // H7/F4 系外设基址 (ART accelerator?)
*/

/*

★ 根因总结 ★

MotionFX_initialize 通过以下流程验证 MCU:

1. 读取 CPUID (0xE000ED00) → 检查 Cortex-M4 (0xC240) 或 M7 (0xC270)
   ✓ STM32F405RG 是 Cortex-M4F → CPUID = 0xC24x → 通过

2. 读取 DBGMCU_IDCODE (0xE0042000) → 检查是否为 0
   ✗ 默认 DBGMCU 时钟未使能 → 读取返回 0
   ✗ 因为 RCC->APB2ENR 的 DBGMCU 位(bit 0) 默认为 0
   ✗ 所以读回 0 → 继续往下走

3. 读取外设 ID 寄存器 1 (0x5C001000) → 提取低 12 位 DEVID
   ✗ 检查 0x450 (H7) ✗  0x483 (F427) ✗  0x485 (F469) ✗
   ★ F405RG 的 DEVID = 0x463 → 不匹配任何一个 ★

4. 读取外设 ID 寄存器 2 (0x50081000) → 检查 DEVID = 0x500
   ✗ 也不匹配 → 跳转到 "不支持" 路径 (地址 34)

5. "不支持" 路径尝试读取未知外设并比对魔数 0xF407A5C2
   ✗ F405RG 上读取返回 0 → 魔数不匹配
   → 跳转到地址 0x4c: "b.n 0x4c" ← ★ 死循环! ★

结论: MotionFX v2.7.0 仅支持 STM32H7 (0x450)、
      STM32F427/437 (0x483)、STM32F469/479 (0x485)
      以及 DEVID=0x500 的系列芯片。
      STM32F405RG (DEVID=0x463) 不在支持列表中。

可能的修复方案 (如果一定要用 MotionFX):
  A. 在调用 MotionFX_initialize 前使能 DBGMCU 时钟
     (RCC->APB2ENR |= 1)
     但 DEVID 0x463 本身就不在支持列表内，即使读到正确值也没用

  B. 魔改二进制，跳过 MCU 检测部分
     不现实且违法 (ST SLA0077 禁止逆向工程)

  C. 改用纯 Rust 的 Mahony/Madgwick 滤波器 (本项目的方案)
     ✓ 已在 lsm6dsrtr/mod.rs 中实现
     ✓ 支持 9 轴融合 (acc+gyro+mag)
     ✓ 不受 MCU 型号限制
*/
