```text
src/lcd/
├── mod.rs          — pub mod 导出
├── st7735.rs       — 仅保留 St7735Driver（底层 SPI 读写）
├── display.rs      — TftLcd（高级绘图：draw_pixel, fill_rect, draw_string 等）
├── color.rs        — RGB565 枚举
├── config.rs       — Config, Rotation
├── font.rs         — FontDef（从 display.rs 移过来）
├── font7x10.rs     — 字体数据（不变）
└── utils.rs        — i32/u32/f32 to_string 工具
```