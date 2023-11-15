use crate::lps::core::gpu::GpuApi;
use crate::lps::rasterize::render_cmds::render_cmd::{RenderCmd, RenderCommandType};

pub struct Draw {}

impl<'a> Draw {
    pub fn new() -> Draw {
        Draw {}
    }
}

impl RenderCmd for Draw {
    fn cmd_type(&self) -> RenderCommandType {
        RenderCommandType::Draw
    }

    fn execute(&self, gpu_buffer: &mut (dyn GpuApi + Sync + Send)) {
        gpu_buffer.draw();
    }
}
