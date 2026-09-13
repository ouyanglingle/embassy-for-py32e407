# compare_offsets.py —— 对比 CMSIS 头文件中寄存器的地址偏移与 PAC 生成的偏移，
# 找出布局不一致（很可能是 SVD 错误）的地方。
import os
import re

ROOT = r"D:\embassy-for-py32e407"
CMSIS = os.path.join(
    ROOT, "参考设计", "PY32E407_Firmware_V0.7.0", "Drivers", "CMSIS", "Device",
    "PUYA", "PY32E407", "Include", "py32e407xE.h",
)
PAC_SRC = os.path.join(ROOT, "py32e407-pac", "src")

# 与 compare_regs.py 相同的映射
from compare_regs import MAP  # noqa: E402

SIZE = {"uint8_t": 1, "uint16_t": 2, "uint32_t": 4, "int8_t": 1, "int16_t": 2, "int32_t": 4}


def parse_cmsis(path):
    """返回 {typedef: {reg: offset}}，偏移优先取注释，否则按 sizeof 累加"""
    text = open(path, encoding="utf-8", errors="replace").read()
    out = {}
    for m in re.finditer(r"typedef\s+struct\s*[^{]*\{(.*?)\}\s*(\w+)\s*;", text, re.S):
        name, body = m.group(2), m.group(1)
        off, auto, regs = None, 0, {}
        for line in body.split("\n"):
            mm = re.match(
                r"\s*(?:__I|__O|__IO|__IOM|__OM|__IM)?\s*"
                r"(uint\d+_t|int\d+_t)\s+(\w+)\s*(?:\[\s*(\d+)\s*\])?\s*;"
                r"(.*)$",
                line,
            )
            if not mm:
                continue
            typ, reg, arr, rest = mm.groups()
            n = int(arr) if arr else 1
            cm = re.search(r"[Oo]ffset:\s*(0x[0-9A-Fa-f]+)", rest)
            if cm:
                o = int(cm.group(1), 16)
                auto = o + SIZE[typ] * n
            else:
                o = auto
                auto += SIZE[typ] * n
            regs[reg] = o
        if regs:
            out[name] = regs
    return out


def parse_pac_offsets(pac_src):
    """返回 {module: {reg: offset}}"""
    out = {}
    for fn in sorted(os.listdir(pac_src)):
        if not fn.endswith(".rs"):
            continue
        text = open(os.path.join(pac_src, fn), encoding="utf-8").read()
        pairs = re.findall(
            r'#\[doc = "(0x[0-9a-fA-F]+)[^"]*"\]\s*#\[inline\(always\)\]\s*pub const fn (\w+)',
            text,
        )
        if pairs:
            out[fn[:-3]] = {r: int(a, 16) for a, r in pairs}
    return out


def main():
    cmsis = parse_cmsis(CMSIS)
    pac = parse_pac_offsets(PAC_SRC)

    ndiff = 0
    for td, regs in sorted(cmsis.items()):
        mods = MAP.get(td) or []
        have = {}
        for mod in mods:
            have.update(pac.get(mod, {}))
        if not have:
            continue
        diffs = []
        for reg, off in regs.items():
            if reg.startswith("RESERVED"):
                continue
            key = reg.lower()
            if key in have and have[key] != off:
                diffs.append((reg, off, have[key]))
        if diffs:
            ndiff += len(diffs)
            print(f"{td} -> {'+'.join(mods)}")
            for reg, c_off, r_off in diffs:
                print(f"    {reg:14s} C=0x{c_off:04X}  PAC=0x{r_off:04X}")
    print("\n偏移不一致总数:", ndiff)


if __name__ == "__main__":
    main()
