use crate::lps::core::gpu::GpuApi;
use crate::lps::rasterize::render_cmds::render_cmd::{RenderCmd, RenderCommandType};

pub struct DrawCmd {}

impl DrawCmd {
    pub fn new() -> DrawCmd {
        DrawCmd {}
    }
}

impl RenderCmd for DrawCmd {
    fn cmd_type(&self) -> RenderCommandType {
        RenderCommandType::Draw
    }

    fn execute(&self, gpu_buffer: &mut (dyn GpuApi + Sync + Send)) {
        gpu_buffer.draw();
    }
}
