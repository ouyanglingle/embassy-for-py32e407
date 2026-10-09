fn main() {
    // 与同工作区其他 bin 一致：链接时关闭按页对齐、加载 cortex-m-rt 的 link.x
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
}