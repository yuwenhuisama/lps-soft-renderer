use crate::lps::common::color::Color;
use bmp::{Image, Pixel};

pub struct RenderTarget {
    width_: u32,
    height_: u32,
    buffer_: Vec<Color>,
}

impl RenderTarget {
    pub fn new(w: u32, h: u32) -> RenderTarget {
        RenderTarget {
            width_: w,
            height_: h,
            buffer_: vec![Color::BLUE; usize::try_from(w * h).unwrap()],
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width_ = width;
        self.height_ = height;
        self.buffer_ = vec![Color::BLUE; usize::try_from(width * height).unwrap()];
    }

    pub fn clear(&mut self, color: Color) {
        for i in 0..self.width() {
            for j in 0..self.height() {
                *self.get_pixel(i, j) = color;
            }
        }
    }

    pub fn get_pixel<'a>(&'a mut self, x: u32, y: u32) -> &'a mut Color {
        let idx = usize::try_from(y * self.width() + x).unwrap();
        &mut self.buffer_[idx]
    }

    pub fn draw_point(&mut self, x: i32, y: i32, color: &Color) {
        // logic screen (0, 0) -> render target (width / 2, height / 2)
        // logic screen (x, y) -> render target (x + width / 2, height / 2 - y)
        let screen_x = (x + self.width() as i32 / 2) as u32;
        let screen_y = (self.height() as i32 / 2 - y) as u32;

        if screen_x >= self.width() || screen_y >= self.height() {
            return;
        }

        *self.get_pixel(screen_x, screen_y) = color.clone();
    }

    pub fn save(&mut self, file_name: &str) -> bool {
        let width = u32::try_from(self.width()).unwrap();
        let height = u32::try_from(self.height()).unwrap();

        let mut img = Image::new(width, height);

        for i in 0..self.width() {
            for j in 0..self.height() {
                let color = self.get_pixel(i, j);
                img.set_pixel(
                    i,
                    j,
                    Pixel {
                        r: color.r,
                        g: color.g,
                        b: color.b,
                    },
                );
            }
        }

        let res = img.save(file_name);
        res.is_ok()
    }

    pub fn width(&self) -> u32 {
        self.width_
    }

    pub fn height(&self) -> u32 {
        self.height_
    }
}
