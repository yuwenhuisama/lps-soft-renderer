use std::any::Any;
use crate::lps::common::math::vec4::Vec4;
use crate::lps::common::math::mat4x4::Mat4x4;
use crate::lps::rasterize::pixel_shader::CustomPixelShader;

use super::{pipeline::VertexShader, vt_input::VertexShaderInput, vt_output::VertexShaderOutput};

pub struct CustomVertexShader<'a> {
    model_matrix_: &'a Mat4x4,
    view_matrix_: &'a Mat4x4,
    proj_matrix_: &'a Mat4x4,
}

impl<'a> CustomPixelShader {
    fn new() -> CustomVertexShader<'a> {
        CustomVertexShader {
            model_matrix_: &Mat4x4::identity(4, 4),
            view_matrix_: &Mat4x4::identity(4, 4),
            proj_matrix_: &Mat4x4::identity(4, 4),
        }
    }
}

impl<'a> VertexShader<VertexShaderInput, VertexShaderOutput> for CustomVertexShader<'a>  {
    fn handle(&self, vertex: &VertexShaderInput) -> VertexShaderOutput {
        let world_pos = self.model_matrix_.clone() * vertex.position.clone();
        let window_pos = self.proj_matrix_.clone() * self.view_matrix_.clone() * world_pos;
        let color = Vec4::new(vertex.color.x, vertex.color.y, vertex.color.z, 1.0);
        let texcoord = vertex.texcoord;
        let normal = vertex.normal;

        VertexShaderOutput::new(world_pos, window_pos, color, texcoord, normal)
    }

    fn init_constant_buffer(&mut self, buffer: &Vec<Option<Box<dyn Any + Send>>>) {
        self.model_matrix_ = buffer[0].unwrap().downcast_ref::<&Mat4x4>().unwrap().clone();
        self.view_matrix_ = buffer[1].unwrap().downcast_ref::<&Mat4x4>().unwrap().clone();
        self.proj_matrix_ = buffer[2].unwrap().downcast_ref::<&Mat4x4>().unwrap().clone();
    }
}