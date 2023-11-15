use crate::lps::core::gpu::{GpuApi};
use crate::lps::rasterize::render_cmds::render_cmd::{RenderCmd, RenderCommandType};

pub struct Draw {}

impl<'a> Draw {
    pub fn new() -> Draw  {
        Draw {}
    }
}

impl<'a> RenderCmd<'a>  for Draw  {
    fn cmd_type(&self) -> RenderCommandType {
        RenderCommandType::Draw
    }

    fn execute(&self, gpu_buffer: &'a mut dyn GpuApi<'a>) {
        gpu_buffer.draw();
    }
}