// 定义字体结构体
#[derive(Copy, Clone)]
pub struct FontDef {
    pub width: u8,
    pub height: u8,
    pub data: &'static [u16],
}

// 定义字体实例
pub const FONT_7X10: FontDef = FontDef {
    width: 7,
    height: 10,
    data: &super::font7x10::FONT7X10,
};
