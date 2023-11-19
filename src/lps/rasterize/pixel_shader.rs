use super::pipeline::PixelShader;
use super::vt_output::VertexShaderOutput;
use crate::lps::common::math::vec4::Vec4;
use std::{any::Any, sync::Arc};

pub struct CustomPixelShader {}

impl CustomPixelShader {
    pub fn new() -> CustomPixelShader {
        CustomPixelShader {}
    }
}

impl PixelShader<VertexShaderOutput> for CustomPixelShader {
    fn handle(&self, pixel_fragment: &VertexShaderOutput) -> Vec4 {
        Vec4::new(
            pixel_fragment.color.x,
            pixel_fragment.color.y,
            pixel_fragment.color.z,
            255.0,
        )
    }

    fn init_constant_buffer(&mut self, _: &Vec<Option<Arc<dyn Any + Send>>>) {}
}
