use crate::lps::common::math::mat4x4::Mat4x4;
use crate::lps::common::math::vec4::Vec4;
use crate::lps::core::gpu::GpuApi;
use crate::lps::rasterize::render_cmds::render_cmd::{RenderCmd, RenderCommandType};
use std::any::Any;
use std::sync::Arc;

pub struct SetConstantBufferCmd {
    pub layout_index: usize,
    pub buffer: Arc<dyn Any + Send + Sync>,
}

impl SetConstantBufferCmd {
    pub fn new_with_mat4x4(layout_index: usize, buffer: Mat4x4) -> SetConstantBufferCmd {
        SetConstantBufferCmd {
            layout_index,
            buffer: Arc::new(buffer),
        }
    }

    pub fn new_with_vec4(layout_index: usize, buffer: Vec4) -> SetConstantBufferCmd {
        SetConstantBufferCmd {
            layout_index,
            buffer: Arc::new(buffer),
        }
    }
}

impl RenderCmd for SetConstantBufferCmd {
    fn cmd_type(&self) -> RenderCommandType {
        RenderCommandType::SetConstantBuffer
    }

    fn execute(&self, gpu_api: &mut (dyn GpuApi + Sync + Send)) {
        gpu_api.set_constant_buffer(self.layout_index, Arc::clone(&self.buffer));
    }
}
