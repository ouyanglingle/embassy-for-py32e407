# gen_pac.py —— 一键从 SVD 重新生成 py32e407-pac。
#
# 流程：svd2rust -> rustfmt 完整 lib.rs -> 提取骨架 -> form 拆分模块
#      -> 还原骨架 lib.rs -> build.rs/device.x 归位 -> cargo fmt
#
# 用法：python gen_pac.py [svd 路径]     默认用 py32e407-pac/svd/PY32E407xx.svd
import os
import re
import shutil
import subprocess
import sys

ROOT = r"D:\embassy-for-py32e407"
PAC = os.path.join(ROOT, "py32e407-pac")
SVD = sys.argv[1] if len(sys.argv) > 1 else os.path.join(PAC, "svd", "PY32E407xx.svd")
SRC = os.path.join(PAC, "src")


def run(cmd, **kw):
    print(">", " ".join(cmd))
    subprocess.run(cmd, check=True, cwd=PAC, **kw)


def extract_skeleton(fmt_path, out_path):
    """把完整 lib.rs 里顶层的 `pub mod X { ... }` 替换成 `pub mod X;`"""
    src = open(fmt_path, encoding="utf-8").read().split("\n")
    out, mods, inmod, depth = [], [], None, 0
    for line in src:
        if inmod is None:
            m = re.match(r"^pub mod (\w+) \{$", line)
            if m:
                inmod, depth = m.group(1), 1
                mods.append(inmod)
            else:
                out.append(line)
        else:
            depth += line.count("{") - line.count("}")
            if depth <= 0:
                out.append(f"pub mod {mods[-1]};")
                inmod = None
    open(out_path, "w", encoding="utf-8").write("\n".join(out))
    return mods


def main():
    if not os.path.isfile(SVD):
        raise SystemExit(f"SVD 不存在: {SVD}")

    # 1. svd2rust
    shutil.rmtree(SRC, ignore_errors=True)
    os.makedirs(SRC, exist_ok=True)
    run(["svd2rust", "-i", SVD, "--target", "cortex-m", "--edition", "2021",
         "-o", "src"])

    # 2. 备份完整 lib.rs 并格式化，便于按行匹配顶层 mod
    full = os.path.join(SRC, "lib.rs")
    backup = os.path.join(PAC, "_lib_full.rs")
    shutil.copy(full, backup)
    subprocess.run(["rustfmt", "--edition", "2021", backup], check=True)

    # 3. 提取骨架
    skel = os.path.join(PAC, "_lib_skeleton.rs")
    mods = extract_skeleton(backup, skel)
    print(f"顶层模块 {len(mods)} 个")

    # 4. form 拆分
    run(["form", "-i", "src/lib.rs", "-o", "src/"])
    os.remove(full)
    shutil.move(skel, full)

    # 5. build.rs / device.x 归位到 crate 根目录
    for f in ("build.rs", "device.x"):
        p = os.path.join(SRC, f)
        if os.path.isfile(p):
            shutil.move(p, os.path.join(PAC, f))

    # 6. 格式化
    run(["cargo", "fmt"])
    os.remove(backup)
    print("\n生成完成，模块:", " ".join(mods))


if __name__ == "__main__":
    main()
