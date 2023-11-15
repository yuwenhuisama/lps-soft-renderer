use crate::lps::core::gpu::GpuApi;

#[derive(Debug)]
pub enum RenderCommandType {
    SetVertexBuffer = 0,
    SetRenderTarget = 1,
    Draw = 2,
    SetConstantBuffer = 3,
}

pub trait RenderCmd: Send {
    fn cmd_type(&self) -> RenderCommandType;
    fn execute(&self, gpu_api: &mut (dyn GpuApi + Sync + Send));
}
