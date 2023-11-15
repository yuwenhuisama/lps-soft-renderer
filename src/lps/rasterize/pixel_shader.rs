use super::{
    pipeline::{Color, PixelShader},
    vt_output::VertexShaderOutput,
};
use std::{any::Any, sync::Arc};

pub struct CustomPixelShader {}

impl CustomPixelShader {
    pub fn new() -> CustomPixelShader {
        CustomPixelShader {}
    }
}

impl PixelShader<VertexShaderOutput> for CustomPixelShader {
    fn handle(&self, pixel_fragment: &VertexShaderOutput) -> Color {
        pixel_fragment.color
    }

    fn init_constant_buffer(&mut self, _: &Vec<Option<Arc<dyn Any + Send>>>) {}
}
