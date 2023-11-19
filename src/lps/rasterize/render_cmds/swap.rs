use crate::lps::core::gpu::GpuApi;
use crate::lps::rasterize::render_cmds::render_cmd::{RenderCmd, RenderCommandType};

pub struct Swap {}

impl Swap {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderCmd for Swap {
    fn cmd_type(&self) -> RenderCommandType {
        RenderCommandType::Swap
    }

    fn execute(&self, gpu_api: &mut (dyn GpuApi + Sync + Send)) {
        gpu_api.swap();
    }
}
