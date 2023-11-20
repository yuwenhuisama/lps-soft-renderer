use crate::lps::common::math::mat4x4::Mat4x4;
use crate::lps::common::math::vec4::Vec4;
use std::{any::Any, sync::Arc};

use super::{pipeline::VertexShader, vt_input::VertexShaderInput, vt_output::VertexShaderOutput};

pub struct CustomVertexShader {
    model_matrix: Option<Mat4x4>,
    view_matrix: Option<Mat4x4>,
    proj_matrix: Option<Mat4x4>,
}

impl CustomVertexShader {
    pub fn new() -> CustomVertexShader {
        CustomVertexShader {
            model_matrix: None,
            view_matrix: None,
            proj_matrix: None,
        }
    }
}

impl VertexShader<VertexShaderInput, VertexShaderOutput> for CustomVertexShader {
    fn handle(&self, vertex: &VertexShaderInput) -> VertexShaderOutput {
        // todo: handle None condition
        let model_matrix = self.model_matrix.as_ref().unwrap();
        let view_matrix = self.view_matrix.as_ref().unwrap();
        let proj_matrix = self.proj_matrix.as_ref().unwrap();

        let world_pos = *model_matrix * vertex.position;
        let window_pos = *proj_matrix * *view_matrix * world_pos;
        let color = Vec4::new(vertex.color.x, vertex.color.y, vertex.color.z, 1.0);
        let texcoord = vertex.texcoord;
        let normal = vertex.normal;

        VertexShaderOutput::new(world_pos, window_pos, color, texcoord, normal)
    }

    fn init_constant_buffer(&mut self, buffer: &Vec<Option<Arc<dyn Any + Send>>>) {
        self.model_matrix = Some(
            buffer[0]
                .as_ref()
                .unwrap()
                .downcast_ref::<Mat4x4>()
                .unwrap()
                .clone(),
        );
        self.view_matrix = Some(
            buffer[1]
                .as_ref()
                .unwrap()
                .downcast_ref::<Mat4x4>()
                .unwrap()
                .clone(),
        );
        self.proj_matrix = Some(
            buffer[2]
                .as_ref()
                .unwrap()
                .downcast_ref::<Mat4x4>()
                .unwrap()
                .clone(),
        );
    }
}
