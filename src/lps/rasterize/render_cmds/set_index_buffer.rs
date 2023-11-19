use crate::lps::rasterize::render_cmds::render_cmd::{RenderCmd, RenderCommandType};

pub struct SetIndexBufferCmd {
    pub index_list: Vec<usize>,
}

impl SetIndexBufferCmd {
    pub fn new(index_list: Vec<usize>) -> Self {
        Self { index_list }
    }
}

impl RenderCmd for SetIndexBufferCmd {
    fn cmd_type(&self) -> RenderCommandType {
        RenderCommandType::SetIndexBuffer
    }

    fn execute(&self, gpu_api: &mut (dyn crate::lps::core::gpu::GpuApi + Sync + Send)) {
        gpu_api.set_index_buffer(self.index_list.clone());
    }
}
