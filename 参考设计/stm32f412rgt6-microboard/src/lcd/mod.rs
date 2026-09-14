pub mod color;
pub mod display;
pub mod font;
pub mod font7x10;
pub mod st7735;
pub mod utils;

/// LCD驱动接口
pub trait LcdDriver {
    /// 初始化LCD驱动
    fn init(&mut self);
    /// 设置LCD地址窗口
    fn set_address_window(&mut self, x0: u16, y0: u16, x1: u16, y1: u16);
    /// 写入像素数据, 只管"发数据"，不管"发到哪"
    fn write_pixels(&mut self, pixels: &[u16]);
    /// 获取LCD宽度
    fn width(&self) -> u16;
    /// 获取LCD高度
    fn height(&self) -> u16;
}
