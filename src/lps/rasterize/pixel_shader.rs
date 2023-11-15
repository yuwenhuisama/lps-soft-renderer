use std::any::Any;
use crate::lps::rasterize::ps_input::PixelShaderInput;
use super::pipeline::{Color, PixelShader};

pub struct CustomPixelShader {
}

impl PixelShader<PixelShaderInput> for CustomPixelShader {
    fn handle(&self, pixel_fragment: &PixelShaderInput) -> Color {
        pixel_fragment.color
    }

    fn init_constant_buffer(&mut self, _: &Vec<Option<Box<dyn Any + Send>>>) {
    }
}
