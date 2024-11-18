#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub const fn black() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }
}
