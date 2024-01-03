use crate::lps::common::math::vec4::Vec4;
use std::{any::Any, sync::Arc};

pub trait VertexShader<Input, Output> {
    fn handle(&self, vertex: &Input) -> Output;

    fn init_constant_buffer(&mut self, buffer: &Vec<Option<Arc<dyn Any + Send>>>);
}

pub trait PixelShader<Input> {
    fn handle(&self, pixel_fragment: &Input) -> Vec4;

    fn init_constant_buffer(&mut self, buffer: &Vec<Option<Arc<dyn Any + Send>>>);
}

pub struct PipeLine<VSInput, VSOutput> {
    vertex_shader: Option<Box<dyn VertexShader<VSInput, VSOutput> + Send + Sync>>,
    pixel_shader: Option<Box<dyn PixelShader<VSOutput> + Send + Sync>>,
}

impl<VSInput, VSOutput> PipeLine<VSInput, VSOutput> {
    pub fn new(
        vertex_shader: Option<Box<dyn VertexShader<VSInput, VSOutput> + Send + Sync>>,
        pixel_shader: Option<Box<dyn PixelShader<VSOutput> + Send + Sync>>,
    ) -> Self {
        PipeLine {
            vertex_shader,
            pixel_shader,
        }
    }

    pub fn bind_vertex_shader(
        &mut self,
        shader: Option<Box<dyn VertexShader<VSInput, VSOutput> + Send + Sync>>,
    ) {
        self.vertex_shader = shader;
    }

    pub fn bind_pixel_shader(
        &mut self,
        shader: Option<Box<dyn PixelShader<VSOutput> + Send + Sync>>,
    ) {
        self.pixel_shader = shader;
    }

    pub fn handle_vertex_shader(
        &mut self,
        vertex: &VSInput,
        constant_buffer: &Vec<Option<Arc<dyn Any + Send>>>,
    ) -> VSOutput {
        if let None = self.vertex_shader {
            panic!("vertex shader is not bound");
        }

        let vertex_shader = self.vertex_shader.as_mut().unwrap();
        vertex_shader.init_constant_buffer(&constant_buffer);
        vertex_shader.handle(vertex)
    }

    pub fn handle_pixel_shader(
        &mut self,
        pixel_fragment: &VSOutput,
        constant_buffer: &Vec<Option<Arc<dyn Any + Send>>>,
    ) -> Vec4 {
        if let None = self.pixel_shader {
            panic!("pixel shader is not bound");
        }

        let pixel_shader = self.pixel_shader.as_mut().unwrap();
        pixel_shader.init_constant_buffer(&constant_buffer);
        return pixel_shader.handle(pixel_fragment);
    }
}
