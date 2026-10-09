//! 构建脚本分两步：
//!
//! 1. 用 `cc` 把 PUYA 官方 **HAL C 驱动**（`Drivers/PY32E407_HAL_Driver/Src/py32e407_hal_*.c`）
//!    交叉编译为静态库 `libpy32e407_hal.a`。HAL 的模块选择由 `csrc/py32e407_hal_conf.h` 决定
//!    （cortex/flash/pwr/rcc/gpio/dma/uart —— 增删模块改那个文件，不改这里）。
//!
//! 2. **不调用 bindgen**：HAL 头层层嵌套（master `py32e4xx_hal.h` → `py32e407_hal_conf.h`
//!    → 各模块头 → `py32e407_hal_def.h` → 设备头），宏开关又选了要 include 哪些模块头，
//!    bindgen 的 libclang 对这套顺序的处理需要反复调整 include 路径才能稳定工作。
//!    所以 Rust 侧的 HAL API 绑定**手写**——HAL 函数总共约 200 个，按外设分组列在
//!    `src/lib.rs` 里，每条都对到官方头声明。
//!
//! ## 为什么这么做（对比 cbind-rs）
//!
//! cbind-rs 走 bindgen 路线是因为 LL 头里**全是 `__STATIC_INLINE` 函数**（约 2058 个），
//! 手写不可能。HAL 不一样：绝大多数函数**真的在 `.c` 里实现**（`Init`/`DeInit`/
//! `Transmit_DMA`/`Receive_DMA` 这些），所以只需要手写一份**声明**。
//!
//! ## 链接期符号
//!
//! HAL 的 `.c` 引用了 LL 的 `LL_*` 函数（HAL 是 LL 之上的状态机封装）。
//! 因此本 crate 的用户**必须**在依赖里同时加 `py32e407cbind-rs` 才能链接成功。

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// 运行交叉编译器，捕获其 stdout（去掉换行），失败则 panic。
fn gcc_query(compiler: &str, arg: &str) -> String {
    let out = Command::new(compiler)
        .arg(arg)
        .output()
        .unwrap_or_else(|e| panic!("[py32e407hal-rs] 无法执行 `{compiler}`：{e}"));
    String::from_utf8(out.stdout)
        .expect("[py32e407hal-rs] 编译器输出不是合法 UTF-8")
        .trim()
        .to_string()
}

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
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

    if Command::new(&compiler).arg("--version").status().is_err() {
        panic!(
            "\n[py32e407hal-rs] 未找到交叉编译器 `{compiler}`。\n\
             请安装 GNU Arm Embedded Toolchain，或设置 PY32_CC 指向编译器。"
        );
    }

    // ---- 第 1 步：编译 HAL .c 为 libpy32e407_hal.a ----
    let mut build = cc::Build::new();
    build
        .compiler(&compiler)
        .target("thumbv7em-none-eabihf")
        .flag("-mcpu=cortex-m4")
        .flag("-mthumb")
        .flag("-mfloat-abi=hard")
        .flag("-mfpu=fpv4-sp-d16")
        .flag("-ffreestanding")
        .flag("-fno-builtin")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        // 芯片型号 + 让 HAL 真正导出代码 + 拉入 LL 头（HAL 调用 LL 函数）
        .define("PY32E407xE", None)
        .define("USE_HAL_DRIVER", None)
        .define("USE_FULL_LL_DRIVER", None)
        // HAL 模块选择配置（哪些 HAL_*_MODULE_ENABLED / HSE_VALUE / TICK_INT_PRIORITY 等）
        .include(&manifest_dir.join("csrc"))
        .include(&cmsis_core)
        .include(&cmsis_dev)
        .include(&hal_inc);

    let mut compiled: Vec<String> = Vec::new();
    for entry in fs::read_dir(&hal_src)
        .unwrap_or_else(|e| panic!("[build.rs] 无法读取 HAL 源目录 {hal_src:?}：{e}"))
    {
        let path = entry
            .unwrap_or_else(|e| panic!("[build.rs] 读取 HAL 源目录条目失败：{e}"))
            .path();
        if path.extension().and_then(|e| e.to_str()) != Some("c") {
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if !name.starts_with("py32e407_hal_") {
            continue;
        }
        build.file(&path);
        compiled.push(name);
    }
    assert!(
        !compiled.is_empty(),
        "[build.rs] 未在 {hal_src:?} 找到任何 py32e407_hal_*.c"
    );

    build.compile("py32e407_hal");

    println!("cargo:rerun-if-changed={}", hal_inc.display());
    println!("cargo:rerun-if-changed={}", hal_src.display());
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("csrc").display()
    );
}
