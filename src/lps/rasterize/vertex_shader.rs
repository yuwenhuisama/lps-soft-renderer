use crate::lps::common::vertex::Vertex;

use super::pipeline::VertexShader;

pub struct CustomVertexShader {

}

impl VertexShader for CustomVertexShader {
    fn handle(&self, vertex: &Vertex) -> Vertex {
        todo!()
    }
}