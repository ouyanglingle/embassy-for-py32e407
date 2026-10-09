//! 构建脚本分三步：
//!
//! 1. 用 `cc` crate 把 PUYA 官方 **全部 C LL 驱动**（`Drivers/PY32E407_HAL_Driver/Src/py32e407_ll_*.c`）
//!    交叉编译为静态库 `libpy32e407_ll.a`。`cc::Build::compile` 会自动向 cargo 输出
//!    `cargo:rustc-link-lib=static=py32e407_ll`，最终链接由它负责（无需手写 `#[link]`）。
//!    扫描 `Src` 目录自动收集所有 `py32e407_ll_*.c`，新增外设无需改这里。
//! 2. 用 `bindgen` 解析动态生成的 `wrapper.h`（包含 `Inc` 下全部 `py32e407_ll_*.h`，
//!    allowlist 到 `LL_.*` / `ErrorStatus` / `.*_BASE` / 设备头全大写宏），生成 Rust 绑定写入
//!    `OUT_DIR/bindings.rs`，由 `src/lib.rs` 通过 `include!` 引入。
//!    其中「全大写宏」那条**不是可选项**：bindgen 求值宏时要拿替换表里的标识符查自己
//!    已生成的常量表，查不到就静默丢弃整条宏，而设备头的寄存器位宏是三层引用结构
//!    （`LL_X` → `PERIPH_REG_FIELD_n` → `PERIPH_REG_FIELD_Pos`），缺一层就丢一片常量。
//!    同一趟里开启 **`wrap_static_fns`**：LL 头里约 2000 个函数是 `__STATIC_INLINE`
//!    （`static` = 内部链接，只产生局部符号，永远不进全局符号表，**既不会被 bindgen
//!    生成声明、手工声明也必然链接失败**）。该选项让 bindgen **生成一个 C 源文件**，
//!    为每个 static inline 函数写一个非 static 包装，命名 `<原名>__ext`。
//! 3. 用 `cc` 把上一步生成的包装源编成**第二个静态库** `libpy32e407_ll_ext.a`
//!    （必须在 bindgen 之后，因为文件名到那才知道）。
//! 2b. 顺带统计「LL 头里有、`bindings.rs` 里没有」的常量并用 `cargo:warning` 报出数量。
//!    bindgen 对解析不了的宏是**静默丢弃**的（典型是带 `((uint32_t)位宏)` 强制转换的那批），
//!    不报出来就永远发现不了。缺失项按 `src/ll_consts.rs` 的格式按官方定义补回。
//!
//! 关键点：bindgen 为包装生成的是
//! `#[link_name = "LL_GPIO_TogglePin__ext"] pub fn LL_GPIO_TogglePin(...)` ——
//! **Rust 侧的函数名就是官方名**，`__ext` 只出现在 `link_name` 里。
//! 所以调用方直接写官方函数名即可，**不需要任何别名层、也不需要手工维护清单**
//! （实测 bindings.rs 里 `pub fn \w+__ext` 零匹配）。
//!
//! 这对应 esp-idf-sys 的“raw 绑定 = bindgen + cc”分层思路。
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
    // 注意：`__STATIC_INLINE` 的包装函数由第 3 步单独编成另一个静态库（须等 bindgen 先生成）。
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

    // 第 2 步的重点之一：把 LL 头里 **全部 `__STATIC_INLINE` 函数**自动"外部化"。
    //
    // 背景：`static` 是内部链接，这些函数只会被内联进用到它们的 `.c`、最多留下**局部符号**
    // （`nm` 里小写 `t`），**永远不产生全局符号** —— 所以 bindgen 压根不会为它们生成声明，
    // 手工声明也必然链接失败（约 2000 个，手写清单不现实）。
    //
    // `wrap_static_fns(true)` 让 bindgen **生成一个 C 源文件**，里面为每个 static inline
    // 函数写一个非 static 的包装，命名 `<原名>__ext`（由 `wrap_static_fns_suffix` 指定，
    // 默认是 `__extern`；这里统一用 `__ext` = externalized）。
    // 然后本脚本用 cc 把它编进静态库（见下面"第 3 步"），Rust 侧即可 FFI 调用。
    //
    // bindgen 会自动为生成的包装写进 bindings.rs，并**自动加正确的扩展名**
    // （此处会得到 `ll_static_fns_ext.h` / `ll_static_fns_ext.c`）。
    let static_fns_path = out_dir.join("ll_static_fns_ext");
    let bindings = bindgen::Builder::default()
        .header(wrapper_path.to_str().expect("OUT_DIR 路径非 UTF-8"))
        .wrap_static_fns(true)
        .wrap_static_fns_path(&static_fns_path)
        .wrap_static_fns_suffix("__ext")
        // no_std 裸机：用 core 而非 std 的类型前缀，且不生成需要 std 的 layout 测试。
        .use_core()
        .layout_tests(false)
        // 暴露所有 LL 外设 API（LL_.*）及其引用到的类型（如 GPIO_TypeDef、LL_TIM_InitTypeDef、
        // ErrorStatus 等）。bindgen 会自动带入被函数签名引用的结构体。
        // `LL_.*` 同样覆盖自动生成的 `<官方名>__ext` 包装（名字仍以 LL_ 开头）。
        .allowlist_item("LL_.*")
        .allowlist_item("ErrorStatus")
        // 各外设的基址宏（设备头里 `#define USART2_BASE (...)`、`#define GPIOC_BASE (0x48000800UL)` …）。
        // 导出后调用方才能在不引入 PAC 的前提下拿到外设指针：
        //     `GPIOC_BASE as *mut GpioType`
        //     `USART2_BASE as *mut UART_TypeDef`
        .allowlist_item(".*_BASE")
        // ---- 设备头里的「全大写宏」（寄存器位定义 / 字段掩码）----
        //
        // 这一条不能省。bindgen 求值一个函数式宏时，**会拿替换表里的标识符去查它自己
        // 已生成的常量表**；查不到就直接**静默丢弃整条宏**（既不报警也不生成）。
        // 而设备头里的寄存器位宏是「宏引用宏」的三层结构：
        //
        //     #define LL_DMA_DIRECTION_MEMORY_TO_PERIPH ((uint32_t)DMA_CH_CTRLL_TT_0)
        //     #define DMA_CH_CTRLL_TT_0                 (0x1UL<<DMA_CH_CTRLL_TT_Pos)
        //     #define DMA_CH_CTRLL_TT_Pos               20U
        //
        // 中间两层（`DMA_CH_CTRLL_TT_0` / `..._Pos`）若不在 allowlist 里，
        // `LL_DMA_DIRECTION_MEMORY_TO_PERIPH` 就会**凭空消失** —— 表现为 Rust 侧
        // 「常量未定义」，而 C 头里明明写得清清楚楚。
        // 受影响的远不止 DMA：实测还会丢 `LL_UART_DATAWIDTH_8B`（`UART_CR1_M_1|UART_CR1_M_0`）、
        // `LL_DMA_SRC_ADDR_FIX`、`LL_DMA_PRIORITY_1..5`、`LL_DMA_*_PERIPH_MAP_2..6` 等等。
        //
        // 设备头里「全大写」的标识符恰好就是这类宏（寄存器位、外设基址、时钟值…），
        // 而所有类型名都含小写字母（`GPIO_TypeDef`、`LL_GPIO_InitTypeDef`），
        // 所以这条规则既够宽（把宏链条补齐）又不会把类型定义搅进来。
        // 纯指针/转换类宏（`#define DMA1 ((DMA_TypeDef *)DMA1_BASE)`）bindgen 求值失败会自动跳过。
        .allowlist_item("[A-Z][A-Z0-9_]*")
        // 标准整型 typedef（`uint32_t` 等）。
        //
        // 同样不能省。LL 头里大量常量写成 `((uint32_t)某个位宏)`，例如
        //     #define LL_DMA_DIRECTION_MEMORY_TO_PERIPH ((uint32_t)DMA_CH_CTRLL_TT_0)
        //     #define LL_UART_DATAWIDTH_8B ((uint32_t)(UART_CR1_M_1) | (UART_CR1_M_0))
        // bindgen 解析宏时要把这层 C 风格强制转换剥掉，而剥离前它得先在**自己的类型表**里
        // 认识 `uint32_t`。不把这类 typedef 导出来，它就认不出 → **整条宏静默丢弃**
        // （现象：`LL_GPIO_SPEED_FREQ_*` 这类纯字面量宏都在，凡是带 `(uint32_t)` 的全没了）。
        .allowlist_item("u?int(8|16|32|64)_t")
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

    // ---- 第 2b 步：把「头里有、绑定里没有」的 LL 常量报出来 ----
    // bindgen 会**静默丢弃**它解析不了的宏；不报出来的话缺口会在几个月后才以
    // 「官方文档上有这个名字、Rust 里却编译不过」的形式出现。详见下面函数注释。
    report_missing_ll_consts(&hal_inc, &out_path);

    // ---- 第 3 步：把 bindgen 生成的包装编成第二个静态库 ----
    // 这些包装必须在 bindgen 跑完后才知道有哪些，所以单独一个 cc::Build。
    // `compile()` 会向 cargo 输出 `cargo:rustc-link-lib=static=py32e407_ll_ext`。
    let ext_c = static_fns_path.with_extension("c");
    assert!(
        ext_c.is_file(),
        "[build.rs] bindgen 未生成包装源文件 {ext_c:?}（wrap_static_fns 是否生效？）"
    );
    cc::Build::new()
        .compiler(&compiler)
        .target("thumbv7em-none-eabihf")
        .flag("-mcpu=cortex-m4")
        .flag("-mthumb")
        .flag("-mfloat-abi=hard")
        .flag("-mfpu=fpv4-sp-d16")
        .flag("-ffreestanding")
        .flag("-fno-builtin")
        // 每个包装一个 section + 链接期 --gc-sections：只用到的才进最终镜像，
        // 所以外部化 2000 个函数并不会让固件变大。
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .define("PY32E407xE", None)
        .define("USE_FULL_LL_DRIVER", None)
        .include(&cmsis_core)
        .include(&cmsis_dev)
        .include(&hal_inc)
        .file(&ext_c)
        .compile("py32e407_ll_ext");

    // 注意：**不需要**再为包装生成别名。bindgen 的 `wrap_static_fns` 已经用
    // `#[link_name = "<官方名>__ext"]` 把符号重定向，而 Rust 侧的函数名就是
    // **官方名本身**（实测 bindings.rs 里 `pub fn \w+__ext` 零匹配），
    // 所以调用方直接写 `LL_USART_TransmitData8(...)` 即可。

    // 固件包头/源变更时触发重新生成 / 重新编译
    println!("cargo:rerun-if-changed={}", hal_inc.display());
    println!("cargo:rerun-if-changed={}", hal_src.display());
}

/// 统计 LL 头里的**对象式宏**（`#define LL_xxx 值`）有多少条没进 `bindings.rs`，
/// 有缺失就打一条 `cargo:warning`。
///
/// # 为什么需要这个检查
///
/// bindgen 对解析不了的宏是**静默丢弃**的：既不报错，也不在绑定里留任何痕迹。
/// 典型受害者是替换表里带 C 风格强制转换的那批（`((uint32_t)DMA_CH_CTRLL_TT_0)`），
/// 而同组的纯字面量宏（`0x00000000U`）却一切正常 —— 于是缺口极难自己发现，
/// 往往要等到几个月后「官方文档上明明有这个名字、Rust 里却编译不过」才暴露。
///
/// 本函数把缺口变成一条**构建期可见的数字**。缺失项按 `src/ll_consts.rs` 的格式补回。
fn report_missing_ll_consts(hal_inc: &std::path::Path, bindings_path: &std::path::Path) {
    let Ok(bindings) = fs::read_to_string(bindings_path) else {
        return;
    };

    // 绑定里已出现的常量名（扫一遍即可，避免对每个名字做一次全文搜索）
    let mut emitted: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for line in bindings.lines() {
        for kw in ["pub const ", "pub static "] {
            if let Some(rest) = line.strip_prefix(kw) {
                if let Some(name) = rest.split([':', ' ', '(']).next() {
                    emitted.insert(name);
                }
            }
        }
    }

    // LL 头里声明的对象式宏名
    let mut declared: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let Ok(entries) = fs::read_dir(hal_inc) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("h") {
            continue;
        }
        let file = path.file_name().unwrap().to_string_lossy().to_string();
        if !file.starts_with("py32e407_ll_") || file.contains("usb") || file.contains("sdmmc") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        for line in text.lines() {
            let Some(rest) = line.trim_start().strip_prefix("#define ") else {
                continue;
            };
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            // 函数式宏（`LL_x(...)`）由 wrap_static_fns 处理，不在这里统计
            if rest[name.len()..].starts_with('(') {
                continue;
            }
            if !name.starts_with("LL_") {
                continue;
            }
            declared.insert(name);
        }
    }

    let missing: Vec<&String> = declared
        .iter()
        .filter(|n| !emitted.contains(n.as_str()))
        .collect();
    if missing.is_empty() {
        return;
    }

    let sample: Vec<&str> = missing.iter().take(6).map(|s| s.as_str()).collect();
    println!(
        "cargo:warning=py32e407cbind-rs: LL 头里有 {} 个常量没进 bindings.rs（bindgen 解析不了带 `(uint32_t)` 之类强制转换的宏），\
         例如 {} …。它们不会报错，只会在写代码时表现为「常量不存在」。补法见 src/ll_consts.rs",
        missing.len(),
        sample.join(" / ")
    );
}
