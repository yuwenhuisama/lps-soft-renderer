use std::any::Any;
use crate::lps::core::gpu::GpuApi;
use crate::lps::rasterize::render_cmds::render_cmd::{RenderCmd, RenderCommandType};

pub struct SetConstantBufferCmd<'a> {
    pub layout_index: usize,
    pub buffer: &'a (dyn Any + Send + Sync),
}

impl<'a> SetConstantBufferCmd<'a> {
    pub fn new(layout_index: usize, buffer: &'a (dyn Any + Send + Sync)) -> SetConstantBufferCmd {
        SetConstantBufferCmd {
            layout_index,
            buffer,
        }
    }
}

impl<'a> RenderCmd<'a> for SetConstantBufferCmd<'a> {
    fn cmd_type(&self) -> RenderCommandType {
        RenderCommandType::SetConstantBuffer
    }

    fn execute(&self, gpu_api: &mut dyn GpuApi<'a>) {
        gpu_api.set_constant_buffer(self.layout_index, self.buffer.clone());
    }
}
