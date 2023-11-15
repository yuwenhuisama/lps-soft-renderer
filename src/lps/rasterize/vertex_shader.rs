use std::any::Any;
use crate::lps::common::math::vec4::Vec4;
use crate::lps::common::math::mat4x4::Mat4x4;
use crate::lps::rasterize::pixel_shader::CustomPixelShader;

use super::{pipeline::VertexShader, vt_input::VertexShaderInput, vt_output::VertexShaderOutput};

pub struct CustomVertexShader {
    model_matrix_: Option<Mat4x4>,
    view_matrix_: Option<Mat4x4>,
    proj_matrix_: Option<Mat4x4>,
}

impl CustomPixelShader {
    fn new() -> CustomVertexShader {
        CustomVertexShader {
            model_matrix_: None,
            view_matrix_: None,
            proj_matrix_: None,
        }
    }
}

impl VertexShader<VertexShaderInput, VertexShaderOutput> for CustomVertexShader  {
    fn handle(&self, vertex: &VertexShaderInput) -> VertexShaderOutput {
        // todo: handle None condition
        let model_matrix = self.model_matrix_.as_ref().unwrap();
        let view_matrix = self.view_matrix_.as_ref().unwrap();
        let proj_matrix = self.proj_matrix_.as_ref().unwrap();

        let world_pos = model_matrix.clone() * vertex.position.clone();
        let window_pos = proj_matrix.clone() * view_matrix.clone() * world_pos;
        let color = Vec4::new(vertex.color.x, vertex.color.y, vertex.color.z, 1.0);
        let texcoord = vertex.texcoord;
        let normal = vertex.normal;

        VertexShaderOutput::new(world_pos, window_pos, color, texcoord, normal)
    }

    fn init_constant_buffer(&mut self, buffer: &Vec<Option<Box<dyn Any + Send>>>) {
        self.model_matrix_ = Some(buffer[0].as_ref().unwrap().downcast_ref::<Mat4x4>().unwrap().clone());
        self.view_matrix_ = Some(buffer[1].as_ref().unwrap().downcast_ref::<Mat4x4>().unwrap().clone());
        self.proj_matrix_ = Some(buffer[2].as_ref().unwrap().downcast_ref::<Mat4x4>().unwrap().clone());
    }
}