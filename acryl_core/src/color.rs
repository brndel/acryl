#[derive(Clone)]
pub enum Color {
    Gray(u8),
    RGB(u8, u8, u8),
    CMYK(u8, u8, u8, u8),
}

impl Color {
    pub fn rgb_from_hex(value: u64) -> Self {
        let b = ((value & 0x000000ff) >> 0) as u8;
        let g = ((value & 0x0000ff00) >> 8) as u8;
        let r = ((value & 0x00ff0000) >> 16) as u8;

        Color::RGB(r, g, b)
    }
}