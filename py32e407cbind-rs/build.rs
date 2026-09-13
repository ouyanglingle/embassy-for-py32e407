//! 构建脚本分两步：
//!
//! 1. 用 `cc` crate 把 PUYA 官方 **全部 C LL 驱动**（`Drivers/PY32E407_HAL_Driver/Src/py32e407_ll_*.c`）
//!    交叉编译为静态库 `libpy32e407_ll.a`。`cc::Build::compile` 会自动向 cargo 输出
//!    `cargo:rustc-link-lib=static=py32e407_ll`，最终链接由它负责（无需手写 `#[link]`）。
//!    扫描 `Src` 目录自动收集所有 `py32e407_ll_*.c`，新增外设无需改这里。
//! 2. 用 `bindgen` 解析动态生成的 `wrapper.h`（包含 `Inc` 下全部 `py32e407_ll_*.h`，allowlist
//!    到 `LL_.*`），自动生成 Rust FFI 绑定写入 `OUT_DIR/bindings.rs`，由 `src/lib.rs` 通过
//!    `include!` 引入。这对应 esp-idf-sys 的“raw 绑定 = bindgen + cc”分层思路。
//!
//! ## 必须环境
//! - `arm-none-eabi-gcc`（交叉编译器）。可用环境变量 `PY32_CC` 覆盖其路径/名称。
//! - `libclang`（bindgen 依赖）。本机已通过 `LIBCLANG_PATH` 指向 esp 工具链的 `libclang.dll`。
//! - 目标 triple：`thumbv7em-none-eabihf`（见 `.cargo/config.toml`）。
//!
//! ## 设计约定
//! - 只挑**纯库、无 `main`** 的 LL 源文件；带 `main()` 的示例绝不能加入。
//! - 不复制官方固件，只读引用 `../参考设计/PY32E407_Firmware_V0.7.0/Drivers/...`。
//! - 必须定义 `PY32E407xE` 与 `USE_FULL_LL_DRIVER`：前者触发外设结构体定义，
//!   后者让 LL 的 `.c`/`.h` 真正导出函数（否则整文件被 `#ifdef` 吃掉，无任何符号）。
//! - **USB / SDMMC 的 LL 驱动依赖 HAL 层类型（`HAL_StatusTypeDef` / `HAL_Delay`），无法脱离
//!   HAL 单独编译，故从本 crate 排除**；其余 ~23 个 LL 外设全部接入。

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// 运行交叉编译器，捕获其 stdout（去掉换行），失败则 panic。
fn gcc_query(compiler: &str, arg: &str) -> String {
    let out = Command::new(compiler)
        .arg(arg)
        .output()
        .unwrap_or_else(|e| panic!("[py32e407cbind-rs] 无法执行 `{compiler}`：{e}"));
    String::from_utf8(out.stdout)
        .expect("[py32e407cbind-rs] 编译器输出不是合法 UTF-8")
        .trim()
        .to_string()
}

/// 依赖 HAL 层、无法脱离 HAL 单独编译的 LL 驱动（按 `.c` 基名排除）。
const HAL_DEPENDENT: &[&str] = &["py32e407_ll_usb.c", "py32e407_ll_sdmmc.c"];

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // crate 位于 `D:/embassy-for-py32e407/py32e407cbind-rs`，上一级即工作区根
    let workspace = manifest_dir
        .parent()
        .expect("[build.rs] crate 不在预期的目录层级");

    let fw = workspace
        .join("参考设计")
        .join("PY32E407_Firmware_V0.7.0");

    let cmsis_core = fw.join("Drivers").join("CMSIS").join("Include");
    let cmsis_dev = fw
        .join("Drivers")
        .join("CMSIS")
        .join("Device")
        .join("PUYA")
        .join("PY32E407")
        .join("Include");
    let hal_inc = fw
        .join("Drivers")
        .join("PY32E407_HAL_Driver")
        .join("Inc");
    let hal_src = fw
        .join("Drivers")
        .join("PY32E407_HAL_Driver")
        .join("Src");

    let compiler = env::var("PY32_CC").unwrap_or_else(|_| "arm-none-eabi-gcc".to_string());

    // 先探测交叉编译器是否存在，缺失时给出清晰指引，而不是把 cc 的晦涩报错甩给用户。
    if Command::new(&compiler).arg("--version").status().is_err() {
        panic!(
            "\n[py32e407cbind-rs] 未找到交叉编译器 `{compiler}`。\n\
             请先安装 GNU Arm Embedded Toolchain（提供 arm-none-eabi-gcc），\n\
             或设置环境变量 PY32_CC 指向你的编译器可执行文件。\n\
             详见 crate 根 README.md 的“环境要求”一节。"
        );
    }

    // ---- 第 1 步：编译全部 LL .c 为 libpy32e407_ll.a（自动扫描 Src 目录）----
    let mut build = cc::Build::new();
    build
        .compiler(&compiler)
        // 关键：build script 永远在 host 上运行，cc 默认会按其 host 目标加 `-m64`，
        // 而 arm-none-eabi-gcc 不认识该选项 → 报错 "unrecognized command-line option '-m64'"。
        // 显式声明目标 triple 即可去掉它，并让 cc 选择 arm 的默认 flags。
        .target("thumbv7em-none-eabihf")
        // Cortex-M4F，硬浮点，与 thumbv7em-none-eabihf 匹配
        .flag("-mcpu=cortex-m4")
        .flag("-mthumb")
        .flag("-mfloat-abi=hard")
        .flag("-mfpu=fpv4-sp-d16")
        // 裸机：不要依赖 libc 的内建实现
        .flag("-ffreestanding")
        .flag("-fno-builtin")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        // 芯片型号 + 让 LL 驱动真正导出函数
        .define("PY32E407xE", None)
        .define("USE_FULL_LL_DRIVER", None)
        .include(&cmsis_core)
        .include(&cmsis_dev)
        .include(&hal_inc);

    let mut compiled: Vec<String> = Vec::new();
    for entry in fs::read_dir(&hal_src)
        .unwrap_or_else(|e| panic!("[build.rs] 无法读取 LL 源目录 {hal_src:?}：{e}"))
    {
        let path = entry
            .unwrap_or_else(|e| panic!("[build.rs] 读取 LL 源目录条目失败：{e}"))
            .path();
        if path.extension().and_then(|e| e.to_str()) != Some("c") {
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        // 只挑 py32e407_ll_*.c，并排除依赖 HAL 的 USB / SDMMC
        if !name.starts_with("py32e407_ll_") {
            continue;
        }
        if HAL_DEPENDENT.contains(&name.as_str()) {
            continue;
        }
        build.file(&path);
        compiled.push(name);
    }
    assert!(
        !compiled.is_empty(),
        "[build.rs] 未在 {hal_src:?} 找到任何 py32e407_ll_*.c"
    );

    // 产出 libpy32e407_ll.a，并自动告知 cargo 以 static 方式链接它。
    build.compile("py32e407_ll");

    // ---- 第 2 步：bindgen 自动生成 Rust 绑定 ----
    // 复用同一个交叉编译器，动态探测 newlib 的 <stdint.h> 目录，避免硬编码路径。
    // <stdint.h> 来自 newlib（sysroot/include，sysroot 本身已是 `.../arm-none-eabi`）；
    // gcc 自带 include 提供 stddef/stdarg 等内建头（clang 也会用自身内建，二者并存无妨）。
    let gcc_internal_inc = gcc_query(&compiler, "--print-file-name=include");
    let sysroot = gcc_query(&compiler, "--print-sysroot");
    let newlib_inc = PathBuf::from(&sysroot).join("include");

    // 动态生成 wrapper.h：包含 Inc 下全部 py32e407_ll_*.h（排除依赖 HAL 的 usb/sdmmc），
    // 避免手写一份会随固件漂移的长列表。
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("[build.rs] 未设置 OUT_DIR"));
    let wrapper_path = out_dir.join("wrapper_gen.h");
    let mut wh = String::from(
        "/* 自动生成，仅供 bindgen 使用：包含全部 LL 头（排除依赖 HAL 的 usb/sdmmc）。 */\n",
    );
    let mut headers: Vec<String> = fs::read_dir(&hal_inc)
        .unwrap_or_else(|e| panic!("[build.rs] 无法读取 LL 头目录 {hal_inc:?}：{e}"))
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n.starts_with("py32e407_ll_") && n.ends_with(".h"))
        // 排除依赖 HAL 的 USB / SDMMC（其 LL 头引用 HAL_StatusTypeDef，无法脱离 HAL 单独绑定）
        .filter(|n| n != "py32e407_ll_usb.h" && n != "py32e407_ll_sdmmc.h")
        .collect();
    headers.sort();
    for h in &headers {
        wh.push_str(&format!("#include \"{h}\"\n"));
    }
    fs::write(&wrapper_path, wh).expect("[build.rs] 无法写入动态 wrapper.h");

    let bindings = bindgen::Builder::default()
        .header(wrapper_path.to_str().expect("OUT_DIR 路径非 UTF-8"))
        // no_std 裸机：用 core 而非 std 的类型前缀，且不生成需要 std 的 layout 测试。
        .use_core()
        .layout_tests(false)
        // 暴露所有 LL 外设 API（LL_.*）及其引用到的类型（如 GPIO_TypeDef、LL_TIM_InitTypeDef、
        // ErrorStatus 等）。bindgen 会自动带入被函数签名引用的结构体。
        .allowlist_item("LL_.*")
        .allowlist_item("ErrorStatus")
        .clang_arg("-target")
        .clang_arg("arm-none-eabi")
        .clang_arg("-mcpu=cortex-m4")
        .clang_arg("-mthumb")
        .clang_arg("-mfloat-abi=hard")
        .clang_arg("-mfpu=fpv4-sp-d16")
        .clang_arg("-DPY32E407xE")
        .clang_arg("-DUSE_FULL_LL_DRIVER")
        .clang_arg(format!("-I{}", cmsis_core.display()))
        .clang_arg(format!("-I{}", cmsis_dev.display()))
        .clang_arg(format!("-I{}", hal_inc.display()))
        .clang_arg(format!("-I{}", newlib_inc.display()))
        .clang_arg(format!("-I{}", gcc_internal_inc))
        .generate()
        .expect("[py32e407cbind-rs] 无法生成 bindgen 绑定，请检查 wrapper.h 与 include 路径");

    let out_path = out_dir.join("bindings.rs");
    bindings
        .write_to_file(&out_path)
        .expect("[py32e407cbind-rs] 无法写入 bindings.rs");

    // 固件包头或源变更时触发重新生成 / 重新编译
    println!("cargo:rerun-if-changed={}", hal_inc.display());
    println!("cargo:rerun-if-changed={}", hal_src.display());
}
