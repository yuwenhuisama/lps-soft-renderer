#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE: Color = Color::new_rgba(255, 255, 255, 255);
    pub const BLACK: Color = Color::new_rgba(0, 0, 0, 255);
    pub const BLUE: Color = Color::new_rgba(0, 0, 255, 255);

    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}
