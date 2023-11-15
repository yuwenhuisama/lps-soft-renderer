use crate::lps::core::gpu::GpuApi;
use crate::lps::rasterize::render_cmds::render_cmd::{RenderCmd, RenderCommandType};
use crate::lps::rasterize::render_target::RenderTarget;

pub struct SetRenderTargetCmd {
    pub render_target: &'static RenderTarget,
}

impl<'a> SetRenderTargetCmd {
    pub fn new(render_target: &'static RenderTarget) -> SetRenderTargetCmd {
        SetRenderTargetCmd {
            render_target,
        }
    }
}

impl RenderCmd for SetRenderTargetCmd {
    fn cmd_type(&self) -> RenderCommandType {
        RenderCommandType::SetRenderTarget
    }

    fn execute(&self, gpu_api: &mut (dyn GpuApi + Sync + Send)) {
        gpu_api.set_render_target(self.render_target);
    }
}
