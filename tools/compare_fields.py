# compare_fields.py —— 比对 C 头文件中的位定义（xxx_REG_FIELD_Pos/Msk）
# 与 PAC 生成的字段访问器，找出 PAC 缺失的位域。
import os
import re
import sys

ROOT = r"D:\embassy-for-py32e407"
CMSIS = os.path.join(
    ROOT, "参考设计", "PY32E407_Firmware_V0.7.0", "Drivers", "CMSIS", "Device",
    "PUYA", "PY32E407", "Include", "py32e407xE.h",
)
PAC_SRC = os.path.join(ROOT, "py32e407-pac", "src")

# C 宏前缀 -> PAC 模块名（可能多个）
PREFIX = {
    "ADC": ["adc1", "adc12_common"],
    "AES": ["aes"],
    "BKP": ["bkp"],
    "COMP": ["comp1"],
    "CORDIC": ["cordic"],
    "CRC": ["crc"],
    "CTC": ["ctc"],
    "DAC": ["dac"],
    "DBGMCU": ["dbg"],
    "DMA": ["dma1"],
    "ESMC": ["esmc"],
    "ETH": ["ethernet_mac", "ethernet_dma", "ethernet_mmc", "ethernet_ptp"],
    "EXTI": ["exti"],
    "FLASH": ["flash"],
    "GPIO": ["gpioa"],
    "I2C": ["i2c1"],
    "IWDG": ["iwdg"],
    "LCDC": ["lcdc"],
    "LPTIM": ["lptim"],
    "LPUART": ["lpuart"],
    "OPA": ["opamp"],
    "PWR": ["pwr"],
    "RCC": ["rcc"],
    "RNG": ["rng"],
    "RTC": ["rtc"],
    "SDIO": ["sdio"],
    "SPI": ["spi1"],
    "SYSCFG": ["syscfg"],
    "TIM": ["tim1", "tim2", "tim6", "tim9", "tim10", "tim15", "tim16"],
    "USART": ["usart1"],
    "UART": ["uart1"],
    "USB": ["usb1_otg_global", "usb1_otg_device", "usb1_otg_host"],
    "WWDG": ["wwdg"],
    "FDCAN": ["fdcan1"],
}


def parse_c_fields(path):
    """-> {(prefix, REG): {FIELD: pos}}"""
    text = open(path, encoding="utf-8", errors="replace").read()
    out = {}
    for m in re.finditer(
        r"^#define\s+([A-Z0-9_]+?)_([A-Z0-9]+)_([A-Z0-9_]+)_Pos\s*\((\d+)U?\)",
        text,
        re.M,
    ):
        periph, reg, field, pos = m.groups()
        out.setdefault((periph, reg), {})[field.lower()] = int(pos)
    return out


def parse_pac_fields(pac_src):
    """-> {(mod, reg): set(field)}"""
    out = {}
    for mod in sorted(os.listdir(pac_src)):
        d = os.path.join(pac_src, mod)
        if not os.path.isdir(d):
            continue
        for fn in sorted(os.listdir(d)):
            if not fn.endswith(".rs"):
                continue
            text = open(os.path.join(d, fn), encoding="utf-8").read()
            # 读取器：pub fn f(&self) / pub fn f(&self, n: u8)
            fields = set(re.findall(r"pub fn (\w+)\(&self(?:, n: u\d+)?\)", text))
            # 写入器：pub fn f(&mut self) / pub fn set_f(&mut self)
            fields |= set(re.findall(r"pub fn (\w+)\(&mut self", text))
            fields |= {f[4:] for f in re.findall(r"pub fn set_(\w+)\(&mut self", text)}
            fields -= {"bits", "read", "write", "clear_bit", "set_bit", "variant",
                       "bit", "field", "new", "variant_or", "clear_bits_by_mask"}
            out[(mod, fn[:-3])] = fields
    return out


def main():
    c = parse_c_fields(CMSIS)
    pac = parse_pac_fields(PAC_SRC)

    # 预先建立 "模块:寄存器" -> 字段集合 的并集（同一 C 前缀可能跨多模块）
    total_missing = 0
    periph_stat = {}
    for (periph, reg), fields in sorted(c.items()):
        mods = PREFIX.get(periph)
        if mods is None:
            continue
        cand = set()
        for mod in mods:
            key = (mod, reg.lower())
            if key in pac:
                cand |= pac[key]
        if not cand:
            # 寄存器整条缺失（已在 compare_regs.py 中报告）
            continue
        def has(f):
            # PAC 对数组字段生成 `moder(&self, n)`，C 侧是 moder0..moder15
            return f in cand or re.sub(r"\d+$", "", f) in cand

        miss = sorted(f for f in fields if not has(f))
        if miss:
            periph_stat.setdefault(periph, []).append((reg, miss))
            total_missing += len(miss)

    detail = "--detail" in sys.argv
    for periph, items in sorted(periph_stat.items()):
        print(f"[{periph}]")
        for reg, miss in items:
            have = sorted(
                set().union(*[pac.get((m, reg.lower()), set()) for m in PREFIX[periph]])
            ) if any((m, reg.lower()) in pac for m in PREFIX[periph]) else []
            if detail:
                print(f"    {reg}:\n        PAC现有: {have}\n        疑似缺: {miss}")
            else:
                print(f"    {reg}: {miss}")
    print("\n缺失字段总数:", total_missing, " 涉及寄存器:",
          sum(len(v) for v in periph_stat.values()))


if __name__ == "__main__":
    main()
