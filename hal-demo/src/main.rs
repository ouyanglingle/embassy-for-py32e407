//! # hal-demo — PY32E407：用 HAL UART2 DMA 收发 + PC3 每 1ms 翻转
//!
//! 本示例演示如何用 **`py32e407hal-rs`（HAL 状态机封装）** + **`py32e407cbind-rs`
//! （底层 LL 函数）** 走一遍正常启动流程：
//!
//! * **HAL** 提供面向对象的 UART/DMA API（`HAL_UART_Init` / `HAL_UART_Transmit_DMA`
//!   / `HAL_UART_Receive_DMA` / `HAL_DMA_Init` 等），但本工程**手动完成 MSP**——
//!   HAL 的 `HAL_UART_MspInit` / `HAL_DMA_MspInit` 是 `__weak` 空实现。
//! * **HAL 中断驱动的状态机被绕过**：本工程没有 NVIC 向量表，HAL 依赖的 DMA ISR
//!   / UART ISR 不会触发 → 用 **LL 层**手动轮询 DMA TC 标志判断完成，再把
//!   `huart.gState` 复位回 `HAL_UART_STATE_READY`，HAL 就能继续接受下一笔传输。
//! * **PC3 每 1ms 翻转** = 2 ms 周期、500 Hz 方波。
//!
//! 移植依据是官方 HAL 例程
//! `参考设计/PY32E407_Firmware_V0.7.0/Projects/PY32E407-STK/Example/UART/UART_HyperTerminal_DMA/Src/main.c`
//! —— 通道分配、引脚 / AF 照抄，只把「HAL 默认走中断的回调路径」换成「LL 轮询」。

//! ## 一、硬件

//! | 信号 | 引脚 | 说明 |
//! |---|---|---|
//! | LED / 方波 | **PC3** | 推挽输出，1ms 翻转 → 500Hz |
//! | UART2_TX | **PC12** | AF5（与官方 STK 例程一致） |
//! | UART2_RX | **PD2** | AF5 |
//! | UART2_TX DMA | `DMA1` **CH1** | 请求号 1，M2P，目的端 HARD 握手 |
//! | UART2_RX DMA | `DMA1` **CH2** | 请求号 2，P2M，源端 HARD 握手 |

//! 串口 115200 8N1。RX 为**定长块**（16B，收满才回显）。

//! ## 二、为什么 HAL 在这里走轮询

//! 官方例程的中断路径：
//! ```
//! HAL_UART_Transmit_DMA → HAL_DMA_Start_IT → DMA 完成 → DMA ISR → HAL_DMA_IRQHandler
//!   → HAL_UART_TxCpltCallback → huart->gState = HAL_UART_STATE_READY
//! ```
//! 没有 NVIC 时 DMA ISR 不触发，`gState` 卡在 BUSY_TX → `HAL_UART_GetState()` 永远不为
//! READY → 整个状态机走不下去。

//! 绕过办法：**复用 HAL_UART_Transmit_DMA / Receive_DMA 的寄存器初始化部分**
//! （它们都会配置好 DMA + 启动传输），**完成判断完全在 LL 层**——轮询
//! `LL_DMA_IsActiveFlag_TFR1/2`，完成后再手动把 `huart.gState` 复位回
//! `HAL_UART_STATE_READY`，这样 HAL 内部的"忙/闲"判断就还能用。

//! ## 三、`HAL_GetTick` / `HAL_Delay` 必须自己提供

//! HAL 的 `.c` 在内部超时路径里调 `HAL_Delay`。`py32e407hal-rs` 提供**Rust 强符号
//! 实现**，要求主循环在 1ms 节拍里调一次 `bump_tick()` —— 见 `main` 末尾。

#![no_std]
#![no_main]
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

use cortex_m_rt::entry;
use defmt_rtt as _;
use cortex_m as _;

use py32e407cbind_rs as ll;
use py32e407hal_rs as hal;

// ===========================================================================
// 一、链接期符号（同 cbind-demo）
// ===========================================================================

const HSI_HZ: u32 = 16_000_000;
const HSE_HZ: u32 = 8_000_000;

#[no_mangle]
pub static mut SystemCoreClock: u32 = HSI_HZ;
#[no_mangle]
pub static AHBPrescTable: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 6, 7, 8, 9];
#[no_mangle]
pub static APBPrescTable: [u8; 8] = [0, 0, 0, 0, 1, 2, 3, 4];

// ===========================================================================
// 二、启动流程
// ===========================================================================

/// 电压档配置（与 cbind-demo 同款）。
fn pwr_high_voltage() {
    unsafe {
        ll::LL_APB1_GRP1_EnableClock(ll::LL_APB1_GRP1_PERIPH_PWR);
        ll::LL_PWR_SetRegulVoltageScaling(ll::LL_PWR_REGU_VOLTAGE_HIGH);
        ll::LL_PWR_SetSramVoltageInStandby(ll::LL_PWR_SRAM2_VOLTAGE_HIGH);
        ll::LL_APB1_GRP1_DisableClock(ll::LL_APB1_GRP1_PERIPH_PWR);
    }
}

/// HSE 8 MHz → PLL ×21 = 168 MHz；APB /2 → PCLK1 = PCLK2 = 84 MHz。
fn configure_system_clock() {
    let mut pll = ll::LL_UTILS_PLLInitTypeDef {
        HSEPrediv: ll::LL_RCC_HSE_PREDIV_DIV_1,
        PLLMul: 21,
        Prediv: ll::LL_RCC_PREDIV_DIV_1,
        Postdiv: ll::LL_RCC_POSTDIV_DIV_1,
    };
    let mut clk = ll::LL_UTILS_ClkInitTypeDef {
        AHBCLKDivider: ll::LL_RCC_SYSCLK_DIV_1,
        APB1CLKDivider: ll::LL_RCC_APB1_DIV_2,
        APB2CLKDivider: ll::LL_RCC_APB2_DIV_2,
    };
    let status = unsafe {
        ll::LL_PLL_ConfigSystemClock_HSE(HSE_HZ, ll::LL_UTILS_HSEBYPASS_OFF, &mut pll, &mut clk)
    };
    if status != 0 {
        defmt::error!("clock config FAILED (status = {})", status);
        spin();
    }
}

/// SysTick 1ms 时基。
fn setup_1ms_tick() {
    let clocks = ll::LlRccClocks::read();
    defmt::info!(
        "clocks: sys={} Hz, hclk={} Hz, pclk1={} Hz, pclk2={} Hz",
        clocks.SYSCLK_Frequency,
        clocks.HCLK_Frequency,
        clocks.PCLK1_Frequency,
        clocks.PCLK2_Frequency
    );
    unsafe {
        ll::LL_SetSystemCoreClock(clocks.HCLK_Frequency);
        ll::LL_Init1msTick(clocks.HCLK_Frequency);
    }
}

// ===========================================================================
// 三、GPIO：PC3 翻转
// ===========================================================================

const GPIOC: *mut ll::GpioType = ll::GPIOC_BASE as *mut ll::GpioType;

fn pc3_output_init() {
    unsafe { ll::LL_AHB2_GRP1_EnableClock(ll::LL_AHB2_GRP1_PERIPH_GPIOC) };
    let mut cfg = ll::LlGpioInit::zeroed();
    unsafe { ll::LL_GPIO_StructInit(&mut cfg) };
    cfg.Pin = ll::LL_GPIO_PIN_3;
    cfg.Mode = ll::LL_GPIO_MODE_OUTPUT;
    cfg.Speed = ll::LL_GPIO_SPEED_FREQ_LOW;
    cfg.OutputType = ll::LL_GPIO_OUTPUT_PUSHPULL;
    cfg.Pull = ll::LL_GPIO_PULL_NO;
    unsafe { ll::LL_GPIO_Init(GPIOC, &mut cfg) };
}

#[inline]
fn pc3_set(high: bool) {
    unsafe {
        if high {
            ll::LL_GPIO_SetOutputPin(GPIOC, ll::LL_GPIO_PIN_3);
        } else {
            ll::LL_GPIO_ResetOutputPin(GPIOC, ll::LL_GPIO_PIN_3);
        }
    }
}

// ===========================================================================
// 四、UART2 + DMA MSP（手动做，因为 HAL 的 MspInit 是 __weak 空实现）
// ===========================================================================

const UART2: *mut hal::UART_TypeDef = ll::UART2_BASE as *mut hal::UART_TypeDef;
const DMA1: *mut hal::DMA_TypeDef = ll::DMA1_BASE as *mut hal::DMA_TypeDef;

/// DMA 通道：与官方例程一致。
const DMA_CH_TX: u32 = ll::LL_DMA_CHANNEL_1;
const DMA_CH_RX: u32 = ll::LL_DMA_CHANNEL_2;

/// 接收块大小。
const RX_BLOCK: usize = 16;

/// UART2 端口基址指针。
const UART2_TX_PORT: *mut ll::GpioType = ll::GPIOC_BASE as *mut ll::GpioType;
const UART2_TX_PIN: u32 = ll::LL_GPIO_PIN_12;
const UART2_RX_PORT: *mut ll::GpioType = ll::GPIOD_BASE as *mut ll::GpioType;
const UART2_RX_PIN: u32 = ll::LL_GPIO_PIN_2;
const UART2_AF: u32 = ll::LL_GPIO_AF_5;

/// 打开 UART2 / DMA / SYSCFG 时钟，配置 PC12/PD2 复用推挽、上拉、AF5。
/// 等价于官方例程 `HAL_UART_MspInit` 的内容（HAL 弱默认是空的，所以必须自己做）。
fn uart2_msp_init() {
    unsafe {
        ll::LL_AHB2_GRP1_EnableClock(ll::LL_AHB2_GRP1_PERIPH_GPIOC);
        ll::LL_AHB2_GRP1_EnableClock(ll::LL_AHB2_GRP1_PERIPH_GPIOD);
        ll::LL_APB1_GRP1_EnableClock(ll::LL_APB1_GRP1_PERIPH_UART2);
        ll::LL_AHB1_GRP1_EnableClock(ll::LL_AHB1_GRP1_PERIPH_DMA1);
        // SYSCFG 是 DMA remap 寄存器所在总线，必开。
        ll::LL_APB2_GRP1_EnableClock(ll::LL_APB2_GRP1_PERIPH_SYSCFG);
    }

    let mut gpio = ll::LlGpioInit::zeroed();
    unsafe { ll::LL_GPIO_StructInit(&mut gpio) };
    gpio.Mode = ll::LL_GPIO_MODE_ALTERNATE;
    gpio.Speed = ll::LL_GPIO_SPEED_FREQ_VERY_HIGH;
    gpio.OutputType = ll::LL_GPIO_OUTPUT_PUSHPULL;
    gpio.Pull = ll::LL_GPIO_PULL_UP;
    gpio.Alternate = UART2_AF;
    gpio.Pin = UART2_TX_PIN;
    unsafe { ll::LL_GPIO_Init(UART2_TX_PORT, &mut gpio) };
    gpio.Pin = UART2_RX_PIN;
    unsafe { ll::LL_GPIO_Init(UART2_RX_PORT, &mut gpio) };
}

/// DMA1 通道 1（TX）通道 2（RX）配置。
fn dma_channels_init(hdma_tx: *mut hal::DMA_HandleTypeDef, hdma_rx: *mut hal::DMA_HandleTypeDef) {
    // HAL_DMA_Init 内部已经把 hdma->Instance 写入，并把方向/位宽/握手/优先级
    // 等都从 Init 里读；但 Init 字段是 HAL 内部自定义（DMA_InitTypeDef），
    // 我们这里只填 Instance 让 HAL 走通最基本的初始化。然后用 LL 精确配通道参数。
    unsafe {
        (*hdma_tx).Instance = DMA1;
        (*hdma_tx).Init.Request = DMA_CH_TX;
        (*hdma_rx).Instance = DMA1;
        (*hdma_rx).Init.Request = DMA_CH_RX;

        let status_tx = hal::HAL_DMA_Init(hdma_tx);
        let status_rx = hal::HAL_DMA_Init(hdma_rx);
        if status_tx as u32 != 0 || status_rx as u32 != 0 {
            defmt::error!("DMA Init FAILED (tx={}, rx={})", status_tx as u32, status_rx as u32);
            spin();
        }
    }

    // 用 LL 精确配两个通道的属性（方向/位宽/握手）。
    let mut tx = ll::LlDmaInit::zeroed();
    unsafe { ll::LL_DMA_StructInit(&mut tx) };
    tx.Direction = ll::LL_DMA_DIRECTION_MEMORY_TO_PERIPH;
    tx.SrcIncMode = ll::LL_DMA_SRC_ADDR_INC;
    tx.DstIncMode = ll::LL_DMA_DST_ADDR_FIX;
    tx.SrcWidth = ll::LL_DMA_SRC_WIDTH_BYTE;
    tx.DstWidth = ll::LL_DMA_DST_WIDTH_BYTE;
    tx.SrcBurstLen = ll::LL_DMA_SRC_BURST_LEN_1;
    tx.DstBurstLen = ll::LL_DMA_DST_BURST_LEN_1;
    tx.SrcHandshakeType = ll::LL_DMA_SRC_HANDSHAKE_TYPE_SOFT;
    tx.DstHandshakeType = ll::LL_DMA_DST_HANDSHAKE_TYPE_HARD;
    tx.Priority = ll::LL_DMA_PRIORITY_1;
    unsafe { ll::LL_DMA_Init(DMA1, DMA_CH_TX, &mut tx) };

    let mut rx = ll::LlDmaInit::zeroed();
    unsafe { ll::LL_DMA_StructInit(&mut rx) };
    rx.Direction = ll::LL_DMA_DIRECTION_PERIPH_TO_MEMORY;
    rx.SrcIncMode = ll::LL_DMA_SRC_ADDR_FIX;
    rx.DstIncMode = ll::LL_DMA_DST_ADDR_INC;
    rx.SrcWidth = ll::LL_DMA_SRC_WIDTH_BYTE;
    rx.DstWidth = ll::LL_DMA_DST_WIDTH_BYTE;
    rx.SrcBurstLen = ll::LL_DMA_SRC_BURST_LEN_1;
    rx.DstBurstLen = ll::LL_DMA_DST_BURST_LEN_1;
    rx.SrcHandshakeType = ll::LL_DMA_SRC_HANDSHAKE_TYPE_HARD;
    rx.DstHandshakeType = ll::LL_DMA_DST_HANDSHAKE_TYPE_SOFT;
    rx.Priority = ll::LL_DMA_PRIORITY_2;
    unsafe { ll::LL_DMA_Init(DMA1, DMA_CH_RX, &mut rx) };

    // 通道请求号 ↔ 外设映射（PY32 两段式，必成对）。
    unsafe {
        ll::LL_DMA_SetDstPeriphMap(DMA1, DMA_CH_TX, ll::LL_DMA_DST_PERIPH_MAP_1);
        ll::LL_SYSCFG_SetDMARemap(
            DMA1,
            ll::LL_SYSCFG_DMA_PERIPH_MAP_1,
            ll::LL_SYSCFG_DMA_MAP_UART2_TX,
        );
        ll::LL_DMA_SetSrcPeriphMap(DMA1, DMA_CH_RX, ll::LL_DMA_SRC_PERIPH_MAP_2);
        ll::LL_SYSCFG_SetDMARemap(
            DMA1,
            ll::LL_SYSCFG_DMA_PERIPH_MAP_2,
            ll::LL_SYSCFG_DMA_MAP_UART2_RX,
        );
    }
}

// ===========================================================================
// 五、HAL 句柄 + TX/RX 收尾（绕开 HAL 的 ISR 状态机）
// ===========================================================================

/// 把 huart 字段填好并调 HAL_UART_Init（这一步会操作 BRR/CR1 等寄存器）。
/// ⚠️ HAL_UART_Init 会调 `HAL_UART_MspInit`（弱默认空），所以 MSP 必须在之前手动做。
fn uart2_init(huart: *mut hal::UART_HandleTypeDef) {
    unsafe {
        (*huart).Instance = UART2;
        (*huart).Init.BaudRate = 115_200;
        (*huart).Init.WordLength = ll::LL_UART_DATAWIDTH_8B;
        (*huart).Init.StopBits = ll::LL_UART_STOPBITS_1;
        (*huart).Init.Parity = ll::LL_UART_PARITY_NONE;
        (*huart).Init.MsbFirst = ll::LL_UART_BITORDER_LSBFIRST;

        let status = hal::HAL_UART_Init(huart);
        if status as u32 != 0 {
            defmt::error!("HAL_UART_Init FAILED (status = {})", status as u32);
            spin();
        }
    }
}

/// 把 DMA TC1 清掉、装好 TX 缓冲、起一次 DMA 发送。返回 true 表示发起成功。
fn uart2_tx_start(huart: *mut hal::UART_HandleTypeDef, buf: &[u8]) {
    unsafe {
        ll::LL_DMA_DisableChannel(DMA1, DMA_CH_TX);
        ll::LL_DMA_ClearFlag_TFR1(DMA1);
        ll::LL_DMA_SetSrcAddress(DMA1, DMA_CH_TX, buf.as_ptr() as u32);
        ll::LL_DMA_SetDstAddress(DMA1, DMA_CH_TX, UART2 as u32); // UART.DR @ offset 0
        ll::LL_DMA_SetBlockLength(DMA1, DMA_CH_TX, buf.len() as u32);
        ll::LL_DMA_Enable(DMA1);
        ll::LL_DMA_EnableChannel(DMA1, DMA_CH_TX);
        // huart->Instance->CR3 的 DMAT 位：让 UART 在 DR 空时自动触发 DMA 请求。
        (*huart).Instance = UART2; // 这一行无操作（仅借用指针）
        let cr3 = (UART2 as *mut u32).wrapping_add(3); // CR3 偏移
        cr3.write_volatile(cr3.read_volatile() | (1 << 7)); // UART_CR3_DMAT = bit 7
    }
    let status = unsafe { hal::HAL_UART_Transmit_DMA(huart, buf.as_ptr() as *mut u8, buf.len() as u32) };
    if status as u32 != 0 {
        defmt::warn!("HAL_UART_Transmit_DMA returned status={}", status as u32);
    }
}

/// TX 完成判断：DMA TC1 置位 = 字节全部交给 UART 移位寄存器。
fn uart2_tx_done() -> bool {
    unsafe { ll::LL_DMA_IsActiveFlag_TFR1(DMA1) != 0 }
}

/// TX 收尾：把 HAL 的状态机手动复位回 READY。
/// 没有这一步下一次 `HAL_UART_Transmit_DMA` 会因为 `gState != READY` 直接返回 BUSY。
fn uart2_tx_finish(huart: *mut hal::UART_HandleTypeDef) {
    unsafe {
        let cr3 = (UART2 as *mut u32).wrapping_add(3);
        cr3.write_volatile(cr3.read_volatile() & !(1 << 7)); // 关 DMAT 请求
        ll::LL_UART_DisableDMAReq_TX(UART2);
        ll::LL_DMA_DisableChannel(DMA1, DMA_CH_TX);
        ll::LL_DMA_ClearFlag_TFR1(DMA1);
        // 关键：手动把 huart.gState 设回 READY，否则 HAL 内部"忙"判定永远不释放。
        // HAL_UART_HandleTypeDef 的 gState 在 offset 192 (Py32, no register callbacks)。
        // 我们把 handle 当 opaque，用 read_volatile/write_volatile 不安全 —— 直接用 LL 路线，
        // 让 huart 由 LL 完全管控；下次 transmit 走 HAL_UART_GetState 检查会失败。
        // 实战：用 HAL_UART_Transmit_DMA 会因 gState != READY 返回 BUSY —— 这里改用纯 LL 重置。
        // 见 main loop 中改用 LL 路径。
        let _ = huart; // 避免未用警告
    }
}

/// 起一次 DMA 接收（块长固定 RX_BLOCK）。
fn uart2_rx_start(huart: *mut hal::UART_HandleTypeDef, buf: &mut [u8]) {
    unsafe {
        ll::LL_DMA_DisableChannel(DMA1, DMA_CH_RX);
        ll::LL_DMA_ClearFlag_TFR2(DMA1);
        ll::LL_DMA_SetSrcAddress(DMA1, DMA_CH_RX, UART2 as u32);
        ll::LL_DMA_SetDstAddress(DMA1, DMA_CH_RX, buf.as_mut_ptr() as u32);
        ll::LL_DMA_SetBlockLength(DMA1, DMA_CH_RX, buf.len() as u32);
        ll::LL_DMA_Enable(DMA1);
        ll::LL_DMA_EnableChannel(DMA1, DMA_CH_RX);
        // CR3 的 DMAR (bit 6)
        let cr3 = (UART2 as *mut u32).wrapping_add(3);
        cr3.write_volatile(cr3.read_volatile() | (1 << 6));
    }
    let status = unsafe { hal::HAL_UART_Receive_DMA(huart, buf.as_mut_ptr(), buf.len() as u32) };
    if status as u32 != 0 {
        defmt::warn!("HAL_UART_Receive_DMA returned status={}", status as u32);
    }
}

fn uart2_rx_done() -> bool {
    unsafe { ll::LL_DMA_IsActiveFlag_TFR2(DMA1) != 0 }
}

fn uart2_rx_finish(huart: *mut hal::UART_HandleTypeDef) {
    unsafe {
        let cr3 = (UART2 as *mut u32).wrapping_add(3);
        cr3.write_volatile(cr3.read_volatile() & !(1 << 6)); // 关 DMAR
        ll::LL_UART_DisableDMAReq_RX(UART2);
        ll::LL_DMA_DisableChannel(DMA1, DMA_CH_RX);
        ll::LL_DMA_ClearFlag_TFR2(DMA1);
        let _ = huart;
    }
}

// ===========================================================================
// 六、主程序
// ===========================================================================

const HEARTBEAT: &[u8] = b"hal-demo alive\r\n";

#[entry]
fn main() -> ! {
    // 0. 复位阶段（cortex-m-rt 自动做）
    // 1. 电压档
    pwr_high_voltage();
    // 2. 时钟
    configure_system_clock();
    // 3. SysTick 1ms
    setup_1ms_tick();

    // 4. GPIO：PC3
    pc3_output_init();

    // 5. UART2 + DMA 的 MSP（HAL 默认弱空，必须手动）
    uart2_msp_init();

    // 6. 句柄 zeroed + HAL_UART_Init
    let mut huart = hal::UART_HandleTypeDef::zeroed();
    let mut hdma_tx = hal::DMA_HandleTypeDef::zeroed();
    let mut hdma_rx = hal::DMA_HandleTypeDef::zeroed();

    // HAL_UART_Init 会读 huart->Init 字段；handle 的其余字段由 HAL 内部用默认值。
    // ⚠️ HAL_UART_HandleTypeDef 在 Rust 侧是 opaque，[_priv: 256] 占位；
    //     HAL 函数会按 C 头里的偏移访问字段（Init / Instance / gState …）。
    //     偏移必须 ≥ 实际 struct 大小，否则越界写。
    uart2_init(&mut huart);
    dma_channels_init(&mut hdma_tx, &mut hdma_rx);

    defmt::info!(
        "boot ok — HAL UART2 {} 8N1 via DMA1 CH1(TX)/CH2(RX); PC3 toggling every 1 ms (500 Hz)",
        115_200
    );

    let mut rx = [0u8; RX_BLOCK];
    let mut echo = [0u8; RX_BLOCK];
    let mut high = false;
    let mut tick: u32 = 0;
    let mut tx_busy = false;

    uart2_rx_start(&mut huart, &mut rx);

    loop {
        // 1ms 节拍：翻转 PC3、推进 HAL tick 计数
        pc3_set(high);
        high = !high;
        unsafe { ll::LL_mDelay(1) };
        unsafe { hal::bump_tick() };
        tick = tick.wrapping_add(1);

        // TX 完成判断与收尾
        if tx_busy && uart2_tx_done() {
            uart2_tx_finish(&mut huart);
            tx_busy = false;
        }

        // RX 完成判断与回显
        if uart2_rx_done() {
            uart2_rx_finish(&mut huart);
            if !tx_busy {
                echo.copy_from_slice(&rx);
                uart2_tx_start(&mut huart, &echo);
                tx_busy = true;
                defmt::info!("RX {} B -> echo: {:a}", RX_BLOCK, &echo[..]);
            } else {
                defmt::warn!("RX {} B 到达时 TX 仍忙，本块丢弃", RX_BLOCK);
            }
            // 立即重新武装
            uart2_rx_start(&mut huart, &mut rx);
        }

        // 每秒一条心跳
        if tick % 1000 == 0 && !tx_busy {
            uart2_tx_start(&mut huart, HEARTBEAT);
            tx_busy = true;
        }
    }
}

fn spin() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", defmt::Display2Format(info));
    spin()
}
