use crate::lps::core::gpu::GpuApi;

pub enum RenderCommandType {
    SetVertexBuffer = 0,
    SetRenderTarget = 1,
    Draw = 2,
    SetConstantBuffer = 3,
}

pub trait RenderCmd<'a>: Send {
    fn cmd_type(&self) -> RenderCommandType;
    fn execute(&self, gpu_api: &'a mut dyn GpuApi<'a>);
}
