use crate::lps::common::math::vec4::Vec4;
use crate::lps::core::gpu::GpuApi;
use crate::lps::rasterize::render_cmds::render_cmd::{RenderCmd, RenderCommandType};

pub struct ClearCmd {
    pub color: Vec4,
}

impl ClearCmd {
    pub fn new(color: Vec4) -> ClearCmd {
        ClearCmd { color }
    }
}

impl RenderCmd for ClearCmd {
    fn cmd_type(&self) -> RenderCommandType {
        RenderCommandType::Clear
    }

    fn execute(&self, gpu_api: &mut (dyn GpuApi + Sync + Send)) {
        gpu_api.clear(&self.color);
    }
}
