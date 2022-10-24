use crate::lps::common::vertex::Vertex;

pub trait VertexShader {
    fn handle(&self, vertex: &Vertex) -> Vertex;
}

pub trait PixelShader {
}

pub struct PipeLine {
    vertex_shader: Option<Box<dyn VertexShader>>,
    pixel_shader: Option<Box<dyn PixelShader>>,
}

impl PipeLine {
    pub fn new(vertex_shader: Box<dyn VertexShader>, pixel_shader: Box<dyn PixelShader>) -> PipeLine {
        PipeLine {
            vertex_shader: Some(vertex_shader),
            pixel_shader: Some(pixel_shader),
        }
    }

    pub fn bind_vertex_shader(&mut self, shader: Box<dyn VertexShader>) {
        self.vertex_shader = Some(shader);
    }

    pub fn bind_pixel_shader(&mut self, shader: Box<dyn PixelShader>) {
        self.pixel_shader = Some(shader);
    }
}