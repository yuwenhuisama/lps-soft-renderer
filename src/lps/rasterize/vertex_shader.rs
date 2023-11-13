use crate::lps::common::math::{matrix::Matrix, vec4::Vec4};

use super::{pipeline::VertexShader, vt_input::VertexShaderInput, vt_output::VertexShaderOutput};

pub struct CustomVertexShader {
    model_matrix: Matrix,
    view_matrix: Matrix,
    proj_matrix: Matrix,
}

impl VertexShader<VertexShaderInput, VertexShaderOutput> for CustomVertexShader  {
    fn handle(&self, vertex: &VertexShaderInput) -> VertexShaderOutput {
        let world_pos = self.model_matrix * vertex.position;
        let window_pos = self.proj_matrix * self.view_matrix * world_pos;
        let color = Vec4::new(vertex.color.x, vertex.color.y, vertex.color.z, 1.0);
        let texcoord = vertex.texcoord;
        let normal = vertex.normal;

        VertexShaderOutput::new(world_pos, window_pos, color, texcoord, normal)
    }
}