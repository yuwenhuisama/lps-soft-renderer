use crate::lps::core::gpu::GpuApi;
use crate::lps::rasterize::render_cmds::render_cmd::{RenderCmd, RenderCommandType};

pub struct DrawCmd {
    draw_with_index: bool,
}

impl DrawCmd {
    pub fn new(draw_with_index: bool) -> DrawCmd {
        DrawCmd {
            draw_with_index: draw_with_index,
        }
    }

    pub fn draw_with_index(&self) -> bool {
        self.draw_with_index
    }
}

impl RenderCmd for DrawCmd {
    fn cmd_type(&self) -> RenderCommandType {
        RenderCommandType::Draw
    }

    fn execute(&self, gpu_buffer: &mut (dyn GpuApi + Sync + Send)) {
        gpu_buffer.draw(self.draw_with_index);
    }
}
