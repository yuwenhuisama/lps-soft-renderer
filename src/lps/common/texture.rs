use crate::lps::common::color::Color;
use crate::lps::common::math::vec2::Vec2;

#[derive(Clone)]
pub struct Texture {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

impl Texture {
    pub fn load(path: &str) -> Texture {
        let img = image::open(path).unwrap();
        let img_raw_data = img.as_bytes();
        let w = img.width();
        let h = img.height();

        println!("Image format: {:?}", img.color());

        let mut data = vec![];
        for y in 0..h {
            for x in 0..w {
                // let color = img_raw_data.get_pixel(x, y);
                let r = img_raw_data[(y * w + x) as usize * 3];
                let g = img_raw_data[(y * w + x) as usize * 3 + 1];
                let b = img_raw_data[(y * w + x) as usize * 3 + 2];
                data.push(Color::new_rgba(r, g, b, 255));
            }
        }

        Texture {
            width: img.width(),
            height: img.height(),
            data,
        }
    }

    pub fn sample2d(&self, texcoord: Vec2) -> Color {
        let x = texcoord.x - texcoord.x.floor();
        let y = texcoord.y - texcoord.y.floor();

        let color_x = (x * (self.width - 1) as f32) as u32;
        let color_y = (y * (self.height - 1) as f32) as u32;

        // println!("color: {:?}, {:?}", x, y);
        return self.data[(color_y * self.width + color_x) as usize];
    }
}
