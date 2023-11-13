use crate::lps::common::math::vec4::Vec4;

pub trait VertexShader<Input, Output> {
    fn handle(&self, vertex: &Input) -> Output;
}

pub trait PixelShader<Input> {
    fn handle(&self, pixel_fragment: &Input) -> Vec4;
}

pub struct PipeLine<VSInput, VSOutput> {
    vertex_shader: Option<Box<dyn VertexShader<VSInput, VSOutput>>>,
    pixel_shader: Option<Box<dyn PixelShader<VSOutput>>>,
}

impl<VSInput, VSOutput, PSInput> PipeLine<VSInput, VSOutput> {
    pub fn new(vertex_shader: Box<dyn VertexShader<VSInput, VSOutput>>, pixel_shader: Box<dyn PixelShader<VSOutput>>) -> Self {
        PipeLine {
            vertex_shader: Some(vertex_shader),
            pixel_shader: Some(pixel_shader),
        }
    }

    pub fn bind_vertex_shader(&mut self, shader: Box<dyn VertexShader<VSInput, VSOutput>>) {
        self.vertex_shader = Some(shader);
    }

    pub fn bind_pixel_shader(&mut self, shader: Box<dyn PixelShader<VSOutput>>) {
        self.pixel_shader = Some(shader);
    }
}