use crate::lps::core::gpu::GpuApi;
use crate::lps::rasterize::render_cmds::render_cmd::{RenderCmd, RenderCommandType};
use crate::lps::rasterize::render_target::RenderTarget;

pub struct SetRenderTargetCmd<'a> {
    pub render_target: &'a RenderTarget,
}

impl<'a> SetRenderTargetCmd<'a> {
    pub fn new(render_target: &'a RenderTarget) -> SetRenderTargetCmd {
        SetRenderTargetCmd {
            render_target,
        }
    }
}

impl<'a> RenderCmd<'a> for SetRenderTargetCmd<'a> {
    fn cmd_type(&self) -> RenderCommandType {
        RenderCommandType::SetRenderTarget
    }

    fn execute(&self, gpu_api: &'a mut dyn GpuApi<'a>) {
        gpu_api.set_render_target(self.render_target);
    }
}
