use std::any::Any;
use std::sync::Arc;
use crate::lps::core::gpu::GpuApi;
use crate::lps::rasterize::render_cmds::render_cmd::{RenderCmd, RenderCommandType};

pub struct SetVertexBufferCmd {
    pub vertex_list: Vec<Arc<dyn Any + Send + Sync>>,
}

impl SetVertexBufferCmd {
    pub fn new (vertex_list: Vec<Arc<dyn Any + Send + Sync>>) -> SetVertexBufferCmd {
        SetVertexBufferCmd {
            vertex_list,
        }
    }
}

impl<'a> RenderCmd<'a>  for SetVertexBufferCmd {
    fn cmd_type(&self) -> RenderCommandType {
        RenderCommandType::SetVertexBuffer
    }

    fn execute(&self, gpu_api: &'a mut dyn GpuApi<'a>) {
        let copy = self.vertex_list.clone();
        gpu_api.set_vertex_buffer(copy);
    }
}
