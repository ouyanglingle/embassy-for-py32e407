# patch_svd.py —— 根据 CMSIS 头文件（C 的 HAL/LL 依赖的权威寄存器定义）
# 补齐 PY32E407xx.svd 中缺失的寄存器与位域，输出 patched SVD。
#
# 补丁表中的位位置会在运行时与 py32e407xE.h 里的 `XXX_REG_FIELD_Pos`
# 自动校验，不一致会直接报错，避免手抄出错。
#
# 用法：python patch_svd.py [输出路径]
import os
import re
import sys
import xml.etree.ElementTree as ET

ROOT = r"D:\embassy-for-py32e407"
SRC_SVD = os.path.join(
    ROOT, "参考设计", "PY32E407_Firmware_V0.7.0", "Pack", "KEIL", "CMSIS", "SVD",
    "PY32E407xx.svd",
)
CMSIS_H = os.path.join(
    ROOT, "参考设计", "PY32E407_Firmware_V0.7.0", "Drivers", "CMSIS", "Device",
    "PUYA", "PY32E407", "Include", "py32e407xE.h",
)
OUT = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
    ROOT, "py32e407-pac", "svd", "PY32E407xx.svd"
)

# SVD 中的 peripheral 名 -> C 头文件里的宏前缀
ALIAS = {
    "ADC1": "ADC", "DAC": "DAC", "DBG": "DBGMCU", "ETHERNET_DMA": "ETH",
    "ETHERNET_MAC": "ETH", "FLASH": "FLASH", "I2C1": "I2C", "PWR": "PWR",
    "RCC": "RCC", "SDIO": "SDIO", "USART1": "USART",
}

# 整条寄存器缺失：(peripheral, register, addressOffset, description, [(field, msb, lsb)])
NEW_REGS = [
    ("DBG", "IDCODE", 0x00, "DBGMCU IDCODE",
     [("REV_ID", 31, 0)]),
    ("I2C1", "TIMEOUTR", 0x24, "I2C timeout register",
     [("TIMEOUTA", 11, 0), ("TIDLE", 12, 12), ("TIMEOUTEN", 15, 15),
      ("TIMEOUTB", 27, 16), ("TEXTEN", 31, 31)]),
    ("SDIO", "FIFODATA", 0x200, "SDIO FIFO data register",
     [("FIFODATA", 31, 0)]),
    ("ETHERNET_MAC", "MACRWUFFR", 0x28, "ETH remote wakeup frame filter register",
     [("D", 31, 0)]),
    ("ETHERNET_DMA", "DMARIWT", 0x24, "ETH DMA receive interrupt watchdog timer",
     [("RIWT", 7, 0)]),
]

# 已有寄存器缺位域：(peripheral, register, [(field, msb, lsb)])
NEW_FIELDS = [
    ("ADC1", "SMPR1", [("SMPPLUS", 31, 31)]),
    ("ADC1", "JSQR", [("JEXTEN", 8, 7)]),
    ("FLASH", "OPTR1", [("IWDG_STOP", 9, 9), ("IWDG_STDBY", 10, 10),
                        ("WWDG_SW", 15, 15)]),
    ("I2C1", "CR1", [("WUPEN", 2, 2), ("WKUP_DIV", 17, 16), ("WKUP_CNT", 19, 18)]),
    ("I2C1", "CCR", [("FP", 13, 13)]),
    ("I2C1", "TRISE", [("THOLDDATA", 11, 7), ("THOLDDATA_SEL", 12, 12)]),
    ("PWR", "CR1", [("SRAM_RETV", 12, 11)]),
    ("RCC", "AHB2ENR", [("ETHTXEN", 16, 16), ("ETHRXEN", 17, 17)]),
    ("RCC", "APB2ENR", [("OPAEN", 28, 28)]),
    ("RCC", "CFGR", [("SRAM2PRE", 23, 23)]),
    ("USART1", "CR2", [("RTOEN", 25, 25)]),
    ("USART1", "SR", [("ABRRQ", 12, 12)]),
    ("ETHERNET_MAC", "MACFFR", [("VTFE", 16, 16)]),
    ("PWR", "SR", [("MR_RDY", 9, 9)]),
    ("RCC", "CR", [("HSIKERON", 2, 2)]),
]

# SVD 中字段名笔误修正：(peripheral, register, 旧名, 新名)
RENAME_FIELDS = [
    ("DAC", "DOR2", "DACC1DOR", "DACC2DOR"),   # DOR2 里写成了 DACC1DOR
    ("TIM1", "CCR6", "CCR5", "CCR6"),          # CCR6 里写成了 CCR5
]


def load_c_pos():
    """{('ADC','JSQR'): {'jexten': pos}}"""
    text = open(CMSIS_H, encoding="utf-8", errors="replace").read()
    out = {}
    for m in re.finditer(
        r"^#define\s+([A-Z0-9_]+?)_([A-Z0-9]+)_([A-Z0-9_]+)_Pos\s*\((\d+)U?\)",
        text, re.M,
    ):
        periph, reg, field, pos = m.groups()
        out.setdefault((periph, reg), {})[field.lower()] = int(pos)
    return out


def make_field(parent, name, msb, lsb, desc=""):
    f = ET.SubElement(parent, "field")
    ET.SubElement(f, "name").text = name
    ET.SubElement(f, "description").text = desc or name
    ET.SubElement(f, "bitRange").text = f"[{msb}:{lsb}]"
    ET.SubElement(f, "access").text = "read-write"
    return f


def find_reg(periph_el, name):
    regs = periph_el.find("registers")
    if regs is None:
        return None
    for r in regs.findall("register"):
        if (r.find("name") is not None and r.find("name").text == name):
            return r
    return None


def main():
    cpos = load_c_pos()
    tree = ET.parse(SRC_SVD)
    root = tree.getroot()
    periphs = {}
    for p in root.iter("peripheral"):
        periphs[p.find("name").text] = p

    nreg = nfield = 0

    def check(periph, reg, field, lsb):
        pre = ALIAS.get(periph, periph)
        got = cpos.get((pre, reg), {}).get(field.lower())
        if got is None:
            print(f"  [警告] C 头文件中没有 {pre}_{reg}_{field}_Pos，按补丁表写入")
        elif got != lsb:
            raise SystemExit(
                f"位位置不一致: {periph}.{reg}.{field} 补丁={lsb} C头文件={got}"
            )

    # 1. 新增寄存器
    for periph, reg, off, desc, fields in NEW_REGS:
        pe = periphs.get(periph)
        if pe is None:
            print(f"  [跳过] SVD 无外设 {periph}")
            continue
        if find_reg(pe, reg) is not None:
            print(f"  [已存在] {periph}.{reg}")
            continue
        for f, msb, lsb in fields:
            check(periph, reg, f, lsb)
        regs = pe.find("registers")
        if regs is None:
            regs = ET.SubElement(pe, "registers")
        r = ET.SubElement(regs, "register")
        ET.SubElement(r, "name").text = reg
        ET.SubElement(r, "description").text = desc
        ET.SubElement(r, "addressOffset").text = f"0x{off:X}"
        fs = ET.SubElement(r, "fields")
        for f, msb, lsb in fields:
            make_field(fs, f, msb, lsb)
        nreg += 1
        print(f"  [+寄存器] {periph}.{reg} @0x{off:X} ({len(fields)} 个字段)")

    # 2. 补位域
    for periph, reg, fields in NEW_FIELDS:
        pe = periphs.get(periph)
        if pe is None:
            print(f"  [跳过] SVD 无外设 {periph}")
            continue
        r = find_reg(pe, reg)
        if r is None:
            print(f"  [跳过] {periph} 无寄存器 {reg}")
            continue
        fs = r.find("fields")
        if fs is None:
            fs = ET.SubElement(r, "fields")
        exist = {f.find("name").text for f in fs.findall("field")}
        add = 0
        for f, msb, lsb in fields:
            if f in exist:
                continue
            check(periph, reg, f, lsb)
            make_field(fs, f, msb, lsb)
            exist.add(f)
            add += 1
            nfield += 1
        if add:
            print(f"  [+字段] {periph}.{reg}: {add} 个")

    # 3. 修正字段名笔误
    for periph, reg, old, new in RENAME_FIELDS:
        pe = periphs.get(periph)
        r = find_reg(pe, reg) if pe is not None else None
        if r is None:
            continue
        for f in r.iter("field"):
            if f.find("name").text == old:
                f.find("name").text = new
                print(f"  [改名] {periph}.{reg}: {old} -> {new}")

    # 4. 寄存器按 addressOffset 排序（svd2rust 依赖顺序计算保留字段）
    def off_key(r):
        t = r.find("addressOffset")
        return int(t.text, 16) if t is not None else 0

    for name in {p for p, *_ in NEW_REGS} | {p for p, *_ in NEW_FIELDS}:
        pe = periphs.get(name)
        if pe is None:
            continue
        regs = pe.find("registers")
        if regs is None:
            continue
        items = list(regs)
        if all(r.tag == "register" for r in items):
            regs[:] = sorted(items, key=off_key)

    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    tree.write(OUT, encoding="utf-8", xml_declaration=True)
    print(f"\n新增寄存器 {nreg} 条、新增字段 {nfield} 个 -> {OUT}")


if __name__ == "__main__":
    main()
