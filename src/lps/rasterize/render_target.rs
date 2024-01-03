use crate::lps::common::color::Color;
use bmp::{Image, Pixel};

pub struct RenderTarget {
    width: u32,
    height: u32,
    buffer: Vec<Color>,
    depth_buffer: Vec<f32>,
}

impl RenderTarget {
    pub fn new(w: u32, h: u32) -> RenderTarget {
        RenderTarget {
            width: w,
            height: h,
            buffer: vec![Color::BLUE; usize::try_from(w * h).unwrap()],
            depth_buffer: vec![10000.0; usize::try_from(w * h).unwrap()],
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.buffer = vec![Color::BLUE; usize::try_from(width * height).unwrap()];
        self.depth_buffer = vec![10000.0; usize::try_from(width * height).unwrap()];
    }

    pub fn clear(&mut self, color: Color) {
        for i in 0..self.width() {
            for j in 0..self.height() {
                *self.get_pixel(i, j) = color;
                *self.get_depth(i, j) = 10000.0;
            }
        }
    }

    pub fn get_screen_depth(&mut self, x: i32, y: i32) -> f32 {
        let screen_x = (x + self.width() as i32 / 2) as u32;
        let screen_y = (self.height() as i32 / 2 - y) as u32;

        *self.get_depth(screen_x, screen_y)
    }

    fn get_depth(&mut self, x: u32, y: u32) -> &mut f32 {
        let idx = usize::try_from(y * self.width() + x).unwrap();
        &mut self.depth_buffer[idx]
    }

    fn get_pixel(&mut self, x: u32, y: u32) -> &mut Color {
        let idx = usize::try_from(y * self.width() + x).unwrap();
        &mut self.buffer[idx]
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

    pub fn draw_depth(&mut self, x: i32, y: i32, depth: f32) {
        let screen_x = (x + self.width() as i32 / 2) as u32;
        let screen_y = (self.height() as i32 / 2 - y) as u32;

        if screen_x >= self.width() || screen_y >= self.height() {
            return;
        }

        *self.get_depth(screen_x, screen_y) = depth;
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
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
