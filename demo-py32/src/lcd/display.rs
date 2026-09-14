use super::LcdDriver;
use crate::lcd::{color::RGB565, font::FontDef};

/// 单字符像素缓冲区最大容量（支持最大 16×16 字体）。
/// 如需更大字体，增大此值即可。
const MAX_FONT_PIXELS: usize = 256;

/// 线段绘制时水平/垂直方向的批量像素缓冲区大小。
const LINE_CHUNK_SIZE: usize = 64;

pub struct Display<D: LcdDriver> {
    driver: D,
}

impl<D: LcdDriver> Display<D> {
    pub fn new(driver: D) -> Self {
        Self { driver }
    }

    pub fn draw_char(
        &mut self,
        x: u16,
        y: u16,
        c: u8,
        font: FontDef,
        fg_color: RGB565,
        bg_color: RGB565,
    ) {
        let font_w = font.width as u16;
        let font_h = font.height as u16;
        let area = (font.width as usize) * (font.height as usize);
        assert!(
            area <= MAX_FONT_PIXELS,
            "字体像素缓冲区溢出，请依据 MAX_FONT_PIXELS 计算出适合的字体大小或调整 MAX_FONT_PIXELS 值"
        );

        self.driver
            .set_address_window(x, y, x + font_w - 1, y + font_h - 1);

        let char_offset = ((c as u16).saturating_sub(32)) * font.height as u16;
        let mut pixels = [0u16; MAX_FONT_PIXELS];

        for i in 0..font.height as usize {
            let bitmap_row = font.data[char_offset as usize + i];
            for j in 0..font.width as usize {
                let idx = i * font.width as usize + j;
                pixels[idx] = if (bitmap_row << j) & 0x8000 != 0 {
                    fg_color.into()
                } else {
                    bg_color.into()
                };
            }
        }

        self.driver.write_pixels(&pixels[..area]);
    }

    pub fn draw_string(
        &mut self,
        x: u16,
        y: u16,
        s: &str,
        font: FontDef,
        fg_color: RGB565,
        bg_color: RGB565,
    ) {
        let mut current_x = x;
        let mut current_y = y;
        let screen_w = self.driver.width();
        let screen_h = self.driver.height();
        let font_w = font.width as u16;
        let font_h = font.height as u16;

        for c in s.chars() {
            // 如果当前行放不下，换行
            if current_x + font_w > screen_w {
                current_x = 0;
                current_y += font_h;
                // 换行后超出屏幕高度则停止
                if current_y + font_h > screen_h {
                    break;
                }
            }

            if current_y + font_h <= screen_h {
                let char_byte = if c.is_ascii() { c as u8 } else { b' ' };
                self.draw_char(current_x, current_y, char_byte, font, fg_color, bg_color);
                current_x += font_w;
            }
        }
    }

    /// 绘制单个像素。
    #[inline]
    fn draw_pixel(&mut self, x: u16, y: u16, color: u16) {
        self.driver.set_address_window(x, y, x, y);
        self.driver.write_pixels(&[color]);
    }

    /// 绘制水平线段（批量优化）。
    fn draw_hline(&mut self, x: u16, y: u16, len: u16, color: u16) {
        let count = len as usize;
        let mut buf = [0u16; LINE_CHUNK_SIZE];
        for chunk_start in (0..count).step_by(LINE_CHUNK_SIZE) {
            let chunk_end = core::cmp::min(chunk_start + LINE_CHUNK_SIZE, count);
            let chunk_len = chunk_end - chunk_start;
            for b in &mut buf[..chunk_len] {
                *b = color;
            }
            self.driver
                .set_address_window(x + chunk_start as u16, y, x + chunk_end as u16 - 1, y);
            self.driver.write_pixels(&buf[..chunk_len]);
        }
    }

    /// 绘制垂直线段（批量优化）。
    fn draw_vline(&mut self, x: u16, y: u16, len: u16, color: u16) {
        let count = len as usize;
        let mut buf = [0u16; LINE_CHUNK_SIZE];
        for chunk_start in (0..count).step_by(LINE_CHUNK_SIZE) {
            let chunk_end = core::cmp::min(chunk_start + LINE_CHUNK_SIZE, count);
            let chunk_len = chunk_end - chunk_start;
            for b in &mut buf[..chunk_len] {
                *b = color;
            }
            self.driver
                .set_address_window(x, y + chunk_start as u16, x, y + chunk_end as u16 - 1);
            self.driver.write_pixels(&buf[..chunk_len]);
        }
    }

    /// 绘制线段（Bresenham 算法）。
    ///
    /// 水平线和垂直线会自动使用批量写入优化，斜线逐点绘制。
    pub fn draw_line(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, color: RGB565) {
        let color16: u16 = color.into();

        // 水平线 —— 批量优化
        if y0 == y1 {
            let (xs, xe) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
            return self.draw_hline(xs, y0, xe - xs + 1, color16);
        }
        // 垂直线 —— 批量优化
        if x0 == x1 {
            let (ys, ye) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
            return self.draw_vline(x0, ys, ye - ys + 1, color16);
        }

        // Bresenham 斜线
        let mut x = x0 as i32;
        let mut y = y0 as i32;
        let dx = (x1 as i32 - x0 as i32).abs();
        let dy = -(y1 as i32 - y0 as i32).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.draw_pixel(x as u16, y as u16, color16);
            if x == x1 as i32 && y == y1 as i32 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// 绘制矩形边框（空心矩形）。
    ///
    /// - `w` / `h`：矩形的宽度和高度（像素数）。
    /// - 线宽固定为 1 像素。
    pub fn draw_rect(&mut self, x: u16, y: u16, w: u16, h: u16, color: RGB565) {
        if w < 2 || h < 2 {
            // 退化矩形直接用水平/垂直线填充
            if w > 0 && h > 0 {
                let color16: u16 = color.into();
                if w >= h {
                    self.draw_hline(x, y, w, color16);
                } else {
                    self.draw_vline(x, y, h, color16);
                }
            }
            return;
        }
        let color16: u16 = color.into();
        self.draw_hline(x, y, w, color16); // 上边
        self.draw_hline(x, y + h - 1, w, color16); // 下边
        self.draw_vline(x, y, h, color16); // 左边
        self.draw_vline(x + w - 1, y, h, color16); // 右边
    }

    /// 绘制实心矩形。
    ///
    /// 内部按行调用 `draw_hline`，不额外分配缓冲区。
    pub fn fill_rect(&mut self, x: u16, y: u16, w: u16, h: u16, color: RGB565) {
        if w == 0 || h == 0 {
            return;
        }
        let color16: u16 = color.into();
        for row in 0..h {
            self.draw_hline(x, y + row, w, color16);
        }
    }

    /// 填充整个屏幕
    pub fn fill_screen(&mut self, color: RGB565) {
        let color16: u16 = color.into();
        let w = self.driver.width();
        let h = self.driver.height();
        for row in 0..h {
            self.draw_hline(0, row, w, color16);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
//  BufferDisplay — 全屏帧缓冲驱动
// ═══════════════════════════════════════════════════════════════════

/// 全屏帧缓冲显示器（const 泛型版本）。
///
/// 缓冲区作为结构体内部的内联数组 `[u16; N]`，由总像素数 `N` 自动决定大小，
/// **无需手动声明外部缓冲区**。
///
/// 所有绘图操作（线条、矩形、字符串等）都先写入内存缓冲区。完成后调用
/// `flush()` 或 `flush_rect()` 将变化批量刷新到物理 LCD 上。
///
/// # 用法
///
/// ```ignore
/// // 160×80 = 12800 像素，缓冲区自动分配
/// let mut bdis = BufferDisplay::<12800>::new(160, 80);
/// bdis.fill_screen(RGB565::Black);
/// bdis.draw_string(0, 0, "Hello!", FONT_7X10, RGB565::Yellow, RGB565::Black);
/// bdis.flush(&mut driver);  // 一次性刷新到 LCD
/// ```
pub struct BufferDisplay<const N: usize> {
    /// 内联显存缓冲区，大小由 N 自动决定
    fb: [u16; N],
    width: u16,
    height: u16,
}

impl<const N: usize> BufferDisplay<N> {
    /// 创建帧缓冲显示器（缓冲区自动分配）。
    ///
    /// `width × height` 必须等于 `N`，否则 panic。
    pub fn new(width: u16, height: u16) -> Self {
        assert!(
            (width as usize) * (height as usize) == N,
            "BufferDisplay: W*H ({}) != N ({})",
            (width as usize) * (height as usize),
            N,
        );
        Self {
            fb: [0u16; N],
            width,
            height,
        }
    }

    /// 返回帧缓冲区的不可变引用。
    pub fn framebuffer(&self) -> &[u16] {
        &self.fb
    }

    /// 返回帧缓冲区的可变引用（用于自定义操作）。
    pub fn framebuffer_mut(&mut self) -> &mut [u16] {
        &mut self.fb
    }

    /// 屏幕宽度（像素）。
    pub fn width(&self) -> u16 {
        self.width
    }

    /// 屏幕高度（像素）。
    pub fn height(&self) -> u16 {
        self.height
    }

    // ─── 缓冲区坐标工具 ─────────────────────────────────────────

    /// 将 (x, y) 映射为缓冲区索引（带边界检查，越界则忽略）。
    #[inline]
    fn idx(&self, x: u16, y: u16) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y as usize * self.width as usize + x as usize)
        } else {
            None
        }
    }

    // ─── 像素操作 ───────────────────────────────────────────────

    /// 绘制单个像素到缓冲区。
    pub fn draw_pixel(&mut self, x: u16, y: u16, color: RGB565) {
        if let Some(i) = self.idx(x, y) {
            self.fb[i] = color.into();
        }
    }

    /// 从缓冲区读取单个像素的颜色。
    pub fn get_pixel(&self, x: u16, y: u16) -> Option<u16> {
        self.idx(x, y).map(|i| self.fb[i])
    }

    // ─── 清除 ───────────────────────────────────────────────────

    /// 用指定颜色填充整个缓冲区。
    pub fn fill_screen(&mut self, color: RGB565) {
        let c: u16 = color.into();
        for px in self.fb.iter_mut() {
            *px = c;
        }
    }

    /// 用指定颜色填充矩形区域（仅缓冲区）。
    pub fn fill_rect(&mut self, x: u16, y: u16, w: u16, h: u16, color: RGB565) {
        if w == 0 || h == 0 {
            return;
        }
        let c: u16 = color.into();
        let x_end = (x + w).min(self.width);
        let y_end = (y + h).min(self.height);
        let fb_w = self.width as usize;
        for row in y..y_end {
            let start = row as usize * fb_w + x as usize;
            let end = row as usize * fb_w + x_end as usize;
            for px in self.fb[start..end].iter_mut() {
                *px = c;
            }
        }
    }

    /// 清空整个缓冲区为黑色。
    pub fn clear(&mut self) {
        self.fill_screen(RGB565::Black);
    }

    // ─── 水平 / 垂直线段 ─────────────────────────────────────

    /// 绘制水平线段到缓冲区。
    pub fn draw_hline(&mut self, x: u16, y: u16, len: u16, color: RGB565) {
        if y >= self.height || len == 0 {
            return;
        }
        let c: u16 = color.into();
        let x_end = (x + len).min(self.width);
        let fb_w = self.width as usize;
        let row_start = y as usize * fb_w;
        for xp in x..x_end {
            self.fb[row_start + xp as usize] = c;
        }
    }

    /// 绘制垂直线段到缓冲区。
    pub fn draw_vline(&mut self, x: u16, y: u16, len: u16, color: RGB565) {
        if x >= self.width || len == 0 {
            return;
        }
        let c: u16 = color.into();
        let y_end = (y + len).min(self.height);
        let fb_w = self.width as usize;
        let col = x as usize;
        for yp in y..y_end {
            self.fb[yp as usize * fb_w + col] = c;
        }
    }

    // ─── 线段（Bresenham） ──────────────────────────────────────

    /// 绘制任意线段到缓冲区（Bresenham 算法）。
    pub fn draw_line(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, color: RGB565) {
        // 水平线
        if y0 == y1 {
            let (xs, xe) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
            return self.draw_hline(xs, y0, xe - xs + 1, color);
        }
        // 垂直线
        if x0 == x1 {
            let (ys, ye) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
            return self.draw_vline(x0, ys, ye - ys + 1, color);
        }
        // Bresenham 斜线
        let c: u16 = color.into();
        let mut x = x0 as i32;
        let mut y = y0 as i32;
        let dx = (x1 as i32 - x0 as i32).abs();
        let dy = -(y1 as i32 - y0 as i32).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if let Some(i) = self.idx(x as u16, y as u16) {
                self.fb[i] = c;
            }
            if x == x1 as i32 && y == y1 as i32 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    // ─── 矩形边框 ──────────────────────────────────────────────

    /// 绘制空心矩形边框到缓冲区（线宽 1 像素）。
    pub fn draw_rect(&mut self, x: u16, y: u16, w: u16, h: u16, color: RGB565) {
        if w < 2 || h < 2 {
            if w > 0 && h > 0 {
                if w >= h {
                    self.draw_hline(x, y, w, color);
                } else {
                    self.draw_vline(x, y, h, color);
                }
            }
            return;
        }
        self.draw_hline(x, y, w, color); // 上边
        self.draw_hline(x, y + h - 1, w, color); // 下边
        self.draw_vline(x, y, h, color); // 左边
        self.draw_vline(x + w - 1, y, h, color); // 右边
    }

    // ─── 字符 / 字符串 ─────────────────────────────────────────

    /// 绘制单个字符到缓冲区。
    pub fn draw_char(
        &mut self,
        x: u16,
        y: u16,
        c: u8,
        font: FontDef,
        fg_color: RGB565,
        bg_color: RGB565,
    ) {
        let char_offset = ((c as u16).saturating_sub(32)) * font.height as u16;
        let fg: u16 = fg_color.into();
        let bg: u16 = bg_color.into();
        let fb_w = self.width as usize;

        for i in 0..font.height as usize {
            let bitmap_row = font.data[char_offset as usize + i];
            for j in 0..font.width as usize {
                let px = x + j as u16;
                let py = y + i as u16;
                if px < self.width && py < self.height {
                    let idx = py as usize * fb_w + px as usize;
                    self.fb[idx] = if (bitmap_row << j) & 0x8000 != 0 {
                        fg
                    } else {
                        bg
                    };
                }
            }
        }
    }

    /// 绘制字符串到缓冲区（支持自动换行）。
    pub fn draw_string(
        &mut self,
        x: u16,
        y: u16,
        s: &str,
        font: FontDef,
        fg_color: RGB565,
        bg_color: RGB565,
    ) {
        let mut current_x = x;
        let mut current_y = y;
        let font_w = font.width as u16;
        let font_h = font.height as u16;

        for ch in s.chars() {
            if current_x + font_w > self.width {
                current_x = 0;
                current_y += font_h;
                if current_y + font_h > self.height {
                    break;
                }
            }
            if current_y + font_h <= self.height {
                let char_byte = if ch.is_ascii() { ch as u8 } else { b' ' };
                self.draw_char(current_x, current_y, char_byte, font, fg_color, bg_color);
                current_x += font_w;
            }
        }
    }

    // ─── 刷新到 LCD ────────────────────────────────────────────

    /// 将整个缓冲区刷新到 LCD。
    pub fn flush<D: LcdDriver>(&self, driver: &mut D) {
        driver.set_address_window(0, 0, self.width - 1, self.height - 1);
        driver.write_pixels(&self.fb);
    }

    /// 将指定矩形区域刷新到 LCD。
    ///
    /// 只刷新有变化的区域，比全屏刷新更高效。
    pub fn flush_rect<D: LcdDriver>(&self, driver: &mut D, x: u16, y: u16, w: u16, h: u16) {
        if w == 0 || h == 0 {
            return;
        }
        let x_end = (x + w).min(self.width);
        let y_end = (y + h).min(self.height);
        let flush_w = x_end - x;

        // 逐行读取缓冲区并发送
        let fb_w = self.width as usize;
        driver.set_address_window(x, y, x + flush_w - 1, y_end - 1);

        for row in y..y_end {
            let start = row as usize * fb_w + x as usize;
            let end = row as usize * fb_w + x_end as usize;
            driver.write_pixels(&self.fb[start..end]);
        }
    }
}
