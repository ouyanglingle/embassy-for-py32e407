fn main() {
    //
    // 链接参数统一在这里给出，.cargo/config.toml 的 rustflags 里**不要**再写 -Tlink.x：
    //   - 传两次 → link.x 解析两次，其中的 INCLUDE memory.x 也执行两次，
    //              rust-lld 报 "region 'FLASH' already defined"；
    //   - 一次都不传 → link.x 没被加载，rust-lld 报
    //              "cannot find entry symbol _start"，且产物入口地址为 0（内存布局是错的）。
    // -Tdefmt.x 与 defmt 日志是绑定的：
    //   - 用了 defmt 日志（main.rs 里有 defmt::info! 等）→ 必须传，否则日志无法解码；
    //   - 完全不打 defmt 日志 → 不能传，否则 .defmt 段缺少版本符号，
    //     probe-rs 报 "Failed to parse defmt data"。
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
