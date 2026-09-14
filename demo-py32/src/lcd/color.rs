/// ST7735 显示屏颜色定义（RGB565 格式）
/// RGB565: 5 位红色 + 6 位绿色 + 5 位蓝色 = 16 位颜色
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RGB565 {
    Black = 0x0000,       // 黑色：无红绿蓝分量
    White = 0xFFFF,       // 白色：红绿蓝最大分量
    Red = 0xF800,         // 红色：最大红色分量，无绿蓝
    Green = 0x07E0,       // 绿色：最大绿色分量，无红蓝
    Blue = 0x001F,        // 蓝色：最大蓝色分量，无红绿
    Cyan = 0x07FF,        // 青色：绿色 + 蓝色（绿 + 蓝=青）
    Magenta = 0xF81F,     // 品红：红色 + 蓝色（红 + 蓝=品红）
    Yellow = 0xFFE0,      // 黄色：红色 + 绿色（红 + 绿=黄）
    Orange = 0xFD20,      // 橙色：高红色 + 部分绿色
    Purple = 0x7C0F,      // 紫色：中等红色 + 蓝色
    Pink = 0xFE19,        // 粉色：高红色 + 少量蓝色
    Lime = 0x07E1,        // 酸橙绿：与 Green 相近但略有不同（避免重复）
    Maroon = 0x8000,      // 栗色：少量红色
    Navy = 0x000F,        // 海军蓝：少量蓝色
    Teal = 0x0410,        // 青色：少量绿色 + 蓝色
    Olive = 0x8400,       // 橄榄绿：少量红色 + 绿色
    Silver = 0xC618,      // 银色：中高红绿蓝分量
    Gray = 0x8410,        // 灰色：中等红绿蓝分量
    Aqua = 0x07FE,        // 水色：与 Cyan 相近但略有不同（避免重复）
    Fuchsia = 0xF81E,     // 紫红色：与 Magenta 相近但略有不同（避免重复）
    DarkGreen = 0x0320,   // 深绿色：低绿色分量
    DarkBlue = 0x0011,    // 深蓝色：低蓝色分量
    DarkRed = 0x9000,     // 深红色：中低红色分量
    LightGreen = 0x97D2,  // 浅绿色：较高红绿蓝分量
    LightBlue = 0xB63B,   // 浅蓝色：较高红蓝分量
    LightYellow = 0xFFEC, // 浅黄色：最大红绿 + 高蓝色
    SkyBlue = 0x867D,     // 天蓝色：中等红绿蓝分量
    Gold = 0xFFD0,        // 金色：最大红色 + 高绿色 + 少量蓝色
}

impl From<RGB565> for u16 {
    fn from(color: RGB565) -> Self {
        color as u16 // 底层就是强制转换，无任何运行时开销
    }
}
