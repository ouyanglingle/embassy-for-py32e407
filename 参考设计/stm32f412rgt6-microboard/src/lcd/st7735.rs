use super::LcdDriver;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiBus;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    NOP = 0x00,
    SWRESET = 0x01,
    RDDID = 0x04,
    RDDST = 0x09,
    SLPIN = 0x10,
    SLPOUT = 0x11,
    PTLON = 0x12,
    NORON = 0x13,
    INVOFF = 0x20,
    INVON = 0x21,
    DISPOFF = 0x28,
    DISPON = 0x29,
    CASET = 0x2A,
    RASET = 0x2B,
    RAMWR = 0x2C,
    RAMRD = 0x2E,
    PTLAR = 0x30,
    COLMOD = 0x3A,
    MADCTL = 0x36,
    FRMCTR1 = 0xB1,
    FRMCTR2 = 0xB2,
    FRMCTR3 = 0xB3,
    INVCTR = 0xB4,
    DISSET5 = 0xB6,
    PWCTR1 = 0xC0,
    PWCTR2 = 0xC1,
    PWCTR3 = 0xC2,
    PWCTR4 = 0xC3,
    PWCTR5 = 0xC4,
    VMCTR1 = 0xC5,
    RDID1 = 0xDA,
    RDID2 = 0xDB,
    RDID3 = 0xDC,
    RDID4 = 0xDD,
    PWCTR6 = 0xFC,
    GMCTRP1 = 0xE0,
    GMCTRN1 = 0xE1,
}
#[derive(Clone, Copy)]
pub enum Orientation {
    Portrait = 0x00,         // 0°
    Landscape = 0x60,        // 90°
    PortraitSwapped = 0xC0,  // 180°
    LandscapeSwapped = 0xA0, // 270°
}

/// 对于ST7735的配置，我提供了如下四种
/// ```rust
/// // 0° 竖屏
/// const CONFIG_PORTRAIT: Config = Config {
///    width: 80,
///    height: 160,
///    orientation: Orientation::Portrait,
///     rgb: false,
///     inverted: true,
///     x_offset: 26,
///     y_offset: 1,
///};
/// // 90° 横屏（顺时针旋转）
/// const CONFIG_LANDSCAPE: Config = Config {
///     width: 160,
///     height: 80,
///     orientation: Orientation::Landscape,
///     rgb: false,
///     inverted: true,
///     x_offset: 26,
///     y_offset: 1,
/// };
/// // 180° 竖屏（倒置）
/// const CONFIG_PORTRAIT_SWAPPED: Config = Config {
///     width: 80,
///     height: 160,
///     orientation: Orientation::PortraitSwapped,
///     rgb: false,
///     inverted: true,
///     x_offset: 26,
///     y_offset: 1,
/// };
/// // 270° 横屏（逆时针旋转）
/// const CONFIG_LANDSCAPE_SWAPPED: Config = Config {
///     width: 160,
///     height: 80,
///     orientation: Orientation::LandscapeSwapped,
///     rgb: false,
///     inverted: true,
///     x_offset: 26,
///     y_offset: 1,
/// };
/// ```
pub struct Config {
    pub width: u16,
    pub height: u16,
    pub orientation: Orientation,
    pub rgb: bool,
    pub inverted: bool,
    pub x_offset: u16,
    pub y_offset: u16,
}

impl Config {
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 80,
            height: 160,
            orientation: Orientation::Portrait,
            rgb: false,
            inverted: true,
            x_offset: 26,
            y_offset: 1,
        }
    }
}

pub struct St7735Driver<SPI, OUT, D>
where
    SPI: SpiBus,
    OUT: OutputPin,
    D: DelayNs,
{
    spi: SPI,
    cs: Option<OUT>,
    dc: OUT,
    rst: Option<OUT>,
    blk_pin: Option<OUT>,
    delay: D,
    config: Config,
}

impl<SPI, OUT, D> St7735Driver<SPI, OUT, D>
where
    SPI: SpiBus,
    OUT: OutputPin,
    D: DelayNs,
{
    pub fn new(
        spi: SPI,
        cs: Option<OUT>,
        dc: OUT,
        rst: Option<OUT>,
        blk_pin: Option<OUT>,
        delay: D,
        config: Config,
    ) -> Self {
        Self {
            spi,
            cs,
            dc,
            rst,
            blk_pin,
            delay,
            config,
        }
    }

    /// 根据当前方向返回 (x_offset, y_offset)
    fn get_offsets(&self) -> (u16, u16) {
        match self.config.orientation {
            Orientation::Portrait | Orientation::PortraitSwapped => {
                (self.config.x_offset, self.config.y_offset)
            }
            Orientation::Landscape | Orientation::LandscapeSwapped => {
                (self.config.y_offset, self.config.x_offset)
            }
        }
    }

    pub fn select(&mut self, select: bool) {
        if let Some(cs) = self.cs.as_mut() {
            if select {
                let _ = cs.set_low();
            } else {
                let _ = cs.set_high();
            }
        }
    }

    pub fn reset(&mut self) {
        if let Some(rst) = self.rst.as_mut() {
            let _ = rst.set_high();
            self.delay.delay_ms(5);
            let _ = rst.set_low();
            self.delay.delay_ms(5);
            let _ = rst.set_high();
        }
    }

    fn write_command(&mut self, command: Instruction, params: &[u8]) -> Result<(), SPI::Error> {
        let _ = self.dc.set_low();
        self.spi.write(&[command as u8])?;
        if !params.is_empty() {
            self.write_data(params)?;
        }
        Ok(())
    }

    pub fn write_data(&mut self, data: &[u8]) -> Result<(), SPI::Error> {
        let _ = self.dc.set_high();
        let result = self.spi.write(data);
        self.spi.flush()?;
        result
    }

    fn set_orientation(&mut self) {
        if self.config.rgb {
            let _ = self.write_command(Instruction::MADCTL, &[self.config.orientation as u8]);
        } else {
            let _ =
                self.write_command(Instruction::MADCTL, &[self.config.orientation as u8 | 0x08]);
        }
    }

    fn start_data(&mut self) {
        let _ = self.dc.set_high();
    }

    fn write_word(&mut self, value: u16) -> Result<(), SPI::Error> {
        self.write_data(&value.to_be_bytes())
    }

    fn write_words_buffered(
        &mut self,
        words: impl IntoIterator<Item = u16>,
    ) -> Result<(), SPI::Error> {
        let mut buffer = [0; 32];
        let mut index = 0;
        for word in words {
            let as_bytes = word.to_be_bytes();
            buffer[index] = as_bytes[0];
            buffer[index + 1] = as_bytes[1];
            index += 2;
            if index >= buffer.len() {
                self.write_data(&buffer)?;
                index = 0;
            }
        }
        self.write_data(&buffer[0..index])
    }
}

impl<SPI, OUT, D> LcdDriver for St7735Driver<SPI, OUT, D>
where
    SPI: SpiBus,
    OUT: OutputPin,
    D: DelayNs,
{
    fn init(&mut self) {
        if let Some(blk) = self.blk_pin.as_mut() {
            let _ = blk.set_high();
        }

        self.reset();
        self.delay.delay_ms(10);

        self.select(true);
        let _ = self.write_command(Instruction::SLPOUT, &[]);
        self.delay.delay_ms(200);

        let _ = self.write_command(Instruction::FRMCTR1, &[0x01, 0x2C, 0x2D]);
        let _ = self.write_command(Instruction::FRMCTR2, &[0x01, 0x2C, 0x2D]);
        let _ = self.write_command(Instruction::FRMCTR3, &[0x01, 0x2C, 0x2D, 0x01, 0x2C, 0x2D]);
        let _ = self.write_command(Instruction::INVCTR, &[0x07]);
        let _ = self.write_command(Instruction::PWCTR1, &[0xA2, 0x02, 0x84]);
        let _ = self.write_command(Instruction::PWCTR2, &[0xC5]);
        let _ = self.write_command(Instruction::PWCTR3, &[0x0A, 0x00]);
        let _ = self.write_command(Instruction::PWCTR4, &[0x8A, 0x2A]);
        let _ = self.write_command(Instruction::PWCTR5, &[0x8A, 0xEE]);
        let _ = self.write_command(Instruction::VMCTR1, &[0x0E]);

        if self.config.inverted {
            let _ = self.write_command(Instruction::INVON, &[]);
        } else {
            let _ = self.write_command(Instruction::INVOFF, &[]);
        }

        let _ = self.write_command(
            Instruction::GMCTRP1,
            &[
                0x02, 0x1c, 0x07, 0x12, 0x37, 0x32, 0x29, 0x2d, 0x29, 0x25, 0x2B, 0x39, 0x00, 0x01,
                0x03, 0x10,
            ],
        );
        let _ = self.write_command(
            Instruction::GMCTRN1,
            &[
                0x03, 0x1d, 0x07, 0x06, 0x2E, 0x2C, 0x29, 0x2D, 0x2E, 0x2E, 0x37, 0x3F, 0x00, 0x00,
                0x02, 0x10,
            ],
        );

        self.set_orientation();

        let _ = self.write_command(Instruction::COLMOD, &[0x05]);
        let _ = self.write_command(Instruction::NORON, &[]);
        self.delay.delay_ms(10);
        let _ = self.write_command(Instruction::DISPON, &[]);
        self.select(false);
        self.delay.delay_ms(100);
    }

    fn set_address_window(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) {
        let (x_off, y_off) = self.get_offsets();
        self.select(true);
        let _ = self.write_command(Instruction::CASET, &[]);
        let _ = self.start_data();
        let _ = self.write_word(x0 + x_off);
        let _ = self.write_word(x1 + x_off);
        let _ = self.write_command(Instruction::RASET, &[]);
        let _ = self.start_data();
        let _ = self.write_word(y0 + y_off);
        let _ = self.write_word(y1 + y_off);
        self.select(false);
    }

    fn write_pixels(&mut self, colors: &[u16]) {
        self.select(true);
        let _ = self.write_command(Instruction::RAMWR, &[]);
        self.start_data();
        for color in colors {
            let _ = self.write_word(*color);
        }
        self.select(false);
    }

    fn width(&self) -> u16 {
        self.config.width
    }

    fn height(&self) -> u16 {
        self.config.height
    }
}
