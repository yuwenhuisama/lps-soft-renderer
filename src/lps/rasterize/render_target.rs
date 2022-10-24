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
        for j in 0..self.width() {
            for i in 0..self.height() {
                *self.get_pixel(i, j) = color;
            }
        }
    }

    pub fn get_pixel<'a>(&'a mut self, x: u32, y: u32) -> &'a mut Color {
        let idx = usize::try_from(x * self.width() + y).unwrap();
        &mut self.buffer_[idx]
    }

    pub fn draw_point(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width() || y >= self.height() {
            return;
        }

        *self.get_pixel(x, y) = color;
    }

    pub fn save(&mut self, file_name: &str) -> bool {
        let width = u32::try_from(self.width()).unwrap();
        let height = u32::try_from(self.width()).unwrap();

        let mut img = Image::new(width, height);

        for j in 0..self.width() {
            for i in 0..self.height() {
                let color = self.get_pixel(i, j);
                img.set_pixel(
                    i,
                    j,
                    Pixel {
                        r: color.r,
                        g: color.g,
                        b: color.g,
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
