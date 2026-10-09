//! HAL 头里**被 bindgen 丢弃**的常量，按官方定义在这里补回。
//!
//! 详见 `py32e407cbind-rs::ll_consts` —— 同样的根因：
//! bindgen 求值宏时认不出 C 风格强制转换，**带 `((uint32_t)…)` 的宏整条被丢弃**（不报错）。
//!
//! 这个文件目前是空壳：先让 build 跑起来，把 build.rs 报出的缺失名字
//! 复制过来按 LL 同样的"逐字转写、不写魔数"方式补就行。
//!
//! > 当前已知问题：先跑 `cargo build --release`，warning 会列出具体哪些名字缺失，
//! > 例如 `HAL_GPIO_SPEED_FREQ_*`、`HAL_UART_WORDLENGTH_*` 这类典型没在绑定里的。
