# compare_regs.py —— 比对 CMSIS 设备头文件 / HAL-LL 驱动 与 svd2rust 生成的 PAC，
# 找出 C 侧存在但 PAC 中缺失的寄存器。
#
# 用法：python compare_regs.py [--json out.json]
import os
import re
import sys
import json

ROOT = r"D:\embassy-for-py32e407"
CMSIS = os.path.join(
    ROOT,
    "参考设计",
    "PY32E407_Firmware_V0.7.0",
    "Drivers",
    "CMSIS",
    "Device",
    "PUYA",
    "PY32E407",
    "Include",
    "py32e407xE.h",
)
HAL = os.path.join(
    ROOT, "参考设计", "PY32E407_Firmware_V0.7.0", "Drivers", "PY32E407_HAL_Driver"
)
PAC = os.path.join(ROOT, "py32e407-pac")

# C typedef -> PAC 模块名（一个 C 结构可能对应多个 Rust 模块）
MAP = {
    "ADC_TypeDef": ["adc1"],
    "ADC_Common_TypeDef": ["adc12_common"],
    "AES_TypeDef": ["aes"],
    "BKP_TypeDef": ["bkp"],
    "COMP_TypeDef": ["comp1"],
    "COMP_Common_TypeDef": ["comp1"],
    "CRC_TypeDef": ["crc"],
    "CTC_TypeDef": ["ctc"],
    "CORDIC_TypeDef": ["cordic"],
    "DAC_TypeDef": ["dac"],
    "DBGMCU_TypeDef": ["dbg"],
    "DMA_TypeDef": ["dma1"],
    "DMA_Channel_TypeDef": ["dma1"],
    "ETH_TypeDef": ["ethernet_mac", "ethernet_dma", "ethernet_mmc", "ethernet_ptp"],
    "EXTI_TypeDef": ["exti"],
    "FLASH_TypeDef": ["flash"],
    "GPIO_TypeDef": ["gpioa"],
    "I2C_TypeDef": ["i2c1"],
    "IWDG_TypeDef": ["iwdg"],
    "LCDC_TypeDef": ["lcdc"],
    "LCDC_Common_TypeDef": ["lcdc"],
    "LPTIM_TypeDef": ["lptim"],
    "OPA_TypeDef": ["opamp"],
    "PWR_TypeDef": ["pwr"],
    "ESMC_TypeDef": ["esmc"],
    "RCC_TypeDef": ["rcc"],
    "RNG_TypeDef": ["rng"],
    "RTC_TypeDef": ["rtc"],
    "SDIO_TypeDef": ["sdio"],
    "SPI_TypeDef": ["spi1"],
    "SYSCFG_TypeDef": ["syscfg"],
    "TIM_TypeDef": ["tim1", "tim2", "tim6", "tim9", "tim10", "tim15", "tim16"],
    "USART_TypeDef": ["usart1"],
    "UART_TypeDef": ["uart1"],
    "LPUART_TypeDef": ["lpuart"],
    "WWDG_TypeDef": ["wwdg"],
    "FDCAN_GlobalTypeDef": ["fdcan1"],
    "FDCAN_TypeDef": ["fdcan1"],
    "CAN_TypeDef": ["fdcan1"],
    "USB_OTG_GlobalTypeDef": ["usb1_otg_global"],
    "USB_OTG_DeviceTypeDef": ["usb1_otg_device"],
    "USB_OTG_HostTypeDef": ["usb1_otg_host"],
    "USB_OTG_PWRCLKTypeDef": ["usb1_otg_pwrclk"],
    "USB_OTG_INEndpointTypeDef": ["usb1_otg_device"],
    "USB_OTG_OUTEndpointTypeDef": ["usb1_otg_device"],
    "USB_OTG_HostChannelTypeDef": ["usb1_otg_host"],
}

REG_RE = re.compile(
    r"^\s*(?:__I|__O|__IO|__IOM|__OM|__IM)?\s*"
    r"(?:uint|int)(?:8|16|32|64)_t\s+(\w+)\s*(?:\[\s*(\d+)\s*\])?\s*;",
    re.M,
)


def parse_cmsis(path):
    """解析 `typedef struct { ... } XXX_TypeDef;` -> {typedef: [寄存器名]}"""
    text = open(path, encoding="utf-8", errors="replace").read()
    out = {}
    for m in re.finditer(r"typedef\s+struct\s*[^{]*\{(.*?)\}\s*(\w+)\s*;", text, re.S):
        name, body = m.group(2), m.group(1)
        regs = [r[0] for r in REG_RE.findall(body)]
        if regs:
            out.setdefault(name, []).extend(regs)
    return out


def parse_pac_blocks(pac_src):
    """解析每个模块的 RegisterBlock 字段 -> {module: [寄存器字段名(小写)]}"""
    out = {}
    for fn in sorted(os.listdir(pac_src)):
        if not fn.endswith(".rs"):
            continue
        mod = fn[:-3]
        text = open(os.path.join(pac_src, fn), encoding="utf-8").read()
        m = re.search(r"pub struct RegisterBlock \{(.*?)\n\}", text, re.S)
        if not m:
            continue
        # svd2rust 生成的 RegisterBlock 字段是模块私有（无 pub）
        fields = re.findall(r"^\s{4}(\w+)\s*:", m.group(1), re.M)
        out[mod] = [f for f in fields if not f.startswith("_reserved")]
    return out


def parse_hal_regs(hal_dir):
    """从 HAL/LL 源码里抽取 `xxx->REG` 形式的寄存器使用（按文件所属外设归并）"""
    out = {}
    for sub in ("Inc", "Src"):
        d = os.path.join(hal_dir, sub)
        if not os.path.isdir(d):
            continue
        for fn in os.listdir(d):
            if not fn.endswith((".c", ".h")):
                continue
            m = re.search(r"py32e407_(?:hal|ll)_(\w+?)(_ex)?\.(?:c|h)$", fn)
            if not m:
                continue
            periph = m.group(1)
            text = open(os.path.join(d, fn), encoding="utf-8", errors="replace").read()
            # 只取全大写的成员名（寄存器），避免抓到 State/Instance 等句柄字段
            regs = set(re.findall(r"->\s*([A-Z][A-Z0-9_]*)\b", text))
            out.setdefault(periph, set()).update(regs)
    return out


def main():
    cmsis = parse_cmsis(CMSIS)
    pac = parse_pac_blocks(os.path.join(PAC, "src"))
    hal = parse_hal_regs(HAL)

    report = {"missing_periph": [], "missing_regs": {}, "hal_only": {}}

    print("=== 1. C 侧外设 -> PAC 模块 缺失检查 ===")
    for td, regs in sorted(cmsis.items()):
        mods = MAP.get(td)
        if not mods:
            report["missing_periph"].append(td)
            print(f"  [无映射/未覆盖] {td}  ({len(regs)} 个寄存器)")
            continue
        for mod in mods:
            if mod not in pac:
                print(f"  [模块缺失] {td} -> PAC 无模块 {mod}")

    print("\n=== 2. 寄存器级缺失（C 有 / PAC 无）===")
    for td, regs in sorted(cmsis.items()):
        mods = MAP.get(td) or []
        have = set()
        for mod in mods:
            have |= set(pac.get(mod, []))
        if not have:
            print(f"  [整块缺失] {td}: PAC 无对应模块 {mods}")
            report["missing_regs"][td] = sorted(set(regs))
            continue
        miss = [r for r in dict.fromkeys(regs) if r.lower() not in have]
        if miss:
            report["missing_regs"][td] = miss
            print(f"  {td} ({'+'.join(mods)}): 缺 {len(miss)} -> {miss}")

    print("\n=== 3. HAL/LL 源码中用到、但对应 PAC 模块没有的寄存器 ===")
    hal2pac = {
        "adc": ["adc1"],
        "comp": ["comp1"],
        "cordic": ["cordic"],
        "crc": ["crc"],
        "ctc": ["ctc"],
        "dac": ["dac"],
        "dma": ["dma1"],
        "dma_ex": ["dma1"],
        "esmc": ["esmc"],
        "eth": ["ethernet_mac", "ethernet_dma", "ethernet_mmc", "ethernet_ptp"],
        "exti": ["exti"],
        "flash": ["flash"],
        "gpio": ["gpioa"],
        "i2c": ["i2c1"],
        "i2s": ["spi1"],
        "iwdg": ["iwdg"],
        "lcdc": ["lcdc"],
        "lptim": ["lptim"],
        "lpuart": ["lpuart"],
        "opa": ["opamp"],
        "pwr": ["pwr"],
        "rcc": ["rcc"],
        "rng": ["rng"],
        "rtc": ["rtc"],
        "sd": ["sdio"],
        "spi": ["spi1"],
        "tim": ["tim1"],
        "uart": ["uart1"],
        "usart": ["usart1"],
        "usbd": ["usb1_otg_global", "usb1_otg_device"],
        "usbh": ["usb1_otg_global", "usb1_otg_host"],
        "wwdg": ["wwdg"],
        "fdcan": ["fdcan1"],
        "cortex": [],
        "uart_ex": ["uart1"],
        "usart_ex": ["usart1"],
        "spi_ex": ["spi1"],
        "flash_ex": ["flash"],
        "rcc_ex": ["rcc"],
        "rtc_ex": ["rtc"],
        "tim_ex": ["tim1"],
        "adc_ex": ["adc1"],
        "dac_ex": ["dac"],
        "i2c_ex": ["i2c1"],
        "irda": ["usart1"],
        "smartcard": ["usart1"],
        "sci": ["usart1"],
        "hal": [],
        "pwr_ex": ["pwr"],
        "usbd_ex": ["usb1_otg_device"],
        "lpuart_ex": ["lpuart"],
        "sci_ex": ["usart1"],
        "smartcard_ex": ["usart1"],
        "crc_ex": ["crc"],
    }
    for periph, regs in sorted(hal.items()):
        mods = hal2pac.get(periph)
        if mods is None:
            print(f"  [未映射的驱动模块] {periph}: {sorted(regs)[:20]}")
            continue
        if not mods:
            continue
        have = set()
        for mod in mods:
            have |= set(pac.get(mod, []))
        miss = sorted(r for r in regs if r.lower() not in have)
        if miss:
            report["hal_only"][periph] = miss
            print(f"  {periph} -> {miss}")

    if "--json" in sys.argv:
        with open(sys.argv[sys.argv.index("--json") + 1], "w", encoding="utf-8") as f:
            json.dump(report, f, indent=2, ensure_ascii=False)
    print("\nC 侧 typedef 数量:", len(cmsis), " PAC 模块数量:", len(pac))


if __name__ == "__main__":
    main()
